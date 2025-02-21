use eserde_test_helper::assert_from_json_inline;
use eserde_test_helper::structs::*;
use eserde_test_helper::test_helper::TestHelper;

#[test]
fn unit() {
    let test = TestHelper::<UnitStruct>::new_serialized(r#"null"#);
    assert_from_json_inline!(test, @r"
    Ok(
        UnitStruct,
    )
    ");
}

#[test]
fn normal() {
    let test = TestHelper::<NormalStruct>::new_serialized(r#"{"foo":"fvsTNa45C","bar":false}"#);
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
    let test = TestHelper::<NewType>::new_serialized(r#""F71VZOS""#);
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
    let test = TestHelper::<TupleStruct>::new_serialized(r#"["FPoREowVSC0CjkC",false]"#);
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
    let test = TestHelper::<RenamedFields>::new_serialized(
        r#"{"camelCase":-1608793701,"new_name":-663097910}"#,
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
    let test = TestHelper::<DenyUnknownFields>::new_serialized(
        r#"{"foo":"3nrBBXVgrpwpQ9tDK8","bar":false}"#,
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
