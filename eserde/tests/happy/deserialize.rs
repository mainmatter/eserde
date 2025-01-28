#![allow(dead_code)]

use itertools::Itertools;

// #[derive(eserde::Deserialize)]
// struct NamedStruct {
//     #[serde(default)]
//     a: Option<u32>,
//     b: TupleStructOneField,
//     c: Vec<TupleStructMultipleFields>,
// }

// #[derive(eserde::Deserialize)]
// struct GenericStruct<T, S> {
//     a: T,
//     b: S,
// }

// #[derive(eserde::Deserialize)]
// struct LifetimeGenericStruct<'a> {
//     a: &'a str,
// }

// #[derive(eserde::Deserialize)]
// struct LifetimeAndGenericStruct<'a, T> {
//     a: &'a str,
//     b: T,
// }

// #[derive(eserde::Deserialize)]
// struct TupleStructOneField(#[serde(default)] Option<u32>);

// #[derive(eserde::Deserialize)]
// struct TupleStructMultipleFields(Option<u32>, u32, #[serde(default)] u64);

// #[derive(eserde::Deserialize)]
// enum CLikeEnumOneVariant {
//     A,
// }

// #[derive(eserde::Deserialize)]
// enum CLikeEnumMultipleVariants {
//     A,
//     B,
// }

// #[derive(eserde::Deserialize)]
// enum EnumWithBothNamedAndTupleVariants {
//     Named { a: u32 },
//     NamedMultiple { a: u32, b: u64 },
//     Tuple(u32),
//     TupleMultiple(u32, u64),
//     Unit,
// }

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
fn human_deserialize() {
    #[derive(Debug, eserde::Deserialize)]
    struct FlatStruct {
        #[serde(default)]
        a: Option<u32>,
        b: u64,
        c: String,
    }

    let payload = r#"{
            "a": -5,
            "c": 8
        }"#;

    let value = eserde::json::from_str::<FlatStruct>(payload);
    let error = value.unwrap_err();
    let error_repr = error.into_iter().map(|e| e.to_string()).join("\n");
    insta::assert_snapshot!(error_repr, @r###"
    invalid value: integer `-5`, expected u32 at line 2 column 19
    missing field `b`
    invalid type: integer `8`, expected a string at line 3 column 18
    "###);

    let value = serde_json::from_str::<FlatStruct>(payload);
    let error_repr = value.unwrap_err().to_string();
    insta::assert_snapshot!(error_repr, @"invalid value: integer `-5`, expected u32 at line 2 column 19");
}
