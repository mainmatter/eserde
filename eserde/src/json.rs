pub use serde_json::Error;

use crate::{
    path, reporter::ErrorReporter, DeserializationError, DeserializationErrorDetails, EDeserialize,
};

pub fn from_str<'a, T>(s: &'a str) -> Result<T, Vec<DeserializationError>>
where
    T: EDeserialize<'a>,
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
    let de = path::Deserializer::new(&mut de);
    match T::deserialize_for_errors(de) {
        Ok(_) => {
            if cfg!(debug_assertions) {
                panic!("Expected human_deserialize to fail since `serde` deserialization failed, instead it succeeded. Original `serde` error: {:?}", error);
            } else {
                let details = DeserializationErrorDetails::Custom {
                    message: error.to_string(),
                };
                Err(vec![DeserializationError {
                    path: None,
                    details: details,
                }])
            }
        }
        Err(_) => Err(ErrorReporter::take_errors()),
    }
}
