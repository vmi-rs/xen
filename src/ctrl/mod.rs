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
pub use self::monitor::{VmEventRing, XenMonitor};
use crate::{Architecture, XenDomainId, XenError};

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
}
