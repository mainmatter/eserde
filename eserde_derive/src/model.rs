use crate::filter_attributes::FilterAttributes;
use quote::ToTokens;
use syn::DeriveInput;

/// A type with exactly the same set of fields/variants as the original type, but with a different name.
/// This type is used to derive `Deserialize`, thus obtaining from `serde` the same deserialize implementation
/// we would get for the original type had we annotated it with `#[derive(Deserialize)]` directly.
pub struct ShadowType(pub DeriveInput);

impl ShadowType {
    pub fn new(ident: syn::Ident, input: &syn::DeriveInput) -> Self {
        let shadow = DeriveInput {
            vis: syn::Visibility::Inherited,
            ident,
            generics: input.generics.clone(),
            // We don't want to keep _all_ attributes for the shadow type, only the `serde` ones
            // (e.g. `#[serde(default)]`), so we filter out the others.
            attrs: input
                .attrs
                .iter()
                .filter(|a| keep_serde_attributes(a))
                .cloned()
                .collect(),
            data: input.data.filter_attributes(|a| keep_serde_attributes(a)),
        };
        Self(shadow)
    }
}

fn keep_serde_attributes(attr: &syn::Attribute) -> bool {
    attr.meta.path().is_ident("serde")
}

impl ToTokens for ShadowType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self(input) = self;
        input.to_tokens(tokens);
    }
}
