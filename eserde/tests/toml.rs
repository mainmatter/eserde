#![cfg(feature = "toml")]
#[derive(eserde::Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Foo {
    #[serde(rename = "route", default = "default_route")]
    route_0: String,
    #[serde(default = "default_route")]
    route_1: String,
}

fn default_route() -> String {
    "/".to_string()
}

#[test]
fn test_happy() {
    assert_eq!(
        Foo {
            route_0: "/".to_owned(),
            route_1: "/".to_owned(),
        },
        eserde::toml::from_str(r#""#).unwrap()
    );

    assert_eq!(
        Foo {
            route_0: "/dev/null".to_owned(),
            route_1: "/".to_owned(),
        },
        eserde::toml::from_str(r#"route = "/dev/null""#).unwrap()
    );
}

#[test]
fn test_fail() {
    let x = eserde::toml::from_str::<Foo>(
        r#"
    route = 0
    route_1 = true
    route_2 = 5.5
    "#,
    );
    assert!(x.is_err(), "Expected Err: {:?}", x);
    let errs = x.unwrap_err();
    insta::assert_snapshot!(errs, @r#"
    Something went wrong during deserialization:
    - route: invalid type: integer `0`, expected a string
    - route_1: invalid type: boolean `true`, expected a string
    - route_2: TOML parse error at line 4, column 5
      |
    4 |     route_2 = 5.5
      |     ^^^^^^^
    unknown field `route_2`, expected `route` or `route_1`
    "#);
}

#[test]
fn test_parse_failure() {
    let x = eserde::toml::from_str::<Foo>(
        r#"
    parse_fail_here
    route = 0
    route_1 = true
    route_2 = 5.5
    "#,
    );
    assert!(x.is_err(), "Expected Err: {:?}", x);
    let errs = x.unwrap_err();
    insta::assert_snapshot!(errs, @r#"
    Something went wrong during deserialization:
    - TOML parse error at line 2, column 20
      |
    2 |     parse_fail_here
      |                    ^
    expected `.`, `=`
    "#);
}
