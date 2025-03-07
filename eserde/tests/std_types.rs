//! Test error reporting for `std` types.
#![allow(dead_code)]

use eserde::DeserializationErrors;

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

#[test]
fn test_transparent() {
    const PAYLOAD: &str = r#"{
        "a": { "a2": -5 },
        "c": 8,
        "d": { "a2": false }
    }"#;

    insta::allow_duplicates! {
        // Fully transparent, all should behave the same.
        check(eserde::json::from_str::<TopLevelStruct>(PAYLOAD));
        check(eserde::json::from_str::<Box<TopLevelStruct>>(PAYLOAD));
        check(eserde::json::from_str::<std::cmp::Reverse<TopLevelStruct>>(PAYLOAD));
        check(eserde::json::from_str::<std::cell::RefCell<TopLevelStruct>>(PAYLOAD));
        // If `PAYLOAD` is not `"null"`, `Option<T>` should behave the same.
        check(eserde::json::from_str::<Option<TopLevelStruct>>(PAYLOAD));
    }
    fn check<T: std::fmt::Debug>(result: Result<T, DeserializationErrors>) {
        let errors = result.unwrap_err();
        insta::assert_snapshot!(errors, @r###"
        Something went wrong during deserialization:
        - a.a2: invalid value: integer `-5`, expected u32 at line 2 column 23
        - c: invalid type: integer `8`, expected a string at line 3 column 14
        - d.a2: invalid type: boolean `false`, expected u32 at line 4 column 26
        - missing field `b`
        "###
        );
    }
}

#[test]
fn test_seqs() {
    const PAYLOAD: &str = r#"[
        {
            "a": { "a2": 15 },
            "b": 42,
            "c": "foo",
            "d": { "a2": 100 }
        },
        {
            "a": { "a2": -5 },
            "c": 8
        },
        {
            "a": { "a2": 15 },
            "b": 42,
            "c": "foo",
            "d": {}
        },
        {
            "a": { "a2": -5 },
            "c": 8
        }
    ]"#;

    insta::allow_duplicates! {
        check(eserde::json::from_str::<[TopLevelStruct; 4]>(PAYLOAD));
        check(eserde::json::from_str::<Box<[TopLevelStruct]>>(PAYLOAD));
        check(eserde::json::from_str::<std::collections::LinkedList<TopLevelStruct>>(PAYLOAD));
        check(eserde::json::from_str::<std::collections::VecDeque<TopLevelStruct>>(PAYLOAD));
        check(eserde::json::from_str::<Vec<TopLevelStruct>>(PAYLOAD));
    }
    fn check<T: std::fmt::Debug>(result: Result<T, DeserializationErrors>) {
        let errors = result.unwrap_err();
        insta::assert_snapshot!(errors, @r###"
        Something went wrong during deserialization:
        - [1].a.a2: invalid value: integer `-5`, expected u32 at line 9 column 27
        - [1].c: invalid type: integer `8`, expected a string at line 10 column 18
        - [1]: missing field `b`
        - [1]: missing field `d`
        - [3].a.a2: invalid value: integer `-5`, expected u32 at line 19 column 27
        - [3].c: invalid type: integer `8`, expected a string at line 20 column 18
        - [3]: missing field `b`
        - [3]: missing field `d`
        "###
        );
    }

    // Input is too long.
    let errors = eserde::json::from_str::<[TopLevelStruct; 3]>(PAYLOAD).unwrap_err();
    insta::assert_snapshot!(errors, @r###"
    Something went wrong during deserialization:
    - [1].a.a2: invalid value: integer `-5`, expected u32 at line 9 column 27
    - [1].c: invalid type: integer `8`, expected a string at line 10 column 18
    - [1]: missing field `b`
    - [1]: missing field `d`
    - [3].a.a2: invalid value: integer `-5`, expected u32 at line 19 column 27
    - [3].c: invalid type: integer `8`, expected a string at line 20 column 18
    - [3]: missing field `b`
    - [3]: missing field `d`
    - expected sequence of 3 elements, found 4 elements.
    "###);

    // Input is too short.
    let errors = eserde::json::from_str::<[TopLevelStruct; 5]>(PAYLOAD).unwrap_err();
    insta::assert_snapshot!(errors, @r###"
    Something went wrong during deserialization:
    - [1].a.a2: invalid value: integer `-5`, expected u32 at line 9 column 27
    - [1].c: invalid type: integer `8`, expected a string at line 10 column 18
    - [1]: missing field `b`
    - [1]: missing field `d`
    - [3].a.a2: invalid value: integer `-5`, expected u32 at line 19 column 27
    - [3].c: invalid type: integer `8`, expected a string at line 20 column 18
    - [3]: missing field `b`
    - [3]: missing field `d`
    - expected sequence of 5 elements, found 4 elements.
    "###);
}

#[test]
fn test_map_basic() {
    const PAYLOAD: &str = r#"{"a": true, "b": 5.5, "c": -5, "d": {}}"#;

    insta::allow_duplicates! {
        check(eserde::json::from_str::<std::collections::HashMap<String, u64>>(PAYLOAD));
        check(eserde::json::from_str::<std::collections::BTreeMap<String, u64>>(PAYLOAD));
    }
    fn check<T: std::fmt::Debug>(x: Result<T, DeserializationErrors>) {
        let errs = x.unwrap_err();
        insta::assert_snapshot!(errs, @r###"
        Something went wrong during deserialization:
        - a: invalid type: boolean `true`, expected u64 at line 1 column 10
        - b: invalid type: floating point `5.5`, expected u64 at line 1 column 20
        - c: invalid value: integer `-5`, expected u64 at line 1 column 29
        - d: invalid type: map, expected u64 at line 1 column 36
        - expected `,` or `}` at line 1 column 37
        "###);
    }
}

#[test]
fn test_map_nested() {
    const PAYLOAD: &str = r#"{
        "foo": {
            "a": { "a2": 15 },
            "b": 42,
            "c": "foo",
            "d": { "a2": 100 }
        },
        "bar": {
            "a": { "a2": -5 },
            "c": 8
        },
        "baz": {
            "a": { "a2": 15 },
            "b": 42,
            "c": "foo",
            "d": {}
        },
        "bing": {
            "a": { "a2": -5 },
            "c": 8
        }
    }"#;

    insta::allow_duplicates! {
        check(eserde::json::from_str::<std::collections::HashMap<String, TopLevelStruct>>(PAYLOAD));
        check(eserde::json::from_str::<std::collections::BTreeMap<String, TopLevelStruct>>(PAYLOAD));
    }
    fn check<T: std::fmt::Debug>(x: Result<T, DeserializationErrors>) {
        let errs = x.unwrap_err();
        insta::assert_snapshot!(errs, @r###"
        Something went wrong during deserialization:
        - bar.a.a2: invalid value: integer `-5`, expected u32 at line 9 column 27
        - bar.c: invalid type: integer `8`, expected a string at line 10 column 18
        - bar: missing field `b`
        - bar: missing field `d`
        - bing.a.a2: invalid value: integer `-5`, expected u32 at line 19 column 27
        - bing.c: invalid type: integer `8`, expected a string at line 20 column 18
        - bing: missing field `b`
        - bing: missing field `d`
        "###);
    }
}
