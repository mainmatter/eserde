use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

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
        input.data,
        &format_ident!("Self"),
        &shadow_binding,
        shadow_struct_ident,
    );
    let expanded = quote! {
        const _: () = {
            #[derive(::eserde::_serde::Deserialize)]
            #[serde(crate = "eserde::_serde")]
            #shadow_type

            #[automatically_derived]
            impl<'de> ::eserde::_serde::Deserialize<'de> for #name {
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

/// Initialize the target type from the shadow type, assigning each field from the shadow type to the
/// corresponding field on the target type.
fn initialize_from_shadow(
    input: Data,
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
