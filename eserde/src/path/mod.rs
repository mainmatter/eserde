//! Represent locations in structured data through a hierarchical path structure.
//!
//! The main type is [`Path`], which represents a full path sequence
//! composed of individual [`Segment`]s.
//!
//! ## Design
//!
//! The design for this module was inspired by the approach followed in
//! [`serde_path_to_error`](https://crates.io/crates/serde_path_to_error).
mod de;
mod path_;
mod tracker;
mod wrap;

#[allow(unused)]
pub(crate) use de::Deserializer;
pub use path_::{Path, Segment, Segments};
pub(crate) use tracker::PathTracker;
