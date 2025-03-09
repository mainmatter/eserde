//! Deserialize TOML documents.

use crate::{
    path, reporter::ErrorReporter, DeserializationError, DeserializationErrors, EDeserialize,
};
use toml;
use serde::Deserialize;

/// Deserialize an instance of type `T` from a string of TOML text.
///
/// # Example
///
/// ```rust
/// #[derive(eserde::Deserialize, Debug)]
/// struct User {
///     fingerprint: String,
///     location: String,
/// }
///
/// # fn main() {
/// let data = r#"
///     fingerprint = "1234567890ABCDEF"
///     location = "Menlo Park, CA"
/// "#;
///
/// let u: User = eserde::toml::from_str(data).unwrap();
/// println!("{:#?}", u);
/// # }
/// ```
pub fn from_str<'a, T>(s: &'a str) -> Result<T, DeserializationErrors>
where
    T: EDeserialize<'a>,
{
    // First pass: attempt direct deserialization
    let intermediate_value = match toml::from_str::<toml::Value>(s) {
        Ok(value) => value,
        Err(e) => {
            return Err(DeserializationErrors::from(vec![DeserializationError {
                path: None,
                details: e.to_string(),
            }]));
        }
    };

    match T::deserialize(intermediate_value.clone()) {
        Ok(v) => return Ok(v),
        Err(_) => (),
    }

    let _guard = ErrorReporter::start_deserialization();

    let de = path::Deserializer::new(intermediate_value);

    let errors = match T::deserialize_for_errors(de) {
        Ok(_) => vec![],
        Err(_) => ErrorReporter::take_errors(),
    };

    if errors.is_empty() {
        Err(DeserializationErrors::from(vec![DeserializationError {
            path: None,
            details: "Unknown deserialization error".into(),
        }]))
    } else {
        Err(DeserializationErrors(errors))
    }
}

