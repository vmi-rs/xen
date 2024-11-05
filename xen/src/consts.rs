pub const PAGE_SHIFT: u64 = ::xen_sys::XC_PAGE_SHIFT as u64;
pub const PAGE_SIZE: u64 = ::xen_sys::XC_PAGE_SIZE as u64;
pub const PAGE_MASK: i64 = ::xen_sys::XC_PAGE_MASK as i64;

pub const CORE_MAGIC: u32 = ::xen_sys::XC_CORE_MAGIC;
pub const CORE_MAGIC_HVM: u32 = ::xen_sys::XC_CORE_MAGIC_HVM;

pub const MAX_ERROR_MSG_LEN: u32 = ::xen_sys::XC_MAX_ERROR_MSG_LEN;
