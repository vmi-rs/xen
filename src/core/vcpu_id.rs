#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VcpuId(pub u16);

impl From<u16> for VcpuId {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<VcpuId> for u16 {
    fn from(value: VcpuId) -> Self {
        value.0
    }
}
