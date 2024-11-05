use xen_sys::{vm_event_emul_insn_data, vm_event_emul_read_data, vm_event_regs_x86};

use super::VmEventRegs;

const SIZE_OF_EMUL_DATA: usize = size_of::<vm_event_regs_x86>() - size_of::<u32>();

#[derive(Debug)]
pub struct VmEventEmulReadData {
    pub size: u32,

    // The struct is used in a union with vm_event_regs_x86.
    pub data: [u8; SIZE_OF_EMUL_DATA],
}

impl Default for VmEventEmulReadData {
    fn default() -> Self {
        Self {
            size: 0,
            data: [0; SIZE_OF_EMUL_DATA],
        }
    }
}

impl From<vm_event_emul_read_data> for VmEventEmulReadData {
    fn from(value: vm_event_emul_read_data) -> Self {
        Self {
            size: value.size,
            data: value.data,
        }
    }
}

impl From<VmEventEmulReadData> for vm_event_emul_read_data {
    fn from(value: VmEventEmulReadData) -> Self {
        Self {
            size: value.size,
            data: value.data,
        }
    }
}

#[derive(Debug, Default)]
pub struct VmEventEmulInsnData {
    pub data: [u8; 16], // Has to be completely filled
}

impl From<vm_event_emul_insn_data> for VmEventEmulInsnData {
    fn from(value: vm_event_emul_insn_data) -> Self {
        Self { data: value.data }
    }
}

impl From<VmEventEmulInsnData> for vm_event_emul_insn_data {
    fn from(value: VmEventEmulInsnData) -> Self {
        Self { data: value.data }
    }
}

#[derive(Debug)]
pub enum VmEventData {
    Registers(VmEventRegs),
    EmulReadData(VmEventEmulReadData),
    EmulInstructionData(VmEventEmulInsnData),
}
