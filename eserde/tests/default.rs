#[derive(eserde::Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Foo {
    #[serde(rename = "route", default = "default_route")]
    route_0: String,
    #[serde(default = "default_route")]
    route_1: String,
    #[serde(default = "NoDefault::new")]
    no_default: NoDefault,
}

fn default_route() -> String {
    "/".to_owned()
}

// Has no `#[derive(Default)]`.
#[derive(eserde::Deserialize, Debug, PartialEq, Eq)]
struct NoDefault;
impl NoDefault {
    fn new() -> Self {
        NoDefault
    }
}

#[test]
fn test_happy() {
    assert_eq!(
        Foo {
            route_0: "/".to_owned(),
            route_1: "/".to_owned(),
            no_default: NoDefault,
        },
        eserde::json::from_str(r#"{}"#).unwrap()
    );

    assert_eq!(
        Foo {
            route_0: "/dev/null".to_owned(),
            route_1: "/".to_owned(),
            no_default: NoDefault,
        },
        eserde::json::from_str(r#"{"route": "/dev/null", "no_default": null}"#).unwrap()
    );
}

#[test]
fn test_fail() {
    let x = eserde::json::from_str::<Foo>(
        r#"{"route": 0, "route_1": true, "no_default": "5", "route_2": 5.5}"#,
    );
    assert!(x.is_err(), "Expected Err: {:?}", x);
    let errs = x.unwrap_err();
    insta::assert_snapshot!(errs, @r###"
    Something went wrong during deserialization:
    - route: invalid type: integer `0`, expected a string at line 1 column 11
    - route_1: invalid type: boolean `true`, expected a string at line 1 column 28
    - no_default: invalid type: string "5", expected unit struct __ImplEDeserializeForNoDefault at line 1 column 47
    - route_2: unknown field `route_2`, expected one of `route`, `route_1`, `no_default` at line 1 column 58
    "###);
}
