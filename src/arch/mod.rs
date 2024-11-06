mod domain;
pub mod x86;

pub trait Architecture {
    type Registers: Default;
}
