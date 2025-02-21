use eserde_test_helper::assert_from_json_inline;
use eserde_test_helper::structs::*;
use eserde_test_helper::test;

#[test]
fn unit() {
    let test = test!(serialized; UnitStruct,
        r#"null"#
    );
    assert_from_json_inline!(test, @r"
    Ok(
        UnitStruct,
    )
    ");
}

#[test]
fn normal() {
    let test = test!(serialized; NormalStruct,
        r#"{"foo":"fvsTNa45C","bar":false}"#
    );
    assert_from_json_inline!(test, @r#"
    Ok(
        NormalStruct {
            foo: "fvsTNa45C",
            bar: false,
        },
    )
    "#);
}

#[test]
fn newtype() {
    let test = test!(serialized; NewType,
        r#""F71VZOS""#
    );
    assert_from_json_inline!(test, @r#"
    Ok(
        NewType(
            "F71VZOS",
        ),
    )
    "#);
}

#[test]
fn tuple() {
    let test = test!(serialized; TupleStruct,
        r#"["FPoREowVSC0CjkC",false]"#
    );
    assert_from_json_inline!(test, @r#"
    Ok(
        TupleStruct(
            "FPoREowVSC0CjkC",
            false,
        ),
    )
    "#);
}

#[test]
fn renamed_fields() {
    let test = test!(serialized; RenamedFields,
        r#"{"camelCase":-1608793701,"new_name":-663097910}"#
    );
    assert_from_json_inline!(test, @r"
    Ok(
        RenamedFields {
            camel_case: -1608793701,
            old_name: -663097910,
        },
    )
    ");
}

#[test]
fn deny_unknown_fields() {
    let test = test!(serialized; DenyUnknownFields,
        r#"{"foo":"3nrBBXVgrpwpQ9tDK8","bar":false}"#
    );
    assert_from_json_inline!(test, @r#"
    Ok(
        DenyUnknownFields {
            foo: "3nrBBXVgrpwpQ9tDK8",
            bar: false,
        },
    )
    "#);
}
