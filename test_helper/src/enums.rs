use std::collections::BTreeMap;

use crate::prelude::*;

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub struct UnitStruct;

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary, Default)]
pub struct Struct {
    foo: i32,
    bar: bool,
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
#[serde(rename_all = "camelCase")]
pub enum External {
    UnitOne,
    StringMap(BTreeMap<String, String>),
    UnitStructNewType(UnitStruct),
    StructNewType(Struct),
    Struct { foo: i32, bar: bool },
    Tuple(i32, bool),
    UnitTwo,
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
#[serde(tag = "tag")]
pub enum Internal {
    UnitOne,
    StringMap(BTreeMap<String, String>),
    UnitStructNewType(UnitStruct),
    StructNewType(Struct),
    Struct { foo: i32, bar: bool },
    UnitTwo,
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
#[serde(tag = "tag", content = "content")]
pub enum Adjacent {
    UnitOne,
    StringMap(BTreeMap<String, String>),
    UnitStructNewType(UnitStruct),
    StructNewType(Struct),
    Struct { foo: i32, bar: bool },
    Tuple(i32, bool),
    UnitTwo,
}

#[derive(serde::Deserialize, Serialize, Dummy, Debug, Arbitrary)]
#[serde(untagged)]
pub enum Untagged {
    UnitOne,
    StringMap(BTreeMap<String, String>),
    UnitStructNewType(UnitStruct),
    StructNewType(Struct),
    Struct { foo: i32, bar: bool },
    Tuple(i32, bool),
    UnitTwo,
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub struct UntaggedWrapper(#[eserde(compat)] Untagged);

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
#[serde(rename_all_fields = "UPPERCASE", rename_all = "snake_case")]
pub enum Renamed {
    StructVariant {
        field: String,
    },
    #[serde(rename = "custom name variant")]
    RenamedStructVariant {
        #[serde(rename = "custom name field")]
        field: String,
    },
}
