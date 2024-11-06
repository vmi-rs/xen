mod handle;
use std::rc::Rc;

use xen_sys::xendevicemodel_inject_event;

pub use self::handle::XenDeviceModelHandle;
use crate::{VcpuId, XenDomainId, XenError};

macro_rules! xc_check_error {
    ($rc:ident) => {
        if $rc < 0 {
            return Err(XenError::Io(std::io::Error::last_os_error()));
        }
    };
}

/*
 * x86 event types. This enumeration is valid for:
 *  Intel VMX: {VM_ENTRY,VM_EXIT,IDT_VECTORING}_INTR_INFO[10:8]
 *  AMD SVM: eventinj[10:8] and exitintinfo[10:8] (types 0-4 only)
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum XenX86EventType {
    /// External interrupt
    ExternalInterrupt,

    /// Reserved
    Reserved,

    /// NMI
    Nmi,

    /// Hardware exception
    HardwareException,

    /// Software interrupt (CD nn)
    SoftwareInterrupt,

    /// ICEBP (F1)
    PrivilegedSoftwareException,

    /// INT3 (CC), INTO (CE)
    SoftwareException,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum XenX86ExceptionVector {
    DivideError = 0,
    DebugException = 1,
    Nmi = 2,
    Breakpoint = 3,
    Overflow = 4,
    BoundRange = 5,
    InvalidOpcode = 6,
    DeviceNotAvailable = 7,
    DoubleFault = 8,
    CoprocessorSegmentOverrun = 9,
    InvalidTss = 10,
    SegmentNotPresent = 11,
    StackSegmentFault = 12,
    GeneralProtectionFault = 13,
    PageFault = 14,
    PicSpuriousInterruptVector = 15,
    MathsFault = 16,
    AlignmentCheck = 17,
    MachineCheck = 18,
    SimdException = 19,
    VirtualisationException = 20,
    ControlFlowProtection = 21,
    HypervisorInjection = 28,
    VmmCommunication = 29,
    SecurityException = 30,
}

#[derive(Debug, Clone)]
pub struct XenDeviceModel {
    pub(crate) handle: Rc<XenDeviceModelHandle>,
    domain_id: XenDomainId,
}

impl XenDeviceModel {
    pub(crate) fn new(domain_id: XenDomainId) -> Result<Self, XenError> {
        Ok(Self {
            handle: Rc::new(XenDeviceModelHandle::new()?),
            domain_id,
        })
    }

    /// This function injects an event into a vCPU to take effect the next time
    /// it resumes.
    ///
    /// Set `error_code` to `!0` to skip.
    ///
    /// Set `extra` to type-specific extra data (`%cr2` for `#PF`, `pending_dbg`
    /// for `#DB`).
    pub fn inject_event(
        &self,
        vcpu: VcpuId,
        vector: XenX86ExceptionVector,
        event_type: XenX86EventType,
        error_code: u32,
        instruction_length: u8,
        extra: u64,
    ) -> Result<(), XenError> {
        let rc = unsafe {
            xendevicemodel_inject_event(
                self.handle.0,
                self.domain_id.0 as _,
                vcpu.0 as _,
                vector as _,
                event_type as _,
                error_code,
                instruction_length,
                extra,
            )
        };
        xc_check_error!(rc);
        Ok(())
    }
}
