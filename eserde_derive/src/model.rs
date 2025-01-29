use crate::filter_attributes::FilterAttributes;
use quote::{quote, ToTokens};
use syn::DeriveInput;

/// A type with exactly the same set of fields/variants as the original type, but with a different name.
/// This type is used to derive `Deserialize`, thus obtaining from `serde` the same deserialize implementation
/// we would get for the original type had we annotated it with `#[derive(Deserialize)]` directly.
pub struct ShadowType(pub DeriveInput);

impl ShadowType {
    pub fn new(ident: syn::Ident, input: &syn::DeriveInput) -> Self {
        fn keep_serde_attributes(attr: &syn::Attribute) -> bool {
            attr.meta.path().is_ident("serde")
        }

        let shadow = DeriveInput {
            vis: syn::Visibility::Inherited,
            ident,
            generics: input.generics.clone(),
            // We don't want to keep _all_ attributes for the shadow type, only the `serde` ones
            // (e.g. `#[serde(default)]`), so we filter out the others.
            attrs: input.attrs.filter_attributes(keep_serde_attributes),
            data: input.data.filter_attributes(keep_serde_attributes),
        };
        Self(shadow)
    }
}

impl ToTokens for ShadowType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self(input) = self;
        input.to_tokens(tokens);
    }
}

/// A companion type that, unlike the original, uses `MaybeInvalidOrMissing<T>` for all fields, where
/// `T` is the original field type.
/// This type should never fail to deserialize, thus allowing us to collect all errors in one go.
pub struct PermissiveCompanionType(pub DeriveInput);

impl PermissiveCompanionType {
    pub fn new(ident: syn::Ident, input: &syn::DeriveInput) -> Self {
        fn keep_serde_attributes(attr: &syn::Attribute) -> bool {
            attr.meta.path().is_ident("serde")
        }

        let mut data = input.data.filter_attributes(keep_serde_attributes);

        match &mut data {
            syn::Data::Struct(data_struct) => {
                match &mut data_struct.fields {
                    syn::Fields::Named(fields_named) => {
                        for field in fields_named.named.iter_mut() {
                            // Check if `#[serde(default)]` is already present on the field.
                            // TODO: handle the `#[serde(default = "..")]` case.
                            //   We'll have to generate a function that wraps around the
                            //   one specified in the attribute.
                            if !has_serde_default(&field.attrs) {
                                let ty_ = &field.ty;
                                let ty_: syn::Type = syn::parse2(quote! {
                                    ::eserde::_macro_impl::MaybeInvalidOrMissing<#ty_>
                                })
                                .unwrap();
                                field.ty = ty_;
                                // Add `#[serde(default)]` to the list:
                                field.attrs.push(syn::parse_quote!(#[serde(default)]));
                            } else {
                                let ty_ = &field.ty;
                                let ty_: syn::Type = syn::parse2(quote! {
                                    ::eserde::_macro_impl::MaybeInvalid<#ty_>
                                })
                                .unwrap();
                                field.ty = ty_;
                            }
                        }
                    }
                    syn::Fields::Unnamed(fields_unnamed) => {
                        let n_fields = fields_unnamed.unnamed.len();
                        for (i, field) in fields_unnamed.unnamed.iter_mut().enumerate() {
                            // Check if `#[serde(default)]` is already present on the field.
                            // TODO: handle the `#[serde(default = "..")]` case.
                            //   We'll have to generate a function that wraps around the
                            //   one specified in the attribute.
                            let is_last_field = i == n_fields - 1;
                            if !has_serde_default(&field.attrs) && is_last_field {
                                let ty_ = &field.ty;
                                let ty_: syn::Type = syn::parse2(quote! {
                                    ::eserde::_macro_impl::MaybeInvalidOrMissing<#ty_>
                                })
                                .unwrap();
                                field.ty = ty_;
                                // Add `#[serde(default)]` to the list:
                                field.attrs.push(syn::parse_quote!(#[serde(default)]));
                            } else {
                                let ty_ = &field.ty;
                                let ty_: syn::Type = syn::parse2(quote! {
                                    ::eserde::_macro_impl::MaybeInvalid<#ty_>
                                })
                                .unwrap();
                                field.ty = ty_;
                            }
                        }
                    }
                    syn::Fields::Unit => unimplemented!(),
                };
            }
            syn::Data::Enum(_) | syn::Data::Union(_) => unimplemented!(),
        };

        let companion = DeriveInput {
            vis: syn::Visibility::Inherited,
            ident,
            generics: input.generics.clone(),
            // We don't want to keep _all_ attributes for the shadow type, only the `serde` ones
            // (e.g. `#[serde(default)]`), so we filter out the others.
            attrs: input.attrs.filter_attributes(keep_serde_attributes),
            data,
        };
        Self(companion)
    }
}

/// Check if the field has a `#[serde(default)]` attribute attached to it.
fn has_serde_default(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        let mut has_default = false;
        if attr.path().is_ident("serde") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("default") {
                    has_default = true;
                }
                Ok(())
            });
        }
        has_default
    })
}

impl ToTokens for PermissiveCompanionType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self(input) = self;
        input.to_tokens(tokens);
    }
}
