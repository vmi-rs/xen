use std::{ffi::c_void, ops::Deref};

use xen_sys::{
    xenforeignmemory_map_resource, xenforeignmemory_resource_handle,
    xenforeignmemory_unmap_resource,
};

use super::{XenForeignMemory, XenForeignMemoryProtection};
use crate::{XenDomainId, XenError, consts::PAGE_SIZE};

/// A mapping of pages that Xen owns for a domain.
///
/// `XenForeignMemory::map_resource` creates it. It derefs to the pages' bytes,
/// and dropping it unmaps the pages.
pub struct XenForeignMemoryResource {
    /// Keeps the library handle open until the pages are unmapped.
    foreignmemory: XenForeignMemory,

    /// The record that libxenforeignmemory needs to unmap the mapping.
    resource: *mut xenforeignmemory_resource_handle,

    /// The address of the first mapped page in this process. The other pages
    /// follow it without gaps.
    ptr: *mut c_void,

    /// The number of pages in the mapping.
    frames: usize,
}

impl XenForeignMemoryResource {
    /// Maps the pages and keeps `foreignmemory` for the unmap.
    ///
    /// The arguments are those of `XenForeignMemory::map_resource`.
    pub(crate) fn new(
        foreignmemory: XenForeignMemory,
        domain_id: XenDomainId,
        resource_type: u32,
        id: u32,
        frame: u64,
        frames: u64,
        protection: XenForeignMemoryProtection,
    ) -> Result<Self, XenError> {
        let mut ptr = std::ptr::null_mut();
        let resource = unsafe {
            xenforeignmemory_map_resource(
                foreignmemory.handle.0,
                domain_id.0 as u16,
                resource_type,
                id,
                frame,
                frames,
                &mut ptr,
                protection.bits(),
                0,
            )
        };

        if resource.is_null() {
            return Err(XenError::Io(std::io::Error::last_os_error()));
        }

        Ok(Self {
            foreignmemory,
            resource,
            ptr,
            frames: frames as usize,
        })
    }
}

impl Drop for XenForeignMemoryResource {
    fn drop(&mut self) {
        unsafe {
            xenforeignmemory_unmap_resource(self.foreignmemory.handle.0, self.resource);
        }
    }
}

impl Deref for XenForeignMemoryResource {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(self.ptr as *const u8, self.frames * PAGE_SIZE as usize)
        }
    }
}

impl AsRef<[u8]> for XenForeignMemoryResource {
    fn as_ref(&self) -> &[u8] {
        self.deref()
    }
}
