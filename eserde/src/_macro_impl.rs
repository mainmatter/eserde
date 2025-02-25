use std::marker::PhantomData;

use crate::{reporter::ErrorReporter, EDeserialize};

#[derive(Debug)]
pub struct MissingFieldError(&'static str);

impl std::fmt::Display for MissingFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "missing field `{}`", self.0)
    }
}

/// The helper type which wraps all fields in generated code to allow deserialization to continue
/// when fields are invalid or missing.
///
/// `ALLOW_MISSING` is set to true for `#[serde(default)]` fields, to avoid reporting missing
/// fields for those.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum MaybeInvalidOrMissing<T, const ALLOW_MISSING: bool = false> {
    Valid(PhantomData<T>),
    Invalid,
    #[default]
    Missing,
}
pub type MaybeInvalid<T> = MaybeInvalidOrMissing<T, true>;

impl<T, const ALLOW_MISSING: bool> MaybeInvalidOrMissing<T, ALLOW_MISSING> {
    pub fn push_error_if_missing(&self, field_name: &'static str) {
        if matches!(self, Self::Missing) && !ALLOW_MISSING {
            ErrorReporter::report(MissingFieldError(field_name));
        }
    }
}

impl<'de, T, const ALLOW_MISSING: bool> serde::Deserialize<'de>
    for MaybeInvalidOrMissing<T, ALLOW_MISSING>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<MaybeInvalidOrMissing<T, ALLOW_MISSING>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = match T::deserialize(deserializer) {
            Ok(_) => Self::Valid(Default::default()),
            Err(error) => {
                ErrorReporter::report(error);
                Self::Invalid
            }
        };
        Ok(v)
    }
}

pub fn maybe_invalid_or_missing<'de, D, T>(
    deserializer: D,
) -> Result<MaybeInvalidOrMissing<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: EDeserialize<'de>,
{
    let v = match T::deserialize_for_errors(deserializer) {
        Ok(_) => MaybeInvalidOrMissing::Valid(Default::default()),
        Err(_) => MaybeInvalidOrMissing::Invalid,
    };
    Ok(v)
}

pub fn maybe_invalid<'de, D, T>(deserializer: D) -> Result<MaybeInvalid<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: EDeserialize<'de>,
{
    let v = match T::deserialize_for_errors(deserializer) {
        Ok(_) => MaybeInvalid::Valid(Default::default()),
        Err(_) => MaybeInvalid::Invalid,
    };
    Ok(v)
}
