#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaybeInvalidOrMissing<T> {
    Valid(T),
    Invalid(String),
    Missing,
}

impl<T> MaybeInvalidOrMissing<T> {
    pub fn error<'de, D>(&self) -> Option<D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error as _;

        match self {
            Self::Valid(_) => None,
            Self::Invalid(e) => Some(D::Error::custom(e)),
            Self::Missing => Some(D::Error::missing_field("This field is missing")),
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
            Ok(value) => MaybeInvalidOrMissing::Valid(value),
            Err(error) => MaybeInvalidOrMissing::Invalid(error.to_string()),
        };
        Ok(v)
    }
}

pub enum MaybeInvalid<T> {
    Valid(T),
    Invalid(String),
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
    pub fn error<'de, D>(&self) -> Option<D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error as _;

        match self {
            Self::Valid(_) => None,
            Self::Invalid(e) => Some(D::Error::custom(e)),
        }
    }

    pub fn value(self) -> Option<T> {
        match self {
            Self::Valid(v) => Some(v),
            _ => None,
        }
    }
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
            Ok(value) => MaybeInvalid::Valid(value),
            Err(e) => MaybeInvalid::Invalid(e.to_string()),
        };
        Ok(v)
    }
}
