mod port;
use std::{
    os::fd::{AsFd, AsRawFd, BorrowedFd, RawFd},
    rc::Rc,
};

use xen_sys::{xenevtchn_close, xenevtchn_fd, xenevtchn_handle, xenevtchn_open, xentoollog_logger};

pub use self::port::XenEventChannelPort;
use crate::XenError;

#[derive(Debug, Clone)]
pub struct XenEventChannel(pub(crate) Rc<*mut xenevtchn_handle>);

impl XenEventChannel {
    pub fn new() -> Result<Self, XenError> {
        Self::new_with_options(None, 0)
    }

    pub fn new_with_options(
        logger: Option<&mut xentoollog_logger>,
        flags: u32,
    ) -> Result<Self, XenError> {
        let handle = unsafe {
            xenevtchn_open(
                logger.map_or_else(std::ptr::null_mut, |p| p as *mut _),
                flags,
            )
        };

        if handle.is_null() {
            return Err(XenError::Other("Failed to open Xen event channel"));
        }

        Ok(Self(Rc::new(handle)))
    }
}

impl Drop for XenEventChannel {
    fn drop(&mut self) {
        tracing::trace!("closing Xen event channel");
        unsafe {
            xenevtchn_close(*self.0);
        }
    }
}

impl AsFd for XenEventChannel {
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
    }
}

impl AsRawFd for XenEventChannel {
    fn as_raw_fd(&self) -> RawFd {
        unsafe { xenevtchn_fd(*self.0) }
    }
}
