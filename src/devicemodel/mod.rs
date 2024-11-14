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

/// Exception vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XenX86ExceptionVector(pub u8);

#[allow(non_upper_case_globals)]
impl XenX86ExceptionVector {
    pub const DivideError: Self = Self(0);
    pub const DebugException: Self = Self(1);
    pub const Nmi: Self = Self(2);
    pub const Breakpoint: Self = Self(3);
    pub const Overflow: Self = Self(4);
    pub const BoundRange: Self = Self(5);
    pub const InvalidOpcode: Self = Self(6);
    pub const DeviceNotAvailable: Self = Self(7);
    pub const DoubleFault: Self = Self(8);
    pub const CoprocessorSegmentOverrun: Self = Self(9);
    pub const InvalidTss: Self = Self(10);
    pub const SegmentNotPresent: Self = Self(11);
    pub const StackSegmentFault: Self = Self(12);
    pub const GeneralProtectionFault: Self = Self(13);
    pub const PageFault: Self = Self(14);
    pub const PicSpuriousInterruptVector: Self = Self(15);
    pub const MathsFault: Self = Self(16);
    pub const AlignmentCheck: Self = Self(17);
    pub const MachineCheck: Self = Self(18);
    pub const SimdException: Self = Self(19);
    pub const VirtualisationException: Self = Self(20);
    pub const ControlFlowProtection: Self = Self(21);
    pub const HypervisorInjection: Self = Self(28);
    pub const VmmCommunication: Self = Self(29);
    pub const SecurityException: Self = Self(30);
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
                vector.0,
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
