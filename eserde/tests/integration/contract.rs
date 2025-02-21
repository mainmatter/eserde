use eserde_test_helper::assert_from_json_inline;
use eserde_test_helper::contract::*;
use eserde_test_helper::test_helper::TestHelper;

#[test]
fn struct_deny_unknown_fields() {
    let test = TestHelper::<StructDenyUnknownFields>::new_serialized(
        r#"{"DEFAULT":true,"SKIP-SERIALIZING-IF":true,"renamed":false,"OPTION":false}"#,
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
    let test = TestHelper::<StructAllowUnknownFields>::new_serialized(
        r#"{"DEFAULT":false,"renamed":false,"OPTION":null}"#,
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
    let test =
        TestHelper::<TupleStruct>::new_serialized(r#"["XVUgTUKTJ7J8r","yZwcp1Ge","nf9hN3"]"#);
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
    let test = TestHelper::<ExternalEnum>::new_serialized(r#""renamed_unit""#);
    assert_from_json_inline!(test, @r"
    Ok(
        RenamedUnit,
    )
    ");
}

#[test]
fn internally_tagged_enum() {
    let test = TestHelper::<InternalEnum>::new_serialized(r#"{"tag":"renamed_unit"}"#);
    assert_from_json_inline!(test, @r"
    Ok(
        RenamedUnit,
    )
    ");
}

#[test]
fn adjacently_tagged_enum() {
    let test = TestHelper::<AdjacentEnum>::new_serialized(r#"{"tag":"renamed_unit"}"#);
    assert_from_json_inline!(test, @r"
    Ok(
        RenamedUnit,
    )
    ");
}
