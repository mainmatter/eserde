use super::path::Segment;
use super::tracker::PathTracker;
use super::wrap::{Wrap, WrapVariant};
use serde::de::{self, DeserializeSeed, Visitor};
use std::fmt;

/// Deserializer adapter that records path to deserialization errors.
pub struct Deserializer<D> {
    de: D,
}

impl<D> Deserializer<D> {
    pub fn new(de: D) -> Self {
        Deserializer { de }
    }
}

// Plain old forwarding impl.
impl<'de, D> de::Deserializer<'de> for Deserializer<D>
where
    D: de::Deserializer<'de>,
{
    type Error = D::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_any(Wrap::new(visitor))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_bool(Wrap::new(visitor))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_u8(Wrap::new(visitor))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_u16(Wrap::new(visitor))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_u32(Wrap::new(visitor))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_u64(Wrap::new(visitor))
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_u128(Wrap::new(visitor))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_i8(Wrap::new(visitor))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_i16(Wrap::new(visitor))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_i32(Wrap::new(visitor))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_i64(Wrap::new(visitor))
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_i128(Wrap::new(visitor))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_f32(Wrap::new(visitor))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_f64(Wrap::new(visitor))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_char(Wrap::new(visitor))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_str(Wrap::new(visitor))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_string(Wrap::new(visitor))
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_bytes(Wrap::new(visitor))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_byte_buf(Wrap::new(visitor))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_option(Wrap::new(visitor))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_unit(Wrap::new(visitor))
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_unit_struct(name, Wrap::new(visitor))
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_newtype_struct(name, Wrap::new(visitor))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_seq(Wrap::new(visitor))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_tuple(len, Wrap::new(visitor))
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de
            .deserialize_tuple_struct(name, len, Wrap::new(visitor))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_map(Wrap::new(visitor))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_struct(name, fields, Wrap::new(visitor))
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_enum(name, variants, Wrap::new(visitor))
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_ignored_any(Wrap::new(visitor))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_identifier(Wrap::new(visitor))
    }

    fn is_human_readable(&self) -> bool {
        self.de.is_human_readable()
    }
}

// Forwarding impl to preserve context.
impl<'de, X> Visitor<'de> for Wrap<X>
where
    X: Visitor<'de>,
{
    type Value = X::Value;

    fn visit_seq<V>(self, visitor: V) -> Result<Self::Value, V::Error>
    where
        V: de::SeqAccess<'de>,
    {
        self.delegate.visit_seq(SeqAccess::new(visitor))
    }

    fn visit_map<V>(self, visitor: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        self.delegate.visit_map(MapAccess::new(visitor))
    }

    fn visit_enum<V>(self, visitor: V) -> Result<Self::Value, V::Error>
    where
        V: de::EnumAccess<'de>,
    {
        self.delegate.visit_enum(Wrap::new(visitor))
    }

    // After this, all boring forwarding methods.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.delegate.expecting(formatter)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_bool(v)
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_i8(v)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_i16(v)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_i32(v)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_i64(v)
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_i128(v)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_u8(v)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_u16(v)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_u32(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_u64(v)
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_u128(v)
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_f32(v)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_f64(v)
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_char(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_str(v)
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_borrowed_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_string(v)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_unit()
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_none()
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self.delegate.visit_some(Deserializer::new(deserializer))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self.delegate
            .visit_newtype_struct(Deserializer::new(deserializer))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_bytes(v)
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_borrowed_bytes(v)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_byte_buf(v)
    }
}

// Forwarding impl to preserve context.
impl<'de, X> de::EnumAccess<'de> for Wrap<X>
where
    X: de::EnumAccess<'de>,
{
    type Error = X::Error;
    type Variant = WrapVariant<X::Variant>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), X::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let mut variant = None;
        let outcome = self
            .delegate
            .variant_seed(CaptureKey::new(seed, &mut variant))
            .map(move |(v, vis)| (v, WrapVariant::new(vis)));

        if let Some(variant) = variant {
            println!("Pushing from enum");
            PathTracker::push(Segment::Enum { variant });
        }
        if outcome.is_err() {
            PathTracker::stash_current_path_for_error();
        }

        outcome
    }
}

