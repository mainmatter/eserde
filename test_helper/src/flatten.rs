use std::collections::BTreeMap;

use crate::prelude::*;

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub struct Flat {
    f: f32,
    b: bool,
    #[serde(default, skip_serializing_if = "str::is_empty")]
    s: String,
    v: Vec<i32>,
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub struct Deep1 {
    f: f32,
    #[serde(flatten)]
    deep2: Deep2,
    v: Vec<i32>,
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub struct Deep2 {
    b: bool,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    deep3: Option<Deep3>,
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub struct Deep3 {
    s: String,
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub struct FlattenValue {
    flag: bool,
    #[serde(flatten)]
    value: JsonValue,
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub struct FlattenMap {
    flag: bool,
    #[serde(flatten)]
    value: BTreeMap<String, JsonValue>,
}
