use syn::DeriveInput;

use crate::filter_attributes::FilterAttributes;

/// Return a compiler error if the input contains data types or
/// `serde` attributes that are not supported by our custom derive.
pub fn reject_unsupported_inputs(input: &DeriveInput) -> Result<(), syn::Error> {
    let mut errors = Vec::new();
    if matches!(input.data, syn::Data::Union(_)) {
        errors.push(syn::Error::new_spanned(input, "Unions are not supported"));
    }

    let input = input.filter_attributes(|a| a.meta.path().is_ident("serde"));
    reject_container_attributes(&mut errors, &input.attrs);

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

    if let Some(attr) = find_serde_path_attr(&attrs, "default") {
        errors.push(syn::Error::new_spanned(
            attr,
            "`eserde::Deserialize` doesn't yet support the `#[serde(default)]` attribute \
            on structs. It is only supported on fields.",
        ));
    }

    if let Some(attr) = find_serde_path_attr(&attrs, "remote") {
        errors.push(syn::Error::new_spanned(
            attr,
            "`eserde::Deserialize` doesn't yet support the `#[serde(remote = \"..\")]` attribute. \
            It can only be derived for local types.",
        ));
    }

    if let Some(attr) = find_serde_path_attr(&attrs, "from") {
        errors.push(syn::Error::new_spanned(
            attr,
            "`eserde::Deserialize` doesn't yet support the `#[serde(from = \"..\")]` attribute.",
        ));
    }

    if let Some(attr) = find_serde_path_attr(&attrs, "try_from") {
        errors.push(syn::Error::new_spanned(
            attr,
            "`eserde::Deserialize` doesn't yet support the `#[serde(try_from = \"..\")]` attribute.",
        ));
    }

    if let Some(attr) = find_serde_path_attr(&attrs, "variant_identifier") {
        errors.push(syn::Error::new_spanned(
            attr,
            "`eserde::Deserialize` doesn't yet support the `#[serde(variant_identifier)]` attribute.",
        ));
    }

    if let Some(attr) = find_serde_path_attr(&attrs, "field_identifier") {
        errors.push(syn::Error::new_spanned(
            attr,
            "`eserde::Deserialize` doesn't yet support the `#[serde(field_identifier)]` attribute.",
        ));
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
