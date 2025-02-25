use std::collections::HashSet;

use crate::filter_attributes::FilterAttributes;
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
    /// The companion type.
    pub ty_: DeriveInput,
    /// Generic type parameters that must be constrained with `::eserde::EDeserialize` instead of `::serde::Deserialize`.
    pub eserde_aware_generics: IndexSet<syn::Ident>,
    /// An impl block; contains methods for `#[serde(deserialize_with)]` attributes.
    pub impl_: Option<syn::ItemImpl>,
}

impl PermissiveCompanionType {
    pub fn new(ident: syn::Ident, input: &syn::DeriveInput) -> Self {
        fn modify_field_types(
            ident: &syn::Ident,
            fields: &mut syn::Fields,
            eserde_aware_generics: &mut IndexSet<syn::Ident>,
            deserialize_withs: &mut Vec<syn::ImplItemFn>,
            generic_params: &HashSet<syn::Ident>,
        ) {
            for (i, field) in fields.iter_mut().enumerate() {
                // Process all `eserde` attributes, then remove them since
                // they are not valid `serde` attributes.
                let is_eserde_compatible = !has_eserde_path_attr(&field.attrs, "compat");
                field.attrs.retain(keep_serde_attributes);

                if is_eserde_compatible {
                    collect_generic_type_params(&field.ty, eserde_aware_generics, generic_params);
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

                // Wrap the field type in `MaybeInvalidOrMissing<T>` or `MaybeInvalid<T>`.
                let wrapper_type = {
                    let ty_ = &field.ty;
                    if has_default {
                        syn::parse_quote! {
                            ::eserde::_macro_impl::MaybeInvalid::<#ty_>
                        }
                    } else {
                        syn::parse_quote! {
                            ::eserde::_macro_impl::MaybeInvalidOrMissing::<#ty_>
                        }
                    }
                };

                // Handle custom `#[serde(deserialize_with)]` attributes.
                // Get the value of `#[serde(deserialize_with)]` if present.
                let deserialize_with_fn_name = {
                    let mut path: Option<syn::ExprPath> = None;
                    // TODO: properly handle removing the old deserialize_with.
                    for idx in 0..field.attrs.len() {
                        let attr = &field.attrs[idx];
                        if attr.path().is_ident("serde") {
                            let mut found = false;
                            let _ = attr.parse_nested_meta(|meta| {
                                if meta.path.is_ident("deserialize_with") {
                                    found = true;
                                    let lit_str: syn::LitStr = meta.value()?.parse()?;
                                    path = syn::parse_str(lit_str.value().as_str()).ok();
                                }
                                Ok(())
                            });
                            if found {
                                field.attrs.remove(idx);
                                break;
                            }
                        }
                    }
                    path.map(| path| {
                        let fn_name = format!("__eserde_deserialize_{}", field.ident.as_ref().map(|ident| ident.to_string()).unwrap_or_else(|| i.to_string()));
                        let fn_ident = syn::Ident::new(&fn_name, field.span());
                        // Add the method to `deserialize_withs`.
                        deserialize_withs.push(syn::parse_quote! {
                            fn #fn_ident<'de, D>(deserializer: D) -> ::core::result::Result<#wrapper_type, D::Error>
                            where
                                D: ::eserde::_serde::Deserializer<'de>,
                            {
                                let v = match #path(deserializer) {
                                    Ok(_) => #wrapper_type::Valid(::core::marker::PhantomData),
                                    Err(e) => {
                                        ::eserde::reporter::ErrorReporter::report(e);
                                        #wrapper_type::Invalid
                                    }
                                };
                                Ok(v)
                            }
                        });
                        fn_name
                    })
                };

                // Set the `#[serde(deserialize_with)]` attribute.
                if let Some(fn_name) = deserialize_with_fn_name {
                    let path_string = format!("{}::{}", ident, fn_name);
                    let path_lit = syn::Lit::Str(syn::LitStr::new(&path_string, field.span()));
                    field
                        .attrs
                        .push(syn::parse_quote!(#[serde(deserialize_with = #path_lit)]));
                } else if is_eserde_compatible {
                    if has_default {
                        field.attrs.push(syn::parse_quote!(#[serde(deserialize_with = "::eserde::_macro_impl::maybe_invalid")]));
                    } else {
                        field.attrs.push(syn::parse_quote!(#[serde(deserialize_with = "::eserde::_macro_impl::maybe_invalid_or_missing")]));
                    }
                }

                // Set the field to the wrapper type.
                field.ty = wrapper_type;
            }
        }

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
        let mut deserialize_withs = Vec::new();
        match &mut companion.data {
            syn::Data::Struct(data_struct) => {
                modify_field_types(
                    &companion.ident,
                    &mut data_struct.fields,
                    &mut eserde_aware_generics,
                    &mut deserialize_withs,
                    &generic_params,
                );
            }
            syn::Data::Enum(data_enum) => {
                data_enum.variants.iter_mut().for_each(|variant| {
                    modify_field_types(
                        &companion.ident,
                        &mut variant.fields,
                        &mut eserde_aware_generics,
                        &mut deserialize_withs,
                        &generic_params,
                    )
                });
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

        let impl_ = if deserialize_withs.is_empty() {
            None
        } else {
            let name = &companion.ident;
            let (impl_generics, ty_generics, where_clause) = companion.generics.split_for_impl();
            Some(syn::parse_quote! {
                impl #impl_generics #name #ty_generics #where_clause {
                    #(#deserialize_withs)*
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

fn has_serde_path_attr(attrs: &[syn::Attribute], path: &str) -> bool {
    attrs.iter().any(|attr| {
        let mut has_attr = false;
        if attr.path().is_ident("serde") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(path) {
                    has_attr = true;
                }
                Ok(())
            });
        }
        has_attr
    })
}

fn has_eserde_path_attr(attrs: &[syn::Attribute], path: &str) -> bool {
    attrs.iter().any(|attr| {
        let mut has_attr = false;
        if attr.path().is_ident("eserde") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(path) {
                    has_attr = true;
                }
                Ok(())
            });
        }
        has_attr
    })
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
