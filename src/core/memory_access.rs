use xen_sys::{
    xenmem_access_t_XENMEM_access_default, xenmem_access_t_XENMEM_access_n,
    xenmem_access_t_XENMEM_access_n2rwx, xenmem_access_t_XENMEM_access_r,
    xenmem_access_t_XENMEM_access_r_pw, xenmem_access_t_XENMEM_access_rw,
    xenmem_access_t_XENMEM_access_rwx, xenmem_access_t_XENMEM_access_rx,
    xenmem_access_t_XENMEM_access_rx2rw, xenmem_access_t_XENMEM_access_w,
    xenmem_access_t_XENMEM_access_wx, xenmem_access_t_XENMEM_access_x,
};

bitflags::bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct MemoryAccess: u8 {
        const NONE      = xenmem_access_t_XENMEM_access_n as u8; // 0b00000000
        const R         = xenmem_access_t_XENMEM_access_r as u8; // 0b00000001;
        const W         = xenmem_access_t_XENMEM_access_w as u8; // 0b00000010;
        const X         = xenmem_access_t_XENMEM_access_x as u8; // 0b00000100;
        const RW        = xenmem_access_t_XENMEM_access_rw as u8; // Self::R.bits() | Self::W.bits();
        const WX        = xenmem_access_t_XENMEM_access_wx as u8; // Self::W.bits() | Self::X.bits();
        const RX        = xenmem_access_t_XENMEM_access_rx as u8; // Self::R.bits() | Self::X.bits();
        const RWX       = xenmem_access_t_XENMEM_access_rwx as u8; // Self::R.bits() | Self::W.bits() | Self::X.bits();

        const RX2RW     = xenmem_access_t_XENMEM_access_rx2rw as u8;
        const N2RWX     = xenmem_access_t_XENMEM_access_n2rwx as u8;
        const R_PW      = xenmem_access_t_XENMEM_access_r_pw as u8;

        const DEFAULT   = xenmem_access_t_XENMEM_access_default as u8;
    }
}

impl std::fmt::Display for MemoryAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = [b'-'; 3];

        if self.contains(MemoryAccess::R) {
            result[0] = b'r';
        }
        if self.contains(MemoryAccess::W) {
            result[1] = b'w';
        }
        if self.contains(MemoryAccess::X) {
            result[2] = b'x';
        }

        // SAFETY: The `result` array is always valid UTF-8.
        f.write_str(unsafe { std::str::from_utf8_unchecked(&result) })
    }
}
