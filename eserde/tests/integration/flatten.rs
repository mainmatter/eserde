use eserde_test_helper::flatten::*;
use eserde_test_helper::test;

#[test]
fn flattened_struct() {
    test!(Deep1).from_json_assert_snapshot();
}

#[test]
fn flattened_value() {
    test!(FlattenValue).from_json_assert_snapshot();
}

#[test]
fn flattened_map() {
    test!(FlattenMap).from_json_assert_snapshot();
}
