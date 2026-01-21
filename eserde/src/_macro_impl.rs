use std::marker::PhantomData;

use serde::de::Deserialize;

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
        D: serde::de::Deserializer<'de>,
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
    D: serde::de::Deserializer<'de>,
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

impl<T> Default for MaybeInvalid<T> {
    fn default() -> Self {
        MaybeInvalid::Valid(PhantomData)
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
        D: serde::de::Deserializer<'de>,
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
    D: serde::de::Deserializer<'de>,
    T: EDeserialize<'de>,
{
    let v = match T::deserialize_for_errors(deserializer) {
        Ok(_) => MaybeInvalid::Valid(Default::default()),
        Err(_) => MaybeInvalid::Invalid,
    };
    Ok(v)
}

/// A way of deserializing any data type from a deserializer.
///
/// Think of this like `serde_json::Value` in that it can be deserialized from any type, except that it
/// format-agnostic.
///
/// Also like [`serde::de::IgnoredAny`], but it actually keeps the data.
///
/// This is used to handle `#[serde(deny_unknown_fields)]` in a roundabout way -- we deserialize extra
/// fields as `#[serde(flatten)] Map<String, TypedAny>`, which we can then turn into errors.
#[derive(Clone, Debug, PartialEq)]
pub enum TypedAny {
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F64,
    Char,
    Str,
    None,
    Some(Box<TypedAny>),
    NewtypeStruct(Box<TypedAny>),
    Unit,
    Seq(Vec<TypedAny>),
    Map(Vec<(TypedAny, TypedAny)>),
    Bytes,
    Enum {
        variant: Box<TypedAny>,
        value: Box<TypedAny>,
    },
}

struct TypedAnyVisitor;
impl<'de> serde::de::Visitor<'de> for TypedAnyVisitor {
    type Value = TypedAny;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("anything at all")
    }

    #[inline]
    fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
        Ok(TypedAny::Bool)
    }

    #[inline]
    fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
        Ok(TypedAny::I8)
    }

    #[inline]
    fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
        Ok(TypedAny::I16)
    }

    #[inline]
    fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
        Ok(TypedAny::I32)
    }

    #[inline]
    fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
        Ok(TypedAny::I64)
    }

    #[inline]
    fn visit_i128<E>(self, _: i128) -> Result<Self::Value, E> {
        Ok(TypedAny::I128)
    }

    #[inline]
    fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
        Ok(TypedAny::U8)
    }

    #[inline]
    fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
        Ok(TypedAny::U16)
    }

    #[inline]
    fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
        Ok(TypedAny::U32)
    }

    #[inline]
    fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
        Ok(TypedAny::U64)
    }

    #[inline]
    fn visit_u128<E>(self, _: u128) -> Result<Self::Value, E> {
        Ok(TypedAny::U128)
    }

    #[inline]
    fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
        Ok(TypedAny::F64)
    }

    #[inline]
    fn visit_str<E>(self, _: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TypedAny::Str)
    }

    #[inline]
    fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TypedAny::Str)
    }

    #[inline]
    fn visit_string<E>(self, _: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TypedAny::Str)
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(TypedAny::None)
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        TypedAny::deserialize(deserializer)
            .map(Box::new)
            .map(TypedAny::Some)
    }

    #[inline]
    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        TypedAny::deserialize(deserializer)
            .map(Box::new)
            .map(TypedAny::NewtypeStruct)
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(TypedAny::Unit)
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut elements = Vec::new();
        while let Some(elem) = seq.next_element()? {
            elements.push(elem);
        }
        Ok(TypedAny::Seq(elements))
    }

    #[inline]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut entries = Vec::new();
        while let Some((key, value)) = map.next_entry()? {
            entries.push((key, value));
        }
        Ok(TypedAny::Map(entries))
    }

    #[inline]
    fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TypedAny::Bytes)
    }

    #[inline]
    fn visit_borrowed_bytes<E>(self, _: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TypedAny::Bytes)
    }

    #[inline]
    fn visit_byte_buf<E>(self, _: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TypedAny::Bytes)
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        let (variant_name, variant_access) = data.variant::<TypedAny>()?;
        let variant_value = serde::de::VariantAccess::newtype_variant(variant_access);
        Ok(TypedAny::Enum {
            variant: Box::new(variant_name),
            value: Box::new(variant_value?),
        })
    }
}

impl<'de> Deserialize<'de> for TypedAny {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<TypedAny, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_any(TypedAnyVisitor)
    }
}
