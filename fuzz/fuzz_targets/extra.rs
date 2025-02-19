#![no_main]
#![allow(dead_code)]

use eserde_fuzz::fuzz_many;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|s: &str| {
    fuzz_many!(s, NamedStruct,
        GenericStruct<NamedStruct, EnumWithBothNamedAndTupleVariants>,
        TupleStructOneField,
        TupleStructOneField,
        TupleStructMultipleFields,
        CLikeEnumOneVariant,
        EnumWithBothNamedAndTupleVariants,
    );

    let _ = eserde::json::from_str::<LifetimeGenericStruct>(s);
});

use std::borrow::Cow;

#[derive(Debug, serde::Serialize, eserde::Deserialize, arbitrary::Arbitrary)]
struct NamedStruct {
    #[serde(default)]
    a: Option<u32>,
    b: TupleStructOneField,
    c: Vec<TupleStructMultipleFields>,
}

#[derive(Debug, serde::Serialize, eserde::Deserialize, arbitrary::Arbitrary)]
struct GenericStruct<T, S> {
    // #[eserde(compat)]
    a: T,
    #[eserde(compat)]
    b: S,
}

#[derive(Debug, serde::Serialize, eserde::Deserialize, arbitrary::Arbitrary)]
struct LifetimeGenericStruct<'a, 'b, 'c, 'd, 'e> {
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

#[derive(Debug, serde::Serialize, eserde::Deserialize, arbitrary::Arbitrary)]
struct TupleStructOneField(#[serde(default)] Option<u32>);

#[derive(Debug, serde::Serialize, eserde::Deserialize, arbitrary::Arbitrary)]
struct TupleStructMultipleFields(Option<u32>, u32, #[serde(default)] u64);

#[derive(Debug, serde::Serialize, eserde::Deserialize, arbitrary::Arbitrary)]
enum CLikeEnumOneVariant {
    A,
}

#[derive(Debug, serde::Serialize, eserde::Deserialize, arbitrary::Arbitrary)]
enum CLikeEnumMultipleVariants {
    A,
    B,
}

#[derive(Debug, serde::Serialize, eserde::Deserialize, arbitrary::Arbitrary)]
enum EnumWithBothNamedAndTupleVariants {
    Named { a: u32 },
    NamedMultiple { a: u32, b: u64 },
    Tuple(u32),
    TupleMultiple(u32, u64),
    Unit,
}
