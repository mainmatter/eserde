use crate::{reporter::ErrorReporter, DeserializationError, HumanDeserialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaybeInvalidOrMissing<T> {
    Valid(T),
    Invalid,
    Missing,
}

impl<T> MaybeInvalidOrMissing<T> {
    pub fn push_error_if_missing(&self, field_name: &'static str) {
        if let Self::Missing = self {
            ErrorReporter::report(DeserializationError::MissingField { field_name });
        }
    }

    pub fn value(self) -> Option<T> {
        match self {
            Self::Valid(v) => Some(v),
            _ => None,
        }
    }
}

impl<T> Default for MaybeInvalidOrMissing<T> {
    fn default() -> Self {
        MaybeInvalidOrMissing::Missing
    }
}

impl<'de, T> serde::Deserialize<'de> for MaybeInvalidOrMissing<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<MaybeInvalidOrMissing<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = match T::deserialize(deserializer) {
            Ok(value) => Self::Valid(value),
            Err(error) => {
                ErrorReporter::report(DeserializationError::Custom {
                    message: error.to_string(),
                });
                Self::Invalid
            }
        };
        Ok(v)
    }
}

pub fn maybe_invalid_or_missing_human_deserialize<'de, D, T>(
    deserializer: D,
) -> Result<MaybeInvalidOrMissing<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: HumanDeserialize<'de>,
{
    let v = match T::human_deserialize(deserializer) {
        Ok(value) => MaybeInvalidOrMissing::Valid(value),
        Err(_) => MaybeInvalidOrMissing::Invalid,
    };
    Ok(v)
}

pub enum MaybeInvalid<T> {
    Valid(T),
    Invalid,
}

impl<T> Default for MaybeInvalid<T>
where
    T: Default,
{
    fn default() -> Self {
        MaybeInvalid::Valid(T::default())
    }
}

impl<T> MaybeInvalid<T> {
    pub fn value(self) -> Option<T> {
        match self {
            Self::Valid(v) => Some(v),
            _ => None,
        }
    }

    /// Added for simplicity in order to avoid having to distinguish in the macro
    /// between `MaybeInvalid` and `MaybeInvalidOrMissing`.
    /// To be removed in the future.
    pub fn push_error_if_missing(&self, _field_name: &'static str) {}
}

impl<'de, T> serde::Deserialize<'de> for MaybeInvalid<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<MaybeInvalid<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = match T::deserialize(deserializer) {
            Ok(value) => Self::Valid(value),
            Err(error) => {
                ErrorReporter::report(DeserializationError::Custom {
                    message: error.to_string(),
                });
                Self::Invalid
            }
        };
        Ok(v)
    }
}

pub fn maybe_invalid_human_deserialize<'de, D, T>(
    deserializer: D,
) -> Result<MaybeInvalid<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: HumanDeserialize<'de>,
{
    let v = match T::human_deserialize(deserializer) {
        Ok(value) => MaybeInvalid::Valid(value),
        Err(_) => MaybeInvalid::Invalid,
    };
    Ok(v)
}
