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

#[test]
fn deserialize() {
    let payloads = [
        r#"{
            "b": 5,
            "c": [[1, 2, 3], [4, 5, 6]]
        }"#,
        r#"{
            "a": 5,
            "b": null,
            "c": [[null, 2, 3], [4, 5, 6]]
        }"#,
    ];
    for payload in payloads {
        assert!(
            serde_json::from_str::<NamedStruct>(payload).is_ok(),
            "Failed to deserialize: {}",
            payload
        );
    }
}
