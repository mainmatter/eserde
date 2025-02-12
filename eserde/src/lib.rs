#[cfg(feature = "json")]
pub mod json;

mod impl_;
pub mod path;
pub mod reporter;

#[doc(hidden)]
pub use serde as _serde;

#[doc(hidden)]
pub mod _macro_impl;

#[cfg(feature = "derive")]
/// A derive macro to automatically implement [`EDeserialize`] and `serde::Deserialize` for a type.
pub use eserde_derive::Deserialize;

#[diagnostic::on_unimplemented(
    note = "Annotate the problematic type with `#[derive(eserde::EDeserialize)]` to implement the missing trait.\n\n\
    It may not always be possible to add the annotation, e.g. if the type is defined in another crate that you don't control.\n\
    If that's the case, and you're using that type for one of your fields, you can annotate the field instead!\n\
    Add `#[eserde(compat)]` on the field to instruct `eserde` to fallback to the vanilla deserialization logic for that type, \
    removing the `EDeserialize` requirement.\n"
)]
/// A variant of `serde::Deserialize` that accumulates as many errors as possible
/// before returning them to the user.
pub trait EDeserialize<'de>: Sized + serde::Deserialize<'de> {
    /// Deserialize this value using the given `serde` deserializer.
    ///
    /// # `EDeserialize` vs `serde::Deserialize`
    ///
    /// `serde::Deserialize` is designed to abort deserialization
    /// as soon as an error is encountered.
    /// This is optimal for speed, but it can result in a frustrating
    /// experience for the user, who has to fix errors one by one.
    ///
    /// `EDeserialize`, instead, tries to accumulate as many errors
    /// as possible before returning them to the user, so that they can fix them all
    /// in one go.
    /// As a result, `EDeserialize` is likely to be
    /// slower than `serde::Deserialize`.
    ///
    /// # Errors
    ///
    /// If deserialization fails, this function will return an `Err(())`.
    /// To retrieve the error details, check the [`ErrorReporter::take_errors`](crate::reporter::ErrorReporter::take_errors) function.
    fn deserialize_for_errors<D>(deserializer: D) -> Result<Self, ()>
    where
        D: serde::Deserializer<'de>;
}

#[derive(Debug)]
/// An error that occurred during deserialization.
pub struct DeserializationError {
    /// The input path at which the error occurred, when available.
    ///
    /// E.g. if the error occurred while deserializing the sub-field `foo` of the top-level
    /// field `bar`, the path would be `bar.foo`.
    pub path: Option<path::Path>,
    /// What went wrong during deserialization.
    pub details: DeserializationErrorDetails,
}

impl std::fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path) = &self.path {
            if !path.segments.is_empty() {
                write!(f, "{}: ", path)?;
            }
        }
        write!(f, "{}", self.details)
    }
}

#[derive(Debug)]
/// Details as to what went wrong during deserialization.
///
/// Part of a [`DeserializationError`].
pub enum DeserializationErrorDetails {
    /// A field was missing during deserialization.
    MissingField { field_name: &'static str },
    /// A failure occurred during deserialization,
    /// with a custom error message.
    Custom { message: String },
}

impl std::fmt::Display for DeserializationErrorDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializationErrorDetails::MissingField { field_name } => {
                write!(f, "missing field `{}`", field_name)
            }
            DeserializationErrorDetails::Custom { message } => write!(f, "{}", message),
        }
    }
}
