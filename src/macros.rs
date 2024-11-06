#[macro_export]
macro_rules! xc_check_error {
    ($handle:expr, $rc:expr) => {
        if $rc < 0 {
            let handle = $handle;
            let rc = $rc;

            unsafe {
                let err = ::xen_sys::xc_get_last_error(handle);
                let desc = ::xen_sys::xc_error_code_to_desc((*err).code as _);
                return Err($crate::error::XenError::Xen($crate::error::XcError {
                    rc,
                    code: (*err).code as _,
                    desc: ::std::ffi::CStr::from_ptr(desc).to_str().unwrap(),
                    backtrace: ::std::backtrace::Backtrace::capture(),
                }));
            }
        }
    };
}

#[macro_export]
macro_rules! __RD2 {
    ($x:expr) => {
        if $x as u32 & 0x00000002 != 0 {
            0x2
        }
        else {
            $x as u32 & 0x1
        }
    };
}

#[macro_export]
macro_rules! __RD4 {
    ($x:expr) => {
        if $x as u32 & 0x0000000c != 0 {
            $crate::__RD2!(($x) >> 2) << 2
        }
        else {
            $crate::__RD2!($x)
        }
    };
}

#[macro_export]
macro_rules! __RD8 {
    ($x:expr) => {
        if $x as u32 & 0x000000f0 != 0 {
            $crate::__RD4!(($x) >> 4) << 4
        }
        else {
            $crate::__RD4!($x)
        }
    };
}

#[macro_export]
macro_rules! __RD16 {
    ($x:expr) => {
        if $x as u32 & 0x0000ff00 != 0 {
            $crate::__RD8!(($x) >> 8) << 8
        }
        else {
            $crate::__RD8!($x)
        }
    };
}

#[macro_export]
macro_rules! __RD32 {
    ($x:expr) => {
        if $x as u32 & 0xffff0000 != 0 {
            $crate::__RD16!(($x) >> 16) << 16
        }
        else {
            $crate::__RD16!($x)
        }
    };
}

#[macro_export]
macro_rules! __RING_SIZE {
    ($s:expr, $sz:ident) => {{
        let s = $s;
        let sz = $sz;

        unsafe {
            $crate::__RD32!(
                ((sz as usize + s as usize - (*s).ring.as_mut_ptr() as *const _ as usize)
                    / ::std::mem::size_of_val(&*(*s).ring.as_ptr())) as u32
            )
        }
    }};
}

#[macro_export]
macro_rules! SHARED_RING_INIT {
    ($s:expr) => {{
        let s = $s as *mut ::xen_sys::vm_event_sring;

        unsafe {
            (*s).req_prod = 0;
            (*s).rsp_prod = 0;
            (*s).req_event = 1;
            (*s).rsp_event = 1;
            (*s).pvt.pvt_pad = std::mem::zeroed();
            (*s).__pad = std::mem::zeroed();
        }
    }};
}

#[macro_export]
macro_rules! BACK_RING_ATTACH {
    ($r:ident, $s:ident, $i:expr, $size:ident) => {
        let _s = $s as *mut ::xen_sys::vm_event_sring;

        $r.rsp_prod_pvt = $i;
        $r.req_cons = $i;
        $r.nr_ents = $crate::__RING_SIZE!(_s, $size);
        $r.sring = $s as _;
    };
}

#[macro_export]
macro_rules! BACK_RING_INIT {
    ($r:ident, $s:ident, $size:ident) => {
        $crate::BACK_RING_ATTACH!($r, $s, 0, $size);
    };
}

#[macro_export]
macro_rules! RING_SIZE {
    ($r:expr) => {
        $r.nr_ents
    };
}

#[macro_export]
macro_rules! RING_HAS_UNCONSUMED_REQUESTS {
    ($r:expr) => {{
        let r = $r;
        let req = unsafe { (*(r.sring)).req_prod - r.req_cons };
        let rsp = $crate::RING_SIZE!(r) - (r.req_cons - r.rsp_prod_pvt);
        if req < rsp {
            req
        }
        else {
            rsp
        }
    }};
}

#[macro_export]
macro_rules! RING_GET_REQUEST {
    ($r:expr, $idx:ident) => {{
        let r = $r;

        unsafe {
            let ring_slice = (*r.sring).ring.as_slice(r.nr_ents as usize);
            ring_slice[($idx & (r.nr_ents - 1)) as usize].req
        }
    }};
}

#[macro_export]
macro_rules! RING_PUT_RESPONSE {
    ($r:expr, $idx:expr, $value:expr) => {{
        let r = $r;
        let idx = $idx;
        let value = $value;

        unsafe {
            let ring_slice = (*r.sring).ring.as_mut_slice(r.nr_ents as usize);
            ring_slice[(idx & (r.nr_ents - 1)) as usize].rsp = value;
        }
    }};
}

#[macro_export]
macro_rules! RING_PUSH_RESPONSES {
    ($name1:expr) => {{
        let name1 = $name1;

        unsafe {
            (*(name1.sring)).rsp_prod = name1.rsp_prod_pvt;
        }
    }};
}
