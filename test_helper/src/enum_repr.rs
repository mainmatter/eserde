use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::prelude::*;

#[derive(Deserialize_repr, Serialize_repr, Debug, Dummy, Arbitrary)]
#[repr(u8)]
#[serde(rename = "EnumWithReprAttr")]
/// Description from comment
pub enum EnumWithReprAttr {
    Zero,
    One,
    Five = 5,
    Six,
    Three = 3,
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub struct Enum(#[eserde(compat)] EnumWithReprAttr);
