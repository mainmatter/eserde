use crate::prelude::*;
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub struct Struct {
    foo: i32,
    bar: bool,
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
#[serde(deny_unknown_fields)]
pub struct StructDenyUnknownFields {
    baz: i32,
    foobar: bool,
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
#[serde(deny_unknown_fields)]
pub enum External {
    Unit,
    StringMap(BTreeMap<String, String>),
    StructNewType(Struct),
    StructDenyUnknownFieldsNewType(StructDenyUnknownFields),
    Struct { foo: i32, bar: bool },
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
#[serde(tag = "tag", deny_unknown_fields)]
pub enum Internal {
    Unit,
    StringMap(BTreeMap<String, String>),
    StructNewType(Struct),
    StructDenyUnknownFieldsNewType(StructDenyUnknownFields),
    Struct { foo: i32, bar: bool },
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
#[serde(tag = "tag", content = "content", deny_unknown_fields)]
pub enum Adjacent {
    Unit,
    StringMap(BTreeMap<String, String>),
    StructNewType(Struct),
    StructDenyUnknownFieldsNewType(StructDenyUnknownFields),
    Struct { foo: i32, bar: bool },
}
