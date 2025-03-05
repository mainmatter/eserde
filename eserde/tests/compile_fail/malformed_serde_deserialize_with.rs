use std::num::TryFromIntError;

#[derive(eserde::Deserialize)]
struct Coord {
    #[serde(alias = "0", deserialize_with = u64_to_u8())]
    x: Result<u8, TryFromIntError>,
    #[serde(alias = "1", deserialize_with = false)]
    y: Result<u8, TryFromIntError>,
}

fn u64_to_u8<'de, D>(deserializer: D) -> Result<Result<u8, TryFromIntError>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let long: u64 = serde::de::Deserialize::deserialize(deserializer)?;
    Ok(u8::try_from(long))
}

fn main() {}
