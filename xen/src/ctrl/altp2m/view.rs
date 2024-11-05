use xen_sys::{
    xc_altp2m_change_gfn, xc_altp2m_create_view, xc_altp2m_destroy_view, xc_altp2m_get_mem_access,
    xc_altp2m_set_mem_access, xc_altp2m_set_mem_access_multi, xc_altp2m_switch_to_view,
};

use crate::{ctrl::XenInterface, xc_check_error, MemoryAccess, XenDomainId, XenError};

pub struct XenAltP2MView {
    interface: XenInterface,
    domain_id: XenDomainId,
    view_id: u16,
}

impl XenAltP2MView {
    pub(crate) fn new(
        interface: XenInterface,
        domain_id: XenDomainId,
        default_access: MemoryAccess,
    ) -> Result<Self, XenError> {
        let mut view_id = 0;
        let rc = unsafe {
            xc_altp2m_create_view(
                interface.handle.0,
                domain_id.0,
                default_access.bits().into(),
                &mut view_id,
            )
        };

        if rc < 0 {
            return Err(XenError::Io(std::io::Error::last_os_error()));
        }

        tracing::trace!(domain_id = domain_id.0, view_id, "created altp2m view");

        Ok(Self {
            interface,
            domain_id,
            view_id,
        })
    }

    pub fn id(&self) -> u16 {
        self.view_id
    }

    pub fn switch(&self) -> Result<(), XenError> {
        let rc = unsafe {
            xc_altp2m_switch_to_view(self.interface.handle.0, self.domain_id.0, self.view_id)
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn get_mem_access(&self, gfn: u64) -> Result<MemoryAccess, XenError> {
        let mut access = 0;
        let rc = unsafe {
            xc_altp2m_get_mem_access(
                self.interface.handle.0,
                self.domain_id.0,
                self.view_id,
                gfn,
                &mut access,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(MemoryAccess::from_bits_truncate(access as _))
    }

    pub fn set_mem_access(&self, gfn: u64, access: MemoryAccess) -> Result<(), XenError> {
        let rc = unsafe {
            xc_altp2m_set_mem_access(
                self.interface.handle.0,
                self.domain_id.0,
                self.view_id,
                gfn,
                access.bits().into(),
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn set_mem_access_multi(
        &self,
        access: &[MemoryAccess],
        gfns: &[u64],
    ) -> Result<(), XenError> {
        let rc = unsafe {
            xc_altp2m_set_mem_access_multi(
                self.interface.handle.0,
                self.domain_id.0,
                self.view_id,
                access.as_ptr() as *mut u8,
                gfns.as_ptr() as *mut u64,
                std::cmp::min(access.len(), gfns.len()) as u32,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn change_gfn(&self, old_gfn: u64, new_gfn: u64) -> Result<(), XenError> {
        let rc = unsafe {
            xc_altp2m_change_gfn(
                self.interface.handle.0,
                self.domain_id.0,
                self.view_id,
                old_gfn,
                new_gfn,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }
}

impl Drop for XenAltP2MView {
    fn drop(&mut self) {
        tracing::trace!(
            domain_id = self.domain_id.0,
            view_id = self.view_id,
            "destroying altp2m view"
        );

        unsafe {
            xc_altp2m_destroy_view(self.interface.handle.0, self.domain_id.0, self.view_id);
        }
    }
}
