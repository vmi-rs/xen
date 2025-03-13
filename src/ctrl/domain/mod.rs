mod info;
use xen_sys::{
    xc_domain_debug_control, xc_domain_decrease_reservation, xc_domain_decrease_reservation_exact,
    xc_domain_getinfolist, xc_domain_increase_reservation, xc_domain_increase_reservation_exact,
    xc_domain_maximum_gpfn, xc_domain_pause, xc_domain_populate_physmap,
    xc_domain_populate_physmap_exact, xc_domain_set_access_required, xc_domain_setmaxmem,
    xc_domain_unpause, xc_get_mem_access, xc_set_mem_access, xen_domctl_getdomaininfo,
};

pub use self::info::XenDomainInfo;
use crate::{
    Architecture, MemoryAccess, VcpuId, XenAltP2M, XenDeviceModel, XenDomainId, XenError,
    XenInterface, XenMonitor, ctrl::VmEventRing, xc_check_error,
};

pub struct XenDomain<Arch>
where
    Arch: Architecture,
{
    pub(crate) interface: XenInterface,
    pub(crate) domain_id: XenDomainId,
    _marker: std::marker::PhantomData<Arch>,
}

impl<Arch> XenDomain<Arch>
where
    Arch: Architecture,
{
    pub(crate) fn new(interface: XenInterface, domain_id: XenDomainId) -> Result<Self, XenError> {
        Ok(Self {
            interface,
            domain_id,
            _marker: std::marker::PhantomData,
        })
    }

    pub fn id(&self) -> XenDomainId {
        self.domain_id
    }

    pub fn info(&self) -> Result<XenDomainInfo, XenError> {
        let mut info = xen_domctl_getdomaininfo::default();
        let rc = unsafe {
            xc_domain_getinfolist(
                self.interface.handle.0,
                self.domain_id.0,
                1,
                &mut info as *mut _,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(info.into())
    }

    pub fn maximum_gpfn(&self) -> Result<u64, XenError> {
        let mut gpfn = 0;
        let rc =
            unsafe { xc_domain_maximum_gpfn(self.interface.handle.0, self.domain_id.0, &mut gpfn) };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(gpfn)
    }

    pub fn pause(&self) -> Result<(), XenError> {
        let rc = unsafe { xc_domain_pause(self.interface.handle.0, self.domain_id.0) };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn unpause(&self) -> Result<(), XenError> {
        let rc = unsafe { xc_domain_unpause(self.interface.handle.0, self.domain_id.0) };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn get_mem_access(&self, gfn: u64) -> Result<MemoryAccess, XenError> {
        let mut access = 0;
        let rc = unsafe {
            xc_get_mem_access(self.interface.handle.0, self.domain_id.0, gfn, &mut access)
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(MemoryAccess::from_bits_truncate(access as _))
    }

    pub fn set_mem_access(&self, gfn: u64, access: MemoryAccess) -> Result<(), XenError> {
        let rc = unsafe {
            xc_set_mem_access(
                self.interface.handle.0,
                self.domain_id.0,
                access.bits().into(),
                gfn,
                1,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn set_access_required(&self, required: bool) -> Result<(), XenError> {
        let rc = unsafe {
            xc_domain_set_access_required(
                self.interface.handle.0,
                self.domain_id.0,
                required.into(),
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn debug_control(&self, vcpu: VcpuId, operation: u32) -> Result<(), XenError> {
        let rc = unsafe {
            xc_domain_debug_control(
                self.interface.handle.0,
                self.domain_id.0,
                operation,
                vcpu.0.into(),
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn set_max_mem(&self, max_memkb: u64) -> Result<(), XenError> {
        let rc =
            unsafe { xc_domain_setmaxmem(self.interface.handle.0, self.domain_id.0, max_memkb) };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn increase_reservation(
        &self,
        extent_order: u32,
        mem_flags: u32,
        extents: &[u64],
    ) -> Result<(), XenError> {
        let rc = unsafe {
            xc_domain_increase_reservation(
                self.interface.handle.0,
                self.domain_id.0,
                extents.len() as u64,
                extent_order,
                mem_flags,
                extents.as_ptr() as *mut _,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn increase_reservation_exact(
        &self,
        extent_order: u32,
        mem_flags: u32,
        extents: &[u64],
    ) -> Result<(), XenError> {
        let rc = unsafe {
            xc_domain_increase_reservation_exact(
                self.interface.handle.0,
                self.domain_id.0,
                extents.len() as u64,
                extent_order,
                mem_flags,
                extents.as_ptr() as *mut _,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn decrease_reservation(&self, extent_order: u32, extents: &[u64]) -> Result<(), XenError> {
        let rc = unsafe {
            xc_domain_decrease_reservation(
                self.interface.handle.0,
                self.domain_id.0,
                extents.len() as u64,
                extent_order,
                extents.as_ptr() as *mut _,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn decrease_reservation_exact(
        &self,
        extent_order: u32,
        extents: &[u64],
    ) -> Result<(), XenError> {
        let rc = unsafe {
            xc_domain_decrease_reservation_exact(
                self.interface.handle.0,
                self.domain_id.0,
                extents.len() as u64,
                extent_order,
                extents.as_ptr() as *mut _,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn populate_physmap(
        &self,
        extent_order: u32,
        mem_flags: u32,
        extents: &[u64],
    ) -> Result<(), XenError> {
        let rc = unsafe {
            xc_domain_populate_physmap(
                self.interface.handle.0,
                self.domain_id.0,
                extents.len() as u64,
                extent_order,
                mem_flags,
                extents.as_ptr() as _,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn populate_physmap_exact(
        &self,
        extent_order: u32,
        mem_flags: u32,
        extents: &[u64],
    ) -> Result<(), XenError> {
        let rc = unsafe {
            xc_domain_populate_physmap_exact(
                self.interface.handle.0,
                self.domain_id.0,
                extents.len() as u64,
                extent_order,
                mem_flags,
                extents.as_ptr() as _,
            )
        };
        xc_check_error!(self.interface.handle.0, rc);
        Ok(())
    }

    pub fn altp2m(&self) -> Result<XenAltP2M, XenError> {
        XenAltP2M::new(self.interface.clone(), self.domain_id)
    }

    pub fn monitor(&self) -> Result<(XenMonitor, VmEventRing), XenError> {
        XenMonitor::new(self.interface.clone(), self.domain_id)
    }

    pub fn device_model(&self) -> Result<XenDeviceModel, XenError> {
        XenDeviceModel::new(self.domain_id)
    }
}
