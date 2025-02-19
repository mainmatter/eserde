use eserde_test_helper::enums_flattened::*;
use eserde_test_helper::test;

#[test]
fn enums_flattened() {
    test!(Container).from_json_assert_snapshot();
}

#[test]
fn enums_flattened_deny_unknown_fields() {
    test!(ContainerDenyUnknownFields).from_json_assert_snapshot();
}
