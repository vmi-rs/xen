#[derive(Debug)]
pub enum VmEventRegs {
    X86(super::VmEventRegsX86),
}
