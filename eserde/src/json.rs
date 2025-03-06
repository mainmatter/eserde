//! Deserialize JSON documents.
//!
//! # Example
//!
//! ```rust
//! #[derive(serde::Serialize, eserde::Deserialize)]
//! struct Person {
//!     name: String,
//!     age: u8,
//!     phones: Vec<String>,
//! }
//!
//! # fn main() {
//! // Some JSON input data as a &str. Maybe this comes from the user.
//! let data = r#"
//!     {
//!         "name": "John Doe",
//!         "age": 43,
//!         "phones": [
//!             "+44 1234567",
//!             "+44 2345678"
//!         ]
//!     }"#;
//!
//! // Try to parse the string of data into a `Person` object.
//! match eserde::json::from_str::<Person>(data) {
//!     Ok(p) => {
//!         println!("Please call {} at the number {}", p.name, p.phones[0]);
//!     }
//!     Err(errors) => {
//!         println!("Something went wrong during deserialization");
//!         for error in errors.iter() {
//!             println!("{error}")
//!         }
//!     }
//! }
//! # }
//! ```
//!
//! # Implementation
//!
//! This module relies on [`serde_json`](https://crates.io/crates/serde_json) as
//! the underlying deserializer.
//!
//! All deserializers in this module follow the same two-pass approach.
//! Start by using `serde::Deserialize` to try to deserialize the target type.
//! If it succeeds, return `Ok(value)`.
//! If it fails, use `eserde::EDeserialize` to visit the input again and
//! accumulate as many deserialization errors as possible.
//! The errors are then returned as a vector in the `Err` variant.
//!
//! # Limitations
//!
//! ## Input must be buffered in memory
//!
//! We don't support deserializing from a reader, since it doesn't allow
//! us to perform two passes over the input.\
//! We are restricted to input types that are buffered in memory (byte slices,
//! string slices, etc.).
use crate::{
    impl_edeserialize_compat, path, reporter::ErrorReporter, DeserializationError,
    DeserializationErrors, EDeserialize,
};

/// Deserialize an instance of type `T` from a string of JSON text.
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
/// // The type of `j` is `&str`
/// let j = "
///     {
///         \"fingerprint\": \"0xF9BA143B95FF6D82\",
///         \"location\": \"Menlo Park, CA\"
///     }";
///
/// let u: User = eserde::json::from_str(j).unwrap();
/// println!("{:#?}", u);
/// # }
/// ```
pub fn from_str<'a, T>(s: &'a str) -> Result<T, DeserializationErrors>
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

    let errors = match T::deserialize_for_errors(de) {
        Ok(_) => vec![],
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

/// Deserialize an instance of type `T` from bytes of JSON text.
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
/// // The type of `j` is `&[u8]`
/// let j = b"
///     {
///         \"fingerprint\": \"0xF9BA143B95FF6D82\",
///         \"location\": \"Menlo Park, CA\"
///     }";
///
/// let u: User = eserde::json::from_slice(j).unwrap();
/// println!("{:#?}", u);
/// # }
/// ```
pub fn from_slice<'a, T>(s: &'a [u8]) -> Result<T, DeserializationErrors>
where
    T: EDeserialize<'a>,
{
    let mut de = serde_json::Deserializer::from_slice(s);
    let error = match T::deserialize(&mut de) {
        Ok(v) => {
            return Ok(v);
        }
        Err(e) => e,
    };
    let _guard = ErrorReporter::start_deserialization();

    let mut de = serde_json::Deserializer::from_slice(s);
    let de = path::Deserializer::new(&mut de);

    let errors = match T::deserialize_for_errors(de) {
        Ok(_) => vec![],
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

impl_edeserialize_compat! {
    serde_json::value::Number,
    serde_json::value::Value,
    serde_json::value::Map<String, serde_json::value::Value>,
}
