#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XenDomainId(pub u32);

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
