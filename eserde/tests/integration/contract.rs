use eserde_test_helper::contract::*;
use eserde_test_helper::test;

#[test]
fn struct_deny_unknown_fields() {
    test!(StructDenyUnknownFields).from_json_assert_snapshot();
}

#[test]
fn struct_allow_unknown_fields() {
    test!(StructAllowUnknownFields).from_json_assert_snapshot();
}

#[test]
fn tuple_struct() {
    test!(TupleStruct).from_json_assert_snapshot();
}

#[test]
fn externally_tagged_enum() {
    test!(ExternalEnum).from_json_assert_snapshot();
}

#[test]
fn internally_tagged_enum() {
    test!(InternalEnum).from_json_assert_snapshot();
}

#[test]
fn adjacently_tagged_enum() {
    test!(AdjacentEnum).from_json_assert_snapshot();
}
