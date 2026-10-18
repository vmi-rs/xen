use xen_sys::{STATE_IOREQ_NONE, XENMEM_resource_ioreq_server, ioreq};

use crate::{
    VcpuId, XenDomainId, XenError, XenForeignMemory, XenForeignMemoryProtection,
    XenForeignMemoryResource,
};

/// Where the page with the vCPUs' device requests starts among an ioreq
/// server's pages.
///
/// Xen gives each ioreq server a few pages, numbered from 0. Page 0 is the
/// buffered ioreq page, for writes that need no answer. The synchronous ioreq
/// pages start at page 1. Xen's `memory.h` computes the number with the macro
/// `XENMEM_resource_ioreq_server_frame_ioreq(n)`, which bindgen cannot
/// translate, so the value is written here.
const SYNCHRONOUS_IOREQ_FRAME: u64 = 1;

/// The page through which Xen passes a domain's device accesses to QEMU (or
/// another ioreq server).
///
/// When a vCPU touches an emulated device, such as the disk controller, Xen
/// stops the vCPU and writes the access into a page that it shares with the
/// device model, usually QEMU. Xen calls such a device model an ioreq server.
/// The page holds one `ioreq_t` per vCPU, indexed by vCPU id, and the vCPU
/// waits until the server answers in its slot. This type maps the page read
/// only, so that a caller can see which vCPUs wait.
pub struct XenIoreqPage {
    /// The mapping of the page. Dropping it unmaps the page.
    page: XenForeignMemoryResource,
}

impl XenIoreqPage {
    /// Maps the ioreq page of an ioreq server, read only.
    ///
    /// `domain_id` is the domain whose vCPUs send the requests, and `server` is
    /// the id of the ioreq server within that domain. A domain with QEMU has
    /// one server.
    pub fn map(
        foreignmemory: &XenForeignMemory,
        domain_id: XenDomainId,
        server: u32,
    ) -> Result<Self, XenError> {
        let page = foreignmemory.map_resource(
            domain_id,
            XENMEM_resource_ioreq_server,
            server,
            SYNCHRONOUS_IOREQ_FRAME,
            1,
            XenForeignMemoryProtection::READ,
        )?;

        Ok(Self { page })
    }

    /// Returns the state of the slot of `vcpu`. Returns `None` if the page has
    /// no slot for `vcpu`.
    ///
    /// The state is one of these constants:
    ///
    /// - `STATE_IOREQ_NONE` means that the vCPU does not wait.
    /// - `STATE_IOREQ_READY` means that the request waits for the ioreq server.
    /// - `STATE_IOREQ_INPROCESS` means that the ioreq server works on the
    ///   request.
    /// - `STATE_IORESP_READY` means that the reply waits for Xen.
    pub fn state(&self, vcpu: VcpuId) -> Option<u8> {
        let offset = usize::from(vcpu.0) * size_of::<ioreq>();
        if offset + size_of::<ioreq>() > self.page.len() {
            return None;
        }

        // Xen and the server change the slot while this process reads it. A
        // volatile read makes the compiler read the memory now and in one copy,
        // instead of reusing an earlier value or reading the fields one by one.
        let request =
            unsafe { std::ptr::read_volatile(self.page.as_ptr().add(offset) as *const ioreq) };
        Some(request.state())
    }

    /// Returns `true` if a vCPU is in the middle of a device access.
    ///
    /// That is the case from the moment Xen hands the access to the server
    /// until Xen gives the answer back to the vCPU. Loading the domain's
    /// context during that time drops the access. See `XenDomain::set_context`.
    pub fn pending(&self) -> bool {
        (0..self.page.len() / size_of::<ioreq>()).any(|vcpu| {
            self.state(VcpuId(vcpu as u16))
                .is_some_and(|state| u32::from(state) != STATE_IOREQ_NONE)
        })
    }
}
