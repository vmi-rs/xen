use xen_sys::{
    hvm_hw_cpu, hvm_hw_lapic, hvm_hw_lapic_regs, xc_domain_hvm_getcontext,
    xc_domain_hvm_getcontext_partial, xc_domain_hvm_setcontext,
};

use super::{
    XenHvmContext, XenHvmSaveRecord,
    x86::{Amd64, LocalApic, LocalApicRegisters, Registers},
};
use crate::{VcpuId, XenDomain, XenError, xc_check_error};

impl XenDomain<Amd64> {
    pub fn get_context_cpu(&self, vcpu: VcpuId) -> Result<Registers, XenError> {
        Ok(self.get_context_partial::<hvm_hw_cpu>(vcpu)?.into())
    }

    pub fn get_context_lapic(&self, vcpu: VcpuId) -> Result<LocalApic, XenError> {
        Ok(self.get_context_partial::<hvm_hw_lapic>(vcpu)?.into())
    }

    pub fn get_context_lapic_regs(&self, vcpu: VcpuId) -> Result<LocalApicRegisters, XenError> {
        Ok(self.get_context_partial::<hvm_hw_lapic_regs>(vcpu)?.into())
    }

    pub fn set_context_cpu(&self, vcpu: VcpuId, registers: Registers) -> Result<(), XenError> {
        self.pause()?;
        let result = self.load_context_cpu(vcpu, registers);

        // The domain must not stay paused when the load fails, so it is
        // unpaused either way. If both fail, the load's error is returned,
        // because it is the cause.
        result.and(self.unpause())
    }

    /// Returns the domain's whole HVM context.
    ///
    /// The context holds the registers of every vCPU, the local APICs and the
    /// timers, the same records that Xen uses to save and migrate a domain.
    /// Change it with `XenHvmContext::update` and load it back with
    /// `set_context`.
    ///
    /// # Notes
    ///
    /// Pause the domain before this call and unpause it after `set_context`.
    /// Otherwise the guest keeps running, and loading the context undoes what
    /// it did in between.
    ///
    /// Each call moves the guest's ACPI PM timer forward. When Xen saves the
    /// timer, it adds the time since the timer's last update to the counter,
    /// and the next update adds the same time again.
    pub fn get_context(&self) -> Result<XenHvmContext, XenError> {
        // With a null buffer, `xc_domain_hvm_getcontext` returns the size that
        // the context needs.
        let size = unsafe {
            xc_domain_hvm_getcontext(
                self.interface.handle.0,
                self.domain_id.0,
                std::ptr::null_mut(),
                0,
            )
        };
        xc_check_error!(self.interface.handle.0, size);

        let mut buffer = vec![0u8; size as usize];
        let length = unsafe {
            xc_domain_hvm_getcontext(
                self.interface.handle.0,
                self.domain_id.0,
                buffer.as_mut_ptr(),
                buffer.len() as u32,
            )
        };
        xc_check_error!(self.interface.handle.0, length);

        buffer.truncate(length as usize);
        Ok(XenHvmContext::new(buffer))
    }

    /// Loads a changed context into the domain.
    ///
    /// Xen replaces the state of the domain's virtual hardware with the records
    /// in `context`, for every vCPU and device at once.
    ///
    /// # Notes
    ///
    /// A vCPU can be in the middle of a device access that the device model,
    /// usually QEMU, emulates. Loading the vCPU's registers throws that access
    /// away, and the guest never gets its answer. A lost disk access can hang
    /// the guest's disk until the guest resets the disk controller. To avoid
    /// it, load only while [`XenIoreqPage::pending`] returns `false`.
    ///
    /// [`XenIoreqPage::pending`]: crate::devicemodel::XenIoreqPage::pending
    pub fn set_context(&self, context: &XenHvmContext) -> Result<(), XenError> {
        let bytes = context.as_bytes();

        // `xc_domain_hvm_setcontext` takes a mutable pointer, but it only
        // copies the bytes into a buffer for the hypercall. The cast from the
        // shared slice is therefore safe.
        let rc = unsafe {
            xc_domain_hvm_setcontext(
                self.interface.handle.0,
                self.domain_id.0,
                bytes.as_ptr() as *mut u8,
                bytes.len() as u32,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    /// Returns one record of a vCPU.
    ///
    /// Xen copies only that record, which is cheaper than reading the whole
    /// context with `get_context`. `T` picks the record, such as `hvm_hw_cpu`
    /// for the registers.
    fn get_context_partial<T>(&self, vcpu: VcpuId) -> Result<T, XenError>
    where
        T: XenHvmSaveRecord + Default,
    {
        let mut record = T::default();

        // SAFETY: The last argument tells Xen how big `record` is, and Xen
        // fails instead of writing more. The sealed `XenHvmSaveRecord` limits
        // `T` to Xen's record structs. These structs hold only integers.
        // Whatever bytes Xen writes therefore leave a valid `T`.
        let rc = unsafe {
            xc_domain_hvm_getcontext_partial(
                self.interface.handle.0,
                self.domain_id.0,
                T::typecode(),
                vcpu.0,
                &mut record as *mut T as *mut _,
                size_of::<T>() as u32,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(record)
    }

    /// Sets the registers of one vCPU.
    ///
    /// Xen loads registers only as part of a whole context, so this reads the
    /// context, changes the vCPU's `hvm_hw_cpu` record and loads the context
    /// back. Returns an error if the context has no record for `vcpu`. The
    /// caller pauses the domain around it.
    fn load_context_cpu(&self, vcpu: VcpuId, registers: Registers) -> Result<(), XenError> {
        let mut context = self.get_context()?;

        let mut found = false;
        context.update(|instance, cpu: &mut hvm_hw_cpu| {
            if instance == vcpu.0 {
                registers.copy_into(cpu);
                found = true;
            }
        });

        if !found {
            return Err(XenError::Other(
                "Failed to find the CPU context of the vCPU",
            ));
        }

        self.set_context(&context)
    }
}
