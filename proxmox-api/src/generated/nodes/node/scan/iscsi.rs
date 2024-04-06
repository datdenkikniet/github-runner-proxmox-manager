pub struct IscsiClient<T> {
    client: T,
    path: String,
}
impl<T> IscsiClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/iscsi"),
        }
    }
}
impl GetParams {
    pub fn new(portal: String) -> Self {
        Self {
            portal,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[doc = "The iSCSI portal (IP or DNS name with optional port)."]
    pub portal: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutputItems {
    pub fn new(portal: String, target: String) -> Self {
        Self {
            portal,
            target,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "The iSCSI portal name."]
    pub portal: String,
    #[doc = "The iSCSI target name."]
    pub target: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> IscsiClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Scan remote iSCSI server."]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
