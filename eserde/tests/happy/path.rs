#![allow(clippy::unreadable_literal, dead_code)]
use eserde::{Deserialize, EDeserialize};
use std::collections::BTreeMap as Map;
use std::fmt::Debug;

#[track_caller]
fn test<'de, T>(json: &'de str, expected: &str)
where
    T: EDeserialize<'de> + Debug,
{
    let result: Result<T, _> = eserde::json::from_str(json);
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    let error = errors.iter().next().unwrap();
    let path = error.path().expect("No path on error");
    assert_eq!(path.to_string(), expected, "The full error:\n\t{}", error);
}

#[track_caller]
fn test_many<'de, T>(json: &'de str, expected: &[&str])
where
    T: EDeserialize<'de> + Debug,
{
    let result: Result<T, _> = eserde::json::from_str(json);
    let errors = result.unwrap_err();
    assert_eq!(
        errors.len(),
        expected.len(),
        "The number of errors does not match the expected number. Reported errors:\n- {}",
        errors
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n- ")
    );
    for (error, expected) in errors.into_iter().zip(expected.into_iter()) {
        let path = error.path().expect("No path on error");
        assert_eq!(&path.to_string(), expected, "The full error:\n\t{}", error);
    }
}

#[test]
fn test_struct() {
    #[derive(Deserialize, Debug)]
    struct Package {
        name: String,
        dependencies: Map<String, Dependency>,
    }

    #[derive(Deserialize, Debug)]
    struct Dependency {
        version: String,
    }

    let j = r#"{
        "name": "demo",
        "dependencies": {
            "serde": {
                "version": 1
            }
        }
    }"#;

    test::<Package>(j, "dependencies.serde.version");
}

#[test]
fn test_vec() {
    #[derive(Deserialize, Debug)]
    struct Package {
        dependencies: Vec<Dependency>,
    }

    #[derive(Deserialize, Debug)]
    struct Dependency {
        name: String,
        version: String,
    }

    let j = r#"{
        "dependencies": [
            {
                "name": "serde",
                "version": "1.0"
            },
            {
                "name": "serde_json",
                "version": 1
            }
        }
    }"#;

    test::<Package>(j, "dependencies[1].version");
}

#[test]
fn test_option() {
    #[derive(Deserialize, Debug)]
    struct Package {
        dependency: Option<Dependency>,
    }

    #[derive(Deserialize, Debug)]
    struct Dependency {
        version: String,
    }

    let j = r#"{
        "dependency": {
            "version": 1
        }
    }"#;

    test::<Package>(j, "dependency.version");
}

#[test]
fn test_struct_variant() {
    #[derive(Deserialize, Debug)]
    struct Package {
        dependency: Dependency,
    }

    #[derive(Deserialize, Debug)]
    enum Dependency {
        Struct { version: String },
    }

    let j = r#"{
        "dependency": {
            "Struct": {
                "version": 1
            }
        }
    }"#;

    test::<Package>(j, "dependency.Struct.version");
}

#[test]
fn test_tuple_variant() {
    #[derive(Deserialize, Debug)]
    struct Package {
        dependency: Dependency,
    }

    #[derive(Deserialize, Debug)]
    enum Dependency {
        Tuple(String, String),
    }

    let j = r#"{
        "dependency": {
            "Tuple": ["serde", 1]
        }
    }"#;

    test::<Package>(j, "dependency.Tuple[1]");
}

#[test]
fn test_multiple_tuple_variant() {
    #[derive(Deserialize, Debug)]
    struct Package {
        dependency: Dependency,
        another_dependency: Dependency,
    }

    #[derive(Deserialize, Debug)]
    enum Dependency {
        Tuple(String, String),
        Single(String),
    }

    let j = r#"{
        "dependency": {
            "Tuple": ["serde", 1]
        },
        "another_dependency": {
            "Single": 2
        }
    }"#;

    test_many::<Package>(j, &["dependency.Tuple[1]", "another_dependency.Single"]);
}

#[test]
fn test_unknown_field() {
    #[derive(Deserialize, Debug)]
    struct Package {
        dependency: Dependency,
    }

    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    struct Dependency {
        version: String,
    }

    let j = r#"{
        "dependency": {
            "version": "1.0",
            "name": "serde",
        }
    }"#;

    test_many::<Package>(j, &["dependency.name", "dependency"]);
}

#[test]
fn test_invalid_length() {
    #[derive(Deserialize, Debug)]
    struct Package {
        dependency: Dependency,
    }

    #[derive(Deserialize, Debug)]
    struct Dependency(String, String);

    let j = r#"{
        "dependency": ["serde"]
    }"#;

    test::<Package>(j, "dependency");
}

#[test]
fn test_syntax_error() {
    #[derive(Deserialize, Debug)]
    struct Package {
        dependency: Dependency,
    }

    #[derive(Deserialize, Debug)]
    struct Dependency {
        version: String,
    }

    let j = r#"{
        "dependency": {
            "error": *
    }"#;

    test_many::<Package>(j, &["dependency.error", "."]);
}

#[test]
fn test_u128() {
    #[derive(Deserialize, Debug)]
    struct Container {
        n: u128,
    }

    let j = r#"{
        "n": 130033514578017493995102500318550798591
    }"#;

    let de = &mut serde_json::Deserializer::from_str(j);
    let container: Container = serde_path_to_error::deserialize(de).expect("failed to deserialize");

    assert_eq!(container.n, 130033514578017493995102500318550798591u128);
}

#[test]
fn test_map_nonstring_key() {
    #[derive(Deserialize, Debug)]
    struct Dependency {
        version: String,
    }

    let j = r#"{
        "100": {
            "version": false
        }
    }"#;

    test::<Map<i32, Dependency>>(j, "100.version");
}

#[ignore = "Reports the top-level path, '.', rather than the content path. Fails for `serde_path_to_error` too."]
// Investigation: why does this happen?
// It is due to the way internally tagged enum are deserialized.
// We first look for the tag field and **stash** the content inside a `serde::__private::de::Content`.
// If the tag is found, we then try to deserialize the content using the appropriate variant.
// At that point though, the current path no longer reflects where we are, thus leading to this issue.
// A possible fix would be to use a custom `Content` variant that includes the original path.
// Unfortunately, that would require changing the code generated by `serde::Deserialize`, and there's
// no easy way to accomplish it as far as I'm aware beyond forking the derive macro.
#[test]
fn test_internally_tagged_enum() {
    #[derive(Debug, Deserialize)]
    #[serde(tag = "type")]
    pub enum TestEnum {
        B { value: u32 },
    }

    let j = r#"
    {
    	"type": "B",
    	"value": "500"
    }"#;

    test::<TestEnum>(j, "value");
}

#[test]
fn test_adjacent_tagged_enum() {
    #[derive(Debug, Deserialize)]
    #[serde(tag = "type", content = "content")]
    pub enum TestEnum {
        A(u32),
        B(u64),
    }

    let j = r#"
    {
        "type": "A",
        "content": "500"
    }"#;

    test::<TestEnum>(j, "content");
}
