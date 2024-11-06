use xen_sys::{
    hvm_hw_cpu, hvm_save_descriptor, xc_domain_hvm_getcontext, xc_domain_hvm_getcontext_partial,
    xc_domain_hvm_setcontext, __HVM_SAVE_TYPE_CPU,
};

use super::x86::{Amd64, Registers};
use crate::{xc_check_error, VcpuId, XenDomain, XenError};

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
}
