use crate::prelude::*;

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub enum Enum1 {
    B(bool),
    S(String),
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub enum Enum2 {
    U(u32),
    F(f64),
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub enum Enum3 {
    B2(bool),
    S2(String),
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub enum Enum4 {
    U2(u32),
    F2(f64),
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub enum Enum5 {
    B3(bool),
    S3(String),
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
pub struct Container {
    f: f32,
    #[serde(flatten)]
    e1: Enum1,
    #[serde(flatten)]
    e2: Enum2,
    #[serde(flatten)]
    e3: Enum3,
    #[serde(flatten)]
    e4: Enum4,
    #[serde(flatten)]
    e5: Enum5,
}

#[derive(Deserialize, Serialize, Dummy, Debug, Arbitrary)]
#[serde(deny_unknown_fields)]
pub struct ContainerDenyUnknownFields {
    f: f32,
    #[serde(flatten)]
    e1: Enum1,
    #[serde(flatten)]
    e2: Enum2,
    #[serde(flatten)]
    e3: Enum3,
    #[serde(flatten)]
    e4: Enum4,
    #[serde(flatten)]
    e5: Enum5,
}
