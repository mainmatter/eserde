#[cfg(feature = "json")]
pub mod json;

#[doc(hidden)]
pub use serde as _serde;

#[doc(hidden)]
pub mod _macro_impl;

#[cfg(feature = "derive")]
pub use eserde_derive::Deserialize;

pub trait HumanDeserialize<'de>: Sized + serde::Deserialize<'de> {
    /// Deserialize this value using the given `serde` deserializer.
    ///
    /// # `HumanDeserialize` vs `serde::Deserialize`
    ///
    /// `serde::Deserialize` is designed to abort deserialization
    /// as soon as an error is encountered.
    /// This is optimal for speed, but it can result in a frustrating
    /// experience for the user, who has to fix errors one by one.
    ///
    /// `HumanDeserialize`, instead, tries to accumulate as many errors
    /// as possible before returning them to the user, so that they can fix them all
    /// in one go.
    /// As a result, `HumanDeserialize` is likely to be
    /// slower than `serde::Deserialize`.
    fn human_deserialize<D>(deserializer: D) -> Result<Self, Vec<D::Error>>
    where
        D: serde::Deserializer<'de>;
}
