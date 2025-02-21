use eserde_test_helper::assert_from_json_inline;
use eserde_test_helper::enums_deny_unknown_fields::*;
use eserde_test_helper::test;

#[test]
fn externally_tagged_enum() {
    let test = test!(serialized; External,
        r#"{"Struct":{"foo":1899123746,"bar":true}}"#
    );
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
    let test = test!(serialized; Internal,
        r#"{"tag":"Struct","foo":-1133362929,"bar":true}"#
    );
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
    let test = test!(serialized; Adjacent,
        r#"{"tag":"Struct","content":{"foo":1566241236,"bar":false}}"#
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
