pub use serde_json::Error;

use crate::HumanDeserialize;

pub fn from_str<'a, T>(s: &'a str) -> Result<T, Vec<serde_json::Error>>
where
    T: HumanDeserialize<'a>,
{
    let mut de = serde_json::Deserializer::from_str(s);
    let error = match T::deserialize(&mut de) {
        Ok(v) => {
            return Ok(v);
        }
        Err(e) => e,
    };
    let mut de = serde_json::Deserializer::from_str(s);
    match T::human_deserialize(&mut de) {
        Ok(_) => {
            if cfg!(debug_assertions)
            {
                panic!("Expected human_deserialize to fail since `serde` deserialization failed, instead it succeeded. Original `serde` error: {:?}", error);
            } else {
                Err(vec![error])
            }
        }
        Err(e) => Err(e),
    }
}
