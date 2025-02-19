use crate::prelude::*;

#[derive(Deserialize, Serialize, Default, Arbitrary, Dummy, Debug)]
pub struct UnitStruct;

#[derive(Deserialize, Serialize, Default, Arbitrary, Dummy, Debug)]
pub struct NormalStruct {
    foo: String,
    bar: bool,
}

#[derive(Deserialize, Serialize, Default, Arbitrary, Dummy, Debug)]
pub struct NewType(String);

#[derive(Deserialize, Serialize, Default, Arbitrary, Dummy, Debug)]
pub struct TupleStruct(String, bool);

#[derive(Deserialize, Serialize, Default, Arbitrary, Dummy, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RenamedFields {
    camel_case: i32,
    #[serde(rename = "new_name")]
    old_name: i32,
}

#[derive(Deserialize, Serialize, Default, Arbitrary, Dummy, Debug)]
#[serde(deny_unknown_fields)]
pub struct DenyUnknownFields {
    foo: String,
    bar: bool,
}
