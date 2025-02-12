use crate::{reporter::ErrorReporter, DeserializationErrorDetails, EDeserialize};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

macro_rules! impl_edeserialize {
    ($($t:ty),* $(,)?) => {
        $(
            impl<'de> EDeserialize<'de> for $t {
                fn deserialize_for_errors<D>(deserializer: D) -> Result<(), ()>
                where
                    D: serde::Deserializer<'de>
                {
                    Self::deserialize(deserializer).map(|_| ()).map_err(|e| {
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
        impl<'de, T> EDeserialize<'de> for $t<T>
        where
            T: EDeserialize<'de> + $($bounds)*,
        {
            fn deserialize_for_errors<D>(deserializer: D) -> Result<(), ()>
            where
                D: serde::Deserializer<'de>
            {
                Self::deserialize(deserializer).map(|_| ()).map_err(|e| {
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
impl_edeserialize!(
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
impl_edeserialize!(Option, 1,);
impl_edeserialize!(Vec, 1,);
impl_edeserialize!(HashSet, 1, std::hash::Hash + std::cmp::Eq);
impl_edeserialize!(BTreeSet, 1, std::cmp::Ord);

// Map types

impl<'de, K, V> EDeserialize<'de> for BTreeMap<K, V>
where
    K: EDeserialize<'de> + std::cmp::Ord,
    V: EDeserialize<'de>,
{
    fn deserialize_for_errors<D>(deserializer: D) -> Result<(), ()>
    where
        D: serde::Deserializer<'de>,
    {
        Self::deserialize(deserializer).map(|_| ()).map_err(|e| {
            ErrorReporter::report(DeserializationErrorDetails::Custom {
                message: e.to_string(),
            });
        })
    }
}

impl<'de, K, V> EDeserialize<'de> for HashMap<K, V>
where
    K: EDeserialize<'de> + std::cmp::Eq + std::hash::Hash,
    V: EDeserialize<'de>,
{
    fn deserialize_for_errors<D>(deserializer: D) -> Result<(), ()>
    where
        D: serde::Deserializer<'de>,
    {
        Self::deserialize(deserializer).map(|_| ()).map_err(|e| {
            ErrorReporter::report(DeserializationErrorDetails::Custom {
                message: e.to_string(),
            });
        })
    }
}

// Special case for Cow due to additional bounds
impl<'de, 'a, T> EDeserialize<'de> for std::borrow::Cow<'a, T>
where
    T: ToOwned + ?Sized,
    T::Owned: EDeserialize<'de>,
{
    fn deserialize_for_errors<D>(deserializer: D) -> Result<(), ()>
    where
        D: serde::Deserializer<'de>,
    {
        Self::deserialize(deserializer).map(|_| ()).map_err(|e| {
            ErrorReporter::report(DeserializationErrorDetails::Custom {
                message: e.to_string(),
            });
        })
    }
}
