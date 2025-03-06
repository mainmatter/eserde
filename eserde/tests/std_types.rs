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

// TODO: these should give more error messages than the first.
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
        }
    ]"#;

    insta::allow_duplicates! {
        check(eserde::json::from_str::<[TopLevelStruct; 3]>(PAYLOAD));
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
        "###
        );
    }

    // Input is too long.
    let errors = eserde::json::from_str::<[TopLevelStruct; 2]>(PAYLOAD).unwrap_err();
    insta::assert_snapshot!(errors, @r###"
    Something went wrong during deserialization:
    - [1].a.a2: invalid value: integer `-5`, expected u32 at line 9 column 27
    "###);

    // Input is too short.
    let errors = eserde::json::from_str::<[TopLevelStruct; 4]>(PAYLOAD).unwrap_err();
    insta::assert_snapshot!(errors, @r###"
    Something went wrong during deserialization:
    - [1].a.a2: invalid value: integer `-5`, expected u32 at line 9 column 27
    "###);
}
