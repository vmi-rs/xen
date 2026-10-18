use xen_sys::{
    __HVM_SAVE_TYPE_CPU, __HVM_SAVE_TYPE_LAPIC, __HVM_SAVE_TYPE_LAPIC_REGS, hvm_hw_cpu,
    hvm_hw_lapic, hvm_hw_lapic_regs, hvm_save_descriptor, xc_domain_hvm_getcontext,
    xc_domain_hvm_getcontext_partial, xc_domain_hvm_setcontext,
};

use super::{
    XenHvmContext,
    x86::{Amd64, LocalApic, LocalApicRegisters, Registers},
};
use crate::{VcpuId, XenDomain, XenError, xc_check_error};

impl XenDomain<Amd64> {
    pub fn get_context_cpu(&self, vcpu: VcpuId) -> Result<Registers, XenError> {
        let result = unsafe { std::mem::zeroed::<hvm_hw_cpu>() };

        let rc = unsafe {
            xc_domain_hvm_getcontext_partial(
                self.interface.handle.0,
                self.domain_id.0,
                size_of_val(&__HVM_SAVE_TYPE_CPU::default().c) as u16,
                vcpu.0,
                &result as *const _ as *mut _,
                size_of_val(&result) as u32,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(result.into())
    }

    pub fn get_context_lapic(&self, vcpu: VcpuId) -> Result<LocalApic, XenError> {
        let result = unsafe { std::mem::zeroed::<hvm_hw_lapic>() };

        let rc = unsafe {
            xc_domain_hvm_getcontext_partial(
                self.interface.handle.0,
                self.domain_id.0,
                size_of_val(&__HVM_SAVE_TYPE_LAPIC::default().c) as u16,
                vcpu.0,
                &result as *const _ as *mut _,
                size_of_val(&result) as u32,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(result.into())
    }

    pub fn get_context_lapic_regs(&self, vcpu: VcpuId) -> Result<LocalApicRegisters, XenError> {
        let result = unsafe { std::mem::zeroed::<hvm_hw_lapic_regs>() };

        let rc = unsafe {
            xc_domain_hvm_getcontext_partial(
                self.interface.handle.0,
                self.domain_id.0,
                size_of_val(&__HVM_SAVE_TYPE_LAPIC_REGS::default().c) as u16,
                vcpu.0,
                &result as *const _ as *mut _,
                size_of_val(&result) as u32,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(result.into())
    }

    pub fn set_context_cpu(&self, vcpu: VcpuId, registers: Registers) -> Result<(), XenError> {
        self.pause()?;

        // Get the context size.
        let size = unsafe {
            xc_domain_hvm_getcontext(
                self.interface.handle.0,
                self.domain_id.0,
                std::ptr::null_mut(),
                0,
            )
        };

        if size <= 0 {
            self.unpause()?;
            return Err(XenError::Other("Failed to get context size"));
        }

        // Allocate a buffer to hold the context.
        let mut buffer = vec![0u8; size as usize];

        // Get the context.
        let rc = unsafe {
            xc_domain_hvm_getcontext(
                self.interface.handle.0,
                self.domain_id.0,
                buffer.as_mut_ptr(),
                size as u32,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);

        // Locate the CPU context.
        let mut offset: u32 = 0;
        let size = size as u32;
        let hvm_save_code_cpu = size_of_val(&__HVM_SAVE_TYPE_CPU::default().c) as u16;

        unsafe {
            while offset < size {
                let descriptor =
                    buffer.as_ptr().offset(offset as isize) as *const hvm_save_descriptor;

                offset += size_of::<hvm_save_descriptor>() as u32;

                if (*descriptor).typecode == hvm_save_code_cpu && (*descriptor).instance == vcpu.0 {
                    let vcpu_context = buffer.as_ptr().offset(offset as isize) as *mut hvm_hw_cpu;
                    registers.copy_into(&mut *vcpu_context);
                    break;
                }

                offset += (*descriptor).length;
            }
        }

        // Set the context.
        let rc = unsafe {
            xc_domain_hvm_setcontext(
                self.interface.handle.0,
                self.domain_id.0,
                buffer.as_mut_ptr(),
                size,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);

        self.unpause()?;

        Ok(())
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
}
