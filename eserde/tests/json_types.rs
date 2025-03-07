//! Test `serde_json::value` types.
#![allow(dead_code)]

use std::collections::HashMap;

#[test]
fn test() {
    use serde_json::value::{Map, Number, Value};

    const PAYLOAD: &str = r#"{
        "a": -0.0,
        "b": true,
        "c": 8,
        "d": { "a2": false },
        "e": "foo"
    }"#;

    assert!(serde_json::from_str::<Value>(PAYLOAD).is_ok());
    assert!(serde_json::from_str::<Map<String, Value>>(PAYLOAD).is_ok());

    let result = eserde::json::from_str::<HashMap<String, Number>>(PAYLOAD);
    let errors = result.unwrap_err();
    // Note the errors degrade after the `"d": {` due to the curly brace.
    insta::assert_snapshot!(errors, @r###"
    Something went wrong during deserialization:
    - b: invalid type: boolean `true`, expected a JSON number at line 3 column 17
    - d: invalid type: map, expected a JSON number at line 5 column 15
    - expected `,` or `}` at line 5 column 16
    "###
    );
}
