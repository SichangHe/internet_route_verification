# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1](https://github.com/SichangHe/internet_route_verification/compare/route_verification-v0.5.0...route_verification-v0.5.1) - 2024-05-10

### Other
- count community filter [#158](https://github.com/SichangHe/internet_route_verification/pull/158)
- RIB stats for all4
- `pypy3` → `python` in Rust to use with rye
- resolve PeerAS at run time
- make new clippy happy
- BGPq3-compatible rules CDF plot [#137](https://github.com/SichangHe/internet_route_verification/pull/137)
- basic last-modified stats&CDF [#130](https://github.com/SichangHe/internet_route_verification/pull/130)
- update filter AS script after verbosity changes

## [0.4.1](https://github.com/SichangHe/internet_route_verification/compare/route_verification-v0.4.0...route_verification-v0.4.1) - 2024-04-19

### Fixed
- fix transit AS stats `self`-related calculation [#134](https://github.com/SichangHe/internet_route_verification/pull/134)
- fix hanging on `route_first_hop_sender` not closed

### Other
- minor simplification
- \#transit AS rule w/ same AS in peering&filter [#134](https://github.com/SichangHe/internet_route_verification/pull/134)
- attempt to get all route objects w/o common mntner;dump route objects
- write to new RIB stats dir
- other only provider policies cases;de-prioritize uphill
- working finished transit_as_rules count [#134](https://github.com/SichangHe/internet_route_verification/pull/134)
- gzip all stats before writing to disk [#141](https://github.com/SichangHe/internet_route_verification/pull/141)
- attempt to collect route first hop stats [#141](https://github.com/SichangHe/internet_route_verification/pull/141)
- collect `spec_import_customer` in `RouteStats`
- release
