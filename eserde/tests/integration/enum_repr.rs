use eserde_test_helper::enum_repr::*;
use eserde_test_helper::test;

#[test]
fn enum_repr() {
    test!(Enum).from_json_assert_snapshot();
}
