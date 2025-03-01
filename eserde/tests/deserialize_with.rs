use std::net::IpAddr;

#[derive(eserde::Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct DeserializeWith {
    #[serde(rename = "number", deserialize_with = "deserialize_u8")]
    num: Result<u8, u64>,

    #[serde(deserialize_with = "parse_generic")]
    ip: Result<IpAddr, String>,
}

fn deserialize_u8<'de, D>(deserializer: D) -> Result<Result<u8, u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let long: u64 = serde::de::Deserialize::deserialize(deserializer)?;
    Ok(u8::try_from(long).map_err(|_| long))
}

fn parse_generic<'de, T, D>(deserializer: D) -> Result<Result<T, String>, D::Error>
where
    T: std::str::FromStr,
    D: serde::Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;
    Ok(s.parse().map_err(|_| s))
}

#[test]
fn test_happy() {
    assert_eq!(
        DeserializeWith {
            num: Ok(255),
            ip: Ok(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))),
        },
        eserde::json::from_str::<DeserializeWith>(r#"{"number": 255, "ip": "127.0.0.1"}"#).unwrap(),
    );
    assert_eq!(
        DeserializeWith {
            num: Err(256),
            ip: Err("localhost".to_owned()),
        },
        eserde::json::from_str::<DeserializeWith>(r#"{"number": 256, "ip": "localhost"}"#).unwrap(),
    );
}

#[test]
fn test_fail() {
    let x = eserde::json::from_str::<DeserializeWith>(
        r#"{"number": "foo", "ip": 100.50, "foo": "bar"}"#,
    );
    assert!(x.is_err(), "Expected Err: {:?}", x);
    let errs = x.unwrap_err();
    insta::assert_snapshot!(errs, @r###"
    Something went wrong during deserialization:
    - number: invalid type: string "foo", expected u64 at line 1 column 16
    - ip: invalid type: floating point `100.5`, expected a string at line 1 column 30
    - foo: unknown field `foo`, expected `number` or `ip` at line 1 column 37
    "###);
}
