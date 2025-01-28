pub(crate) trait FilterAttributes {
    /// Remove all attributes that don't satisfy the predicate.
    fn filter_attributes(&self, filter: impl Fn(&syn::Attribute) -> bool) -> Self;
}

impl FilterAttributes for syn::Data {
    fn filter_attributes(&self, filter: impl Fn(&syn::Attribute) -> bool) -> Self {
        match self {
            syn::Data::Struct(data) => syn::Data::Struct(data.filter_attributes(filter)),
            syn::Data::Enum(data) => syn::Data::Enum(data.filter_attributes(filter)),
            syn::Data::Union(_) => unimplemented!(),
        }
    }
}

impl FilterAttributes for syn::DataStruct {
    fn filter_attributes(&self, filter: impl Fn(&syn::Attribute) -> bool) -> Self {
        Self {
            fields: self.fields.filter_attributes(&filter),
            struct_token: self.struct_token,
            semi_token: self.semi_token,
        }
    }
}

impl FilterAttributes for syn::DataEnum {
    fn filter_attributes(&self, filter: impl Fn(&syn::Attribute) -> bool) -> Self {
        let variants = self
            .variants
            .iter()
            .map(|variant| variant.filter_attributes(&filter))
            .collect();
        Self {
            variants,
            enum_token: self.enum_token,
            brace_token: self.brace_token,
        }
    }
}

impl FilterAttributes for syn::Variant {
    fn filter_attributes(&self, filter: impl Fn(&syn::Attribute) -> bool) -> Self {
        syn::Variant {
            attrs: self.attrs.filter_attributes(&filter),
            fields: self.fields.filter_attributes(&filter),
            ..self.clone()
        }
    }
}

impl FilterAttributes for syn::Fields {
    fn filter_attributes(&self, filter: impl Fn(&syn::Attribute) -> bool) -> Self {
        match &self {
            syn::Fields::Named(fields) => syn::Fields::Named(fields.filter_attributes(filter)),
            syn::Fields::Unnamed(fields) => syn::Fields::Unnamed(fields.filter_attributes(filter)),
            syn::Fields::Unit => syn::Fields::Unit,
        }
    }
}

impl FilterAttributes for syn::FieldsNamed {
    fn filter_attributes(&self, filter: impl Fn(&syn::Attribute) -> bool) -> Self {
        let named = self
            .named
            .iter()
            .map(|field| syn::Field {
                attrs: field.attrs.filter_attributes(&filter),
                ..field.clone()
            })
            .collect();
        Self {
            named,
            brace_token: self.brace_token,
        }
    }
}

impl FilterAttributes for syn::FieldsUnnamed {
    fn filter_attributes(&self, filter: impl Fn(&syn::Attribute) -> bool) -> Self {
        let unnamed = self
            .unnamed
            .iter()
            .map(|field| syn::Field {
                attrs: field.attrs.filter_attributes(&filter),
                ..field.clone()
            })
            .collect();
        Self {
            unnamed,
            paren_token: self.paren_token,
        }
    }
}

impl FilterAttributes for Vec<syn::Attribute> {
    fn filter_attributes(&self, filter: impl Fn(&syn::Attribute) -> bool) -> Self {
        self.iter().filter(|a| filter(a)).cloned().collect()
    }
}
