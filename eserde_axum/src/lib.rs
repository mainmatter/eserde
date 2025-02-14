//! # eserde_axum
//!
//! A collection of [`axum`] extractors built on top of [`eserde`] to
//! provide exhaustive error reports when deserialization fails.
//! They are designed to be drop-in replacement for their official [`axum`]
//! counterpart.
//!
//! Check out [`Json`] for working with JSON payloads.
//!
//! [`axum`]: https://docs.rs/axum
//! [`eserde`]: https://docs.rs/eserde
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub mod json;

#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub use json::Json;

pub(crate) mod details;
