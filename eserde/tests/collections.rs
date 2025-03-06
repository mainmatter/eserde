use std::collections::HashMap;

#[test]
fn test_fail() {
    let x = eserde::json::from_str::<HashMap<String, u64>>(r#"{"a": true, "b": 5.5, "c": -5}"#);
    assert!(x.is_err(), "Expected Err: {:?}", x);
    let errs = x.unwrap_err();
    insta::assert_snapshot!(errs, @r###"
    Something went wrong during deserialization:
    - a: invalid type: boolean `true`, expected u64 at line 1 column 10
    "###);
}
