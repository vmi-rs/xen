pub mod arch;
mod data;
mod flags;
mod reason;
mod regs;
mod selector;

use xen_sys::{
    vm_event_st, VM_EVENT_INTERFACE_VERSION, VM_EVENT_REASON_CPUID,
    VM_EVENT_REASON_DEBUG_EXCEPTION, VM_EVENT_REASON_DESCRIPTOR_ACCESS,
    VM_EVENT_REASON_EMUL_UNIMPLEMENTED, VM_EVENT_REASON_GUEST_REQUEST, VM_EVENT_REASON_INTERRUPT,
    VM_EVENT_REASON_IO_INSTRUCTION, VM_EVENT_REASON_MEM_ACCESS, VM_EVENT_REASON_MEM_PAGING,
    VM_EVENT_REASON_MEM_SHARING, VM_EVENT_REASON_MOV_TO_MSR, VM_EVENT_REASON_PRIVILEGED_CALL,
    VM_EVENT_REASON_SINGLESTEP, VM_EVENT_REASON_SOFTWARE_BREAKPOINT, VM_EVENT_REASON_UNKNOWN,
    VM_EVENT_REASON_VMEXIT, VM_EVENT_REASON_WRITE_CTRLREG,
};

pub use self::{
    arch::x86::VmEventRegsX86,
    data::{VmEventData, VmEventEmulInsnData, VmEventEmulReadData},
    flags::VmEventFlag,
    reason::{
        VmEventCpuid, VmEventCtrlReg, VmEventDebug, VmEventDescriptorAccess, VmEventFastSinglestep,
        VmEventInterrupt, VmEventIo, VmEventMemAccess, VmEventMovToMsr, VmEventPaging,
        VmEventReason, VmEventSharing, VmEventSinglestep, VmEventVmExit, VmEventWriteCtrlReg,
    },
    regs::VmEventRegs,
    selector::VmEventSelectorReg,
};
use crate::VcpuId;

#[derive(Debug, Default)]
pub struct VmEventFlagOptions {
    pub fast_singlestep: Option<VmEventFastSinglestep>,
}

#[derive(Debug)]
pub struct VmEvent {
    pub flags: VmEventFlag,
    pub reason: VmEventReason,
    pub vcpu_id: VcpuId,
    pub altp2m_idx: u16, // may be used during request and response
    pub options: Option<VmEventFlagOptions>,
    pub data: Option<VmEventData>,
}

impl From<vm_event_st> for VmEvent {
    fn from(value: vm_event_st) -> Self {
        let flags = VmEventFlag::from_bits_truncate(value.flags);

        let reason = unsafe {
            match value.reason {
                VM_EVENT_REASON_MEM_ACCESS => {
                    VmEventReason::MemoryAccess(value.u.mem_access.into())
                }
                VM_EVENT_REASON_MEM_SHARING => {
                    VmEventReason::MemorySharing(value.u.mem_sharing.into())
                }
                VM_EVENT_REASON_MEM_PAGING => {
                    VmEventReason::MemoryPaging(value.u.mem_paging.into())
                }
                VM_EVENT_REASON_WRITE_CTRLREG => {
                    VmEventReason::WriteCtrlReg(value.u.write_ctrlreg.into())
                }
                VM_EVENT_REASON_MOV_TO_MSR => VmEventReason::MovToMsr(value.u.mov_to_msr.into()),
                VM_EVENT_REASON_SOFTWARE_BREAKPOINT => {
                    VmEventReason::SoftwareBreakpoint(value.u.software_breakpoint.into())
                }
                VM_EVENT_REASON_SINGLESTEP => VmEventReason::Singlestep(value.u.singlestep.into()),
                VM_EVENT_REASON_GUEST_REQUEST => VmEventReason::GuestRequest,
                VM_EVENT_REASON_DEBUG_EXCEPTION => {
                    VmEventReason::DebugException(value.u.debug_exception.into())
                }
                VM_EVENT_REASON_CPUID => VmEventReason::Cpuid(value.u.cpuid.into()),
                VM_EVENT_REASON_PRIVILEGED_CALL => VmEventReason::PrivilegedCall,
                VM_EVENT_REASON_INTERRUPT => VmEventReason::Interrupt(value.u.interrupt.x86.into()),
                VM_EVENT_REASON_DESCRIPTOR_ACCESS => {
                    VmEventReason::DescriptorAccess(value.u.desc_access.into())
                }
                VM_EVENT_REASON_EMUL_UNIMPLEMENTED => VmEventReason::EmulUnimplemented,
                VM_EVENT_REASON_VMEXIT => VmEventReason::VmExit(value.u.vmexit.into()),
                VM_EVENT_REASON_IO_INSTRUCTION => VmEventReason::IoInstruction(value.u.io.into()),
                _ => VmEventReason::Unknown,
            }
        };

        let data = unsafe {
            if flags.contains(VmEventFlag::SET_EMUL_READ_DATA) {
                Some(VmEventData::EmulReadData(value.data.emul.read.into()))
            }
            else if flags.contains(VmEventFlag::SET_EMUL_INSN_DATA) {
                Some(VmEventData::EmulInstructionData(
                    value.data.emul.insn.into(),
                ))
            }
            else
            /* if flags.contains(VmEventFlag::SET_REGISTERS) */
            {
                Some(VmEventData::Registers(value.data.regs.x86.into()))
            }
        };

        Self {
            flags,
            reason,
            vcpu_id: VcpuId(value.vcpu_id as u16),
            altp2m_idx: value.altp2m_idx,
            options: None,
            data,
        }
    }
}

