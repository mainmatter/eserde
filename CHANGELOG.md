# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.6](https://github.com/mainmatter/eserde/compare/0.1.5...0.1.6) - 2025-03-12


### ⛰️ Features
- implement `EDeserialize` for rest of `std`/`serde_json` types, make `impl_edeserialize_compat!` public, fix #26 #27 #37 ([#36](https://github.com/mainmatter/eserde/pull/36)) (by @MingweiSamuel) - #36
- support `#[serde(with = "..")]` on fields, #18 ([#40](https://github.com/mainmatter/eserde/pull/40)) (by @MingweiSamuel) - #40



### 🐛 Bug Fixes
- allow `!Default` for `#[serde(default = "..")]` fields ([#35](https://github.com/mainmatter/eserde/pull/35)) (by @MingweiSamuel) - #35


### Contributors

* @MingweiSamuel

## [0.1.5](https://github.com/mainmatter/eserde/compare/0.1.4...0.1.5) - 2025-03-05


### ⛰️ Features
- support `#[serde(deserialize_with = "..")]`, `#[serde(default = "..")]` for fields, fix #21 ([#23](https://github.com/mainmatter/eserde/pull/23)) (by @MingweiSamuel) - #23


### Contributors

* @MingweiSamuel

## [0.1.4](https://github.com/mainmatter/eserde/compare/0.1.3...0.1.4) - 2025-03-03


### 🐛 Bug Fixes
- handle generic params with bounds ([#28](https://github.com/mainmatter/eserde/pull/28)) (by @MingweiSamuel) - #28


### Contributors

* @MingweiSamuel

## [0.1.3](https://github.com/mainmatter/eserde/compare/0.1.2...0.1.3) - 2025-03-03


### 🐛 Bug Fixes
- error message ordering (by @MingweiSamuel) - #25
- ensure `parse_nested_meta` properly handles values, fix [#24](https://github.com/mainmatter/eserde/pull/24) (by @MingweiSamuel) - #25



### 🧪 Testing
- add basic trybuild tests (by @MingweiSamuel) - #25


### Contributors

* @MingweiSamuel
* @hdoordt

## [0.1.2](https://github.com/mainmatter/eserde/compare/0.1.1...0.1.2) - 2025-02-14


### ⛰️ Features
- Introduce `eserde_axum`, to provide `axum` extractors built on top of `eserde`. (by @LukeMathWalker) - #11



### 📚 Documentation
- Expand `eserde`'s crate documentation to mention `eserde_axum` as well as the underlying deserialization mechanism. (by @LukeMathWalker) - #11


### Contributors

* @LukeMathWalker

## [0.1.1](https://github.com/mainmatter/eserde/compare/0.1.0...0.1.1) - 2025-02-13


### 📚 Documentation
- Enable the unstable rustdoc feature required to show, on docs.rs, what feature flags must be enabled for specific items. (by @LukeMathWalker)


### Contributors

* @LukeMathWalker
