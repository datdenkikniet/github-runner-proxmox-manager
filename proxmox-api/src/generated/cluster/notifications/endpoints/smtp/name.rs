pub struct NameClient<T> {
    client: T,
    path: String,
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Mode {
    #[serde(rename = "insecure")]
    Insecure,
    #[serde(rename = "starttls")]
    Starttls,
    #[serde(rename = "tls")]
    Tls,
}
impl Default for Mode {
    fn default() -> Self {
        Self::Tls
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Remove smtp endpoint"]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl GetOutput {
    pub fn new(from_address: String, name: String, server: String) -> Self {
        Self {
            from_address,
            name,
            server,
            author: Default::default(),
            comment: Default::default(),
            digest: Default::default(),
            disable: Default::default(),
            mailto: Default::default(),
            mailto_user: Default::default(),
            mode: Default::default(),
            port: Default::default(),
            username: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Author of the mail. Defaults to 'Proxmox VE'."]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment"]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    pub digest: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable this target"]
    pub disable: Option<bool>,
    #[serde(rename = "from-address")]
    #[doc = "`From` address for the mail"]
    pub from_address: String,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of email recipients"]
    pub mailto: Vec<String>,
    #[serde(rename = "mailto-user")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of users"]
    pub mailto_user: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Determine which encryption method shall be used for the connection."]
    pub mode: Option<Mode>,
    #[doc = "The name of the endpoint."]
    pub name: String,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The port to be used. Defaults to 465 for TLS based connections, 587 for STARTTLS based connections and port 25 for insecure plain-text connections."]
    pub port: Option<u64>,
    #[doc = "The address of the SMTP server."]
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Username for SMTP authentication"]
    pub username: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Return a specific smtp endpoint"]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Author of the mail. Defaults to 'Proxmox VE'."]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment"]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "A list of settings you want to delete."]
    pub delete: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    pub digest: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Determine which encryption method shall be used for the connection."]
    pub mode: Option<Mode>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Password for SMTP authentication"]
    pub password: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The port to be used. Defaults to 465 for TLS based connections, 587 for STARTTLS based connections and port 25 for insecure plain-text connections."]
    pub port: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The address of the SMTP server."]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Username for SMTP authentication"]
    pub username: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update existing smtp endpoint"]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
