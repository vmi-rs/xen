mod handle;
pub use self::handle::XenForeignMemoryHandle;

mod mapped;
use std::rc::Rc;

pub use self::mapped::XenForeignMemoryMapped;
use crate::{XenDomainId, XenError};

bitflags::bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct XenForeignMemoryProtection: i32 {
        const READ = libc::PROT_READ;
        const WRITE = libc::PROT_WRITE;
        const EXECUTE = libc::PROT_EXEC;
    }
}

#[derive(Debug, Clone)]
pub struct XenForeignMemory {
    pub(crate) handle: Rc<XenForeignMemoryHandle>,
}

impl XenForeignMemory {
    pub fn new() -> Result<Self, XenError> {
        Ok(Self {
            handle: Rc::new(XenForeignMemoryHandle::new()?),
        })
    }

    pub fn map(
        &self,
        domain_id: XenDomainId,
        protection: XenForeignMemoryProtection,
        arr: &[u64],
        err: Option<&mut [i32]>,
    ) -> Result<XenForeignMemoryMapped, XenError> {
        XenForeignMemoryMapped::new(self.clone(), domain_id, protection, arr, err)
    }
}
