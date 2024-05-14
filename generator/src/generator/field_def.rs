use std::ops::Deref;

use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::Ident;

use super::proxmox_api_str;
use super::type_def::PrimitiveTypeDef;
use super::NumItemsDef;
use super::TypeDef;

#[derive(Debug, Clone, PartialEq)]
pub struct FieldDef {
    rename: Option<String>,
    name: String,
    optional: bool,
    doc: Vec<String>,
    ty: Box<TypeDef>,
}

impl FieldDef {
    pub fn new<S, D>(name: String, ty: TypeDef, optional: bool, doc: D) -> Self
    where
        D: Iterator<Item = S>,
        S: Into<String>,
    {
        let multi = name.ends_with("[n]");
        let fixed_name = if multi {
            crate::name_to_underscore_name(&name[..name.len() - 3])
        } else {
            crate::name_to_underscore_name(&name)
        };

        let rename = if fixed_name != name {
            Some(name.to_string())
        } else {
            None
        };

        let ty = if multi {
            let def = NumItemsDef::new(&fixed_name, ty);
            TypeDef::NumberedItems(Box::new(def))
        } else {
            ty
        };

        Self {
            name: fixed_name,
            rename,
            ty: Box::new(ty),
            optional,
            doc: doc.map(Into::into).collect(),
        }
    }

    pub fn optional(&self) -> bool {
        self.optional
            || matches!(
                self.ty.deref(),
                TypeDef::Enum(en) if en.has_default()
            )
    }

    pub fn numbered_items(&self) -> Option<&NumItemsDef> {
        self.ty.numbered_items()
    }

    pub fn ty(&self) -> TokenStream {
        let (_, ty) = self.ty.as_field_ty(self.optional);
        ty
    }

    pub fn name(&self) -> String {
        if self.numbered_items().is_some() {
            format!("{}s", self.name)
        } else {
            self.name.clone()
        }
    }

    pub fn to_tokens(&self, tokens: &mut TokenStream, serde_attrs: bool) {
        let FieldDef {
            rename,
            name: _,
            ty,
            optional,
            doc,
        } = self;

        let name = self.name();

        let name = syn::parse_str::<Ident>(&name)
            .unwrap_or_else(|_| Ident::new_raw(&name, quote!().span()));

        let rename = if let Some(rename) = rename {
            let renamed = Literal::string(rename);
            Some(quote!(#[serde(rename = #renamed)]))
        } else {
            None
        };

        let ser_des = |name: &str, optional: bool| {
            let name = if optional {
                format!("{name}_optional")
            } else {
                name.to_string()
            };

            let ser_fn = proxmox_api_str(format!("types::serialize_{name}"));
            let des_fn = proxmox_api_str(format!("types::deserialize_{name}"));
            Some(quote! { #[serde(serialize_with = #ser_fn, deserialize_with = #des_fn )] })
        };

        let flatten = self.numbered_items().map(|_| quote!(#[serde(flatten)]));

        let serialize = if let Some(num_items) = self.numbered_items() {
            let numbered_name = num_items.name();
            ser_des(&format!("multi::<{}, _>", numbered_name), false)
        } else if let Some(primitive) = ty.primitive() {
            let ser_des = |name: &str| ser_des(name, *optional);

            match primitive {
                PrimitiveTypeDef::String => None,
                PrimitiveTypeDef::Number => ser_des("number"),
                PrimitiveTypeDef::Integer => ser_des("int"),
                PrimitiveTypeDef::Boolean => ser_des("bool"),
            }
        } else {
            None
        };

        let (empty_check, ty) = ty.as_field_ty(self.optional);
        let empty_check = empty_check.map(Literal::string);

        let doc = doc.iter().flat_map(super::clean_doc).map(|v| {
            let doc_literal = Literal::string(&v);
            quote! {
                #[doc = #doc_literal]
            }
        });

        let rename = serde_attrs.then_some(rename).flatten();
        let serialize = serde_attrs.then_some(serialize).flatten();
        let empty_check = serde_attrs.then_some(empty_check).flatten().into_iter();
        let flatten = serde_attrs.then_some(flatten).flatten();

        tokens.extend(quote! {
            #rename
            #serialize
            #(#[serde(skip_serializing_if = #empty_check, default)])*
            #flatten
            #(#doc)*
            pub #name: #ty,
        })
    }
}
