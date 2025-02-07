use syn::DeriveInput;

use crate::filter_attributes::FilterAttributes;

/// Return a compiler error if the input contains data types or
/// `serde` attributes that are not supported by our custom derive.
pub fn reject_unsupported_inputs(input: &DeriveInput) -> Result<(), syn::Error> {
    let mut errors = Vec::new();

    let input = input.filter_attributes(|a| a.meta.path().is_ident("serde"));
    reject_container_attributes(&mut errors, &input.attrs);

    match &input.data {
        syn::Data::Struct(data_struct) => {
            data_struct.fields.iter().for_each(|field| {
                reject_field_attributes(&mut errors, field);
            });
        }
        syn::Data::Enum(data_enum) => {
            data_enum.variants.iter().for_each(|variant| {
                reject_variant_attributes(&mut errors, variant);

                variant.fields.iter().for_each(|field| {
                    reject_field_attributes(&mut errors, field);
                });
            });
        }
        syn::Data::Union(_) => {
            errors.push(syn::Error::new_spanned(&input, "Unions are not supported"));
        }
    }

    if let Some(first_error) = errors.pop() {
        let error = errors.into_iter().fold(first_error, |mut acc, e| {
            acc.combine(e);
            acc
        });
        Err(error)
    } else {
        Ok(())
    }
}

/// Attributes from <https://serde.rs/container-attrs.html> that we either
/// can't support or haven't implemented yet.
fn reject_container_attributes(errors: &mut Vec<syn::Error>, attrs: &[syn::Attribute]) {
    // We can't support `#[serde(untagged)]` because our permissive deserialization
    // strategy conflicts with the "try-until-it-succeeds" mechanism used by
    // `untagged` deserialization to find the correct variant.
    if let Some(attr) = find_serde_path_attr(&attrs, "untagged") {
        errors.push(syn::Error::new_spanned(
            attr,
            "`eserde::Deserialize` can't be derived for enums that use the untagged representation. \
            Use a plain `#[derive(serde::Deserialize)]` instead.",
        ));
    }

    for (path, example, additional) in [
        (
            "default",
            "`#[serde(default)]`",
            " It is only supported on fields.",
        ),
        (
            "remote",
            "`#[serde(remote  = \"..\")]`",
            " It can only be derived for local types.",
        ),
        ("try_from", "`#[serde(try_from = \"..\")]`", ""),
        ("from", "`#[serde(from = \"..\")]`", ""),
        ("bound", "`#[serde(bound = \"..\")]`", ""),
        ("variant_identifier", "`#[serde(variant_identifier)]`", ""),
        ("field_identifier", "`#[serde(field_identifier)]`", ""),
    ] {
        if let Some(attr) = find_serde_path_attr(&attrs, path) {
            errors.push(syn::Error::new_spanned(
                attr,
                format!("`eserde::Deserialize` doesn't yet support the {example} attribute.{additional}",
            )));
        }
    }
}

/// Attributes from <https://serde.rs/field-attrs.html> that we either
/// can't support or haven't implemented yet.
fn reject_field_attributes(errors: &mut Vec<syn::Error>, field: &syn::Field) {
    for (path, example) in [
        ("skip_deserializing", "`#[serde(skip_deserializing)]`"),
        ("deserialize_with", "`#[serde(deserialize_with = \"..\")]`"),
        ("with", "`#[serde(with = \"..\")]`"),
        ("bound", "`#[serde(bound = \"..\")]`"),
    ] {
        if find_serde_path_attr(&field.attrs, path).is_some() {
            errors.push(syn::Error::new_spanned(
                field,
                format!(
                    "`eserde::Deserialize` doesn't yet support the {example} attribute on fields."
                ),
            ));
        }
    }

    if find_serde_path_attr_with_value(&field.attrs, "default").is_some() {
        errors.push(syn::Error::new_spanned(
            field,
            format!(
                "`eserde::Deserialize` doesn't yet support the `#[serde(default = \"..\")]` attribute. \
                It only supports `#[serde(default)]`, which defers to the `Default` trait to generate the default value."
            ),
        ));
    }
}

/// Attributes from <https://serde.rs/variant-attrs.html> that we either
/// can't support or haven't implemented yet.
fn reject_variant_attributes(errors: &mut Vec<syn::Error>, variant: &syn::Variant) {
    for (path, example) in [
        ("skip_deserializing", "`#[serde(skip_deserializing)]`"),
        ("deserialize_with", "`#[serde(deserialize_with = \"..\")]`"),
        ("with", "`#[serde(with = \"..\")]`"),
        ("bound", "`#[serde(bound = \"..\")]`"),
        ("untagged", "`#[serde(untagged)]`"),
    ] {
        if find_serde_path_attr(&variant.attrs, path).is_some() {
            errors.push(syn::Error::new_spanned(
                variant,
                format!(
                    "`eserde::Deserialize` doesn't yet support the {example} attribute on enum variants."
                ),
            ));
        }
    }
}

/// Check if the field has a `#[serde({ident})]` attribute attached to it.
fn find_serde_path_attr<'a>(
    attrs: &'a [syn::Attribute],
    ident: &str,
) -> Option<&'a syn::Attribute> {
    attrs.iter().find(|attr| {
        let mut matches = false;
        if attr.path().is_ident("serde") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(ident) {
                    matches = true;
                }
                Ok(())
            });
        }
        matches
    })
}

/// Check if the field has a `#[serde({ident} = "{value}")]` attribute attached to it.
fn find_serde_path_attr_with_value<'a>(
    attrs: &'a [syn::Attribute],
    ident: &str,
) -> Option<&'a syn::Attribute> {
    attrs.iter().find(|attr| {
        let mut matches = false;
        if attr.path().is_ident("serde") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(ident) && meta.value().is_ok() {
                    matches = true;
                }
                Ok(())
            });
        }
        matches
    })
}
