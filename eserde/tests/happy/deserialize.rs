#[derive(eserde::Deserialize)]
struct NamedStruct {
    #[serde(default)]
    a: Option<u32>,
    b: TupleStructOneField,
    c: Vec<TupleStructMultipleFields>,
}

#[derive(eserde::Deserialize)]
struct TupleStructOneField(#[serde(default)] Option<u32>);

#[derive(eserde::Deserialize)]
struct TupleStructMultipleFields(#[serde(default)] Option<u32>, u32, u64);
