mod altp2m;
pub use self::altp2m::{XenAltP2M, XenAltP2MView};

mod domain;
pub use self::domain::{XenDomain, XenDomainInfo};

mod event;
pub use self::event::{
    VmEvent, VmEventCpuid, VmEventCtrlReg, VmEventData, VmEventDebug, VmEventDescriptorAccess,
    VmEventEmulInsnData, VmEventEmulReadData, VmEventFastSinglestep, VmEventFlag,
    VmEventFlagOptions, VmEventInterrupt, VmEventIo, VmEventMemAccess, VmEventMovToMsr,
    VmEventPaging, VmEventReason, VmEventRegs, VmEventRegsX86, VmEventSelectorReg, VmEventSharing,
    VmEventSinglestep, VmEventVmExit, VmEventWriteCtrlReg,
};

mod handle;
pub use self::handle::XenInterfaceHandle;

mod interface;
pub use self::interface::XenInterface;

mod monitor;
use xen_sys::{evtchn_status, xc_evtchn_status};

pub use self::monitor::{VmEventRing, XenMonitor};
use crate::{Architecture, XenDomainId, XenError, XenEventChannelStatus};

pub struct XenControl {
    interface: XenInterface,
}

impl XenControl {
    pub fn new() -> Result<Self, XenError> {
        Ok(Self {
            interface: XenInterface::new()?,
        })
    }

    pub fn attach(interface: XenInterface) -> Result<Self, XenError> {
        Ok(Self { interface })
    }

    pub fn domain<Arch>(&self, id: XenDomainId) -> Result<XenDomain<Arch>, XenError>
    where
        Arch: Architecture,
    {
        XenDomain::new(self.interface.clone(), id)
    }

    /// Returns the status of a domain's event channel port.
    ///
    /// Port numbers are separate for each domain. Pass [`XenDomainId::SELF`]
    /// for a port of the calling domain.
    ///
    /// # Notes
    ///
    /// Only a privileged domain may query the ports of another domain. A
    /// failure returns [`XenError::Io`] with the error number from Xen.
    pub fn evtchn_status(
        &self,
        domain_id: XenDomainId,
        port: u32,
    ) -> Result<XenEventChannelStatus, XenError> {
        let mut status = evtchn_status {
            dom: domain_id.0 as _,
            port,
            ..Default::default()
        };
        let rc = unsafe { xc_evtchn_status(self.interface.handle.0, &mut status) };

        // xc_evtchn_status() reports a failure only through errno. The error
        // details that xc_check_error! reads stay empty.
        if rc < 0 {
            return Err(XenError::Io(std::io::Error::last_os_error()));
        }

        status.try_into()
    }
}
