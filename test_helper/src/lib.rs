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

#[macro_export]
macro_rules! test_name {
    ($type:ty, $($suffix:expr)?) => {{
        fn f() {}
        fn type_name_of_val<T>(_: T) -> &'static str {
            core::any::type_name::<T>()
        }
        let test_fn_name = type_name_of_val(f)
            .trim_end_matches("::f")
            .split("::")
            .last()
            .unwrap();

        let suffix = stringify!($type);
        $(let suffix = $suffix;)?

        let name = format!("{}:{}~{}", core::file!(), test_fn_name, suffix);

        name
    }};
}

#[macro_export]
macro_rules! test_dir {
    () => {{
        let package_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let snapshot_dir = std::path::PathBuf::from(file!())
            .parent()
            .unwrap()
            .to_owned();
        package_dir.parent().unwrap().join(snapshot_dir)
    }};
}

#[macro_export]
macro_rules! test {
    ($type:ty $(; $suffix:expr)?) => {
        $crate::test_helper::TestHelper::<$type>::new_fake_persisted($crate::test_name!($type, $($suffix)?), $crate::test_dir!())
    };
    (serialized; $type:ty, $serialized:expr$(; $suffix:expr)?) => {
        $crate::test_helper::TestHelper::<$type>::new_serialized($crate::test_name!($type, $($suffix)?), $crate::test_dir!(), $serialized)
    };
}
