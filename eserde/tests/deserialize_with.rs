use std::num::TryFromIntError;

#[derive(eserde::Deserialize, Debug)]
struct DeserializeWith {
    #[serde(deserialize_with = "deserialize_u8")]
    n: Result<u8, TryFromIntError>,
}
fn deserialize_u8<'de, D>(deserializer: D) -> Result<Result<u8, TryFromIntError>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let long: u64 = serde::de::Deserialize::deserialize(deserializer)?;
    Ok(u8::try_from(long))
}

#[test]
fn test_ok() {
    let x: DeserializeWith = eserde::json::from_str("{\"n\": 255}").unwrap();
    assert_eq!(Ok(255), x.n);
}

#[test]
fn test_err() {
    let x: DeserializeWith = eserde::json::from_str("{\"n\": 256}").unwrap();
    assert!(x.n.is_err(), "Expected Err: {:?}", x.n);
}
