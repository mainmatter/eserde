#[derive(eserde::Deserialize)]
struct Foo {
    #[serde(default)]
    field: NoDefault,
}

#[derive(eserde::Deserialize)]
struct NoDefault;

fn main() {}
