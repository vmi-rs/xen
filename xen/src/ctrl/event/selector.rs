use xen_sys::vm_event_x86_selector_reg;

#[derive(Debug)]
pub struct VmEventSelectorReg {
    // The limit field is right-shifted by 12 bits if .ar.g is set.
    pub limit: u32,
    pub ar: u32,
}

impl From<vm_event_x86_selector_reg> for VmEventSelectorReg {
    fn from(value: vm_event_x86_selector_reg) -> Self {
        Self {
            limit: value.limit(),
            ar: value.ar(),
        }
    }
}

impl From<VmEventSelectorReg> for vm_event_x86_selector_reg {
    fn from(value: VmEventSelectorReg) -> Self {
        Self {
            _bitfield_align_1: Default::default(),
            _bitfield_1: Self::new_bitfield_1(value.limit, value.ar),
        }
    }
}
