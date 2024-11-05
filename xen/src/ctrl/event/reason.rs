use xen_sys::{
    vm_event_cpuid, vm_event_debug, vm_event_desc_access, vm_event_fast_singlestep,
    vm_event_interrupt_x86, vm_event_io, vm_event_mem_access, vm_event_mov_to_msr, vm_event_paging,
    vm_event_sharing, vm_event_singlestep, vm_event_vmexit, vm_event_write_ctrlreg,
    VM_EVENT_X86_CR0, VM_EVENT_X86_CR3, VM_EVENT_X86_CR4, VM_EVENT_X86_XCR0,
};

use crate::XenX86EventType;

#[derive(Debug)]
pub struct VmEventMemAccess {
    pub gfn: u64,
    pub offset: u64,
    pub gla: u64,
    pub flags: u32,
}

impl From<vm_event_mem_access> for VmEventMemAccess {
    fn from(value: vm_event_mem_access) -> Self {
        Self {
            gfn: value.gfn,
            offset: value.offset,
            gla: value.gla,
            flags: value.flags,
        }
    }
}

impl From<VmEventMemAccess> for vm_event_mem_access {
    fn from(value: VmEventMemAccess) -> Self {
        Self {
            gfn: value.gfn,
            offset: value.offset,
            gla: value.gla,
            flags: value.flags,
            _pad: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct VmEventSharing {
    pub gfn: u64,
    pub p2mt: u32,
}

impl From<vm_event_sharing> for VmEventSharing {
    fn from(value: vm_event_sharing) -> Self {
        Self {
            gfn: value.gfn,
            p2mt: value.p2mt,
        }
    }
}

impl From<VmEventSharing> for vm_event_sharing {
    fn from(value: VmEventSharing) -> Self {
        Self {
            gfn: value.gfn,
            p2mt: value.p2mt,
            _pad: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct VmEventPaging {
    pub gfn: u64,
    pub p2mt: u32,
    pub flags: u32,
}

impl From<vm_event_paging> for VmEventPaging {
    fn from(value: vm_event_paging) -> Self {
        Self {
            gfn: value.gfn,
            p2mt: value.p2mt,
            flags: value.flags,
        }
    }
}

impl From<VmEventPaging> for vm_event_paging {
    fn from(value: VmEventPaging) -> Self {
        Self {
            gfn: value.gfn,
            p2mt: value.p2mt,
            flags: value.flags,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum VmEventCtrlReg {
    Cr0 = VM_EVENT_X86_CR0,
    Cr3 = VM_EVENT_X86_CR3,
    Cr4 = VM_EVENT_X86_CR4,
    Xcr0 = VM_EVENT_X86_XCR0,
}

impl From<u32> for VmEventCtrlReg {
    fn from(value: u32) -> Self {
        match value {
            VM_EVENT_X86_CR0 => Self::Cr0,
            VM_EVENT_X86_CR3 => Self::Cr3,
            VM_EVENT_X86_CR4 => Self::Cr4,
            VM_EVENT_X86_XCR0 => Self::Xcr0,
            _ => Self::Cr0,
        }
    }
}

impl From<VmEventCtrlReg> for u32 {
    fn from(value: VmEventCtrlReg) -> Self {
        value as u32
    }
}

#[derive(Debug)]
pub struct VmEventWriteCtrlReg {
    pub index: VmEventCtrlReg,
    pub new_value: u64,
    pub old_value: u64,
}

impl From<vm_event_write_ctrlreg> for VmEventWriteCtrlReg {
    fn from(value: vm_event_write_ctrlreg) -> Self {
        Self {
            index: value.index.into(),
            new_value: value.new_value,
            old_value: value.old_value,
        }
    }
}

impl From<VmEventWriteCtrlReg> for vm_event_write_ctrlreg {
    fn from(value: VmEventWriteCtrlReg) -> Self {
        Self {
            index: value.index.into(),
            new_value: value.new_value,
            old_value: value.old_value,
            _pad: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct VmEventMovToMsr {
    pub msr: u64,
    pub new_value: u64,
    pub old_value: u64,
}

impl From<vm_event_mov_to_msr> for VmEventMovToMsr {
    fn from(value: vm_event_mov_to_msr) -> Self {
        Self {
            msr: value.msr,
            new_value: value.new_value,
            old_value: value.old_value,
        }
    }
}

impl From<VmEventMovToMsr> for vm_event_mov_to_msr {
    fn from(value: VmEventMovToMsr) -> Self {
        Self {
            msr: value.msr,
            new_value: value.new_value,
            old_value: value.old_value,
        }
    }
}

#[derive(Debug)]
pub struct VmEventDebug {
    pub gfn: u64,
    pub pending_dbg: u64, // Behaves like the VT-x PENDING_DBG field.
    pub insn_length: u32,
    pub typ: XenX86EventType,
}

impl From<vm_event_debug> for VmEventDebug {
    fn from(value: vm_event_debug) -> Self {
        Self {
            gfn: value.gfn,
            pending_dbg: value.pending_dbg,
            insn_length: value.insn_length,
            typ: unsafe { std::mem::transmute::<u8, XenX86EventType>(value.type_) },
        }
    }
}

impl From<VmEventDebug> for vm_event_debug {
    fn from(value: VmEventDebug) -> Self {
        Self {
            gfn: value.gfn,
            pending_dbg: value.pending_dbg,
            insn_length: value.insn_length,
            type_: value.typ as u8,
            _pad: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct VmEventSinglestep {
    pub gfn: u64,
}

impl From<vm_event_singlestep> for VmEventSinglestep {
    fn from(value: vm_event_singlestep) -> Self {
        Self { gfn: value.gfn }
    }
}

impl From<VmEventSinglestep> for vm_event_singlestep {
    fn from(value: VmEventSinglestep) -> Self {
        Self { gfn: value.gfn }
    }
}

#[derive(Debug)]
pub struct VmEventFastSinglestep {
    pub p2midx: u16,
}

impl From<vm_event_fast_singlestep> for VmEventFastSinglestep {
    fn from(value: vm_event_fast_singlestep) -> Self {
        Self {
            p2midx: value.p2midx,
        }
    }
}

impl From<VmEventFastSinglestep> for vm_event_fast_singlestep {
    fn from(value: VmEventFastSinglestep) -> Self {
        Self {
            p2midx: value.p2midx,
        }
    }
}

#[derive(Debug)]
pub struct VmEventCpuid {
    pub insn_length: u32,
    pub leaf: u32,
    pub subleaf: u32,
}

impl From<vm_event_cpuid> for VmEventCpuid {
    fn from(value: vm_event_cpuid) -> Self {
        Self {
            insn_length: value.insn_length,
            leaf: value.leaf,
            subleaf: value.subleaf,
        }
    }
}

impl From<VmEventCpuid> for vm_event_cpuid {
    fn from(value: VmEventCpuid) -> Self {
        Self {
            insn_length: value.insn_length,
            leaf: value.leaf,
            subleaf: value.subleaf,
            _pad: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct VmEventInterrupt {
    pub vector: u32,
    pub ty: u32,
    pub error_code: u32,
    pub cr2: u64,
}

impl From<vm_event_interrupt_x86> for VmEventInterrupt {
    fn from(value: vm_event_interrupt_x86) -> Self {
        Self {
            vector: value.vector,
            ty: value.type_,
            error_code: value.error_code,
            cr2: value.cr2,
        }
    }
}

impl From<VmEventInterrupt> for vm_event_interrupt_x86 {
    fn from(value: VmEventInterrupt) -> Self {
        Self {
            vector: value.vector,
            type_: value.ty,
            error_code: value.error_code,
            cr2: value.cr2,
            _pad: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct VmEventDescriptorAccess {
    pub instr_info: u32,         // VMX: VMCS Instruction-Information
    pub exit_qualification: u64, // VMX: VMCS Exit Qualification
    pub descriptor: u8,          // VM_EVENT_DESC_*
    pub is_write: u8,
}

impl From<vm_event_desc_access> for VmEventDescriptorAccess {
    fn from(value: vm_event_desc_access) -> Self {
        Self {
            instr_info: unsafe { value.arch.vmx.instr_info },
            exit_qualification: unsafe { value.arch.vmx.exit_qualification },
            descriptor: value.descriptor,
            is_write: value.is_write,
        }
    }
}

impl From<VmEventDescriptorAccess> for vm_event_desc_access {
    fn from(value: VmEventDescriptorAccess) -> Self {
        let mut result = Self::default();
        result.arch.vmx.instr_info = value.instr_info;
        result.arch.vmx.exit_qualification = value.exit_qualification;
        result.descriptor = value.descriptor;
        result.is_write = value.is_write;
        result
    }
}

#[derive(Debug)]
pub struct VmEventVmExit {
    pub reason: u64,
    pub qualification: u64,
}

impl From<vm_event_vmexit> for VmEventVmExit {
    fn from(value: vm_event_vmexit) -> Self {
        Self {
            reason: value.arch.vmx.reason,
            qualification: value.arch.vmx.qualification,
        }
    }
}

impl From<VmEventVmExit> for vm_event_vmexit {
    fn from(value: VmEventVmExit) -> Self {
        let mut result = Self::default();
        result.arch.vmx.reason = value.reason;
        result.arch.vmx.qualification = value.qualification;
        result
    }
}

#[derive(Debug)]
pub struct VmEventIo {
    pub bytes: u32,    // size of access
    pub port: u16,     // port number
    pub direction: u8, // direction (0 = OUT, 1 = IN)
    pub str: u8,       // string instruction (0 = not string, 1 = string)
}

impl From<vm_event_io> for VmEventIo {
    fn from(value: vm_event_io) -> Self {
        Self {
            bytes: value.bytes,
            port: value.port,
            direction: value.in_,
            str: value.str_,
        }
    }
}

impl From<VmEventIo> for vm_event_io {
    fn from(value: VmEventIo) -> Self {
        Self {
            bytes: value.bytes,
            port: value.port,
            in_: value.direction,
            str_: value.str,
        }
    }
}

#[derive(Debug)]
#[repr(u32)]
pub enum VmEventReason {
    /// Default case
    Unknown,

    /// Memory access violation
    MemoryAccess(VmEventMemAccess),

    /// Memory sharing event
    MemorySharing(VmEventSharing),

    /// Memory paging event
    MemoryPaging(VmEventPaging),

    /// A control register was updated
    WriteCtrlReg(VmEventWriteCtrlReg),

    /// An MSR was updated.
    MovToMsr(VmEventMovToMsr),

    /// Debug operation executed (e.g. int3)
    SoftwareBreakpoint(VmEventDebug),

    /// Single-step (e.g. MTF)
    Singlestep(VmEventSinglestep),

    /// An event has been requested via HVMOP_guest_request_vm_event.
    GuestRequest,

    /// A debug exception was caught
    DebugException(VmEventDebug),

    /// CPUID executed
    Cpuid(VmEventCpuid),

    /// Privileged call executed (e.g. SMC).
    /// Note: event may be generated even if SMC condition check fails on some CPUs.
    ///       As this behavior is CPU-specific, users are advised to not rely on it.
    ///       These kinds of events will be filtered out in future versions.
    PrivilegedCall,

    /// An interrupt has been delivered.
    Interrupt(VmEventInterrupt),

    /// A descriptor table register was accessed.
    DescriptorAccess(VmEventDescriptorAccess),

    /// Current instruction is not implemented by the emulator
    EmulUnimplemented,

    /// VMEXIT
    VmExit(VmEventVmExit),

    /// IN/OUT Instruction executed
    IoInstruction(VmEventIo),
}
