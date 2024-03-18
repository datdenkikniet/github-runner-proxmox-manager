use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TreeNode<'a> {
    #[serde(borrow, flatten)]
    pub value: Value<'a>,
    #[serde(borrow, default)]
    pub children: Vec<TreeNode<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Value<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leaf: Option<u32>,
    #[serde(borrow)]
    pub path: Cow<'a, str>,
    #[serde(borrow)]
    pub text: Cow<'a, str>,
    #[serde(borrow, default)]
    pub info: BTreeMap<Cow<'a, str>, Info<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Info<'a> {
    #[serde(
        rename = "allowtoken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_token: Option<u32>,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub method: Cow<'a, str>,
    #[serde(borrow, default, skip_serializing_if = "Parameters::is_empty")]
    pub parameters: Parameters<'a>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permission>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub returns: Option<Returns<'a>>,
    pub protected: Option<u32>,
    #[serde(
        rename = "proxyto",
        borrow,
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub proxy_to: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Parameters<'a> {
    #[serde(rename = "additionalProperties", default)]
    pub additional_properties: Option<u32>,
    #[serde(borrow, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<Cow<'a, str>, Property<'a>>,
    #[serde(rename = "type", borrow, default)]
    pub ty: Cow<'a, str>,
}

impl<'a> Parameters<'a> {
    pub fn is_empty(&self) -> bool {
        self.ty.is_empty() && self.additional_properties.is_none() && self.properties.is_empty()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Property<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Cow<'a, str>>,
    // Basically always u32, but not aaalways :/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    // Basically always u32, but not aaalways :/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<serde_json::Value>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(rename = "type", borrow)]
    pub ty: Option<Cow<'a, str>>,
    #[serde(rename = "typetext", borrow, default)]
    pub ty_text: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub format: Option<Format<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub format_description: Option<Cow<'a, str>>,
    pub pattern: Option<String>,
    #[serde(rename = "maxLength", default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    #[serde(rename = "minLength", default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,
    // Basically always u64, but not aalway :/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<u32>,
    #[serde(
        borrow,
        rename = "enum",
        default,
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub enum_values: HashSet<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Items<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub requires: Option<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub verbose_description: Option<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub renderer: Option<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<Cow<'a, str>, Property<'a>>,
    #[serde(
        alias = "additionalProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_properties: Option<ParametersOrU32<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum ParametersOrU32<'a> {
    U32(u32),
    #[serde(borrow)]
    Parameters(Parameters<'a>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Format<'a> {
    #[serde(borrow)]
    StringDescription(Cow<'a, str>),
    #[serde(borrow)]
    Properties(BTreeMap<Cow<'a, str>, FormatProperty<'a>>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FormatProperty<'a> {
    // This is practically always `minimum`, but only in the next-id case is it `min`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<u32>,
    // This is practically always `maximum`, but only in the next-id case is it `max`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<u32>,
    // Practically always a u32, but sometimes in string form, because
    // god knows why
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<u32>,
    // Practically always String, but not always, of course...
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<u32>,
    #[serde(rename = "type", borrow)]
    pub ty: Option<Cow<'a, str>>,
    #[serde(rename = "typetext", default, skip_serializing_if = "Option::is_none")]
    pub ty_text: Option<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub format: Option<Box<Format<'a>>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub format_description: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_key: Option<u32>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<Cow<'a, str>>,
    #[serde(
        borrow,
        rename = "enum",
        default,
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub enum_values: HashSet<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub verbose_description: Option<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<Cow<'a, str>>,
    #[serde(
        alias = "keyAlias",
        borrow,
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key_alias: Option<Cow<'a, str>>,
    #[serde(alias = "maxLength", default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    #[serde(alias = "minLength", default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    // TODO: parse permissions...
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Returns<'a> {
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    // If this is not set, validate that the object is empty in its entirety!
    #[serde(borrow, rename = "type")]
    pub ty: Option<Cow<'a, str>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub items: Option<Items<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<u32>,
    #[serde(borrow, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<Cow<'a, str>, Property<'a>>,
    #[serde(rename = "additionalProperties", default)]
    pub additional_properties: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<Format<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Items<'a> {
    #[serde(rename = "type", borrow)]
    pub ty: Cow<'a, str>,
    #[serde(
        rename = "additionalProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_properties: Option<u32>,
    #[serde(borrow, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<Cow<'a, str>, Property<'a>>,
    // Wat, nexted items?
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Items<'a>>>,
    #[serde(
        borrow,
        rename = "enum",
        default,
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub enum_values: HashSet<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<Format<'a>>,
    #[serde(alias = "minLength", default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,
    #[serde(alias = "maxLength", default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Link<'a> {
    pub href: Cow<'a, str>,
    pub rel: Cow<'a, str>,
}
