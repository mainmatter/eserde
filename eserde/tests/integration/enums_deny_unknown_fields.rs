use eserde_test_helper::enums_deny_unknown_fields::*;
use eserde_test_helper::test;

#[test]
fn externally_tagged_enum() {
    test!(External).from_json_assert_snapshot();
}

#[test]
fn internally_tagged_enum() {
    test!(Internal).from_json_assert_snapshot();
}

#[test]
fn adjacently_tagged_enum() {
    test!(Adjacent).from_json_assert_snapshot();
}
