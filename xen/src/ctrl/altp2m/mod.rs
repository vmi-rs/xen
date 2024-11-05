mod view;
pub use self::view::XenAltP2MView;
use crate::{ctrl::XenInterface, xc_check_error, MemoryAccess, XenDomainId, XenError};

pub struct XenAltP2M {
    interface: XenInterface,
    domain_id: XenDomainId,
}

impl XenAltP2M {
    pub(crate) fn new(interface: XenInterface, domain_id: XenDomainId) -> Result<Self, XenError> {
        let rc =
            unsafe { xen_sys::xc_altp2m_set_domain_state(interface.handle.0, domain_id.0, true) };
        xc_check_error!(interface.handle.0, rc);
        Ok(Self {
            interface,
            domain_id,
        })
    }

    pub fn create_view(&self, default_access: MemoryAccess) -> Result<XenAltP2MView, XenError> {
        XenAltP2MView::new(self.interface.clone(), self.domain_id, default_access)
    }

    pub fn reset_view(&self) -> Result<(), XenError> {
        let rc = unsafe {
            xen_sys::xc_altp2m_switch_to_view(self.interface.handle.0, self.domain_id.0, 0)
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }
}

impl Drop for XenAltP2M {
    fn drop(&mut self) {
        tracing::trace!(?self.domain_id, "disabling altp2m");
        let _ = self.reset_view();
        unsafe {
            xen_sys::xc_altp2m_set_domain_state(self.interface.handle.0, self.domain_id.0, false);
        }
    }
}
