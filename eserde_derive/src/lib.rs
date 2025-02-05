use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, GenericParam, Generics, Lifetime};
use unsupported::reject_unsupported_inputs;

mod filter_attributes;
mod model;
mod unsupported;

#[proc_macro_derive(Deserialize, attributes(serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Err(e) = reject_unsupported_inputs(&input) {
        return e.into_compile_error().into();
    }

    let name = &input.ident;
    let shadow_type = model::ShadowType::new(format_ident!("__ImplDeserializeFor{}", name), &input);
    let shadow_type_ident = &shadow_type.0.ident;

    let shadow_binding = format_ident!("__shadowed");
    let initialize_from_shadow = initialize_from_shadow(
        &input.data,
        &format_ident!("Self"),
        &shadow_binding,
        shadow_type_ident,
    );

    let companion_type = model::PermissiveCompanionType::new(
        format_ident!("__ImplHumanDeserializeFor{}", name),
        &input,
    );
    let companion_type_ident = &companion_type.0.ident;
    let companion_binding = format_ident!("__companion");
    let deserializer_generic_ident = format_ident!("__D");
    let initialize_from_companion = initialize_from_companion(
        &input.data,
        &format_ident!("Self"),
        &companion_type_ident,
        &companion_binding,
        &deserializer_generic_ident,
    );

    let deser_generics = ImplDeserGenerics::new(&input);
    let (impl_generics, ty_generics, where_clause) = deser_generics.split_for_impl();

    let expanded = quote! {
        const _: () = {
            #[derive(::eserde::_serde::Deserialize)]
            #[serde(crate = "eserde::_serde")]
            #companion_type

            #[derive(::eserde::_serde::Deserialize)]
            #[serde(crate = "eserde::_serde")]
            #shadow_type

            #[automatically_derived]
            impl #impl_generics ::eserde::HumanDeserialize<'de> for #name #ty_generics
            #where_clause
            {
                fn human_deserialize<#deserializer_generic_ident>(__deserializer: #deserializer_generic_ident) -> Result<Self, ()>
                where
                    #deserializer_generic_ident: ::eserde::_serde::Deserializer<'de>,
                {
                    let #companion_binding = <#companion_type_ident #ty_generics as ::eserde::_serde::Deserialize>::deserialize(__deserializer)
                        .map_err(|e| {
                            ::eserde::DESERIALIZATION_ERRORS.with_borrow_mut(|errors| errors.push(::eserde::DeserializationError::Custom { message: e.to_string() }));
                            ()
                        })?;
                    #initialize_from_companion
                }
            }

            #[automatically_derived]
            impl #impl_generics ::eserde::_serde::Deserialize<'de> for #name #ty_generics
            #where_clause
            {
                fn deserialize<#deserializer_generic_ident>(__deserializer: #deserializer_generic_ident) -> Result<Self, #deserializer_generic_ident::Error>
                where
                    #deserializer_generic_ident: ::eserde::_serde::Deserializer<'de>,
                {
                    let #shadow_binding = #shadow_type_ident::deserialize(__deserializer)?;
                    Ok(#initialize_from_shadow)
                }
            }
        };
    };

    TokenStream::from(expanded)
}

struct ImplDeserGenerics<'a> {
    deser_generics: Generics,
    input_generics: &'a Generics,
}

