use std::ops::ControlFlow::{self, Break, Continue};

use proc_macro2::Span;
use quote::ToTokens;
use syn::{meta::ParseNestedMeta, punctuated::Punctuated, Error, Expr, Meta, Path, Result, Token};

/// Represents a single meta item within an attribute.
///
/// For example, `default` and `rename = "foo"` within `#[serde(default, rename = "foo")]`).
#[derive(Clone, Debug)]
pub struct MetaItem {
    pub key: Path,
    pub value: Option<(Token![=], Expr)>,
}
impl MetaItem {
    pub fn parse(parser: ParseNestedMeta) -> Result<Self> {
        let key = parser.path;
        let value = if parser.input.peek(Token![=]) {
            let eq = parser.input.parse().unwrap();
            let expr = parser.input.parse()?;
            Some((eq, expr))
        } else {
            None
        };
        Ok(Self { key, value })
    }
}
impl ToTokens for MetaItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.key.to_tokens(tokens);
        if let Some((eq, expr)) = &self.value {
            eq.to_tokens(tokens);
            expr.to_tokens(tokens);
        }
    }
}

/// Returns if `#[ATTR_NAME(META_KEY)]` exists within any of the attributes.
///
/// ```rust
/// # mod attr { include!("attr.rs"); } use attr::find_attr_meta; // hack for proc_macro doctests.
/// # fn main() {
/// use syn::parse_quote;
///
/// // `Some(..)`
/// assert!(find_attr_meta(&[parse_quote!( #[serde(rename)] )], "serde", "rename").is_some());
/// assert!(find_attr_meta(&[parse_quote!( #[serde(rename = "foo")] )], "serde", "rename").is_some());
/// assert!(find_attr_meta(&[parse_quote!( #[serde(default, rename)] )], "serde", "rename").is_some());
/// assert!(find_attr_meta(&[parse_quote!( #[serde(default, rename = "bar")] )], "serde", "rename").is_some());
/// assert!(find_attr_meta(&[parse_quote!( #[serde(rename, rename = "baz", rename)] )], "serde", "rename").is_some());
/// assert!(find_attr_meta(&[parse_quote!( #[serde(rename, rename)] )], "serde", "rename").is_some());
/// // `None`
/// assert!(find_attr_meta(&[parse_quote!( #[serde(default)] )], "serde", "rename").is_none());
/// assert!(find_attr_meta(&[parse_quote!( #[ignore(rename = "bing")] )], "serde", "rename").is_none());
/// assert!(find_attr_meta(&[parse_quote!( #[serde = "rename"] )], "serde", "rename").is_none());
/// # }
/// ```
pub fn find_attr_meta(
    attrs: &[syn::Attribute],
    attr_name: &str,
    meta_key: &str,
) -> Option<MetaItem> {
    visit_attr_metas(attrs, attr_name, |meta_item| {
        if meta_item.key.is_ident(meta_key) {
            Break(meta_item)
        } else {
            Continue(())
        }
    })
}

/// Visits each meta item within each `#[ATTR_NAME(...)]` attribute, calling `visitor` on each one.
///
/// If `visitor` returns `Break`, the iteration stops and the value is returned.
pub fn visit_attr_metas<T>(
    attrs: &[syn::Attribute],
    attr_name: &str,
    mut visitor: impl FnMut(MetaItem) -> ControlFlow<T>,
) -> Option<T> {
    attrs.iter().find_map(move |attr| {
        if !attr.path().is_ident(attr_name) {
            return None;
        }
        let mut out = None;
        let _ = attr.parse_nested_meta(|meta_item| {
            let meta_item = MetaItem::parse(meta_item)?;
            if let Break(value) = (visitor)(meta_item) {
                out = Some(value);
                // Exit `parse_nested_meta` early.
                Err(Error::new(Span::call_site(), ""))
            } else {
                Ok(())
            }
        });
        out
    })
}

/// Removes the first `#[ATTR_NAME(META_KEY)]` within the attributes and returns it.
pub fn remove_attr_meta(
    attrs: &mut [syn::Attribute],
    attr_name: &str,
    meta_key: &str,
) -> Option<MetaItem> {
    attrs.iter_mut().find_map(|attr| {
        if !attr.path().is_ident(attr_name) {
            return None;
        }
        let Meta::List(meta_list) = &mut attr.meta else {
            return None;
        };
        let mut out = None;
        // The transformed meta items.
        let mut transformed = Punctuated::<MetaItem, Token![,]>::new();
        let result = meta_list.parse_nested_meta(|meta_item| {
            let meta_item = MetaItem::parse(meta_item)?;
            if meta_item.key.is_ident(meta_key) && out.is_none() {
                // Remove the item.
                out = Some(meta_item);
            } else {
                // Keep the original item.
                transformed.push(meta_item.clone());
            }
            Ok(())
        });

        if result.is_err() {
            // Parsing error occured, leave this attribute unchanged, continue.
            None
        } else {
            // Replace the attribute's list tokens with the transformed ones.
            meta_list.tokens = transformed.into_token_stream();
            out
        }
    })
}
