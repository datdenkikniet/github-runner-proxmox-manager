pub mod name;
pub struct MatchersClient<T> {
    client: T,
    path: String,
}
impl<T> MatchersClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/matchers"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Mode {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "any")]
    Any,
}
impl Default for Mode {
    fn default() -> Self {
        Self::All
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Origin {
    #[serde(rename = "builtin")]
    Builtin,
    #[serde(rename = "modified-builtin")]
    ModifiedBuiltin,
    #[serde(rename = "user-created")]
    UserCreated,
}
impl GetOutputItems {
    pub fn new(name: String, origin: Origin) -> Self {
        Self {
            name,
            origin,
            comment: Default::default(),
            disable: Default::default(),
            invert_match: Default::default(),
            match_calendar: Default::default(),
            match_field: Default::default(),
            match_severity: Default::default(),
            mode: Default::default(),
            target: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment"]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable this matcher"]
    pub disable: Option<bool>,
    #[serde(rename = "invert-match")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Invert match of the whole matcher"]
    pub invert_match: Option<bool>,
    #[serde(rename = "match-calendar")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Match notification timestamp"]
    pub match_calendar: Vec<String>,
    #[serde(rename = "match-field")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Metadata fields to match (regex or exact match). Must be in the form (regex|exact):\\<field\\>=\\<value\\>"]
    pub match_field: Vec<String>,
    #[serde(rename = "match-severity")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Notification severities to match"]
    pub match_severity: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Choose between 'all' and 'any' for when multiple properties are specified"]
    pub mode: Option<Mode>,
    #[doc = "Name of the matcher."]
    pub name: String,
    #[doc = "Show if this entry was created by a user or was built-in"]
    pub origin: Origin,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Targets to notify on match"]
    pub target: Vec<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> MatchersClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Returns a list of all matchers"]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl PostParams {
    pub fn new(name: String) -> Self {
        Self {
            name,
            comment: Default::default(),
            disable: Default::default(),
            invert_match: Default::default(),
            match_calendar: Default::default(),
            match_field: Default::default(),
            match_severity: Default::default(),
            mode: Default::default(),
            target: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment"]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable this matcher"]
    pub disable: Option<bool>,
    #[serde(rename = "invert-match")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Invert match of the whole matcher"]
    pub invert_match: Option<bool>,
    #[serde(rename = "match-calendar")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Match notification timestamp"]
    pub match_calendar: Vec<String>,
    #[serde(rename = "match-field")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Metadata fields to match (regex or exact match). Must be in the form (regex|exact):\\<field\\>=\\<value\\>"]
    pub match_field: Vec<String>,
    #[serde(rename = "match-severity")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Notification severities to match"]
    pub match_severity: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Choose between 'all' and 'any' for when multiple properties are specified"]
    pub mode: Option<Mode>,
    #[doc = "Name of the matcher."]
    pub name: String,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Targets to notify on match"]
    pub target: Vec<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> MatchersClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new matcher"]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> MatchersClient<T>
where
    T: crate::client::Client,
{
    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
