use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::generator::{FieldDef, PrimitiveTypeDef, TypeDef};

use super::{Format, Optional};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Type<'a> {
    #[serde(flatten, borrow, default, skip_serializing_if = "Option::is_none")]
    pub ty: Option<TypeKind<'a>>,

    #[serde(
        rename = "typetext",
        borrow,
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_text: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Optional::is_empty")]
    pub optional: Optional,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verbose_description: Option<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub format: Option<Box<Format<'a>>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub format_description: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_key: Option<u32>,
    // Another field in the parent object is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renderer: Option<Cow<'a, str>>,
    // This type is an alias for a field in the parent object
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<Cow<'a, str>>,
    // This type is an alias for a field in the parent object.
    #[serde(
        alias = "keyAlias",
        borrow,
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key_alias: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum IntOrTy<'a> {
    Int(u32),
    #[serde(borrow)]
    Ty(Type<'a>),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum TypeKind<'a> {
    Null,
    String {
        #[serde(rename = "maxLength", default, skip_serializing_if = "Option::is_none")]
        max_length: Option<u32>,
        #[serde(rename = "minLength", default, skip_serializing_if = "Option::is_none")]
        min_length: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pattern: Option<Cow<'a, str>>,

        // If it's an enum
        #[serde(rename = "enum", default, skip_serializing_if = "Option::is_none")]
        enum_values: Option<Vec<Cow<'a, str>>>,
        #[serde(default)]
        default: Option<Cow<'a, str>>,
    },
    Number {
        #[serde(default, alias = "min", skip_serializing_if = "Option::is_none")]
        minimum: Option<serde_json::Value>,
        #[serde(default, alias = "max", skip_serializing_if = "Option::is_none")]
        maximum: Option<f32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default: Option<f32>,
    },
    Integer {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        minimum: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        maximum: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default: Option<u32>,
    },
    Boolean {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default: Option<u32>,
    },
    Array {
        items: Box<Type<'a>>,
    },
    Object {
        #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
        properties: Option<HashMap<Cow<'a, str>, Type<'a>>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "additionalProperties"
        )]
        additional_properties: Option<Box<IntOrTy<'a>>>,
    },
}

impl Type<'_> {
    pub fn type_def(&self, field_name: &str, struct_suffix: &str) -> TypeDef {
        if let Some(ty) = self.ty.as_ref() {
            match ty {
                TypeKind::Null => TypeDef::Unit,
                TypeKind::String {
                    enum_values,
                    default,
                    ..
                } => {
                    if let Some(enum_values) = enum_values {
                        let default = if enum_values.iter().any(|v| Some(v) == default.as_ref()) {
                            default.as_ref().map(Cow::to_string)
                        } else {
                            None
                        };

                        let enum_values = enum_values
                            .iter()
                            .map(Cow::to_string)
                            .chain(default.clone())
                            .collect();
                        let no_derives = Option::<&str>::None;

                        let name = crate::name_to_ident(field_name);
                        TypeDef::new_enum(name, no_derives, enum_values, default)
                    } else {
                        TypeDef::Primitive(PrimitiveTypeDef::String)
                    }
                }
                TypeKind::Number { .. } => TypeDef::Primitive(PrimitiveTypeDef::Number),
                TypeKind::Integer { .. } => TypeDef::Primitive(PrimitiveTypeDef::Integer),
                TypeKind::Boolean { .. } => TypeDef::Primitive(PrimitiveTypeDef::Boolean),
                TypeKind::Array { items } => {
                    let inner = items.type_def(field_name, &format!("{struct_suffix}Items"));

                    TypeDef::Array {
                        inner: Box::new(inner),
                    }
                }
                TypeKind::Object {
                    properties,
                    additional_properties,
                } => {
                    if let Some(IntOrTy::Ty(additional_props)) = additional_properties.as_deref() {
                        assert!(properties.is_none(), "Cannot handle combination of typed additional properties & normal properties.");

                        additional_props.type_def(field_name, struct_suffix)
                    } else if let Some(props) = properties {
                        let mut external_defs: Vec<TypeDef> = Vec::new();

                        let fields: Vec<_> = props
                            .iter()
                            .map(|(original_name, ty)| {
                                let field_name = crate::name_to_ident(&original_name);
                                let inner = ty
                                    .type_def(&field_name, &format!("{struct_suffix}{field_name}"));

                                external_defs.push(inner.clone());

                                FieldDef::new(original_name.to_string(), inner, ty.optional.get())
                            })
                            .collect();

                        let field_name = crate::name_to_ident(field_name);
                        let suffix = crate::name_to_ident(struct_suffix);

                        let struct_name = format!("{field_name}{suffix}");
                        TypeDef::new_struct(struct_name, fields, external_defs)
                    } else {
                        TypeDef::Unit
                    }
                }
            }
        } else {
            TypeDef::Unit
        }
    }
}
