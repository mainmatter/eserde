#[derive(eserde::Deserialize)]
struct Foo {
    #[serde(default - this parses but is not Meta Item Attribute Syntax, serde errors "expected `,`")]
    field: u32,
}

fn main() {}
