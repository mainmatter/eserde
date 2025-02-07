use std::fmt::{self, Display};
use std::slice;

/// Path to the error value in the input, like `dependencies.serde.typo1`.
///
/// Use `path.to_string()` to get a string representation of the path with
/// segments separated by periods, or use `path.iter()` to iterate over
/// individual segments of the path.
#[derive(Clone, Debug)]
pub struct Path {
    pub segments: Vec<Segment>,
}

impl Default for Path {
    fn default() -> Self {
        Path {
            segments: Vec::new(),
        }
    }
}

/// Single segment of a path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Segment {
    Seq { index: usize },
    Map { key: String },
    Enum { variant: String },
}

impl Path {
    /// Returns an iterator with element type [`&Segment`][Segment].
    pub fn iter(&self) -> Segments {
        Segments {
            iter: self.segments.iter(),
        }
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

impl<'a> DoubleEndedIterator for Segments<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a> ExactSizeIterator for Segments<'a> {
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
