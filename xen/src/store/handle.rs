use xen_sys::{xs_close, xs_handle, xs_open};

use crate::XenError;

#[derive(Debug, Clone)]
pub struct XenStoreHandle(pub(crate) *mut xs_handle);

impl XenStoreHandle {
    pub fn new() -> Result<Self, XenError> {
        let handle = unsafe { xs_open(0) };

        if handle.is_null() {
            return Err(XenError::Other("Failed to open xen store"));
        }

        Ok(Self(handle))
    }
}

impl Drop for XenStoreHandle {
    fn drop(&mut self) {
        unsafe {
            xs_close(self.0);
        }
    }
}
