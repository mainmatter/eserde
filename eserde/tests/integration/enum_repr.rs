use eserde_test_helper::assert_from_json_inline;
use eserde_test_helper::enum_repr::*;
use eserde_test_helper::test_helper::TestHelper;

#[test]
fn enum_repr() {
    let test = TestHelper::<Enum>::new_serialized(r#"5"#);
    assert_from_json_inline!(test, @r"
    Ok(
        Enum(
            Five,
        ),
    )
    ");
}
