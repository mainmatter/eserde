//! Test `serde_json::value` types.
#![allow(dead_code)]

use std::collections::HashMap;

// TODO: these should give more error messages than the first.
#[test]
fn test() {
    use serde_json::value::{Map, Number, Value};

    const PAYLOAD: &str = r#"{
        "a": -0.0,
        "b": { "a2": -5 },
        "c": 8,
        "d": { "a2": false }
    }"#;

    assert!(serde_json::from_str::<Value>(PAYLOAD).is_ok());
    assert!(serde_json::from_str::<Map<String, Value>>(PAYLOAD).is_ok());

    let result = eserde::json::from_str::<HashMap<String, Number>>(PAYLOAD);
    let errors = result.unwrap_err();
    insta::assert_snapshot!(errors, @r###"
    Something went wrong during deserialization:
    - b: invalid type: map, expected a JSON number at line 3 column 15
    "###
    );
}
