use std::collections::HashSet;

use crate::{
    attr::{find_attr_meta, remove_attr_meta},
    filter_attributes::FilterAttributes,
};
use indexmap::IndexSet;
use syn::{spanned::Spanned, DeriveInput};

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
            ..input.filter_attributes(|attr| attr.meta.path().is_ident("serde"))
        };
        Self(shadow)
    }
}

/// A companion type that, unlike the original, uses `MaybeInvalidOrMissing<T>` for all fields, where
/// `T` is the original field type.
/// This type should never fail to deserialize, thus allowing us to collect all errors in one go.
pub struct PermissiveCompanionType {
    pub ty_: DeriveInput,
    /// Generic type parameters that must be constrained with `::eserde::EDeserialize` instead of `::serde::Deserialize`.
    pub eserde_aware_generics: IndexSet<syn::Ident>,
    /// Optional impl block; contains methods for `#[serde(deserialize_with)]` attributes.
    pub impl_: Option<syn::ItemImpl>,
}

impl PermissiveCompanionType {
    pub fn new(ident: syn::Ident, input: &syn::DeriveInput) -> Self {
        let mut companion = DeriveInput {
            vis: syn::Visibility::Inherited,
            ident,
            generics: input.generics.clone(),
            ..input.filter_attributes(|attr| {
                attr.meta.path().is_ident("serde") || attr.meta.path().is_ident("eserde")
            })
        };

        // We keep track of the generic parameters that are used in the fields of the companion type
        // that have not been marked with `#[serde(compat)]`. We'll use this information to generate
        // the correct `#[serde(bound)]` attributes for them.
        let generic_params: HashSet<syn::Ident> = companion
            .generics
            .type_params()
            .map(|param| param.ident.clone())
            .collect();
        let mut eserde_aware_generics = IndexSet::new();

        let mut impl_items: Vec<syn::ImplItem> = Vec::new();

        let mut modify_field_types = |fields: &mut syn::Fields| {
            for (i, field) in fields.iter_mut().enumerate() {
                let span = field.span();

                // Process all `eserde` attributes, then remove them since
                // they are not valid `serde` attributes.
                let is_eserde_compatible =
                    find_attr_meta(&field.attrs, "eserde", "compat").is_none();
                field.attrs.retain(keep_serde_attributes);

                if is_eserde_compatible {
                    collect_generic_type_params(
                        &field.ty,
                        &mut eserde_aware_generics,
                        &generic_params,
                    );
                }

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
                    if add_borrow_attr && find_attr_meta(&field.attrs, "serde", "borrow").is_none()
                    {
                        field
                            .attrs
                            .push(syn::parse_quote_spanned!(span=> #[serde(borrow)]));
                    }
                }

                // Check if `#[serde(default)]` is already present on the field.
                // TODO: handle the `#[serde(default = "..")]` case.
                //   We'll have to generate a function that wraps around the
                //   one specified in the attribute.
                let has_default = find_attr_meta(&field.attrs, "serde", "default").is_some();
                // Add `#[serde(default)]` to the list:
                if !has_default {
                    field
                        .attrs
                        .push(syn::parse_quote_spanned!(span=> #[serde(default)]));
                }

                let field_ty = &field.ty;
                let wrapper_ty = if has_default {
                    syn::parse_quote_spanned!(field_ty.span()=> ::eserde::_macro_impl::MaybeInvalid::<#field_ty>)
                } else {
                    syn::parse_quote_spanned!(field_ty.span()=> ::eserde::_macro_impl::MaybeInvalidOrMissing::<#field_ty>)
                };

                if is_eserde_compatible {
                    // Add or replace `#[serde(deserialize_with = "..")]` for our wrapper.

                    // Handle user `#[serde(deserialize_with)]` attributes.
                    let dewith_path =
                        remove_attr_meta(&mut field.attrs, "serde", "deserialize_with")
                            .and_then(|meta_item| meta_item.value)
                            .and_then(|(_eq, expr)| {
                                // Get `expr` as a string literal and parse as a path.
                                let syn::Expr::Lit(syn::ExprLit {
                                    attrs: _,
                                    lit: syn::Lit::Str(lit_str),
                                }) = expr
                                else {
                                    return None;
                                };
                                syn::parse_str::<syn::Path>(lit_str.value().as_str()).ok()
                            });

                    let attr = if let Some(dewith_path) = dewith_path {
                        // User specified a custom `deserialize_with` function.
                        // We need to wrap it in our own function.

                        let fn_name = format!(
                            "__eserde_deserialize_with_{}",
                            field
                                .ident
                                .as_ref()
                                .map(|ident| ident.to_string())
                                .unwrap_or_else(|| i.to_string()),
                        );
                        let fn_ident = syn::Ident::new(&fn_name, field.span());

                        // Add the method to `deserialize_withs`.
                        impl_items.push(syn::parse_quote_spanned! {field.span()=>
                                fn #fn_ident<'de, D>(deserializer: D) -> ::core::result::Result<#wrapper_ty, D::Error>
                                where
                                    D: ::eserde::_serde::Deserializer<'de>,
                                {
                                    let result: ::core::result::Result<#field_ty, D::Error> = (#dewith_path)(deserializer);
                                    let value = match result {
                                        Ok(_) => #wrapper_ty::Valid(::core::marker::PhantomData),
                                        Err(e) => {
                                            ::eserde::reporter::ErrorReporter::report(e);
                                            #wrapper_ty::Invalid
                                        }
                                    };
                                    Ok(value)
                                }
                            });

                        // `"self::fn_name"`
                        let new_path = syn::LitStr::new(
                            &format!("{}::{}", companion.ident, fn_name),
                            field.span(),
                        );
                        syn::parse_quote!(#[serde(deserialize_with = #new_path)])
                    } else if has_default {
                        syn::parse_quote_spanned!(span=> #[serde(deserialize_with = "::eserde::_macro_impl::maybe_invalid")])
                    } else {
                        syn::parse_quote_spanned!(span=> #[serde(deserialize_with = "::eserde::_macro_impl::maybe_invalid_or_missing")])
                    };
                    field.attrs.push(attr);
                }

                // Done last for ownership.
                field.ty = wrapper_ty;
            }
        };

        match &mut companion.data {
            syn::Data::Struct(data_struct) => {
                (modify_field_types)(&mut data_struct.fields);
            }
            syn::Data::Enum(data_enum) => {
                data_enum
                    .variants
                    .iter_mut()
                    .for_each(|variant| (modify_field_types)(&mut variant.fields));
            }
            syn::Data::Union(_) => unreachable!(),
        };

        let bounds: Vec<String> = companion
            .generics
            .type_params()
            // `serde` will infer that the type parameters of the companion type must implement
            // the `Default` trait, on top of the `Deserialize` trait, since we marked fields
            // that use those type parameters with `#[serde(default)]`.
            // That's unnecessary, so we override the bounds here using `#[serde(bound(deserialize = "..."))]`.
            .map(|param| {
                if eserde_aware_generics.contains(&param.ident) {
                    format!("{}: ::eserde::EDeserialize<'de>", param.ident)
                } else {
                    format!("{}: ::eserde::_serde::Deserialize<'de>", param.ident)
                }
            })
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

        // Impl block, if needed.
        let impl_ = if impl_items.is_empty() {
            None
        } else {
            let name = &companion.ident;
            let (impl_generics, ty_generics, where_clause) = companion.generics.split_for_impl();
            Some(syn::parse_quote! {
                impl #impl_generics #name #ty_generics #where_clause {
                    #(#impl_items)*
                }
            })
        };

        Self {
            ty_: companion,
            eserde_aware_generics,
            impl_,
        }
    }
}

fn collect_generic_type_params(
    ty_: &syn::Type,
    set: &mut IndexSet<syn::Ident>,
    generic_params: &HashSet<syn::Ident>,
) {
    match ty_ {
        syn::Type::Path(path) => {
            // Generic type parameters are represented as single-segment paths.
            if let Some(ident) = path.path.get_ident() {
                if generic_params.contains(ident) {
                    set.insert(ident.clone());
                }
            } else {
                for seg in &path.path.segments {
                    if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                        for arg in &args.args {
                            if let syn::GenericArgument::Type(ty) = arg {
                                collect_generic_type_params(ty, set, generic_params);
                            }
                        }
                    }
                }
            }
        }
        syn::Type::Reference(ref_) => {
            collect_generic_type_params(&ref_.elem, set, generic_params);
        }
        syn::Type::Slice(slice) => {
            collect_generic_type_params(&slice.elem, set, generic_params);
        }
        syn::Type::Array(type_array) => {
            collect_generic_type_params(&type_array.elem, set, generic_params);
        }
        syn::Type::TraitObject(_) => {}
        syn::Type::Tuple(type_tuple) => {
            for elem in &type_tuple.elems {
                collect_generic_type_params(elem, set, generic_params);
            }
        }
        t => {
            unimplemented!("{:?}", t);
        }
    }
}
