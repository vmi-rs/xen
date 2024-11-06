use xen_sys::{xc_interface, xc_interface_close, xc_interface_open, xentoollog_logger};

use crate::XenError;

#[derive(Debug, Clone)]
pub struct XenInterfaceHandle(pub(crate) *mut xc_interface);

impl XenInterfaceHandle {
    pub fn new() -> Result<Self, XenError> {
        Self::new_with_options(None, None, 0)
    }

    pub fn new_with_options(
        logger: Option<&mut xentoollog_logger>,
        dombuild_logger: Option<&mut xentoollog_logger>,
        flags: u32,
    ) -> Result<Self, XenError> {
        let handle = unsafe {
            xc_interface_open(
                logger.map_or_else(std::ptr::null_mut, |p| p as *mut _),
                dombuild_logger.map_or_else(std::ptr::null_mut, |p| p as *mut _),
                flags,
            )
        };

        if handle.is_null() {
            return Err(XenError::Other("Failed to open Xen control interface"));
        }

        Ok(Self(handle))
    }
}

impl Drop for XenInterfaceHandle {
    fn drop(&mut self) {
        tracing::trace!("closing Xen control interface");
        unsafe {
            xc_interface_close(self.0);
        }
    }
}
