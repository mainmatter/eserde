#[test]
fn test_typed_any_json() {
    use eserde::_macro_impl::TypedAny;

    let json_data = r#"
    {
        "foo": 42,
        "bar": "Hello, world!",
        "baz": [1, 2, 4.5, true, null],
        "qux": {"nested": "object"}
    }
    "#;

    let x: TypedAny = serde_json::from_str(json_data).unwrap();
    insta::assert_debug_snapshot!(x, @r"
    Map(
        [
            (
                Str,
                U64,
            ),
            (
                Str,
                Str,
            ),
            (
                Str,
                Seq(
                    [
                        U64,
                        U64,
                        F64,
                        Bool,
                        Unit,
                    ],
                ),
            ),
            (
                Str,
                Map(
                    [
                        (
                            Str,
                            Str,
                        ),
                    ],
                ),
            ),
        ],
    )
    ");
}
