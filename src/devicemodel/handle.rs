use xen_sys::{
    xendevicemodel_close, xendevicemodel_handle, xendevicemodel_open, xentoollog_logger,
};

use crate::XenError;

#[derive(Debug, Clone)]
pub struct XenDeviceModelHandle(pub(crate) *mut xendevicemodel_handle);

impl XenDeviceModelHandle {
    pub fn new() -> Result<Self, XenError> {
        Self::new_with_options(None, 0)
    }

    pub fn new_with_options(
        logger: Option<&mut xentoollog_logger>,
        flags: u32,
    ) -> Result<Self, XenError> {
        let handle = unsafe {
            xendevicemodel_open(
                logger.map_or_else(std::ptr::null_mut, |p| p as *mut _),
                flags,
            )
        };

        if handle.is_null() {
            return Err(XenError::Other("Failed to open Xen event channel"));
        }

        Ok(Self(handle))
    }
}

impl Drop for XenDeviceModelHandle {
    fn drop(&mut self) {
        tracing::trace!("closing Xen foreign memory handle");
        unsafe {
            xendevicemodel_close(self.0);
        }
    }
}
