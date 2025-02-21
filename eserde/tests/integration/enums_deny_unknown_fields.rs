use eserde_test_helper::assert_from_json_inline;
use eserde_test_helper::enums_deny_unknown_fields::*;
use eserde_test_helper::test_helper::TestHelper;

#[test]
fn externally_tagged_enum() {
    let test =
        TestHelper::<External>::new_serialized(r#"{"Struct":{"foo":1899123746,"bar":true}}"#);
    assert_from_json_inline!(test, @r"
    Ok(
        Struct {
            foo: 1899123746,
            bar: true,
        },
    )
    ");
}

#[test]
fn internally_tagged_enum() {
    let test =
        TestHelper::<Internal>::new_serialized(r#"{"tag":"Struct","foo":-1133362929,"bar":true}"#);
    assert_from_json_inline!(test, @r"
    Ok(
        Struct {
            foo: -1133362929,
            bar: true,
        },
    )
    ");
}

#[test]
fn adjacently_tagged_enum() {
    let test = TestHelper::<Adjacent>::new_serialized(
        r#"{"tag":"Struct","content":{"foo":1566241236,"bar":false}}"#,
    );
    assert_from_json_inline!(test, @r"
    Ok(
        Struct {
            foo: 1566241236,
            bar: false,
        },
    )
    ");
}
