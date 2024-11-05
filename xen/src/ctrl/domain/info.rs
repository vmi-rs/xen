use xen_sys::xen_domctl_getdomaininfo;

pub struct XenDomainInfo {
    pub total_pages: u64,
    pub max_pages: u64,
    pub outstanding_pages: u64,
    pub shared_pages: u64,
    pub paged_pages: u64,
    pub shared_info_frame: u64,
    pub cpu_time: u64,
    pub nr_online_vcpus: u32,
    pub max_vcpu_id: u16,
}

impl From<xen_domctl_getdomaininfo> for XenDomainInfo {
    fn from(value: xen_domctl_getdomaininfo) -> Self {
        Self {
            total_pages: value.tot_pages,
            max_pages: value.max_pages,
            outstanding_pages: value.outstanding_pages,
            shared_pages: value.shr_pages,
            paged_pages: value.paged_pages,
            shared_info_frame: value.shared_info_frame,
            cpu_time: value.cpu_time,
            nr_online_vcpus: value.nr_online_vcpus,
            max_vcpu_id: value.max_vcpu_id as _,
        }
    }
}
