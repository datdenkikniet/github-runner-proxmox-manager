use std::{collections::BTreeSet, sync::Arc};

use parking_lot::Mutex;
use proc_macro2::{Literal, TokenStream};

use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Ident};

#[derive(Clone, Debug)]
pub struct EnumDef {
    name: Arc<Mutex<String>>,
    derives: Vec<String>,
    values: BTreeSet<String>,
    default: Option<String>,
    doc: Vec<String>,
}

impl PartialEq for EnumDef {
    fn eq(&self, other: &Self) -> bool {
        let equal = *self.name.lock() == *other.name.lock()
            && self.derives == other.derives
            && self.values == other.values
            && self.default == other.default;

        if equal && other.doc != self.doc {
            eprintln!("Found enums that were equal besides on docs.");
        }

        equal
    }
}

impl EnumDef {
    pub fn has_default(&self) -> bool {
        self.default.is_some()
    }

    pub fn name(&self) -> String {
        self.name.lock().to_string()
    }

    pub fn set_name(&mut self, name: &str) {
        let mut my_name = self.name.lock();

        if *my_name != name {
            *my_name = name.to_string();
        }
    }

    pub fn new<I: AsRef<str>, T: IntoIterator<Item = I>>(
        name: String,
        extra_derives: T,
        values: BTreeSet<String>,
        default: Option<String>,
        doc: Vec<String>,
    ) -> Self {
        if let Some(default) = default.as_ref() {
            assert!(values.contains(default));
        }

        let me = EnumDef {
            name: Arc::new(Mutex::new(name)),
            derives: super::TypeDef::DEFAULT_DERIVES
                .into_iter()
                .map(str::to_string)
                .chain(Some("PartialEq".to_string()))
                .chain(extra_derives.into_iter().map(|e| e.as_ref().to_string()))
                .map(|v| v.to_string())
                .collect(),
            values,
            default,
            doc,
        };

        me
    }

    fn to_variant(name: &String) -> String {
        let name = if name.chars().next().unwrap().is_numeric() {
            format!("_{name}")
        } else {
            name.to_string()
        };

        crate::name_to_ident(&name)
    }
}

impl ToTokens for EnumDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name: _,
            derives,
            values,
            default,
            doc,
        } = self;

        let name = Ident::new(&self.name(), quote!().span());

        let enum_doc = doc.iter().map(|doc| {
            let doc = Literal::string(doc);
            quote!(#[doc = #doc])
        });

        let derives = derives.iter().map(|v| {
            let parsed: TokenStream = v.parse().unwrap();
            quote! { #parsed, }
        });

        let variants = values.iter().map(|orig| {
            let v = Self::to_variant(orig);

            let rename = if orig != &v {
                let orig = Literal::string(orig);
                Some(quote!(#[serde(rename = #orig)]))
            } else {
                None
            };

            let ident = Ident::new(&v, quote!().span());

            quote! {
                #rename
                #ident,
            }
        });

        tokens.extend(quote! {
            #[derive(#(#derives)*)]
            #(#enum_doc)*
            pub enum #name {
                #(#variants)*
            }
        });

        let variants = values
            .iter()
            .map(Self::to_variant)
            .map(|v| Ident::new(&v, quote!().span()));
        let values_iter = values.iter().map(|v| Literal::string(v));
        tokens.extend(quote! {
            impl TryFrom<&str> for #name {
                type Error = String;

                fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
                    match value {
                        #(#values_iter => Ok(Self::#variants),)*
                        v => Err(format!("Unknown variant {v}"))
                    }
                }
            }
        });

        if let Some(default) = default {
            let default = Self::to_variant(default);
            let default_ident = Ident::new(&default, quote!().span());

            tokens.extend(quote! {
                impl Default for #name {
                    fn default() -> Self {
                        Self::#default_ident
                    }
                }
            })
        }
    }
}
