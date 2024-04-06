pub struct DownloadClient<T> {
    client: T,
    path: String,
}
impl<T> DownloadClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/download"),
        }
    }
}
impl GetParams {
    pub fn new(filepath: String, volume: String) -> Self {
        Self {
            filepath,
            volume,
            tar: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[doc = "base64-path to the directory or file to download."]
    pub filepath: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Download dirs as 'tar.zst' instead of 'zip'."]
    pub tar: Option<bool>,
    #[doc = "Backup volume ID or name. Currently only PBS snapshots are supported."]
    pub volume: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> DownloadClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Extract a file or directory (as zip archive) from a PBS backup."]
    pub fn get(&self, params: GetParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
