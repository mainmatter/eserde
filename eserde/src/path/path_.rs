use std::fmt::{self, Display};
use std::slice;

/// Logical path to the error location.
///
/// The path can target specific positions in sequences (`[0]`), mappings (`foo`),
/// and enum variants (`Bar`). Multiple levels can be chained together
/// with periods, for example `foo[0].bar`.
///
/// Use `path.to_string()` to get a string representation of the path with
/// segments separated by periods, or use [`path.iter()`](Path::iter) to iterate over
/// individual segments of the path.
#[derive(Clone, Debug, Default)]
pub struct Path {
    segments: Vec<Segment>,
}

impl From<Vec<Segment>> for Path {
    fn from(segments: Vec<Segment>) -> Self {
        Self { segments }
    }
}

/// Single segment of a path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Segment {
    /// An index into a sequence.
    ///
    /// Represented with the pattern `[0]`.
    Seq {
        /// The index pointing at the problematic element.
        index: usize,
    },
    /// A key for a map or struct type.
    Map {
        /// The name of the key.
        key: String,
    },
    /// A variant within an enum type.
    Enum {
        /// The name of the variant.
        variant: String,
    },
}

impl Path {
    /// Returns `true` if there are no segments in this path,
    /// `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Returns an iterator with element type [`&Segment`][Segment].
    pub fn iter(&self) -> Segments {
        Segments {
            iter: self.segments.iter(),
        }
    }

    pub(crate) fn segments(&self) -> &[Segment] {
        &self.segments
    }
}

impl<'a> IntoIterator for &'a Path {
    type Item = &'a Segment;
    type IntoIter = Segments<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over segments of a path.
///
/// Returned by [`Path::iter`].
pub struct Segments<'a> {
    iter: slice::Iter<'a, Segment>,
}

impl<'a> Iterator for Segments<'a> {
    type Item = &'a Segment;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl DoubleEndedIterator for Segments<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl ExactSizeIterator for Segments<'_> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl Display for Path {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if self.segments.is_empty() {
            return formatter.write_str(".");
        }

        let mut separator = "";
        for segment in self {
            if !matches!(segment, Segment::Seq { .. }) {
                formatter.write_str(separator)?;
            }
            write!(formatter, "{}", segment)?;
            separator = ".";
        }

        Ok(())
    }
}

impl Display for Segment {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Segment::Seq { index } => write!(formatter, "[{}]", index),
            Segment::Map { key } | Segment::Enum { variant: key } => {
                write!(formatter, "{}", key)
            }
        }
    }
}
