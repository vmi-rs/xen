use xen_sys::{
    VM_EVENT_FLAG_ALTERNATE_P2M, VM_EVENT_FLAG_DENY, VM_EVENT_FLAG_EMULATE,
    VM_EVENT_FLAG_EMULATE_NOWRITE, VM_EVENT_FLAG_FAST_SINGLESTEP, VM_EVENT_FLAG_FOREIGN,
    VM_EVENT_FLAG_GET_NEXT_INTERRUPT, VM_EVENT_FLAG_NESTED_P2M, VM_EVENT_FLAG_RESET_FORK_MEMORY,
    VM_EVENT_FLAG_RESET_FORK_STATE, VM_EVENT_FLAG_RESET_VMTRACE, VM_EVENT_FLAG_SET_EMUL_INSN_DATA,
    VM_EVENT_FLAG_SET_EMUL_READ_DATA, VM_EVENT_FLAG_SET_REGISTERS, VM_EVENT_FLAG_TOGGLE_SINGLESTEP,
    VM_EVENT_FLAG_VCPU_PAUSED,
};

bitflags::bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VmEventFlag: u32 {
        // VCPU_PAUSED in a request signals that the vCPU triggering the event has been paused
        // VCPU_PAUSED in a response signals to unpause the vCPU
        const VCPU_PAUSED = VM_EVENT_FLAG_VCPU_PAUSED;

        // Flags to aid debugging vm_event
        const FOREIGN = VM_EVENT_FLAG_FOREIGN;

        // The following flags can be set in response to a mem_access event.
        //
        // Emulate the fault-causing instruction (if set in the event response flags).
        // This will allow the guest to continue execution without lifting the page
        // access restrictions.
        const EMULATE = VM_EVENT_FLAG_EMULATE;

        // Same as VM_EVENT_FLAG_EMULATE, but with write operations or operations
        // potentially having side effects (like memory mapped or port I/O) disabled.
        const EMULATE_NO_WRITE = VM_EVENT_FLAG_EMULATE_NOWRITE;

        // Toggle singlestepping on vm_event response.
        // Requires the vCPU to be paused already (synchronous events only).
        const TOGGLE_SINGLESTEP = VM_EVENT_FLAG_TOGGLE_SINGLESTEP;

        // Data is being sent back to the hypervisor in the event response, to be
        // returned by the read function when emulating an instruction.
        // This flag is only useful when combined with VM_EVENT_FLAG_EMULATE
        // and takes precedence if combined with VM_EVENT_FLAG_EMULATE_NOWRITE
        // (i.e. if both VM_EVENT_FLAG_EMULATE_NOWRITE and
        // VM_EVENT_FLAG_SET_EMUL_READ_DATA are set, only the latter will be honored).
        const SET_EMUL_READ_DATA = VM_EVENT_FLAG_SET_EMUL_READ_DATA;

        // Deny completion of the operation that triggered the event.
        // Currently only useful for MSR and control-register write events.
        // Requires the vCPU to be paused already (synchronous events only).
        const DENY = VM_EVENT_FLAG_DENY;

        // This flag can be set in a request or a response
        //
        // On a request, indicates that the event occurred in the alternate p2m
        // specified by the altp2m_idx request field.
        //
        // On a response, indicates that the VCPU should resume in the alternate p2m
        // specified by the altp2m_idx response field if possible.
        const ALTERNATE_P2M = VM_EVENT_FLAG_ALTERNATE_P2M;

        // Set the vCPU registers to the values in the  vm_event response.
        // At the moment x86-only, applies to EAX-EDX, ESP, EBP, ESI, EDI, R8-R15,
        // EFLAGS, and EIP.
        // Requires the vCPU to be paused already (synchronous events only).
        const SET_REGISTERS = VM_EVENT_FLAG_SET_REGISTERS;

        // Instruction cache is being sent back to the hypervisor in the event response
        // to be used by the emulator. This flag is only useful when combined with
        // VM_EVENT_FLAG_EMULATE and does not take presedence if combined with
        // VM_EVENT_FLAG_EMULATE_NOWRITE or VM_EVENT_FLAG_SET_EMUL_READ_DATA, (i.e.
        // if any of those flags are set, only those will be honored).
        const SET_EMUL_INSN_DATA = VM_EVENT_FLAG_SET_EMUL_INSN_DATA;

        // Have a one-shot VM_EVENT_REASON_INTERRUPT event sent for the first
        // interrupt pending after resuming the VCPU.
        const GET_NEXT_INTERRUPT = VM_EVENT_FLAG_GET_NEXT_INTERRUPT;

        // Execute fast singlestepping on vm_event response.
        // Requires the vCPU to be paused already (synchronous events only).
        //
        // On a response requires setting the p2midx field of fast_singlestep to which
        // Xen will switch the vCPU to on the occurance of the first singlestep, after
        // which singlestep gets automatically disabled.
        const FAST_SINGLESTEP = VM_EVENT_FLAG_FAST_SINGLESTEP;

        // Set if the event comes from a nested VM and thus npt_base is valid.
        const NESTED_P2M = VM_EVENT_FLAG_NESTED_P2M;

        // Reset the vmtrace buffer (if vmtrace is enabled)
        const RESET_VMTRACE = VM_EVENT_FLAG_RESET_VMTRACE;

        // Reset the VM state (if VM is fork)
        const RESET_FORK_STATE = VM_EVENT_FLAG_RESET_FORK_STATE;

        // Remove unshared entries from physmap (if VM is fork)
        const RESET_FORK_MEMORY = VM_EVENT_FLAG_RESET_FORK_MEMORY;
    }
}
