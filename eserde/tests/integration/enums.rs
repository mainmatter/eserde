use eserde_test_helper::assert_from_json_inline;
use eserde_test_helper::enums::*;
use eserde_test_helper::test;

#[test]
fn externally_tagged_enum() {
    let test = test!(serialized; External,
        r#"{"unitStructNewType":null}"#
    );
    assert_from_json_inline!(test, @r"
    Ok(
        UnitStructNewType(
            UnitStruct,
        ),
    )
    ");
}

#[test]
fn internally_tagged_enum() {
    let test = test!(serialized; Internal,
        r#"{"tag":"UnitOne"}"#
    );
    assert_from_json_inline!(test, @r"
    Ok(
        UnitOne,
    )
    ");
}

#[test]
fn adjacently_tagged_enum() {
    let test = test!(serialized; Adjacent,
        r#"{"tag":"Tuple","content":[-427070648,true]}"#
    );
    assert_from_json_inline!(test, @r"
    Ok(
        Tuple(
            -427070648,
            true,
        ),
    )
    ");
}

#[test]
fn untagged_enum() {
    let test = test!(serialized; UntaggedWrapper,
        r#"[-521833035,true]"#
    );
    assert_from_json_inline!(test, @r"
    Ok(
        UntaggedWrapper(
            StructNewType(
                Struct {
                    foo: -521833035,
                    bar: true,
                },
            ),
        ),
    )
    ");
}

#[test]
fn renamed() {
    let test = test!(serialized; Renamed,
        r#"{"struct_variant":{"FIELD":"8AtP50nUcNy1f"}}"#
    );
    assert_from_json_inline!(test, @r#"
    Ok(
        StructVariant {
            field: "8AtP50nUcNy1f",
        },
    )
    "#);
}
