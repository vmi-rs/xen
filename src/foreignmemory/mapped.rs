use std::{
    ffi::c_void,
    ops::{Deref, DerefMut},
};

use xen_sys::{xenforeignmemory_map, xenforeignmemory_unmap};

use super::{XenForeignMemory, XenForeignMemoryProtection};
use crate::{XenDomainId, XenError, consts::PAGE_SIZE};

pub struct XenForeignMemoryMapped {
    foreignmemory: XenForeignMemory,
    ptr: *mut c_void,
    pages: usize,
}

impl XenForeignMemoryMapped {
    pub(crate) fn new(
        foreignmemory: XenForeignMemory,
        domain_id: XenDomainId,
        protection: XenForeignMemoryProtection,
        arr: &[u64],
        err: Option<&mut [i32]>,
    ) -> Result<Self, XenError> {
        if let Some(err) = &err {
            debug_assert_eq!(arr.len(), err.len());
        }

        let ptr = unsafe {
            xenforeignmemory_map(
                foreignmemory.handle.0,
                domain_id.0,
                protection.bits(),
                arr.len(),
                arr.as_ptr() as *const _,
                err.map_or_else(std::ptr::null_mut, <[_]>::as_mut_ptr),
            )
        };

        if ptr.is_null() {
            return Err(XenError::Other("Failed to map foreign memory"));
        }

        Ok(Self {
            foreignmemory,
            ptr,
            pages: arr.len(),
        })
    }
}

impl Drop for XenForeignMemoryMapped {
    fn drop(&mut self) {
        //tracing::trace!("unmapping foreign memory");
        unsafe {
            xenforeignmemory_unmap(self.foreignmemory.handle.0, self.ptr, self.pages);
        }
    }
}

impl Deref for XenForeignMemoryMapped {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(self.ptr as *const u8, self.pages * PAGE_SIZE as usize)
        }
    }
}

impl DerefMut for XenForeignMemoryMapped {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr as *mut u8, self.pages * PAGE_SIZE as usize)
        }
    }
}

impl AsRef<[u8]> for XenForeignMemoryMapped {
    fn as_ref(&self) -> &[u8] {
        self.deref()
    }
}

impl AsMut<[u8]> for XenForeignMemoryMapped {
    fn as_mut(&mut self) -> &mut [u8] {
        self.deref_mut()
    }
}