// Forwarding impl to preserve context.
impl<'de, X> de::VariantAccess<'de> for WrapVariant<X>
where
    X: de::VariantAccess<'de>,
{
    type Error = X::Error;

    fn unit_variant(self) -> Result<(), X::Error> {
        let o = self.delegate.unit_variant();
        PathTracker::pop();
        o
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, X::Error>
    where
        T: DeserializeSeed<'de>,
    {
        let outcome = self.delegate.newtype_variant_seed(TrackedSeed::new(seed));
        PathTracker::pop();
        outcome
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        let outcome = self.delegate.tuple_variant(len, Wrap::new(visitor));
        PathTracker::pop();
        outcome
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        let outcome = self.delegate.struct_variant(fields, Wrap::new(visitor));
        PathTracker::pop();
        outcome
    }
}

// Seed that saves the string into the given optional during `visit_str` and
// `visit_string`.
struct CaptureKey<'a, X> {
    delegate: X,
    key: &'a mut Option<String>,
}

impl<'a, X> CaptureKey<'a, X> {
    fn new(delegate: X, key: &'a mut Option<String>) -> Self {
        CaptureKey { delegate, key }
    }
}

// Forwarding impl.
impl<'a, 'de, X> DeserializeSeed<'de> for CaptureKey<'a, X>
where
    X: DeserializeSeed<'de>,
{
    type Value = X::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<X::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self.delegate
            .deserialize(CaptureKey::new(deserializer, self.key))
    }
}

