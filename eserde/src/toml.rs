//! Deserialize TOML documents.

use crate::{
    path, reporter::ErrorReporter, DeserializationError, DeserializationErrors, EDeserialize,
};
use toml;

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
pub fn from_str<T>(s: &str) -> Result<T, DeserializationErrors>
where
    T: for<'a> EDeserialize<'a>,
{
    let de = toml::Deserializer::parse(s);
    let error = match de {
        Ok(v) => match T::deserialize(v) {
            Ok(v) => return Ok(v),
            Err(e) => e,
        },
        Err(e) => e,
    };
    let _guard = ErrorReporter::start_deserialization();

    let de = toml::Deserializer::parse(s).and_then(|de| Ok(path::Deserializer::new(de)));

    let errors = match de {
        Ok(v) => match T::deserialize_for_errors(v) {
            Ok(_) => vec![],
            Err(_) => ErrorReporter::take_errors(),
        },
        Err(_) => ErrorReporter::take_errors(),
    };
    let errors = if errors.is_empty() {
        vec![DeserializationError {
            path: None,
            details: error.to_string(),
        }]
    } else {
        errors
    };

    Err(DeserializationErrors::from(errors))
}
