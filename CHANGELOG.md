# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
