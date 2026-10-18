# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

### Added

- `XenDomain::get_context` returns a domain's whole HVM context as a
  `XenHvmContext`, and `XenDomain::set_context` loads it back.
  `XenHvmContext::get` returns a copy of one record, and
  `XenHvmContext::update` changes records in place. A record's struct, such
  as `hvm_hw_cpu`, implements the sealed trait `XenHvmSaveRecord`.
- `XenForeignMemory::map_resource` maps pages that Xen owns for a domain into
  the calling process, such as the pages that Xen shares with an ioreq
  server. Xen calls such a group of pages a resource. The mapping is a
  `XenForeignMemoryResource`.
- `XenIoreqPage` maps the page through which Xen passes a domain's device
  accesses to its ioreq server, such as QEMU. `XenIoreqPage::pending` returns
  `true` while a vCPU waits for such an access to finish.
- libxen-sys binds `xen/hvm/ioreq.h`. The shipped 4.20 and 4.21 bindings
  come from their bindgen versions 0.71.1 and 0.72.1. They leave out
  `buf_ioreq` and `buffered_iopage`, because those versions get the layout of
  the two structs wrong.

### Fixed

- `XenStore::domain_id_from_name` skips a domain that disappears while the
  lookup runs. Before, if another program destroyed a domain at that moment,
  the whole lookup failed with "Failed to read xen store".

## [0.8.0] - 2026-10-15

### Added

- `XenEventChannelPort::pending` and `unmask` are public, so that a caller
  can keep the port masked after an event and unmask it later.

### Fixed

- `RING_HAS_UNCONSUMED_REQUESTS`, and with it
  `VmEventRing::unconsumed_requests` and `has_unconsumed_requests`, read the
  ring's `req_prod` with an acquire load. A loop that waits for a request
  sees each new one, and the request is read after its index.

## [0.7.0] - 2026-10-15

### Changed

- **Breaking:** libxen-sys generates its bindings with bindgen 0.73.2 instead
  of 0.72.1. A struct with bitfields names its alignment field
  `_bindgen_align` instead of `_bitfield_align_1`, and opaque handle types
  such as `xc_interface_core` no longer derive `Copy` and `Clone`. The
  shipped Xen 4.22 bindings, which docs.rs and `XEN_SYS_USE_BINDINGS` use,
  are regenerated with bindgen 0.73.2 against glibc 2.41. The shipped 4.20
  and 4.21 bindings keep the output of bindgen 0.72.
- A lock file that already holds prettyplease 0.2 and syn 3 can make bindgen
  0.73.2 fail to compile, with `ParseQuote` not implemented for `syn::File`
  (rust-bindgen#3448). `cargo update -p prettyplease` fixes the lock file.

### Added

- `XenControl::evtchn_status` returns the status of a domain's event channel
  port as a `XenEventChannelStatus`. The status holds the port's binding, a
  `XenEventChannelState`, and the vCPU that receives the port's events.
- `XenDomainId::SELF` stands for the calling domain, `DOMID_SELF` in Xen's
  headers.

### Fixed

- libxen-sys reruns its build script when a Xen header, `wrapper.h` or the
  selected shipped bindings change, and when `DOCS_RS` or
  `XEN_SYS_USE_BINDINGS` changes. Before, a Xen upgrade or a change of either
  variable kept the old bindings.

## [0.6.0] - 2026-08-27

This is the last release before this changelog. The git history lists its
changes and those of earlier releases.
