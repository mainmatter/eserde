#[derive(eserde::Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Foo {
    foo: String,
}

#[test]
fn test_happy() {
    assert_eq!(
        Foo {
            foo: "bar".to_owned(),
        },
        eserde::json::from_str(r#"{"foo": "bar"}"#).unwrap()
    );
}

#[test]
fn test_unknown_field() {
    let x = eserde::json::from_str::<Foo>(r#"{"foo": "bar", "unknown": 5}"#);
    assert!(x.is_err(), "Expected Err: {:?}", x);
    let errs = x.unwrap_err();
    insta::assert_snapshot!(errs, @r"
    Something went wrong during deserialization:
    - unknown field `unknown` of type U64
    ");
}

#[test]
fn test_multiple_unknown_fields() {
    let x = eserde::json::from_str::<Foo>(r#"{"foo": "bar", "unk": 5, "nown": false}"#);
    assert!(x.is_err(), "Expected Err: {:?}", x);
    let errs = x.unwrap_err();
    insta::assert_snapshot!(errs, @r"
    Something went wrong during deserialization:
    - unknown field `unk` of type U64
    - unknown field `nown` of type Bool
    ");
}

#[test]
fn test_unknown_fields_collections() {
    let x = eserde::json::from_str::<Foo>(r#"{"foo": "bar", "unk": [1, 2, 3], "nown": {"a": 5}}"#);
    assert!(x.is_err(), "Expected Err: {:?}", x);
    let errs = x.unwrap_err();
    insta::assert_snapshot!(errs, @r"
    Something went wrong during deserialization:
    - unknown field `unk` of type Seq([U64, U64, U64])
    - unknown field `nown` of type Map([(Str, U64)])
    ");
}
