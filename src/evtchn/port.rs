use std::os::fd::{AsFd, AsRawFd, BorrowedFd, RawFd};

use xen_sys::{
    xenevtchn_bind_interdomain, xenevtchn_notify, xenevtchn_pending, xenevtchn_unbind,
    xenevtchn_unmask,
};

use super::XenEventChannel;
use crate::{XenDomainId, XenError};

macro_rules! xc_check_error {
    ($self:ident, $rc:ident) => {
        if $rc < 0 {
            return Err(XenError::Io(std::io::Error::last_os_error()));
        }
    };
}

#[derive(Debug, Clone)]
pub struct XenEventChannelPort {
    evtchn: XenEventChannel,
    remote_port: u32,
    local_port: u32,
}

impl XenEventChannelPort {
    pub(crate) fn bind_interdomain(
        domain_id: XenDomainId,
        remote_port: u32,
    ) -> Result<Self, XenError> {
        let evtchn = XenEventChannel::new()?;
        let rc = unsafe { xenevtchn_bind_interdomain(*evtchn.0, domain_id.0, remote_port) };
        xc_check_error!(self, rc);

        let local_port = rc as u32;
        Ok(Self {
            evtchn,
            remote_port,
            local_port,
        })
    }

    pub fn local_port(&self) -> u32 {
        self.local_port
    }

    pub fn remote_port(&self) -> u32 {
        self.remote_port
    }

    pub fn wait(&self) -> Result<(), XenError> {
        let port = self.pending()?;
        assert_eq!(port, self.local_port);
        self.unmask()?;
        Ok(())
    }

    pub fn notify(&self) -> Result<(), XenError> {
        let rc = unsafe { xenevtchn_notify(*self.evtchn.0, self.local_port) };
        xc_check_error!(self, rc);
        Ok(())
    }

    fn pending(&self) -> Result<u32, XenError> {
        let rc = unsafe { xenevtchn_pending(*self.evtchn.0) };
        xc_check_error!(self, rc);

        let port = rc as u32;
        Ok(port)
    }

    fn unmask(&self) -> Result<(), XenError> {
        let rc = unsafe { xenevtchn_unmask(*self.evtchn.0, self.local_port) };
        xc_check_error!(self, rc);
        Ok(())
    }
}

impl Drop for XenEventChannelPort {
    fn drop(&mut self) {
        tracing::trace!(
            local_port = self.local_port,
            remote_port = self.remote_port,
            "unbinding Xen event channel port"
        );
        unsafe {
            xenevtchn_unbind(*self.evtchn.0, self.local_port);
        }
    }
}

impl AsFd for XenEventChannelPort {
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
    }
}

impl AsRawFd for XenEventChannelPort {
    fn as_raw_fd(&self) -> RawFd {
        self.evtchn.as_raw_fd()
    }
}
