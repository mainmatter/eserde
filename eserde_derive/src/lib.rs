use emit::{collect_missing_errors, initialize_from_shadow, ImplDeserGenerics};
use indexmap::IndexSet;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};
use unsupported::reject_unsupported_inputs;

mod emit;
mod filter_attributes;
mod model;
mod unsupported;

#[proc_macro_derive(Deserialize, attributes(serde, eserde))]
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

    let companion_type =
        model::PermissiveCompanionType::new(format_ident!("__ImplEDeserializeFor{}", name), &input);
    let companion_type_ident = &companion_type.ty_.ident;
    let companion_binding = format_ident!("__companion");
    let deserializer_generic_ident = format_ident!("__D");
    let n_errors = format_ident!("__n_errors");
    let collect_missing_errors = collect_missing_errors(
        &input.data,
        &companion_type_ident,
        &companion_binding,
        &n_errors,
    );

    let deser_generics = ImplDeserGenerics::new(&input, &IndexSet::new());
    let (impl_generics, ty_generics, where_clause) = deser_generics.split_for_impl();

    let human_deser_generics =
        ImplDeserGenerics::new(&input, &companion_type.eserde_aware_generics);
    let (human_impl_generics, human_ty_generics, human_where_clause) =
        human_deser_generics.split_for_impl();

    let expanded = quote! {
        const _: () = {
            #companion_type

            #shadow_type

            #[automatically_derived]
            impl #human_impl_generics ::eserde::EDeserialize<'de> for #name #human_ty_generics
            #human_where_clause
            {
                fn deserialize_for_errors<#deserializer_generic_ident>(__deserializer: #deserializer_generic_ident) -> Result<(), ()>
                where
                    #deserializer_generic_ident: ::eserde::_serde::Deserializer<'de>,
                {
                    let #n_errors = ::eserde::reporter::ErrorReporter::n_errors();
                    let #companion_binding = <#companion_type_ident #ty_generics as ::eserde::_serde::Deserialize>::deserialize(__deserializer)
                        .map_err(|e| {
                            ::eserde::reporter::ErrorReporter::report(::eserde::DeserializationErrorDetails::Custom { message: e.to_string() });
                        })?;
                    #collect_missing_errors
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
