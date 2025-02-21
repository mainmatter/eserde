use eserde::EDeserialize;

use fake::{Dummy, Fake, Faker};
use serde::Serialize;

use std::{
    fmt::Debug,
    io::{Read, Write},
    marker::PhantomData,
    path::{Path, PathBuf},
};

use crate::json::JsonValue;

pub struct TestHelper<T> {
    name: String,
    test_dir: PathBuf,
    value_serialized: String,
    _marker: PhantomData<T>,
}

/// Configure [`insta`] such that it writes its snapshots to the right directory,
/// and doesn't prepend the module path to the snapshot file name.
#[macro_export]
macro_rules! with_insta_config {
    ($self:expr, $body:tt) => {
        insta::with_settings!({ snapshot_path => $self.test_dir().join("snapshots"), prepend_module_to_snapshot => false }, {
            $body
        });
    };
}

#[macro_export]
macro_rules! assert_from_json_inline {
    ($helper:expr, @$expected:expr) => {
        $crate::with_insta_config!($helper, {
            let result = $helper.from_json();
            insta::assert_debug_snapshot!(result, $helper.name().clone(), @$expected);
        })
    };
}

impl<'de, T> TestHelper<T>
where
    T: EDeserialize<'de>,
{
    /// Create a new test helper, that holds a value generated with [`fake`],
    /// serialized to JSON for use in assertions.
    /// This will write the serialized value to a cache file, so that test outputs
    /// become stable.
    /// Use [`crate::test`] to invoke this function to have it generate a unique,
    /// stable name and figure out the directory the test lives in.
    pub fn new_fake_persisted(name: String, test_dir: PathBuf) -> Self
    where
        T: Dummy<Faker> + Serialize,
    {
        let serialized = Self::read_or_create_persisted(&name, &test_dir, || {
            serde_json::to_string(&Faker.fake::<T>()).unwrap()
        });

        Self::new_serialized(name, test_dir, serialized)
    }

    /// Create a new test helper, that holds a fake [`JsonValue`] serialized to
    /// JSON for use in assertions.
    /// This will write the serialized value to a cache file, so that test outputs
    /// become stable.
    /// Use [`crate::test`] to invoke this function to have it generate a unique,
    /// stable name and figure out the directory the test lives in.
    pub fn new_random_persisted(name: String, test_dir: PathBuf) -> Self
    where
        T: Dummy<Faker> + Serialize,
    {
        let serialized = Self::read_or_create_persisted(&name, &test_dir, || {
            serde_json::to_string(&JsonValue::fake()).unwrap()
        });

        Self::new_serialized(name, test_dir, serialized)
    }

    /// Create a new test helper, that holds the passed string for use in assertions.
    /// Use [`crate::test`] to invoke this function to have it generate a unique,
    /// stable name and figure out the directory the test lives in.
    pub fn new_serialized(
        name: String,
        test_dir: PathBuf,
        value_serialized: impl ToString,
    ) -> Self {
        Self {
            name,
            value_serialized: value_serialized.to_string(),
            test_dir,
            _marker: PhantomData,
        }
    }

    /// Try to deserialize the held data using [`eserde::json::from_str`],
    /// and assert the correctness of the outcome using [`insta::assert_debug_snapshot`].
    pub fn from_json_assert_snapshot(&'de self) -> &'de Self
    where
        T: Debug,
    {
        with_insta_config!(&self, {
            let result = self.from_json();
            insta::assert_debug_snapshot!(self.name(), result);
        });
        self
    }
    
    /// Try to deserialize the held data using [`eserde::json::from_str`],
    /// returning the result.
    pub fn from_json(&'de self) -> Result<T, eserde::DeserializationErrors> {
        eserde::json::from_str(&self.value_serialized)
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn test_dir(&self) -> &Path {
        &self.test_dir
    }

    /// Determine the path of the file the serialized value gets persisted,
    /// based on the test name and the test directory.
    fn cache_file_path(test_dir: &Path, name: &str, ext: &str) -> PathBuf {
        let file_name = name.replace('/', "__");

        test_dir
            .join("serialized")
            .join(format!("{file_name}.{ext}"))
    }

    /// Read cached value if it exists, or generate a new one using the passed
    /// closure and write that in the cache. Returns the serialized value.
    fn read_or_create_persisted<F: FnOnce() -> String>(
        name: &str,
        test_dir: &Path,
        generate: F,
    ) -> String {
        let cache_file_path = Self::cache_file_path(test_dir, name, "json");
        if let Some(cache_file_dir) = cache_file_path.parent() {
            std::fs::create_dir_all(cache_file_dir).unwrap();
        }

        let mut f = std::fs::File::options()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(cache_file_path)
            .unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();

        if contents.is_empty() {
            f.set_len(0).unwrap();
            let value_serialized = generate();
            f.write_all(value_serialized.as_bytes()).unwrap();
            value_serialized
        } else {
            contents
        }
    }
}
