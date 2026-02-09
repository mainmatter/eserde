//! Deserialize `application/x-www-form-urlencoded` documents.
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
//! // Some `application/x-www-form-urlencoded` input data as a &str. Maybe this comes from the user.
//! let data = r#"name=Max&age=25&phones=123456&phones=5623461"#;
//!
//! // Try to parse the string of data into a `Person` object.
//! match eserde::html_form::from_str::<Person>(data) {
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
//! This module relies on [`serde_html_form`](https://crates.io/crates/serde_html_form) as
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
    path, reporter::ErrorReporter, DeserializationError, DeserializationErrors, EDeserialize,
};

/// Deserialize an instance of type `T` from a string of `application/x-www-form-urlencoded` text.
///
/// # Example
///
/// ```rust
/// #[derive(eserde::Deserialize, Debug)]
/// struct User {
///     age: u8,
///     name: String,
/// }
///
/// # fn main() {
/// // The type of `j` is `&str`
/// let j = "age=25&name=Max";
///
/// let u: User = eserde::html_form::from_str(j).unwrap();
/// println!("{:#?}", u);
/// # }
/// ```
pub fn from_str<'a, T>(s: &'a str) -> Result<T, DeserializationErrors>
where
    T: EDeserialize<'a>,
{
    from_bytes(s.as_bytes())
}

/// Deserialize an instance of type `T` from bytes of `application/x-www-form-urlencoded` text.
///
/// # Example
///
/// ```rust
/// #[derive(eserde::Deserialize, Debug)]
/// struct User {
///     age: u8,
///     name: String,
/// }
///
/// # fn main() {
/// // The type of `j` is `&str`
/// let j = b"age=25&name=Max";
///
/// let u: User = eserde::html_form::from_bytes(j).unwrap();
/// println!("{:#?}", u);
/// # }
/// ```
pub fn from_bytes<'a, T>(s: &'a [u8]) -> Result<T, DeserializationErrors>
where
    T: EDeserialize<'a>,
{
    let de = serde_html_form::Deserializer::from_bytes(s);
    let error = match T::deserialize(de) {
        Ok(v) => {
            return Ok(v);
        }
        Err(e) => e,
    };
    let _guard = ErrorReporter::start_deserialization();

    let de = serde_html_form::Deserializer::from_bytes(s);
    let de = path::Deserializer::new(de);

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
