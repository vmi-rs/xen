use xen_sys::{hvm_hw_cpu, hvm_hw_lapic, hvm_hw_lapic_regs};

pub struct Amd64;

impl super::Architecture for Amd64 {
    type Registers = Registers;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Registers {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,

    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,

    pub dr0: u64,
    pub dr1: u64,
    pub dr2: u64,
    pub dr3: u64,
    pub dr6: u64,
    pub dr7: u64,

    pub cs_base: u64,
    pub cs_limit: u32,
    pub cs_sel: u32,
    pub cs_arbytes: u32,

    pub ds_base: u64,
    pub ds_limit: u32,
    pub ds_sel: u32,
    pub ds_arbytes: u32,

    pub es_base: u64,
    pub es_limit: u32,
    pub es_sel: u32,
    pub es_arbytes: u32,

    pub fs_base: u64,
    pub fs_limit: u32,
    pub fs_sel: u32,
    pub fs_arbytes: u32,

    pub gs_base: u64,
    pub gs_limit: u32,
    pub gs_sel: u32,
    pub gs_arbytes: u32,

    pub ss_base: u64,
    pub ss_limit: u32,
    pub ss_sel: u32,
    pub ss_arbytes: u32,

    pub tr_base: u64,
    pub tr_limit: u32,
    pub tr_sel: u32,
    pub tr_arbytes: u32,

    pub ldtr_base: u64,
    pub ldtr_limit: u32,
    pub ldtr_sel: u32,
    pub ldtr_arbytes: u32,

    pub idtr_base: u64,
    pub idtr_limit: u32,

    pub gdtr_base: u64,
    pub gdtr_limit: u32,

    pub sysenter_cs: u64,
    pub sysenter_esp: u64,
    pub sysenter_eip: u64,
    pub shadow_gs: u64,

    pub msr_flags: u64,
    pub msr_lstar: u64,
    pub msr_star: u64,
    pub msr_cstar: u64,
    pub msr_syscall_mask: u64,
    pub msr_efer: u64,
    pub msr_tsc_aux: u64,
}

impl Registers {
    pub fn copy_into(&self, value: &mut hvm_hw_cpu) {
        value.rax = self.rax;
        value.rbx = self.rbx;
        value.rcx = self.rcx;
        value.rdx = self.rdx;
        value.rbp = self.rbp;
        value.rsi = self.rsi;
        value.rdi = self.rdi;
        value.rsp = self.rsp;
        value.r8 = self.r8;
        value.r9 = self.r9;
        value.r10 = self.r10;
        value.r11 = self.r11;
        value.r12 = self.r12;
        value.r13 = self.r13;
        value.r14 = self.r14;
        value.r15 = self.r15;
        value.rip = self.rip;
        value.rflags = self.rflags;

        value.cr0 = self.cr0;
        value.cr2 = self.cr2;
        value.cr3 = self.cr3;
        value.cr4 = self.cr4;

        // Copy just the fields as libvmi does.
        value.fs_base = self.fs_base;
        value.gs_base = self.gs_base;
        value.cs_arbytes = self.cs_arbytes;
        value.sysenter_cs = self.sysenter_cs;
        value.sysenter_esp = self.sysenter_esp;
        value.sysenter_eip = self.sysenter_eip;
        value.msr_lstar = self.msr_lstar;
        value.msr_efer = self.msr_efer;
        value.msr_star = self.msr_star;

        /////////////////////////////////////

        /*
        value.dr0 = self.dr0;
        value.dr1 = self.dr1;
        value.dr2 = self.dr2;
        value.dr3 = self.dr3;
        value.dr6 = self.dr6;
        value.dr7 = self.dr7;

        value.cs_base = self.cs_base;
        value.cs_limit = self.cs_limit;
        value.cs_sel = self.cs_sel;
        value.cs_arbytes = self.cs_arbytes;

        value.ds_base = self.ds_base;
        value.ds_limit = self.ds_limit;
        value.ds_sel = self.ds_sel;
        value.ds_arbytes = self.ds_arbytes;

        value.es_base = self.es_base;
        value.es_limit = self.es_limit;
        value.es_sel = self.es_sel;
        value.es_arbytes = self.es_arbytes;

        value.fs_base = self.fs_base;
        value.fs_limit = self.fs_limit;
        value.fs_sel = self.fs_sel;
        value.fs_arbytes = self.fs_arbytes;

        value.gs_base = self.gs_base;
        value.gs_limit = self.gs_limit;
        value.gs_sel = self.gs_sel;
        value.gs_arbytes = self.gs_arbytes;

        value.ss_base = self.ss_base;
        value.ss_limit = self.ss_limit;
        value.ss_sel = self.ss_sel;
        value.ss_arbytes = self.ss_arbytes;

        value.tr_base = self.tr_base;
        value.tr_limit = self.tr_limit;
        value.tr_sel = self.tr_sel;
        value.tr_arbytes = self.tr_arbytes;

        value.ldtr_base = self.ldtr_base;
        value.ldtr_limit = self.ldtr_limit;
        value.ldtr_sel = self.ldtr_sel;
        value.ldtr_arbytes = self.ldtr_arbytes;

        value.idtr_base = self.idtr_base;
        value.idtr_limit = self.idtr_limit;

        value.gdtr_base = self.gdtr_base;
        value.gdtr_limit = self.gdtr_limit;

        value.sysenter_cs = self.sysenter_cs;
        value.sysenter_esp = self.sysenter_esp;
        value.sysenter_eip = self.sysenter_eip;
        value.shadow_gs = self.shadow_gs;

        value.msr_flags = self.msr_flags;
        value.msr_lstar = self.msr_lstar;
        value.msr_star = self.msr_star;
        value.msr_cstar = self.msr_cstar;
        value.msr_syscall_mask = self.msr_syscall_mask;
        value.msr_efer = self.msr_efer;
        value.msr_tsc_aux = self.msr_tsc_aux;
        */
    }
}

impl From<hvm_hw_cpu> for Registers {
    fn from(value: hvm_hw_cpu) -> Self {
        Self {
            rax: value.rax,
            rbx: value.rbx,
            rcx: value.rcx,
            rdx: value.rdx,
            rbp: value.rbp,
            rsi: value.rsi,
            rdi: value.rdi,
            rsp: value.rsp,
            r8: value.r8,
            r9: value.r9,
            r10: value.r10,
            r11: value.r11,
            r12: value.r12,
            r13: value.r13,
            r14: value.r14,
            r15: value.r15,
            rip: value.rip,
            rflags: value.rflags,

            cr0: value.cr0,
            cr2: value.cr2,
            cr3: value.cr3,
            cr4: value.cr4,

            dr0: value.dr0,
            dr1: value.dr1,
            dr2: value.dr2,
            dr3: value.dr3,
            dr6: value.dr6,
            dr7: value.dr7,

            cs_base: value.cs_base,
            cs_limit: value.cs_limit,
            cs_sel: value.cs_sel,
            cs_arbytes: value.cs_arbytes,

            ds_base: value.ds_base,
            ds_limit: value.ds_limit,
            ds_sel: value.ds_sel,
            ds_arbytes: value.ds_arbytes,

            es_base: value.es_base,
            es_limit: value.es_limit,
            es_sel: value.es_sel,
            es_arbytes: value.es_arbytes,

            fs_base: value.fs_base,
            fs_limit: value.fs_limit,
            fs_sel: value.fs_sel,
            fs_arbytes: value.fs_arbytes,

            gs_base: value.gs_base,
            gs_limit: value.gs_limit,
            gs_sel: value.gs_sel,
            gs_arbytes: value.gs_arbytes,

            ss_base: value.ss_base,
            ss_limit: value.ss_limit,
            ss_sel: value.ss_sel,
            ss_arbytes: value.ss_arbytes,

            tr_base: value.tr_base,
            tr_limit: value.tr_limit,
            tr_sel: value.tr_sel,
            tr_arbytes: value.tr_arbytes,

            ldtr_base: value.ldtr_base,
            ldtr_limit: value.ldtr_limit,
            ldtr_sel: value.ldtr_sel,
            ldtr_arbytes: value.ldtr_arbytes,

            idtr_base: value.idtr_base,
            idtr_limit: value.idtr_limit,

            gdtr_base: value.gdtr_base,
            gdtr_limit: value.gdtr_limit,

            sysenter_cs: value.sysenter_cs,
            sysenter_esp: value.sysenter_esp,
            sysenter_eip: value.sysenter_eip,
            shadow_gs: value.shadow_gs,

            msr_flags: value.msr_flags,
            msr_lstar: value.msr_lstar,
            msr_star: value.msr_star,
            msr_cstar: value.msr_cstar,
            msr_syscall_mask: value.msr_syscall_mask,
            msr_efer: value.msr_efer,
            msr_tsc_aux: value.msr_tsc_aux,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LocalApic {
    pub apic_base_msr: u64,
    pub disabled: u32,
    pub timer_divisor: u32,
    pub tdt_msr: u64,
}

impl From<hvm_hw_lapic> for LocalApic {
    fn from(value: hvm_hw_lapic) -> Self {
        Self {
            apic_base_msr: value.apic_base_msr,
            disabled: value.disabled,
            timer_divisor: value.timer_divisor,
            tdt_msr: value.tdt_msr,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalApicRegisters {
    pub data: [u8; 1024],
}

impl Default for LocalApicRegisters {
    fn default() -> Self {
        Self { data: [0; 1024] }
    }
}

impl From<hvm_hw_lapic_regs> for LocalApicRegisters {
    fn from(value: hvm_hw_lapic_regs) -> Self {
        Self { data: value.data }
    }
}
