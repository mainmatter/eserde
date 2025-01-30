use crate::filter_attributes::FilterAttributes;
use quote::{quote, ToTokens};
use syn::DeriveInput;

/// A type with exactly the same set of fields/variants as the original type, but with a different name.
/// This type is used to derive `Deserialize`, thus obtaining from `serde` the same deserialize implementation
/// we would get for the original type had we annotated it with `#[derive(Deserialize)]` directly.
pub struct ShadowType(pub DeriveInput);

fn keep_serde_attributes(attr: &syn::Attribute) -> bool {
    attr.meta.path().is_ident("serde")
}

impl ShadowType {
    pub fn new(ident: syn::Ident, input: &syn::DeriveInput) -> Self {
        let shadow = DeriveInput {
            vis: syn::Visibility::Inherited,
            ident,
            // We don't want to keep _all_ attributes for the shadow type, only the `serde` ones
            // (e.g. `#[serde(default)]`), so we filter out the others.
            ..input.filter_attributes(keep_serde_attributes)
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
        fn modify_field_types(fields: &mut syn::Fields) {
            for field in fields.iter_mut() {
                // If `&str` or `&[u8]` are used, we need to add a `#[serde(bound)]` attribute
                // on the wrapped field to make sure `serde` applies the right lifetime constraints.
                if let syn::Type::Reference(ref_) = &field.ty {
                    let mut add_borrow_attr = false;
                    if let syn::Type::Path(path) = &*ref_.elem {
                        if path.path.is_ident("str") {
                            add_borrow_attr = true;
                        }
                    }

                    if let syn::Type::Slice(slice) = &*ref_.elem {
                        if let syn::Type::Path(path) = &*slice.elem {
                            if path.path.is_ident("u8") {
                                add_borrow_attr = true;
                            }
                        }
                    }
                    if add_borrow_attr && !has_serde_path_attr(&field.attrs, "borrow") {
                        field.attrs.push(syn::parse_quote!(#[serde(borrow)]));
                    }
                }

                // Check if `#[serde(default)]` is already present on the field.
                // TODO: handle the `#[serde(default = "..")]` case.
                //   We'll have to generate a function that wraps around the
                //   one specified in the attribute.
                let has_default = has_serde_path_attr(&field.attrs, "default");
                // Add `#[serde(default)]` to the list:
                if !has_default {
                    field.attrs.push(syn::parse_quote!(#[serde(default)]));
                }

                field.ty = {
                    let ty_ = &field.ty;
                    let tokens = if !has_default {
                        quote! {
                            ::eserde::_macro_impl::MaybeInvalidOrMissing<#ty_>
                        }
                    } else {
                        quote! {
                            ::eserde::_macro_impl::MaybeInvalid<#ty_>
                        }
                    };
                    syn::parse2(tokens).unwrap()
                }
            }
        }

        let mut companion = DeriveInput {
            vis: syn::Visibility::Inherited,
            ident,
            generics: input.generics.clone(),
            ..input.filter_attributes(keep_serde_attributes)
        };

        let bounds: Vec<String> = companion
            .generics
            .type_params()
            // `serde` will infer that the type parameters of the companion type must implement
            // the `Default` trait, on top of the `Deserialize` trait, since we marked fields
            // that use those type parameters with `#[serde(default)]`.
            // That's unnecessary, so we override the bounds here using `#[serde(bound(deserialize = "..."))]`.
            .map(|param| format!("{}: ::eserde::_serde::Deserialize<'de>", param.ident))
            .collect::<Vec<_>>();
        if !bounds.is_empty() {
            let bound = bounds.join(", ");
            // TODO: when we start supporting `serde(bound = "...")`, we'll have to
            // concatenate the new bound with the existing ones otherwise `serde`
            // will complain about duplicate attributes.
            companion
                .attrs
                .push(syn::parse_quote!(#[serde(bound(deserialize = #bound))]));
        }

        match &mut companion.data {
            syn::Data::Struct(data_struct) => {
                modify_field_types(&mut data_struct.fields);
            }
            syn::Data::Enum(data_enum) => {
                data_enum
                    .variants
                    .iter_mut()
                    .for_each(|variant| modify_field_types(&mut variant.fields));
            }
            syn::Data::Union(_) => unreachable!(),
        };
        Self(companion)
    }
}

/// Check if the field has a `#[serde(default)]` attribute attached to it.
fn has_serde_path_attr(attrs: &[syn::Attribute], path: &str) -> bool {
    attrs.iter().any(|attr| {
        let mut has_default = false;
        if attr.path().is_ident("serde") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(path) {
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
