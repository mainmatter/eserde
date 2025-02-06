pub use serde_json::Error;

use crate::{reporter::ErrorReporter, DeserializationError, HumanDeserialize};

pub fn from_str<'a, T>(s: &'a str) -> Result<T, Vec<DeserializationError>>
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
    let _guard = ErrorReporter::start_deserialization();
    let mut de = serde_json::Deserializer::from_str(s);
    match T::human_deserialize(&mut de) {
        Ok(_) => {
            if cfg!(debug_assertions) {
                panic!("Expected human_deserialize to fail since `serde` deserialization failed, instead it succeeded. Original `serde` error: {:?}", error);
            } else {
                let error = DeserializationError::Custom {
                    message: error.to_string(),
                };
                Err(vec![error])
            }
        }
        Err(_) => Err(ErrorReporter::take_errors()),
    }
}
