use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, GenericParam, Generics, Lifetime};

mod filter_attributes;
mod model;

#[proc_macro_derive(Deserialize, attributes(serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if matches!(input.data, Data::Union(_)) {
        return syn::Error::new_spanned(input, "Unions are not supported")
            .to_compile_error()
            .into();
    }

    let name = &input.ident;
    let shadow_type = model::ShadowType::new(format_ident!("__ImplDeserializeFor{}", name), &input);
    let shadow_struct_ident = &shadow_type.0.ident;

    let shadow_binding = format_ident!("__shadowed");
    let initialize_from_shadow = initialize_from_shadow(
        &input.data,
        &format_ident!("Self"),
        &shadow_binding,
        shadow_struct_ident,
    );

    let deser_generics = ImplDeserGenerics::new(&input);
    let (impl_generics, ty_generics, where_clause) = deser_generics.split_for_impl();

    let expanded = quote! {
        const _: () = {
            #[derive(::eserde::_serde::Deserialize)]
            #[serde(crate = "eserde::_serde")]
            #shadow_type

            #[automatically_derived]
            impl #impl_generics ::eserde::_serde::Deserialize<'de> for #name #ty_generics #where_clause {
                fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                where
                    __D: ::eserde::_serde::Deserializer<'de>,
                {
                    let #shadow_binding = #shadow_struct_ident::deserialize(__deserializer)?;
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
