use eserde::EDeserialize;

use std::marker::PhantomData;

#[macro_export]
macro_rules! assert_from_json_inline {
    ($helper:expr, @$expected:expr) => {
        let result = $helper.from_json();
        insta::assert_debug_snapshot!(result, @$expected);
    };
}

pub struct TestHelper<T> {
    value_serialized: String,
    _marker: PhantomData<T>,
}

impl<'de, T> TestHelper<T>
where
    T: EDeserialize<'de>,
{
    /// Create a new test helper, that holds the passed string for use in assertions.
    /// Use [`crate::test`] to invoke this function to have it generate a unique,
    /// stable name and figure out the directory the test lives in.
    pub fn new_serialized(value_serialized: impl ToString) -> Self {
        Self {
            value_serialized: value_serialized.to_string(),
            _marker: PhantomData,
        }
    }

    /// Try to deserialize the held data using [`eserde::json::from_str`],
    /// returning the result.
    pub fn from_json(&'de self) -> Result<T, eserde::DeserializationErrors> {
        eserde::json::from_str(&self.value_serialized)
    }
}
