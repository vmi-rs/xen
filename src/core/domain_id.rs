#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XenDomainId(pub u32);

impl XenDomainId {
    /// ID that stands for the calling domain, `DOMID_SELF` in Xen's headers.
    ///
    /// # Notes
    ///
    /// Xen accepts this ID only in some calls.
    /// [`XenControl::evtchn_status`](crate::XenControl::evtchn_status) is one
    /// of them.
    pub const SELF: Self = Self(0x7FF0);
}

impl From<u32> for XenDomainId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<XenDomainId> for u32 {
    fn from(value: XenDomainId) -> Self {
        value.0
    }
}

impl std::fmt::Display for XenDomainId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
