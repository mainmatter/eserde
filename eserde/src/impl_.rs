use crate::{reporter::ErrorReporter, DeserializationErrorDetails, HumanDeserialize};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

macro_rules! impl_human_deserialize {
    ($($t:ty),* $(,)?) => {
        $(
            impl<'de> HumanDeserialize<'de> for $t {
                fn human_deserialize<D>(deserializer: D) -> Result<Self, ()>
                where
                    D: serde::Deserializer<'de>
                {
                    Self::deserialize(deserializer).map_err(|e| {
                        ErrorReporter::report(DeserializationErrorDetails::Custom {
                            message: e.to_string(),
                        });
                    })
                }
            }
        )*
    };
    // For generic types with a single type parameter
    ($t:ident, 1, $($bounds:tt)*) => {
        impl<'de, T> HumanDeserialize<'de> for $t<T>
        where
            T: HumanDeserialize<'de> + $($bounds)*,
        {
            fn human_deserialize<D>(deserializer: D) -> Result<Self, ()>
            where
                D: serde::Deserializer<'de>
            {
                Self::deserialize(deserializer).map_err(|e| {
                    ErrorReporter::report(DeserializationErrorDetails::Custom {
                        message: e.to_string(),
                    });
                })
            }
        }
    };
    // For map types with two type parameters
    ($t:ident, 2, $($k_bounds:tt)* | $($v_bounds:tt)*) => {
    };
}

// Primitive types
impl_human_deserialize!(
    bool,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    char,
    String,
    &'de str,
    &'de [u8],
);

// Generic collections
impl_human_deserialize!(Option, 1,);
impl_human_deserialize!(Vec, 1,);
impl_human_deserialize!(HashSet, 1, std::hash::Hash + std::cmp::Eq);
impl_human_deserialize!(BTreeSet, 1, std::cmp::Ord);

// Map types

impl<'de, K, V> HumanDeserialize<'de> for BTreeMap<K, V>
where
    K: HumanDeserialize<'de> + std::cmp::Ord,
    V: HumanDeserialize<'de>,
{
    fn human_deserialize<D>(deserializer: D) -> Result<Self, ()>
    where
        D: serde::Deserializer<'de>,
    {
        Self::deserialize(deserializer).map_err(|e| {
            ErrorReporter::report(DeserializationErrorDetails::Custom {
                message: e.to_string(),
            });
        })
    }
}

impl<'de, K, V> HumanDeserialize<'de> for HashMap<K, V>
where
    K: HumanDeserialize<'de> + std::cmp::Eq + std::hash::Hash,
    V: HumanDeserialize<'de>,
{
    fn human_deserialize<D>(deserializer: D) -> Result<Self, ()>
    where
        D: serde::Deserializer<'de>,
    {
        Self::deserialize(deserializer).map_err(|e| {
            ErrorReporter::report(DeserializationErrorDetails::Custom {
                message: e.to_string(),
            });
        })
    }
}

// Special case for Cow due to additional bounds
impl<'de, 'a, T> HumanDeserialize<'de> for std::borrow::Cow<'a, T>
where
    T: ToOwned + ?Sized,
    T::Owned: HumanDeserialize<'de>,
{
    fn human_deserialize<D>(deserializer: D) -> Result<Self, ()>
    where
        D: serde::Deserializer<'de>,
    {
        Self::deserialize(deserializer).map_err(|e| {
            ErrorReporter::report(DeserializationErrorDetails::Custom {
                message: e.to_string(),
            });
        })
    }
}