// Forwarding impl.
impl<'a, 'de, X> de::Deserializer<'de> for CaptureKey<'a, X>
where
    X: de::Deserializer<'de>,
{
    type Error = X::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_any(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_bool(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_u8(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_u16(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_u32(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_u64(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_u128(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_i8(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_i16(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_i32(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_i64(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_i128(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_f32(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_f64(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_char(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_str(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_string(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_bytes(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_byte_buf(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_option(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_unit(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_unit_struct(name, CaptureKey::new(visitor, self.key))
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_newtype_struct(name, CaptureKey::new(visitor, self.key))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_seq(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_tuple(len, CaptureKey::new(visitor, self.key))
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_tuple_struct(name, len, CaptureKey::new(visitor, self.key))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_map(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_struct(name, fields, CaptureKey::new(visitor, self.key))
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_enum(name, variants, CaptureKey::new(visitor, self.key))
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_ignored_any(CaptureKey::new(visitor, self.key))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, X::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate
            .deserialize_identifier(CaptureKey::new(visitor, self.key))
    }

    fn is_human_readable(&self) -> bool {
        self.delegate.is_human_readable()
    }
}

// Forwarding impl except `visit_str` and `visit_string` which save the string.
impl<'a, 'de, X> Visitor<'de> for CaptureKey<'a, X>
where
    X: Visitor<'de>,
{
    type Value = X::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.delegate.expecting(formatter)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let string = if v { "true" } else { "false" };
        *self.key = Some(string.to_owned());
        self.delegate.visit_bool(v)
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        *self.key = Some(itoa::Buffer::new().format(v).to_owned());
        self.delegate.visit_i8(v)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        *self.key = Some(itoa::Buffer::new().format(v).to_owned());
        self.delegate.visit_i16(v)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        *self.key = Some(itoa::Buffer::new().format(v).to_owned());
        self.delegate.visit_i32(v)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        *self.key = Some(itoa::Buffer::new().format(v).to_owned());
        self.delegate.visit_i64(v)
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        *self.key = Some(itoa::Buffer::new().format(v).to_owned());
        self.delegate.visit_i128(v)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        *self.key = Some(itoa::Buffer::new().format(v).to_owned());
        self.delegate.visit_u8(v)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        *self.key = Some(itoa::Buffer::new().format(v).to_owned());
        self.delegate.visit_u16(v)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        *self.key = Some(itoa::Buffer::new().format(v).to_owned());
        self.delegate.visit_u32(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        *self.key = Some(itoa::Buffer::new().format(v).to_owned());
        self.delegate.visit_u64(v)
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        *self.key = Some(itoa::Buffer::new().format(v).to_owned());
        self.delegate.visit_u128(v)
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_f32(v)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_f64(v)
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_char(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        *self.key = Some(v.to_owned());
        self.delegate.visit_str(v)
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        *self.key = Some(v.to_owned());
        self.delegate.visit_borrowed_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        *self.key = Some(v.clone());
        self.delegate.visit_string(v)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_unit()
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_none()
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self.delegate.visit_some(deserializer)
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self.delegate.visit_newtype_struct(deserializer)
    }

    fn visit_seq<V>(self, visitor: V) -> Result<Self::Value, V::Error>
    where
        V: de::SeqAccess<'de>,
    {
        self.delegate.visit_seq(visitor)
    }

    fn visit_map<V>(self, visitor: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        self.delegate.visit_map(visitor)
    }

    fn visit_enum<V>(self, visitor: V) -> Result<Self::Value, V::Error>
    where
        V: de::EnumAccess<'de>,
    {
        self.delegate.visit_enum(visitor)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_bytes(v)
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_borrowed_bytes(v)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_byte_buf(v)
    }
}

// Seed used for map values, sequence elements and newtype variants to track
// their path.
struct TrackedSeed<X> {
    seed: X,
}

impl<X> TrackedSeed<X> {
    fn new(seed: X) -> Self {
        TrackedSeed { seed }
    }
}

impl<'de, X> DeserializeSeed<'de> for TrackedSeed<X>
where
    X: DeserializeSeed<'de>,
{
    type Value = X::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<X::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self.seed.deserialize(Deserializer { de: deserializer })
    }
}

// Seq visitor that tracks the index of its elements.
struct SeqAccess<X> {
    delegate: X,
    index: usize,
}

impl<X> SeqAccess<X> {
    fn new(delegate: X) -> Self {
        SeqAccess { delegate, index: 0 }
    }
}

// Forwarding impl to preserve context.
impl<'de, X> de::SeqAccess<'de> for SeqAccess<X>
where
    X: de::SeqAccess<'de>,
{
    type Error = X::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, X::Error>
    where
        T: DeserializeSeed<'de>,
    {
        PathTracker::push(Segment::Seq { index: self.index });
        self.index += 1;
        let outcome = self.delegate.next_element_seed(TrackedSeed::new(seed));
        if outcome.is_err() {
            PathTracker::stash_current_path_for_error();
        }
        PathTracker::pop();
        outcome
    }

    fn size_hint(&self) -> Option<usize> {
        self.delegate.size_hint()
    }
}

// Map visitor that captures the string value of its keys and uses that to track
// the path to its values.
struct MapAccess<X> {
    delegate: X,
    key: Option<String>,
}

impl<X> MapAccess<X> {
    fn new(delegate: X) -> Self {
        MapAccess {
            delegate,
            key: None,
        }
    }
}

impl<'de, X> de::MapAccess<'de> for MapAccess<X>
where
    X: de::MapAccess<'de>,
{
    type Error = X::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, X::Error>
    where
        K: DeserializeSeed<'de>,
    {
        let key = &mut self.key;
        let outcome = self.delegate.next_key_seed(CaptureKey::new(seed, key));
        if let Some(key) = key.take() {
            PathTracker::push(Segment::Map { key });
        }
        if outcome.is_err() {
            PathTracker::stash_current_path_for_error();
        }
        outcome
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, X::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let outcome = self.delegate.next_value_seed(TrackedSeed::new(seed));
        if outcome.is_err() {
            PathTracker::stash_current_path_for_error();
        }
        PathTracker::pop();
        outcome
    }

    fn size_hint(&self) -> Option<usize> {
        self.delegate.size_hint()
    }
}
