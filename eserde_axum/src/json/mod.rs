//! Supporting types for the [`Json`] extractor.
mod json_;
mod rejections;

#[doc(hidden)]
pub use json_::Json;
pub use rejections::*;
