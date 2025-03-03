#![allow(dead_code)]

use itertools::Itertools;
use std::borrow::Cow;

#[derive(eserde::Deserialize)]
struct NamedStruct {
    #[serde(default)]
    a: Option<u32>,
    b: TupleStructOneField,
    c: Vec<TupleStructMultipleFields>,
}

#[derive(eserde::Deserialize)]
struct GenericStruct<T, S: std::any::Any> {
    // #[eserde(compat)]
    a: T,
    #[eserde(compat)]
    b: S,
}

#[derive(eserde::Deserialize)]
struct LifetimeGenericStruct<'a, 'b, 'c, 'd, 'e: 'a> {
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

#[derive(eserde::Deserialize)]
struct TupleStructOneField(#[serde(default)] Option<u32>);

#[derive(eserde::Deserialize)]
struct TupleStructMultipleFields(Option<u32>, u32, #[serde(default)] u64);

#[derive(eserde::Deserialize)]
enum CLikeEnumOneVariant {
    A,
}

#[derive(eserde::Deserialize)]
enum CLikeEnumMultipleVariants {
    A,
    B,
}

#[derive(eserde::Deserialize)]
enum EnumWithBothNamedAndTupleVariants {
    Named { a: u32 },
    NamedMultiple { a: u32, b: u64 },
    Tuple(u32),
    TupleMultiple(u32, u64),
    Unit,
}

// #[test]
// fn deserialize() {
//     let payloads = [
//         r#"{
//             "b": 5,
//             "c": [[1, 2, 3], [4, 5, 6]]
//         }"#,
//         r#"{
//             "a": 5,
//             "b": null,
//             "c": [[null, 2, 3], [4, 5, 6]]
//         }"#,
//     ];
//     for payload in payloads {
//         assert!(
//             serde_json::from_str::<NamedStruct>(payload).is_ok(),
//             "Failed to deserialize: {}",
//             payload
//         );
//     }
// }

#[test]
fn deser_for_errors() {
    #[derive(Debug, eserde::Deserialize)]
    struct TopLevelStruct {
        a: LeafStruct,
        b: u64,
        c: String,
        #[eserde(compat)]
        d: IncompatibleLeafStruct,
    }

    #[derive(Debug, eserde::Deserialize)]
    struct LeafStruct {
        #[serde(default)]
        a2: Option<u32>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct IncompatibleLeafStruct {
        #[serde(default)]
        a2: Option<u32>,
    }

    let payload = r#"{
    "a": { "a2": -5 },
    "c": 8
}"#;

    let value = eserde::json::from_str::<TopLevelStruct>(payload);
    let error = value.unwrap_err();
    let error_repr = error.into_iter().map(|e| e.to_string()).join("\n");
    insta::assert_snapshot!(error_repr, @r###"
    a.a2: invalid value: integer `-5`, expected u32 at line 2 column 19
    c: invalid type: integer `8`, expected a string at line 3 column 10
    missing field `b`
    missing field `d`
    "###);

    let value = serde_json::from_str::<TopLevelStruct>(payload);
    let error_repr = value.unwrap_err().to_string();
    insta::assert_snapshot!(error_repr, @"invalid value: integer `-5`, expected u32 at line 2 column 19");
}
