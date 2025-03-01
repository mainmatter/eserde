use crate::prelude::*;
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize, Arbitrary)]
pub struct NamedStruct {
    #[serde(default)]
    a: Option<u32>,
    b: TupleStructOneField,
    c: Vec<TupleStructMultipleFields>,
}

#[derive(Debug, Serialize, Deserialize, Arbitrary)]
pub struct GenericStruct<T: Default, S> {
    // #[eserde(compat)]
    a: T,
    #[eserde(compat)]
    b: S,
}

#[derive(Debug, Serialize, Deserialize, Arbitrary)]
pub struct LifetimeGenericStruct<'a, 'b, 'c, 'd, 'e: 'a> {
    #[serde(borrow)]
    a: Cow<'a, str>,
    // `&str` and `&[u8]` are special-cased by `serde`
    // and treated as if `#[serde(borrow)]` was applied.
    b: &'b str,
    c: &'c [u8],
    d: Cow<'d, str>,
    // Check that we don't add `borrow` twice, angering `serde`
    #[serde(borrow)]
    e: &'e str,
}

#[derive(Debug, Serialize, Deserialize, Arbitrary)]
pub struct TupleStructOneField(#[serde(default)] Option<u32>);

#[derive(Debug, Serialize, Deserialize, Arbitrary)]
pub struct TupleStructMultipleFields(Option<u32>, u32, #[serde(default)] u64);

#[derive(Debug, Serialize, Deserialize, Arbitrary)]
pub enum CLikeEnumOneVariant {
    A,
}

#[derive(Debug, Serialize, Deserialize, Arbitrary)]
pub enum CLikeEnumMultipleVariants {
    A,
    B,
}

#[derive(Debug, Serialize, Deserialize, Arbitrary)]
pub enum EnumWithBothNamedAndTupleVariants {
    Named { a: u32 },
    NamedMultiple { a: u32, b: u64 },
    Tuple(u32),
    TupleMultiple(u32, u64),
    Unit,
}
