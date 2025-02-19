#![allow(clippy::disallowed_names)]

pub mod contract;
pub mod enum_repr;
pub mod enums;
pub mod enums_deny_unknown_fields;
pub mod enums_flattened;
pub mod flatten;
pub mod json;

pub mod test_helper;
// mod from_value;
// mod garde;
// #[cfg(feature = "indexmap2")]
// mod indexmap;
// mod inline_subschemas;
// mod macros;
// mod remote_derive;
// mod same_name;
// mod schema_name;
// mod schema_with;
// #[cfg(feature = "semver1")]
// mod semver;
// mod settings;
// mod skip;
// #[cfg(feature = "smallvec1")]
// mod smallvec;
// #[cfg(feature = "smol_str02")]
// mod smol_str;
// mod std_types;
pub mod structs;
// mod transform;
// mod transparent;
// #[cfg(feature = "url2")]
// mod url;
// #[cfg(feature = "uuid1")]
// mod uuid;
// mod validator;

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
    (json; $type:ty $(; $suffix:expr)?) => {
        $crate::test_helper::TestHelper::<$type>::new_random_persisted($crate::test_name!($type, $($suffix)?), $crate::test_dir!())
    };
    (serialized; $type:ty, $serialized:expr$(; $suffix:expr)?) => {
        $crate::test_helper::TestHelper::<$type>::new_serialized($crate::test_name!($type, $($suffix)?), $crate::test_dir!(), $serialized)
    };
}
