use eserde_test_helper::assert_from_json_inline;
use eserde_test_helper::contract::*;
use eserde_test_helper::test;

#[test]
fn struct_deny_unknown_fields() {
    let test = test!(serialized; StructDenyUnknownFields,
        r#"{"DEFAULT":true,"SKIP-SERIALIZING-IF":true,"renamed":false,"OPTION":false}"#
    );
    assert_from_json_inline!(test, @r#"
    Err(
        DeserializationErrors(
            [
                DeserializationError {
                    path: Some(
                        Path {
                            segments: [],
                        },
                    ),
                    details: "missing field `write_only`",
                },
            ],
        ),
    )
    "#);
}

#[test]
fn struct_allow_unknown_fields() {
    let test = test!(serialized; StructAllowUnknownFields,
        r#"{"DEFAULT":false,"renamed":false,"OPTION":null}"#
    );
    assert_from_json_inline!(test, @r#"
    Err(
        DeserializationErrors(
            [
                DeserializationError {
                    path: Some(
                        Path {
                            segments: [],
                        },
                    ),
                    details: "missing field `write_only`",
                },
                DeserializationError {
                    path: Some(
                        Path {
                            segments: [],
                        },
                    ),
                    details: "missing field `skip_serializing_if`",
                },
            ],
        ),
    )
    "#);
}

#[test]
fn tuple_struct() {
    let test = test!(serialized; TupleStruct,
        r#"["XVUgTUKTJ7J8r","yZwcp1Ge","nf9hN3"]"#
    );
    assert_from_json_inline!(test, @r#"
    Ok(
        TupleStruct(
            "XVUgTUKTJ7J8r",
            false,
            "yZwcp1Ge",
            "nf9hN3",
        ),
    )
    "#);
}

#[test]
fn externally_tagged_enum() {
    let test = test!(serialized; ExternalEnum,
        r#""renamed_unit""#
    );
    assert_from_json_inline!(test, @r"
    Ok(
        RenamedUnit,
    )
    ");
}

#[test]
fn internally_tagged_enum() {
    let test = test!(serialized; InternalEnum,
        r#"{"tag":"renamed_unit"}"#
    );
    assert_from_json_inline!(test, @r"
    Ok(
        RenamedUnit,
    )
    ");
}

#[test]
fn adjacently_tagged_enum() {
    let test = test!(serialized; AdjacentEnum,
        r#"{"tag":"renamed_unit"}"#
    );
    assert_from_json_inline!(test, @r"
    Ok(
        RenamedUnit,
    )
    ");
}
