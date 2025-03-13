mod ring;
use xen_sys::{
    vm_event_back_ring, xc_monitor_cpuid, xc_monitor_debug_exceptions,
    xc_monitor_descriptor_access, xc_monitor_disable, xc_monitor_emul_unimplemented,
    xc_monitor_emulate_each_rep, xc_monitor_enable, xc_monitor_get_capabilities,
    xc_monitor_guest_request, xc_monitor_inguest_pagefault, xc_monitor_io, xc_monitor_mov_to_msr,
    xc_monitor_privileged_call, xc_monitor_resume, xc_monitor_singlestep,
    xc_monitor_software_breakpoint, xc_monitor_vmexit, xc_monitor_write_ctrlreg,
};

pub use self::ring::VmEventRing;
use crate::{
    BACK_RING_INIT, SHARED_RING_INIT, XenDomainId,
    consts::PAGE_SIZE,
    ctrl::{VmEventCtrlReg, XenInterface},
    error::{XcError, XenError},
    evtchn::XenEventChannelPort,
    xc_check_error,
};

pub struct XenMonitor {
    interface: XenInterface,
    domain_id: XenDomainId,
    port: u32,
}

impl XenMonitor {
    pub(crate) fn new(
        interface: XenInterface,
        domain_id: XenDomainId,
    ) -> Result<(Self, VmEventRing), XenError> {
        let mut port: u32 = 0;
        let ring_page = unsafe { xc_monitor_enable(interface.handle.0, domain_id.0, &mut port) };

        if ring_page.is_null() {
            return Err(XcError::new(-1, 0, "Failed to enable monitor").into());
        }

        SHARED_RING_INIT!(ring_page);

        let mut back_ring = unsafe { std::mem::zeroed::<vm_event_back_ring>() };
        BACK_RING_INIT!(back_ring, ring_page, PAGE_SIZE);

        Ok((
            Self {
                interface,
                domain_id,
                port,
            },
            VmEventRing::new(ring_page, back_ring),
        ))
    }

    pub fn channel(&self) -> Result<XenEventChannelPort, XenError> {
        XenEventChannelPort::bind_interdomain(self.domain_id, self.port)
    }

    pub fn port(&self) -> u32 {
        self.port
    }

    pub fn resume(&self) -> Result<(), XenError> {
        let rc = unsafe { xc_monitor_resume(self.interface.handle.0, self.domain_id.0) };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn get_capabilities(&self) -> Result<u32, XenError> {
        let mut capabilities = 0;
        let rc = unsafe {
            xc_monitor_get_capabilities(
                self.interface.handle.0,
                self.domain_id.0,
                &mut capabilities,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(capabilities)
    }

    pub fn write_ctrlreg(
        &self,
        index: VmEventCtrlReg,
        enable: bool,
        sync: bool,
        bitmask: u64,
        onchangeonly: bool,
    ) -> Result<(), XenError> {
        let rc = unsafe {
            xc_monitor_write_ctrlreg(
                self.interface.handle.0,
                self.domain_id.0,
                index as u16,
                enable,
                sync,
                bitmask,
                onchangeonly,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn mov_to_msr(&self, msr: u32, enable: bool, onchangeonly: bool) -> Result<(), XenError> {
        let rc = unsafe {
            xc_monitor_mov_to_msr(
                self.interface.handle.0,
                self.domain_id.0,
                msr,
                enable,
                onchangeonly,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn singlestep(&self, singlestep: bool) -> Result<(), XenError> {
        let rc =
            unsafe { xc_monitor_singlestep(self.interface.handle.0, self.domain_id.0, singlestep) };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn software_breakpoint(&self, enable: bool) -> Result<(), XenError> {
        let rc = unsafe {
            xc_monitor_software_breakpoint(self.interface.handle.0, self.domain_id.0, enable)
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn descriptor_access(&self, enable: bool) -> Result<(), XenError> {
        let rc = unsafe {
            xc_monitor_descriptor_access(self.interface.handle.0, self.domain_id.0, enable)
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn guest_request(
        &self,
        enable: bool,
        sync: bool,
        allow_userspace: bool,
    ) -> Result<(), XenError> {
        let rc = unsafe {
            xc_monitor_guest_request(
                self.interface.handle.0,
                self.domain_id.0,
                enable,
                sync,
                allow_userspace,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn inguest_pagefault(&self, disable: bool) -> Result<(), XenError> {
        let rc = unsafe {
            xc_monitor_inguest_pagefault(self.interface.handle.0, self.domain_id.0, disable)
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn debug_exceptions(&self, enable: bool, sync: bool) -> Result<(), XenError> {
        let rc = unsafe {
            xc_monitor_debug_exceptions(self.interface.handle.0, self.domain_id.0, enable, sync)
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn cpuid(&self, enable: bool) -> Result<(), XenError> {
        let rc = unsafe { xc_monitor_cpuid(self.interface.handle.0, self.domain_id.0, enable) };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn privileged_call(&self, enable: bool) -> Result<(), XenError> {
        let rc = unsafe {
            xc_monitor_privileged_call(self.interface.handle.0, self.domain_id.0, enable)
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn emul_unimplemented(&self, enable: bool) -> Result<(), XenError> {
        let rc = unsafe {
            xc_monitor_emul_unimplemented(self.interface.handle.0, self.domain_id.0, enable)
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn vmexit(&self, enable: bool, sync: bool) -> Result<(), XenError> {
        let rc =
            unsafe { xc_monitor_vmexit(self.interface.handle.0, self.domain_id.0, enable, sync) };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn io(&self, enable: bool) -> Result<(), XenError> {
        let rc = unsafe { xc_monitor_io(self.interface.handle.0, self.domain_id.0, enable) };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn emulate_each_rep(&self, enable: bool) -> Result<(), XenError> {
        let rc = unsafe {
            xc_monitor_emulate_each_rep(self.interface.handle.0, self.domain_id.0, enable)
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }
}

impl Drop for XenMonitor {
    fn drop(&mut self) {
        tracing::trace!(?self.domain_id, "disabling monitor");
        unsafe {
            xc_monitor_disable(self.interface.handle.0, self.domain_id.0);
        }
    }
}
