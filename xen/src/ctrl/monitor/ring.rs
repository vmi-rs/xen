use std::ffi::c_void;

use xen_sys::vm_event_back_ring;

use crate::{
    consts::PAGE_SIZE, ctrl::VmEvent, RING_GET_REQUEST, RING_HAS_UNCONSUMED_REQUESTS,
    RING_PUSH_RESPONSES, RING_PUT_RESPONSE,
};

pub struct VmEventRing {
    ring_page: *mut c_void,
    back_ring: vm_event_back_ring,
}

impl VmEventRing {
    pub(crate) fn new(ring_page: *mut c_void, back_ring: vm_event_back_ring) -> Self {
        Self {
            ring_page,
            back_ring,
        }
    }

    pub fn unconsumed_requests(&self) -> usize {
        RING_HAS_UNCONSUMED_REQUESTS!(self.back_ring) as usize
    }

    pub fn has_unconsumed_requests(&self) -> bool {
        self.unconsumed_requests() != 0
    }

    pub fn get_request(&mut self) -> VmEvent {
        let mut req_cons = self.back_ring.req_cons;

        // Copy request
        let req = RING_GET_REQUEST!(self.back_ring, req_cons);
        req_cons += 1;

        // Update ring
        self.back_ring.req_cons = req_cons;
        unsafe {
            (*(self.back_ring.sring)).req_event = req_cons + 1;
        }
        req.into()
    }

    pub fn put_response(&mut self, rsp: VmEvent) {
        let mut rsp_prod = self.back_ring.rsp_prod_pvt;

        // Copy response
        RING_PUT_RESPONSE!(self.back_ring, rsp_prod, rsp.into());
        rsp_prod += 1;

        // Update ring
        self.back_ring.rsp_prod_pvt = rsp_prod;
        RING_PUSH_RESPONSES!(self.back_ring);
    }
}

impl Drop for VmEventRing {
    fn drop(&mut self) {
        tracing::trace!("unmapping ring page");
        unsafe {
            libc::munmap(self.ring_page, PAGE_SIZE as usize);
        }
    }
}
