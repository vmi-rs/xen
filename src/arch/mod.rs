mod context;
mod domain;
pub mod x86;

pub use self::context::{XenHvmContext, XenHvmSaveRecord};

pub trait Architecture {
    type Registers: Default;
}
