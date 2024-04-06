pub mod name;
pub struct SendmailClient<T> {
    client: T,
    path: String,
}
impl<T> SendmailClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/sendmail"),
        }
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
            author: Default::default(),
            comment: Default::default(),
            disable: Default::default(),
            from_address: Default::default(),
            mailto: Default::default(),
            mailto_user: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Author of the mail"]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment"]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable this target"]
    pub disable: Option<bool>,
    #[serde(rename = "from-address")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "`From` address for the mail"]
    pub from_address: Option<String>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of email recipients"]
    pub mailto: Vec<String>,
    #[serde(rename = "mailto-user")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of users"]
    pub mailto_user: Vec<String>,
    #[doc = "The name of the endpoint."]
    pub name: String,
    #[doc = "Show if this entry was created by a user or was built-in"]
    pub origin: Origin,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> SendmailClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Returns a list of all sendmail endpoints"]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl PostParams {
    pub fn new(name: String) -> Self {
        Self {
            name,
            author: Default::default(),
            comment: Default::default(),
            disable: Default::default(),
            from_address: Default::default(),
            mailto: Default::default(),
            mailto_user: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Author of the mail"]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment"]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable this target"]
    pub disable: Option<bool>,
    #[serde(rename = "from-address")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "`From` address for the mail"]
    pub from_address: Option<String>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of email recipients"]
    pub mailto: Vec<String>,
    #[serde(rename = "mailto-user")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of users"]
    pub mailto_user: Vec<String>,
    #[doc = "The name of the endpoint."]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> SendmailClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new sendmail endpoint"]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> SendmailClient<T>
where
    T: crate::client::Client,
{
    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
