#[doc(hidden)]
pub use serde as _serde;

#[cfg(feature = "derive")]
pub use eserde_derive::Deserialize;
