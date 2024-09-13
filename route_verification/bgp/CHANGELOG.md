# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.1](https://github.com/SichangHe/internet_route_verification/compare/route_verification_bgp-v0.7.0...route_verification_bgp-v0.7.1) - 2024-09-13

### Other

- rm WIP notice for ARTIFACTS.md
- mention irv server

## [0.7.0](https://github.com/SichangHe/internet_route_verification/compare/route_verification_bgp-v0.6.0...route_verification_bgp-v0.7.0) - 2024-08-18

### Other
- explain every crate
- link main README to each crate for [#194](https://github.com/SichangHe/internet_route_verification/pull/194)
- inline pub use for [#194](https://github.com/SichangHe/internet_route_verification/pull/194)
- Align "Export Self" (`spec_export_customers`) implementation w/ text ([#193](https://github.com/SichangHe/internet_route_verification/pull/193))
- explain the project
- make clippy happy on indentation;separate rust checks

## [0.6.0](https://github.com/SichangHe/internet_route_verification/compare/route_verification_bgp-v0.5.0...route_verification_bgp-v0.6.0) - 2024-05-14

### Other
- resolve PeerAS at run time

## [0.5.0](https://github.com/SichangHe/internet_route_verification/compare/route_verification_bgp-v0.4.0...route_verification_bgp-v0.5.0) - 2024-04-19

### Other
- minor simplification
- other only provider policies cases;de-prioritize uphill
- collect `spec_import_customer` in `RouteStats`

## [0.4.0](https://github.com/SichangHe/internet_route_verification/compare/route_verification_bgp-v0.3.0...route_verification_bgp-v0.4.0) - 2024-03-27

### Other
- Changes in `Report`: relax only provider policies special cases; add `SpecImportCustomer` ([#145](https://github.com/SichangHe/internet_route_verification/pull/145))
