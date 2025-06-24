use crate::path::Path;

/// A collection of errors encountered while trying to deserialize a type.
///
/// Use [`.iter()`](Self::iter) to iterator over the underlying [`DeserializationError`].
#[derive(Debug)]
pub struct DeserializationErrors(Vec<DeserializationError>);

impl From<Vec<DeserializationError>> for DeserializationErrors {
    fn from(errors: Vec<DeserializationError>) -> Self {
        DeserializationErrors(errors)
    }
}

impl DeserializationErrors {
    /// Iterate over references to the underlying [`DeserializationError`]s.
    ///
    /// Use [`.into_iter()`](Self::into_iter) if you need owned [`DeserializationError`]s
    /// from the iterator.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &DeserializationError> {
        self.0.iter()
    }

    /// The number of errors in the collection.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the collection contains no errors.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl IntoIterator for DeserializationErrors {
    type Item = DeserializationError;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::fmt::Display for DeserializationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Something went wrong during deserialization:")?;
        for error in self.iter() {
            writeln!(f, "- {error}")?;
        }
        Ok(())
    }
}

// Due to the design of `std`'s `Error` trait, we really don't have a good
// story for combining multiple errors into a single error.
// In particular, it's unclear/ill-defined what should be returned from
// `source`.
// We leave it to `None`, but that really sucks...
impl std::error::Error for DeserializationErrors {}

#[derive(Debug)]
/// An error that occurred during deserialization.
pub struct DeserializationError {
    pub(crate) path: Option<Path>,
    pub(crate) details: String,
}

impl DeserializationError {
    /// An explanation of what went wrong during deserialization.
    pub fn message(&self) -> &str {
        self.details.as_ref()
    }

    /// The input path at which the error occurred, when available.
    ///
    /// E.g. if the error occurred while deserializing the sub-field `foo` of the top-level
    /// field `bar`, the path would be `bar.foo`.
    pub fn path(&self) -> Option<&Path> {
        self.path.as_ref()
    }
}

impl std::fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path) = &self.path {
            if !path.is_empty() {
                write!(f, "{}: ", path)?;
            }
        }
        write!(f, "{}", self.details.trim())
    }
}