impl From<VmEvent> for vm_event_st {
    fn from(value: VmEvent) -> Self {
        let mut result = Self {
            version: VM_EVENT_INTERFACE_VERSION,
            flags: value.flags.bits(),
            vcpu_id: value.vcpu_id.0.into(),
            altp2m_idx: value.altp2m_idx,
            ..Default::default()
        };

        match value.reason {
            VmEventReason::Unknown => {
                result.reason = VM_EVENT_REASON_UNKNOWN;
            }
            VmEventReason::MemoryAccess(data) => {
                result.reason = VM_EVENT_REASON_MEM_ACCESS;
                result.u.mem_access = data.into();
            }
            VmEventReason::MemorySharing(data) => {
                result.reason = VM_EVENT_REASON_MEM_SHARING;
                result.u.mem_sharing = data.into();
            }
            VmEventReason::MemoryPaging(data) => {
                result.reason = VM_EVENT_REASON_MEM_PAGING;
                result.u.mem_paging = data.into();
            }
            VmEventReason::WriteCtrlReg(data) => {
                result.reason = VM_EVENT_REASON_WRITE_CTRLREG;
                result.u.write_ctrlreg = data.into();
            }
            VmEventReason::MovToMsr(data) => {
                result.reason = VM_EVENT_REASON_MOV_TO_MSR;
                result.u.mov_to_msr = data.into();
            }
            VmEventReason::SoftwareBreakpoint(data) => {
                result.reason = VM_EVENT_REASON_SOFTWARE_BREAKPOINT;
                result.u.software_breakpoint = data.into();
            }
            VmEventReason::Singlestep(data) => {
                result.reason = VM_EVENT_REASON_SINGLESTEP;
                result.u.singlestep = data.into();
            }
            VmEventReason::GuestRequest => {
                result.reason = VM_EVENT_REASON_GUEST_REQUEST;
            }
            VmEventReason::DebugException(data) => {
                result.reason = VM_EVENT_REASON_DEBUG_EXCEPTION;
                result.u.debug_exception = data.into();
            }
            VmEventReason::Cpuid(data) => {
                result.reason = VM_EVENT_REASON_CPUID;
                result.u.cpuid = data.into();
            }
            VmEventReason::PrivilegedCall => {
                result.reason = VM_EVENT_REASON_PRIVILEGED_CALL;
            }
            VmEventReason::Interrupt(data) => {
                result.reason = VM_EVENT_REASON_INTERRUPT;
                result.u.interrupt.x86 = data.into();
            }
            VmEventReason::DescriptorAccess(data) => {
                result.reason = VM_EVENT_REASON_DESCRIPTOR_ACCESS;
                result.u.desc_access = data.into();
            }
            VmEventReason::EmulUnimplemented => {
                result.reason = VM_EVENT_REASON_EMUL_UNIMPLEMENTED;
            }
            VmEventReason::VmExit(data) => {
                result.reason = VM_EVENT_REASON_VMEXIT;
                result.u.vmexit = data.into();
            }
            VmEventReason::IoInstruction(data) => {
                result.reason = VM_EVENT_REASON_IO_INSTRUCTION;
                result.u.io = data.into();
            }
        }

        #[expect(clippy::single_match)]
        match value.options {
            Some(VmEventFlagOptions {
                fast_singlestep: Some(fast_singlestep),
            }) => {
                if value.flags.contains(VmEventFlag::FAST_SINGLESTEP) {
                    result.u.fast_singlestep = fast_singlestep.into();
                }
                else {
                    result.flags &= !VmEventFlag::FAST_SINGLESTEP.bits();
                }
            }
            _ => {}
        }

        match value.data {
            Some(VmEventData::EmulReadData(data)) => {
                result.data.emul.read = data.into();
            }
            Some(VmEventData::EmulInstructionData(data)) => {
                result.data.emul.insn = data.into();
            }
            Some(VmEventData::Registers(data)) => {
                result.data.regs.x86 = data.into();
            }
            _ => {}
        }

        result
    }
}
