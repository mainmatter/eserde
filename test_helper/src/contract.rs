use crate::prelude::*;

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", deny_unknown_fields)]
pub struct StructDenyUnknownFields {
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    write_only: bool,
    #[serde(default)]
    default: bool,
    #[serde(skip_serializing_if = "core::ops::Not::not")]
    skip_serializing_if: bool,
    #[serde(rename = "renamed")]
    renamed: bool,
    option: Option<bool>,
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub struct StructAllowUnknownFields {
    #[serde(flatten)]
    inner: StructDenyUnknownFields,
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub struct TupleStruct(
    String,
    #[allow(dead_code)]
    #[serde(skip)]
    bool,
    String,
    String,
);

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename_all_fields = "PascalCase")]
pub enum ExternalEnum {
    #[serde(skip_serializing)]
    WriteOnlyStruct { i: isize },
    #[serde(rename = "renamed_unit")]
    RenamedUnit,
    #[serde(rename = "renamed_struct")]
    RenamedStruct { b: bool },
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
#[serde(
    tag = "tag",
    rename_all = "SCREAMING-KEBAB-CASE",
    rename_all_fields = "PascalCase"
)]
pub enum InternalEnum {
    #[serde(skip_serializing)]
    WriteOnlyStruct { i: isize },
    #[serde(rename = "renamed_unit")]
    RenamedUnit,
    #[serde(rename = "renamed_struct")]
    RenamedStruct { b: bool },
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
#[serde(
    tag = "tag",
    content = "content",
    rename_all = "SCREAMING-KEBAB-CASE",
    rename_all_fields = "PascalCase"
)]
pub enum AdjacentEnum {
    #[serde(skip_serializing)]
    WriteOnlyStruct { i: isize },
    #[serde(rename = "renamed_unit")]
    RenamedUnit,
    #[serde(rename = "renamed_struct")]
    RenamedStruct { b: bool },
}
