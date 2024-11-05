mod domain_id;
mod memory_access;
mod vcpu_id;

pub use self::{domain_id::XenDomainId, memory_access::MemoryAccess, vcpu_id::VcpuId};
