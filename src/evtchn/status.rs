use xen_sys::{
    EVTCHNSTAT_closed, EVTCHNSTAT_interdomain, EVTCHNSTAT_ipi, EVTCHNSTAT_pirq, EVTCHNSTAT_unbound,
    EVTCHNSTAT_virq, evtchn_status,
};

use crate::{VcpuId, XenDomainId, XenError};

/// Status of an event channel port, as `EVTCHNOP_status` reports it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XenEventChannelStatus {
    /// Binding when Xen answered. The binding can change right after, for
    /// example when the remote domain closes its end.
    pub state: XenEventChannelState,

    /// vCPU of the queried domain that receives the port's events.
    pub vcpu: VcpuId,
}

/// Binding of an event channel port. The binding decides which source raises
/// the port's events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XenEventChannelState {
    /// The port is free. A port whose remote end closes becomes
    /// [`Unbound`](Self::Unbound) instead.
    Closed,

    /// The port waits for a remote domain to connect to it.
    Unbound {
        /// The only domain that may connect to the port.
        remote_domain_id: XenDomainId,
    },

    /// The port is connected to a port of a remote domain, which can be the
    /// port's own domain.
    Interdomain {
        remote_domain_id: XenDomainId,

        /// Port number within the remote domain.
        remote_port: u32,
    },

    /// The port delivers a physical IRQ. The value is the IRQ's number in the
    /// port's domain.
    Pirq(u32),

    /// The port delivers a virtual IRQ. The value is a `VIRQ_*` number.
    Virq(u32),

    /// The port carries interprocessor interrupts between the vCPUs of its
    /// domain.
    Ipi,
}

impl TryFrom<evtchn_status> for XenEventChannelStatus {
    type Error = XenError;

    #[expect(non_upper_case_globals)]
    fn try_from(value: evtchn_status) -> Result<Self, Self::Error> {
        // The status value selects the union member that Xen wrote.
        let state = match value.status {
            EVTCHNSTAT_closed => XenEventChannelState::Closed,
            EVTCHNSTAT_unbound => XenEventChannelState::Unbound {
                remote_domain_id: XenDomainId(unsafe { value.u.unbound.dom }.into()),
            },
            EVTCHNSTAT_interdomain => XenEventChannelState::Interdomain {
                remote_domain_id: XenDomainId(unsafe { value.u.interdomain.dom }.into()),
                remote_port: unsafe { value.u.interdomain.port },
            },
            EVTCHNSTAT_pirq => XenEventChannelState::Pirq(unsafe { value.u.pirq }),
            EVTCHNSTAT_virq => XenEventChannelState::Virq(unsafe { value.u.virq }),
            EVTCHNSTAT_ipi => XenEventChannelState::Ipi,
            _ => return Err(XenError::Other("Unknown event channel status")),
        };

        Ok(Self {
            state,
            vcpu: VcpuId(value.vcpu as u16),
        })
    }
}
