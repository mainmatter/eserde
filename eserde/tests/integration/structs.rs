use eserde_test_helper::structs::*;
use eserde_test_helper::test;

#[test]
fn unit() {
    test!(UnitStruct).from_json_assert_snapshot();
}

#[test]
fn normal() {
    test!(NormalStruct).from_json_assert_snapshot();
}

#[test]
fn newtype() {
    test!(NewType).from_json_assert_snapshot();
}

#[test]
fn tuple() {
    test!(TupleStruct).from_json_assert_snapshot();
}

#[test]
fn renamed_fields() {
    test!(RenamedFields).from_json_assert_snapshot();
}

#[test]
fn deny_unknown_fields() {
    test!(DenyUnknownFields).from_json_assert_snapshot();
}
