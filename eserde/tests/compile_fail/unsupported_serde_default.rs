#[derive(eserde::Deserialize)]
struct Foo {
    #[serde(rename = "route", default = "default_route")]
    route_0: String,
    #[serde(default = "default_route")]
    route_1: String,
}

fn default_route() -> String {
    "/".to_string()
}

fn main() {}
