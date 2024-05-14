use std::{ops::Deref, sync::Arc};

use parking_lot::Mutex;
use proc_macro2::{Punct, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::{spanned::Spanned, Ident};

use super::{proxmox_api, proxmox_api_str, FieldDef, TypeDef};

use quote::quote;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
enum Kind {
    Normal {
        fields: Vec<FieldDef>,
        additional_props: AdditionalProperties,
    },
    FormattedString(Vec<FieldDef>),
}

#[derive(Clone, Debug)]
pub struct StructDef {
    name: Arc<Mutex<String>>,
    kind: Kind,
}

impl PartialEq for StructDef {
    fn eq(&self, other: &Self) -> bool {
        self.name.lock().deref() == other.name.lock().deref() && self.kind == other.kind
    }
}

impl StructDef {
    pub fn new(
        name: String,
        fields: Vec<FieldDef>,
        additional_props: AdditionalProperties,
    ) -> Self {
        Self {
            name: Arc::new(Mutex::new(name)),
            kind: Kind::Normal {
                fields,
                additional_props,
            },
        }
    }

    pub fn new_formatted_string(name: String, fields: Vec<FieldDef>) -> Self {
        Self {
            name: Arc::new(Mutex::new(name)),
            kind: Kind::FormattedString(fields),
        }
    }

    pub fn name(&self) -> String {
        self.name.lock().to_string()
    }

    #[allow(unused)]
    fn formatted_string_to_tokens(name: Ident, fields: &[FieldDef], tokens: &mut TokenStream) {}

    fn normal_to_tokens(
        name: Ident,
        fields: &[FieldDef],
        additional_props: &AdditionalProperties,
        tokens: &mut TokenStream,
    ) {
        let additional_props_ty = match additional_props {
            AdditionalProperties::None => None,
            AdditionalProperties::Untyped => {
                Some(quote!(::std::collections::HashMap<String, ::serde_json::Value>))
            }
            AdditionalProperties::Type(ty) => {
                ty.to_tokens(tokens);

                let (_, ty) = ty.as_field_ty(false);
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
                let name = Ident::new(&name, quote!().span());
                quote!(#name: Default::default())
            });

            let args = non_optional_fields.clone().map(|f| {
                let name = f.name();
                let name = Ident::new(&name, quote!().span());
                let ty = f.ty();
                quote!(#name: #ty)
            });

            let arg_setters = non_optional_fields.map(|f| {
                let name = f.name();
                let name = Ident::new(&name, quote!().span());
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

        let (additional_props, test) = if let Some(additional_props) = additional_props_ty {
            let multis: Vec<_> = fields
                .iter()
                .filter_map(|f| f.numbered_items())
                .map(|m| m.name())
                .collect();

            let (serde_attr, test) = if multis.is_empty() {
                let test = None;
                let serde = quote! {
                    #[serde(flatten, default, skip_serializing_if = "::std::collections::HashMap::is_empty")]
                };
                (serde, test)
            } else {
                let idents = multis.into_iter().map(|v| Ident::new(&v, quote!().span()));
                let test = proxmox_api(quote!(types::multi::Test));
                let numbered_items = proxmox_api(quote!(types::multi::NumberedItems));
                let test = Some(quote! {
                    impl #test for #name {
                        fn test_fn() -> fn(&str) -> bool {
                            fn the_test(input: &str) -> bool {
                                let array = [
                                    #(<#idents as #numbered_items>::key_matches as fn(&str) -> bool,)*
                                ];

                                array.iter().any(|f| f(input))
                            }

                            the_test as _
                        }
                    }
                });

                let path = proxmox_api_str(format!(
                    "types::multi::deserialize_additional_data::<'_, {name}, _, _>"
                ));
                let serde = quote! {
                    #[serde(flatten, deserialize_with = #path)]
                };
                (serde, test)
            };

            let additional_props = quote! {
                #serde_attr
                pub additional_properties: #additional_props,
            };

            (Some(additional_props), test)
        } else {
            (None, None)
        };

        tokens.extend(quote! {
            #[derive(#(#derives)*)]
            pub struct #name
        });

        tokens.append(Punct::new('{', proc_macro2::Spacing::Alone));

        fields.iter().for_each(|f| {
            f.to_tokens(tokens, true);
        });

        tokens.extend(additional_props);

        tokens.append(Punct::new('}', proc_macro2::Spacing::Alone));

        tokens.extend(test);
    }
}

impl ToTokens for StructDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { name: _, kind } = self;
        let name = Ident::new(&self.name(), quote!().span());

        match &kind {
            Kind::Normal {
                fields,
                additional_props,
            } => Self::normal_to_tokens(name, fields, additional_props, tokens),
            Kind::FormattedString(fields) => Self::formatted_string_to_tokens(name, fields, tokens),
        };
    }
}
