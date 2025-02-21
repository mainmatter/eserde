#![allow(clippy::disallowed_names)]

pub mod contract;
pub mod enum_repr;
pub mod enums;
pub mod enums_deny_unknown_fields;
pub mod enums_flattened;
pub mod extra;
pub mod flatten;
pub mod json;
pub mod structs;

pub mod test_helper;

mod prelude {
    pub(crate) use crate::json::JsonValue;
    pub(crate) use arbitrary::Arbitrary;
    pub(crate) use eserde::Deserialize;
    pub(crate) use fake::Dummy;
    pub(crate) use serde::Serialize;
}

// #[macro_export]
// macro_rules! test {
//     (serialized; $type:ty, $serialized:expr$(; $suffix:expr)?) => {
//         $crate::test_helper::TestHelper::<$type>::new_serialized($serialized)
//     };
// }
