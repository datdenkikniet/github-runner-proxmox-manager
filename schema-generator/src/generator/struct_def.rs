use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{spanned::Spanned, Ident};

use super::{EnumDef, FieldDef, TypeDef};

use quote::quote;

#[derive(Clone, Debug)]
pub enum AdditionalProperties {
    None,
    Untyped,
    Type(Box<TypeDef>),
}

impl AdditionalProperties {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

#[derive(Clone, Debug)]
pub struct StructDef {
    name: String,
    fields: Vec<FieldDef>,
    external_defs: Vec<TypeDef>,
    additional_props: AdditionalProperties,
}

impl StructDef {
    pub fn new(
        name: String,
        fields: Vec<FieldDef>,
        additional_props: AdditionalProperties,
        external_defs: Vec<TypeDef>,
    ) -> Self {
        Self {
            name,
            fields,
            external_defs,
            additional_props,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fields(&self) -> &[FieldDef] {
        &self.fields
    }

    pub fn external_defs(&self) -> &[TypeDef] {
        &self.external_defs
    }

    pub fn hoist_enum_defs(&mut self, output: &mut BTreeMap<String, EnumDef>) {
        self.external_defs
            .iter_mut()
            .for_each(|v| v.hoist_enum_defs(output));

        self.external_defs.retain(|v| !matches!(v, TypeDef::Unit));

        if let AdditionalProperties::Type(ty) = &mut self.additional_props {
            ty.hoist_enum_defs(output);
        }
    }
}

impl ToTokens for StructDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            name,
            fields,
            external_defs,
            additional_props,
        } = self;

        let name = Ident::new(name, quote!().span());

        external_defs.iter().for_each(|def| def.to_tokens(tokens));

        let additional_props_ty = match additional_props {
            AdditionalProperties::None => None,
            AdditionalProperties::Untyped => {
                Some(quote!(::std::collections::HashMap<String, ::serde_json::Value>))
            }
            AdditionalProperties::Type(ty) => {
                ty.to_tokens(tokens);

                let ty = ty.as_field_ty(false);
                Some(quote! (::std::collections::HashMap<String, #ty>))
            }
        };

        let default_derive = if fields.iter().all(|f| f.optional()) {
            Some("Default")
        } else {
            let optional_fields = fields.iter().filter(|f| f.optional());
            let non_optional_fields = fields.iter().filter(|f| !f.optional());

            let default_fields = optional_fields.map(|f| {
                let name = f.name();
                let name = Ident::new(name, quote!().span());
                quote!(#name: Default::default())
            });

            let args = non_optional_fields.clone().map(|f| {
                let name = f.name();
                let name = Ident::new(name, quote!().span());
                let ty = f.ty();
                quote!(#name: #ty)
            });

            let arg_setters = non_optional_fields.map(|f| {
                let name = f.name();
                let name = Ident::new(name, quote!().span());
                quote!(#name)
            });

            let additional_props = additional_props_ty
                .as_ref()
                .map(|_| quote!(additional_properties: Default::default(),))
                .into_iter();

            tokens.extend(quote! {
                impl #name {
                    pub fn new(#(#args),*) -> Self {
                        Self {
                            #(#arg_setters,)*
                            #(#default_fields,)*
                            #(#additional_props),*
                        }
                    }
                }
            });

            None
        };

        let derives = TypeDef::DEFAULT_DERIVES
            .iter()
            .chain(default_derive.iter())
            .map(|v| {
                let parsed: TokenStream = v.parse().unwrap();
                quote! { #parsed, }
            });

        let name = &name;

        let additional_props = additional_props_ty.as_ref().map(|v| {
            quote! {
                #[serde(flatten, default, skip_serializing_if = "::std::collections::HashMap::is_empty")]
                pub additional_properties: #v,
            }
        });

        tokens.extend(quote! {
            #[derive(#(#derives)*)]
            pub struct #name {
                #(#fields)*
                #additional_props
            }
        });
    }
}
