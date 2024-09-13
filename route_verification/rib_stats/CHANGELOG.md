# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.5](https://github.com/SichangHe/internet_route_verification/compare/route_verification_rib_stats-v0.1.4...route_verification_rib_stats-v0.1.5) - 2024-09-13

### Other

- rm WIP notice for ARTIFACTS.md
- mention irv server
- new RIB stats

## [0.1.4](https://github.com/SichangHe/internet_route_verification/compare/route_verification_rib_stats-v0.1.3...route_verification_rib_stats-v0.1.4) - 2024-08-18

### Other
- explain every crate
- link main README to each crate for [#194](https://github.com/SichangHe/internet_route_verification/pull/194)

## [0.1.3](https://github.com/SichangHe/internet_route_verification/compare/route_verification_rib_stats-v0.1.2...route_verification_rib_stats-v0.1.3) - 2024-05-14

### Other
- remember to bump all RIB stats index
- RIB stats for all4

## [0.1.2](https://github.com/SichangHe/internet_route_verification/compare/route_verification_rib_stats-v0.1.1...route_verification_rib_stats-v0.1.2) - 2024-04-19

### Fixed
- fix hanging on `route_first_hop_sender` not closed

### Other
- write to new RIB stats dir
- gzip all stats before writing to disk [#141](https://github.com/SichangHe/internet_route_verification/pull/141)
- attempt to collect route first hop stats [#141](https://github.com/SichangHe/internet_route_verification/pull/141)
