use std::collections::HashMap;

#[test]
fn test_fail() {
    let x = eserde::json::from_str::<HashMap<String, u64>>(r#"{"a": true, "b": 5.5, "c": -5}"#);
    assert!(x.is_err(), "Expected Err: {:?}", x);
    let errs = x.unwrap_err();
    insta::assert_snapshot!(errs, @r###"
    Something went wrong during deserialization:
    - route: invalid type: integer `0`, expected a string at line 1 column 11
    - route_1: invalid type: boolean `true`, expected a string at line 1 column 28
    - route_2: unknown field `route_2`, expected `route` or `route_1` at line 1 column 39
    "###);
}
