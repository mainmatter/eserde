[![crates.io](https://img.shields.io/crates/v/eserde.svg)](https://crates.io/crates/eserde)
[![Documentation](https://img.shields.io/docsrs/eserde)](https://docs.rs/eserde)
[![Downloads crates.io](https://img.shields.io/crates/d/eserde.svg?label=crates.io%20downloads)](https://crates.io/crates/eserde)

<!-- cargo-rdme start -->

# eserde

Don't stop at the first deserialization error.

> [!NOTE]
> This project has been created by [Mainmatter](https://mainmatter.com/rust-consulting/).  
> Check out our [landing page](https://mainmatter.com/rust-consulting/) if you're looking for Rust consulting or training!

## The problem

[`serde`](https://serde.rs) is **the** Rust library for (de)serialization.\
There's a catch, though: `serde` is designed to abort deserialization as soon as an error occurs.
This becomes an issue when relying on `serde` for deserializing user-provided payloads—e.g. a
request body for a REST API.\
There may be _several_ errors in the submitted payload, but [`serde_json`](https://crates.io/crates/serde_json)
will only report the first one it encounters before stopping deserialization.
The API consumer is then forced into a slow and frustrating feedback loop:

1. Send request
2. Receive a single error back
3. Fix the error
4. Back to 1., until there are no more errors to be fixed

That's a poor developer experience. We should do better!\
We should report _multiple_ errors at once, thus reducing the number of API interactions
required to converge to a well-formed payload.

That's the problem `eserde` was born to solve.

## Case study: an invalid JSON payload

Let's consider this schema as our reference example:

```rust
#[derive(Debug, serde::Deserialize)]
struct Package {
    version: Version,
    source: String,
}

#[derive(Debug, eserde::Deserialize)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}
```

We'll try to deserialize an invalid JSON payload into it via `serde_json`:

```rust
let payload = r#"
    {
        "version": {
            "major": 1,
            "minor": "2"
        },
        "source": null
    }"#;
let error = serde_json::from_str::<Package>(&payload).unwrap_err();
assert_eq!(
    error.to_string(),
    r#"invalid type: string "2", expected u32 at line 5 column 24"#
);
```

Only the first error is returned, as expected. But we know there's more than that!\
We're missing the `patch` field in the `Version` struct and the `source` field can't
be null.\
Let's switch to `eserde`:

```rust
#[derive(Debug, eserde::Deserialize)]
//              ^^^^^^^^^^^^^^^^^^^
//          Using `eserde::Deserialize`
//        instead of `serde::Deserialize`!
struct Package {
    version: Version,
    source: String,
}

#[derive(Debug, eserde::Deserialize)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

let payload = r#"
    {
        "version": {
            "major": 1,
            "minor": "2"
        },
        "source": null
    }"#;
let errors = eserde::json::from_str::<Package>(&payload).unwrap_err();
//           ^^^^^^^^^^^^
//      We're not using `serde_json` directly here!
assert_eq!(
    errors.to_string(),
    r#"Something went wrong during deserialization:
- version.minor: invalid type: string "2", expected u32 at line 5 column 24
- version: missing field `patch`
- source: invalid type: null, expected a string at line 7 column 22
"#
);
```

Much better, isn't it?\
We can now inform the users _in one go_ that they have to fix three different schema violations.

## Adopting `eserde`

To use `eserde` in your projects, add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
eserde = { version = "0.1" }
serde = "1"
```

You then have to:

- Replace all instances of `#[derive(serde::Deserialize)]` with `#[derive(eserde::Deserialize)]`
- Switch to an `eserde`-based deserialization function

### JSON

`eserde` provides first-class support for JSON deserialization, gated behind the `json` Cargo feature.

```toml
[dependencies]
# Activating the `json` feature
eserde = { version = "0.1", features = ["json"] }
serde = "1"
```

If you're working with JSON:
- Replace `serde_json::from_str` with `eserde::json::from_str`
- Replace `serde_json::from_slice` with `eserde::json::from_slice`

`eserde::json` doesn't support deserializing from a reader, i.e. there is no equivalent to
`serde_json::from_reader`.

### Other formats

The approach used by `eserde` is compatible, in principle, with all existing `serde`-based
deserializers.\
Refer to [the source code of `eserde::json::from_str`](https://github.com/mainmatter/eserde/blob/main/eserde/src/json.rs)
as a blueprint to follow for building an `eserde`-powered deserialization function
for another format.

## Compatibility

`eserde` is designed to be maximally compatible with `serde`.

[`derive(eserde::Deserialize)`](Deserialize) will implement both
`serde::Deserialize` and `eserde::EDeserialize`, honoring the behaviour of all
the `serde` attributes it supports.

If one of your fields doesn't implement `eserde::EDeserialize`, you can annotate it with
`#[eserde(compat)]` to fall back to `serde`'s default deserialization logic for that
portion of the input.

```rust
#[derive(eserde::Deserialize)]
struct Point {
    // 👇 Use the `eserde::compat` attribute if you need to use
    //    a field type that doesn't implement `eserde::EDeserialize`
    //    and you can't derive `eserde::EDeserialize` for it (e.g. a third-party type)
    #[eserde(compat)]
    x: Latitude,
    // [...]
}
```

Check out the documentation of `eserde`'s derive macro for more details.

## Limitations and downsides

`eserde` is a new library—there may be issues and bugs that haven't been uncovered yet.
Test it thoroughly before using it in production. If you encounter any problems, please
open an issue on our [GitHub repository](https://github.com/mainmatter/eserde).

Apart from defects, there are some downsides inherent in `eserde`'s design:

- The input needs to be visited twice, hence it can't deserialize from a non-replayable reader.
- The input needs to be visited twice, hence it's going to be _slower_ than a single `serde::Deserialize`
  pass.
- `#[derive(eserde::Deserialize)]` generates more code than `serde::Deserialize` (roughly twice as much),
  so it'll have a bigger impact than vanilla `serde` on your compilation times.

We believe the trade-off is worthwhile for user-facing payloads, but you should walk in with your
eyes wide open.

## Future plans

We plan to add first-class support for more data formats, in particular YAML and TOML. They are frequently
used for configuration files, another scenario where batch error reporting would significantly improve
our developer experience.

We plan to incrementally support more and more `#[serde]` attributes,
thus minimising the friction to adopting `eserde` in your codebase.

We plan to add first-class support for validation, with a syntax similar to [`garde`](https://docs.rs/garde/latest/garde/)
and [`validator`](https://docs.rs/validator/latest/validator/).
The key difference: validation would be performed _as part of_ the deserialization process. No need to
remember to call `.validate()` afterwards.

<!-- cargo-rdme end -->
