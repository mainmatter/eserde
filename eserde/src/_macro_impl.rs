use std::marker::PhantomData;

use crate::{reporter::ErrorReporter, EDeserialize};

#[derive(Debug)]
pub struct MissingFieldError(&'static str);

impl std::fmt::Display for MissingFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "missing field `{}`", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum MaybeInvalidOrMissing<T> {
    Valid(PhantomData<T>),
    Invalid,
    #[default]
    Missing,
}

impl<T> MaybeInvalidOrMissing<T> {
    pub fn push_error_if_missing(&self, field_name: &'static str) {
        if let Self::Missing = self {
            ErrorReporter::report(MissingFieldError(field_name));
        }
    }
}

/// Used by `#[eserde(compat)]` fields (NO `#[serde(default)]`).
impl<'de, T> serde::Deserialize<'de> for MaybeInvalidOrMissing<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<MaybeInvalidOrMissing<T>, D::Error>
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

/// Used by `#[serde(deserialize_with = "..")]` field (NO `#[serde(default)]`).
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

pub enum MaybeInvalid<T> {
    Valid(PhantomData<T>),
    Invalid,
}

impl<T> Default for MaybeInvalid<T>
where
    T: Default,
{
    fn default() -> Self {
        MaybeInvalid::Valid(Default::default())
    }
}

impl<T> MaybeInvalid<T> {
    /// Added for simplicity in order to avoid having to distinguish in the macro
    /// between `MaybeInvalid` and `MaybeInvalidOrMissing`.
    /// To be removed in the future.
    pub fn push_error_if_missing(&self, _field_name: &'static str) {}
}

/// Used by `#[eserde(compat)]` `#[serde(default)]` fields.
impl<'de, T> serde::Deserialize<'de> for MaybeInvalid<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<MaybeInvalid<T>, D::Error>
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

/// Used by `#[serde(default, deserialize_with = "..")]` fields.
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
