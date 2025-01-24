use quote::ToTokens;
use syn::DeriveInput;

/// A type with exactly the same set of fields/variants as the original type, but with a different name.
/// This type is used to derive `Deserialize`, thus obtaining from `serde` the same deserialize implementation
/// we would get for the original type had we annotated it with `#[derive(Deserialize)]` directly.
pub struct ShadowType(pub DeriveInput);

impl ShadowType {
    pub fn new(ident: syn::Ident, input: &syn::DeriveInput) -> Self {
        let data = match &input.data {
            syn::Data::Struct(data) => {
                let fields = match &data.fields {
                    syn::Fields::Named(fields) => {
                        let named = fields
                            .named
                            .iter()
                            .map(|field| {
                                let attrs = Self::extract_serde_attributes(&field.attrs);
                                syn::Field {
                                    attrs,
                                    ..field.clone()
                                }
                            })
                            .collect();
                        syn::Fields::Named(syn::FieldsNamed {
                            named,
                            brace_token: fields.brace_token,
                        })
                    }
                    syn::Fields::Unnamed(fields) => {
                        let unnamed = fields
                            .unnamed
                            .iter()
                            .map(|field| {
                                let attrs = Self::extract_serde_attributes(&field.attrs);
                                syn::Field {
                                    attrs,
                                    ..field.clone()
                                }
                            })
                            .collect();
                        syn::Fields::Unnamed(syn::FieldsUnnamed {
                            unnamed,
                            paren_token: fields.paren_token,
                        })
                    }
                    syn::Fields::Unit => syn::Fields::Unit,
                };
                let data_struct = syn::DataStruct {
                    fields,
                    struct_token: data.struct_token,
                    semi_token: data.semi_token,
                };
                syn::Data::Struct(data_struct)
            }
            syn::Data::Enum(_) => unimplemented!(),
            syn::Data::Union(_) => unimplemented!(),
        };
        let shadow = DeriveInput {
            attrs: Self::extract_serde_attributes(&input.attrs),
            vis: syn::Visibility::Inherited,
            ident,
            generics: input.generics.clone(),
            data,
        };
        Self(shadow)
    }

    /// We don't want to keep _all_ attributes for the shadow type, only the `serde` ones
    /// (e.g. `#[serde(default)]`), so we filter out the others.
    fn extract_serde_attributes(attributes: &[syn::Attribute]) -> Vec<syn::Attribute> {
        attributes
            .iter()
            .filter(|attr| !attr.meta.path().is_ident("serde"))
            .cloned()
            .collect()
    }
}

impl ToTokens for ShadowType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self(input) = self;
        input.to_tokens(tokens);
    }
}
