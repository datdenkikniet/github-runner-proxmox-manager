pub struct MacrosClient<T> {
    client: T,
    path: String,
}
impl<T> MacrosClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/macros"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a MacrosClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> MacrosClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List available macros"]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(descr: String, macro_def: String) -> Self {
        Self {
            descr,
            macro_def,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "More verbose description (if available)."]
    #[doc = ""]
    pub descr: String,
    #[serde(rename = "macro")]
    #[doc = "Macro name."]
    #[doc = ""]
    pub macro_def: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
