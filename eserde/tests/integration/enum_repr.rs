use eserde_test_helper::assert_from_json_inline;
use eserde_test_helper::enum_repr::*;
use eserde_test_helper::test;

#[test]
fn enum_repr() {
    let test = test!(serialized; Enum,
        r#"5"#
    );
    assert_from_json_inline!(test, @r"
    Ok(
        Enum(
            Five,
        ),
    )
    ");
}
