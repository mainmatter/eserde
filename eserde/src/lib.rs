#[cfg(feature = "json")]
pub mod json;

mod error;
mod impl_;

pub use error::DeserializationError;

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
    ///
    /// # Errors
    ///
    /// If deserialization fails, this function will return an `Err(())`.
    /// To retrieve the error details, check the [`DESERIALIZATION_ERRORS`] thread-local.
    ///
    /// You're expected to set the thread-local to an empty vector before starting a deserialization
    /// operation, otherwise errors from previous deserializations will be included in the current
    /// deserialization.
    ///
    /// This is usually taken care of by the format-specific functions provided by `eserde`,
    /// such as [`eserde::json::from_str`].
    ///
    /// [`DESERIALIZATION_ERRORS`]: thread_local!@DESERIALIZATION_ERRORS
    fn human_deserialize<D>(deserializer: D) -> Result<Self, ()>
    where
        D: serde::Deserializer<'de>;
}

thread_local! {
    /// Errors that occurred during deserialization.
    ///
    /// # Why a thread-local?
    ///
    /// We use a thread-local since we are constrained by the signature of `serde`'s `Deserialize`
    /// trait, so we can't pass down a `&mut Vec<_>` to accumulate errors.
    ///
    /// # Constraints
    ///
    /// It is the responsibility of the user to ensure that the thread-local is empty before
    /// starting a deserialization operation, otherwise errors from previous deserializations
    /// will be included in the current deserialization.
    /// This is usually taken care of by the format-specific functions provided by `eserde`,
    /// such as `eserde::json::from_str`.
    pub static DESERIALIZATION_ERRORS: std::cell::RefCell<Vec<DeserializationError>> = std::cell::RefCell::new(Vec::new());
}
