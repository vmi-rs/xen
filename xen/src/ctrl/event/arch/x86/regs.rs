use xen_sys::vm_event_regs_x86;

use super::super::super::{VmEventRegs, VmEventSelectorReg};

#[derive(Debug)]
pub struct VmEventRegsX86 {
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rflags: u64,
    pub dr6: u64,
    pub dr7: u64,
    pub rip: u64,
    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub sysenter_cs: u64,
    pub sysenter_esp: u64,
    pub sysenter_eip: u64,
    pub msr_efer: u64,
    pub msr_star: u64,
    pub msr_lstar: u64,
    pub gdtr_base: u64,

    // When VM_EVENT_FLAG_NESTED_P2M is set, this event comes from a nested
    // VM.  npt_base is the guest physical address of the L1 hypervisors
    // EPT/NPT tables for the nested guest.
    //
    // All bits outside of architectural address ranges are reserved for
    // future metadata.
    pub npt_base: u64,

    // Current position in the vmtrace buffer, or ~0 if vmtrace is not active.
    //
    // For Intel Processor Trace, it is the upper half of MSR_RTIT_OUTPUT_MASK.
    pub vmtrace_pos: u64,

    pub cs_base: u32,
    pub ss_base: u32,
    pub ds_base: u32,
    pub es_base: u32,
    pub fs_base: u64,
    pub gs_base: u64,
    pub cs: VmEventSelectorReg,
    pub ss: VmEventSelectorReg,
    pub ds: VmEventSelectorReg,
    pub es: VmEventSelectorReg,
    pub fs: VmEventSelectorReg,
    pub gs: VmEventSelectorReg,
    pub shadow_gs: u64,
    pub gdtr_limit: u16,
    pub cs_sel: u16,
    pub ss_sel: u16,
    pub ds_sel: u16,
    pub es_sel: u16,
    pub fs_sel: u16,
    pub gs_sel: u16,
}

impl From<vm_event_regs_x86> for VmEventRegsX86 {
    fn from(value: vm_event_regs_x86) -> Self {
        Self {
            rax: value.rax,
            rcx: value.rcx,
            rdx: value.rdx,
            rbx: value.rbx,
            rsp: value.rsp,
            rbp: value.rbp,
            rsi: value.rsi,
            rdi: value.rdi,
            r8: value.r8,
            r9: value.r9,
            r10: value.r10,
            r11: value.r11,
            r12: value.r12,
            r13: value.r13,
            r14: value.r14,
            r15: value.r15,
            rflags: value.rflags,
            dr6: value.dr6,
            dr7: value.dr7,
            rip: value.rip,
            cr0: value.cr0,
            cr2: value.cr2,
            cr3: value.cr3,
            cr4: value.cr4,
            sysenter_cs: value.sysenter_cs,
            sysenter_esp: value.sysenter_esp,
            sysenter_eip: value.sysenter_eip,
            msr_efer: value.msr_efer,
            msr_star: value.msr_star,
            msr_lstar: value.msr_lstar,
            gdtr_base: value.gdtr_base,
            npt_base: value.npt_base,
            vmtrace_pos: value.vmtrace_pos,
            cs_base: value.cs_base,
            ss_base: value.ss_base,
            ds_base: value.ds_base,
            es_base: value.es_base,
            fs_base: value.fs_base,
            gs_base: value.gs_base,
            cs: value.cs.into(),
            ss: value.ss.into(),
            ds: value.ds.into(),
            es: value.es.into(),
            fs: value.fs.into(),
            gs: value.gs.into(),
            shadow_gs: value.shadow_gs,
            gdtr_limit: value.gdtr_limit,
            cs_sel: value.cs_sel,
            ss_sel: value.ss_sel,
            ds_sel: value.ds_sel,
            es_sel: value.es_sel,
            fs_sel: value.fs_sel,
            gs_sel: value.gs_sel,
        }
    }
}

impl From<VmEventRegsX86> for vm_event_regs_x86 {
    fn from(value: VmEventRegsX86) -> Self {
        Self {
            rax: value.rax,
            rcx: value.rcx,
            rdx: value.rdx,
            rbx: value.rbx,
            rsp: value.rsp,
            rbp: value.rbp,
            rsi: value.rsi,
            rdi: value.rdi,
            r8: value.r8,
            r9: value.r9,
            r10: value.r10,
            r11: value.r11,
            r12: value.r12,
            r13: value.r13,
            r14: value.r14,
            r15: value.r15,
            rflags: value.rflags,
            dr6: value.dr6,
            dr7: value.dr7,
            rip: value.rip,
            cr0: value.cr0,
            cr2: value.cr2,
            cr3: value.cr3,
            cr4: value.cr4,
            sysenter_cs: value.sysenter_cs,
            sysenter_esp: value.sysenter_esp,
            sysenter_eip: value.sysenter_eip,
            msr_efer: value.msr_efer,
            msr_star: value.msr_star,
            msr_lstar: value.msr_lstar,
            gdtr_base: value.gdtr_base,
            npt_base: value.npt_base,
            vmtrace_pos: value.vmtrace_pos,
            cs_base: value.cs_base,
            ss_base: value.ss_base,
            ds_base: value.ds_base,
            es_base: value.es_base,
            fs_base: value.fs_base,
            gs_base: value.gs_base,
            cs: value.cs.into(),
            ss: value.ss.into(),
            ds: value.ds.into(),
            es: value.es.into(),
            fs: value.fs.into(),
            gs: value.gs.into(),
            shadow_gs: value.shadow_gs,
            gdtr_limit: value.gdtr_limit,
            cs_sel: value.cs_sel,
            ss_sel: value.ss_sel,
            ds_sel: value.ds_sel,
            es_sel: value.es_sel,
            fs_sel: value.fs_sel,
            gs_sel: value.gs_sel,
            _pad: Default::default(),
        }
    }
}

impl From<vm_event_regs_x86> for VmEventRegs {
    fn from(value: vm_event_regs_x86) -> Self {
        Self::X86(value.into())
    }
}

impl From<VmEventRegs> for vm_event_regs_x86 {
    fn from(value: VmEventRegs) -> Self {
        match value {
            VmEventRegs::X86(regs) => regs.into(),
        }
    }
}
