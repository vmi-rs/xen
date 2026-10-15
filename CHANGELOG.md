# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **Breaking:** libxen-sys generates its bindings with bindgen 0.73.2 instead
  of 0.72.1. A struct with bitfields names its alignment field
  `_bindgen_align` instead of `_bitfield_align_1`, and opaque handle types
  such as `xc_interface_core` no longer derive `Copy` and `Clone`. The
  bindings shipped for docs.rs and `XEN_SYS_USE_BINDINGS` are unchanged.
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