impl<'a> ImplDeserGenerics<'a> {
    fn new(input: &'a DeriveInput) -> ImplDeserGenerics<'a> {
        let mut deser_generics = input.generics.clone();
        deser_generics.make_where_clause();
        if let Some(where_clause) = &mut deser_generics.where_clause {
            // Each type parameter must implement `Deserialize` for the
            // type to implement `Deserialize`.
            //
            // TODO: Take into account the `#[serde(bound)]` attribute https://serde.rs/container-attrs.html#bound
            for ty_param in input.generics.type_params() {
                where_clause
                    .predicates
                    .push(syn::parse_quote! { #ty_param: ::eserde::_serde::Deserialize<'de> });
            }

            // Each lifetime parameter must be outlived by `'de`, the lifetime of the `Deserialize` trait.
            for lifetime in input.generics.lifetimes() {
                where_clause
                    .predicates
                    .push(syn::parse_quote! { 'de: #lifetime });
            }
        } else {
            unreachable!()
        }

        // The `'de` lifetime of the `Deserialize` trait.
        // There is no way to add a lifetime to the `impl_generics` returned by `split_for_impl`, so we
        // have to create a new set of generics with the lifetime added and then split again.
        let param = GenericParam::Lifetime(syn::LifetimeParam::new(Lifetime::new(
            "'de",
            Span::call_site(),
        )));
        deser_generics.params.push(param);

        Self {
            deser_generics,
            input_generics: &input.generics,
        }
    }

    fn split_for_impl(
        &self,
    ) -> (
        syn::ImplGenerics<'_>,
        syn::TypeGenerics,
        Option<&syn::WhereClause>,
    ) {
        let (impl_generics, _, where_clause) = self.deser_generics.split_for_impl();
        let (_, ty_generics, _) = self.input_generics.split_for_impl();
        (impl_generics, ty_generics, where_clause)
    }
}

/// Initialize the target type from the shadow type, assigning each field from the shadow type to the
/// corresponding field on the target type.
fn initialize_from_shadow(
    input: &Data,
    type_ident: &syn::Ident,
    shadow_binding: &syn::Ident,
    shadow_type_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    match input {
        Data::Struct(data) => {
            let fields = data.fields.members().map(|field| {
                quote! {
                    #field: #shadow_binding.#field
                }
            });
            quote! {
                #type_ident {
                    #(#fields),*
                }
            }
        }
        Data::Enum(e) => {
            let variants = e.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                match &variant.fields {
                    syn::Fields::Named(fields) => {
                        let fields: Vec<_> = fields.named.iter().map(|field| {
                            let field = field.ident.as_ref().unwrap();
                            quote! {
                                #field
                            }
                        }).collect();
                        quote! {
                            #shadow_type_ident::#variant_ident { #(#fields),* } => #type_ident::#variant_ident { #(#fields),* }
                        }
                    }
                    syn::Fields::Unnamed(fields) => {
                        let fields: Vec<_> = fields.unnamed.iter().enumerate().map(|(i, _)| {
                            let i = format_ident!("__v{i}");
                            quote! {
                                #i
                            }
                        }).collect();
                        quote! {
                            #shadow_type_ident::#variant_ident(#(#fields),*) => #type_ident::#variant_ident(#(#fields),*)
                        }
                    }
                    syn::Fields::Unit => {
                        quote! {
                            #shadow_type_ident::#variant_ident => #type_ident::#variant_ident
                        }
                    }
                }
            });
            quote! {
                match #shadow_binding {
                    #(#variants),*
                }
            }
        }
        Data::Union(_) => unimplemented!(),
    }
}

/// Initialize the target type from the companion type, assigning each field from the shadow companion to the
/// corresponding field on the target type in case of success, otherwise accumulating errors.
fn initialize_from_companion(
    input: &Data,
    type_ident: &syn::Ident,
    companion_type: &syn::Ident,
    companion_binding: &syn::Ident,
    deserializer_type: &syn::Ident,
) -> proc_macro2::TokenStream {
    let has_errored = format_ident!("__has_errored");
    match input {
        Data::Struct(data) => {
            let assign = data.fields.members().map(|field| {
                quote! {
                    #field: #companion_binding.#field.value().unwrap()
                }
            });
            let accumulate = data.fields.members().map(|field| {
                let field_str = match &field {
                    syn::Member::Named(ident) => ident.to_string(),
                    // TODO: Improve naming for unnamed fields
                    syn::Member::Unnamed(index) => format!("{}", index.index)
                };
                quote! {
                    if let Some(err) = #companion_binding.#field.error::<#deserializer_type>(#field_str) {
                        ::eserde::DESERIALIZATION_ERRORS.with(|errors| errors.borrow_mut().push(err));
                        #has_errored = true;
                    }
                }
            });
            quote! {
                let mut #has_errored = false;
                #(#accumulate)*
                if #has_errored {
                    return Err(());
                }
                Ok(#type_ident {
                    #(#assign),*
                })
            }
        }
        Data::Enum(e) => {
            let variants = e.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;

                if matches!(variant.fields, syn::Fields::Unit) {
                    return quote! {
                        #companion_type::#variant_ident => #type_ident::#variant_ident
                    };
                }
                let bindings: Vec<_> = variant
                    .fields
                    .members()
                    .enumerate()
                    .map(|(i, _)| format_ident!("__v{}", i))
                    .collect();
                let destructure =
                    variant
                        .fields
                        .members()
                        .zip(bindings.iter())
                        .map(|(field, v)| {
                            quote! {
                                #field: #v
                            }
                        });
                let assign = variant
                    .fields
                    .members()
                    .zip(bindings.iter())
                    .map(|(field, v)| {
                        quote! {
                            #field: #v.value().unwrap()
                        }
                    });
                let accumulate = variant
                    .fields
                    .members()
                    .zip(bindings.iter())
                    .map(|(field, v)| {
                        let field_str = match &field {
                            syn::Member::Named(ident) => ident.to_string(),
                            // TODO: Improve naming for unnamed fields
                            syn::Member::Unnamed(index) => format!("{}", index.index),
                        };
                        quote! {
                            if let Some(err) = #v.error::<#deserializer_type>(#field_str) {
                                ::eserde::DESERIALIZATION_ERRORS.with(|errors| errors.borrow_mut().push(err));
                                #has_errored = true;
                            }
                        }
                    });
                quote! {
                    #companion_type::#variant_ident { #(#destructure),* } => {
                        let mut #has_errored = false;
                        #(#accumulate)*
                        if #has_errored {
                            return Err(());
                        }
                        #type_ident::#variant_ident { #(#assign),* }
                    }
                }
            });
            quote! {
                Ok(match #companion_binding {
                    #(#variants),*
                })
            }
        }
        Data::Union(_) => unreachable!(),
    }
}
