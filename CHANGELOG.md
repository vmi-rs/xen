# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `XenControl::evtchn_status` returns the status of a domain's event channel
  port as a `XenEventChannelStatus`. The status holds the port's binding, a
  `XenEventChannelState`, and the vCPU that receives the port's events.
- `XenDomainId::SELF` stands for the calling domain, `DOMID_SELF` in Xen's
  headers.

## [0.6.0] - 2026-08-27

This is the last release before this changelog. The git history lists its
changes and those of earlier releases.
