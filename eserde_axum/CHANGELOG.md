# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.4](https://github.com/mainmatter/eserde/compare/eserde_axum-0.1.3...eserde_axum-0.1.4) - 2025-03-05


### ⛰️ Features
- support `#[serde(deserialize_with = "..")]`, `#[serde(default = "..")]` for fields, fix #21 ([#23](https://github.com/mainmatter/eserde/pull/23)) (by @MingweiSamuel) - #23


### Contributors

* @MingweiSamuel

## [0.1.3](https://github.com/mainmatter/eserde/compare/eserde_axum-0.1.2...eserde_axum-0.1.3) - 2025-03-03


### 🐛 Bug Fixes
- handle generic params with bounds ([#28](https://github.com/mainmatter/eserde/pull/28)) (by @MingweiSamuel) - #28


### Contributors

* @MingweiSamuel

## [0.1.2](https://github.com/mainmatter/eserde/compare/eserde_axum-0.1.1...eserde_axum-0.1.2) - 2025-03-03


### 🐛 Bug Fixes
- error message ordering (by @MingweiSamuel) - #25
- ensure `parse_nested_meta` properly handles values, fix [#24](https://github.com/mainmatter/eserde/pull/24) (by @MingweiSamuel) - #25


### Contributors

* @MingweiSamuel

## [0.1.1](https://github.com/mainmatter/eserde/compare/eserde_axum-0.1.0...eserde_axum-0.1.1) - 2025-02-14


### 📚 Documentation
- Enable unstable rustdoc feature to provide feature-flag information docs.rs (by @LukeMathWalker) - #14


### Contributors

* @LukeMathWalker

## [0.1.0](https://github.com/mainmatter/eserde/releases/tag/eserde_axum-0.1.0) - 2025-02-14


### ⛰️ Features
- Introduce `eserde_axum`, to provide `axum` extractors built on top of `eserde`. (by @LukeMathWalker) - #11


### Contributors

* @LukeMathWalker
