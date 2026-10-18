mod handle;
pub use self::handle::XenForeignMemoryHandle;

mod mapped;
mod resource;
use std::rc::Rc;

pub use self::{mapped::XenForeignMemoryMapped, resource::XenForeignMemoryResource};
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

    /// Maps pages that Xen owns for a domain into this process.
    ///
    /// Besides the guest's memory, Xen keeps pages of its own for a domain,
    /// such as the pages that it shares with an ioreq server. Xen calls such a
    /// group of pages a resource.
    ///
    /// - `resource_type` picks the kind of resource. Use one of the
    ///   `XENMEM_resource_*` constants.
    /// - `id` picks one resource of this kind, for example the ID of an ioreq
    ///   server.
    /// - `frame` is the frame number of the first page to map.
    /// - `frames` is the number of pages to map.
    /// - `protection` sets the access of this process to the pages.
    pub fn map_resource(
        &self,
        domain_id: XenDomainId,
        resource_type: u32,
        id: u32,
        frame: u64,
        frames: u64,
        protection: XenForeignMemoryProtection,
    ) -> Result<XenForeignMemoryResource, XenError> {
        XenForeignMemoryResource::new(
            self.clone(),
            domain_id,
            resource_type,
            id,
            frame,
            frames,
            protection,
        )
    }
}
