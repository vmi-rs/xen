mod handle;
use std::{
    ffi::{CStr, CString, c_char, c_void},
    rc::Rc,
};

use xen_sys::{XBT_NULL, xs_directory, xs_read};

pub use self::handle::XenStoreHandle;
use crate::{XenDomainId, XenError};

#[derive(Debug, Clone)]
pub struct XenStore {
    pub(crate) handle: Rc<XenStoreHandle>,
}

impl XenStore {
    pub fn new() -> Result<Self, XenError> {
        Ok(Self {
            handle: Rc::new(XenStoreHandle::new()?),
        })
    }

    pub fn domain_id_from_name(name: &str) -> Result<Option<XenDomainId>, XenError> {
        let xs = Self::new()?;

        for domain in xs.directory("/local/domain")? {
            let domain_name = xs.read(&format!("/local/domain/{domain}/name"))?;
            if domain_name == name {
                match domain.parse() {
                    Ok(domain) => return Ok(Some(XenDomainId(domain))),
                    Err(_) => return Err(XenError::Other("Failed to parse domain id")),
                }
            }
        }

        Ok(None)
    }

    pub fn directory(&self, path: &str) -> Result<Vec<String>, XenError> {
        let path = CString::new(path).unwrap();
        let mut num = 0;
        let result = unsafe { xs_directory(self.handle.0, XBT_NULL, path.as_ptr(), &mut num) };

        if result.is_null() {
            return Err(XenError::Other("Failed to read xen store directory"));
        }

        let mut entries = Vec::with_capacity(num as usize);
        for i in 0..num {
            let entry = unsafe { CStr::from_ptr(*result.offset(i as isize)) };
            entries.push(entry.to_string_lossy().into());
        }

        unsafe {
            libc::free(result as *mut c_void);
        }

        Ok(entries)
    }

    pub fn read(&self, path: &str) -> Result<String, XenError> {
        let mut len = 0;
        let path = CString::new(path).unwrap();
        let result = unsafe { xs_read(self.handle.0, XBT_NULL, path.as_ptr(), &mut len) };
        if result.is_null() {
            return Err(XenError::Other("Failed to read xen store"));
        }

        let value = unsafe { CStr::from_ptr(result as *const c_char) };
        let value = value.to_string_lossy().into_owned();

        unsafe {
            libc::free(result);
        }

        Ok(value)
    }
}
