pub struct PrunebackupsClient<T> {
    client: T,
    path: String,
}
impl<T> PrunebackupsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/prunebackups"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Mark {
    #[serde(rename = "keep")]
    Keep,
    #[serde(rename = "protected")]
    Protected,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "renamed")]
    Renamed,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Type {
    #[serde(rename = "lxc")]
    Lxc,
    #[serde(rename = "qemu")]
    Qemu,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct DeleteParams {
    #[serde(rename = "prune-backups")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use these retention options instead of those from the storage configuration."]
    pub prune_backups: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Either 'qemu' or 'lxc'. Only consider backups for guests of this type."]
    pub ty: Option<Type>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only prune backups for this VM."]
    pub vmid: Option<crate::VmId>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> PrunebackupsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Prune backups. Only those using the standard naming scheme are considered."]
    pub fn delete(&self, params: DeleteParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(rename = "prune-backups")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use these retention options instead of those from the storage configuration."]
    pub prune_backups: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Either 'qemu' or 'lxc'. Only consider backups for guests of this type."]
    pub ty: Option<Type>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only consider backups for this guest."]
    pub vmid: Option<crate::VmId>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutputItems {
    pub fn new(ctime: u64, mark: Mark, ty: String, volid: String) -> Self {
        Self {
            ctime,
            mark,
            ty,
            volid,
            vmid: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::serialize_int",
        deserialize_with = "crate::deserialize_int"
    )]
    #[doc = "Creation time of the backup (seconds since the UNIX epoch)."]
    pub ctime: u64,
    #[doc = "Whether the backup would be kept or removed. Backups that are protected or don't use the standard naming scheme are not removed."]
    pub mark: Mark,
    #[serde(rename = "type")]
    #[doc = "One of 'qemu', 'lxc', 'openvz' or 'unknown'."]
    pub ty: String,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The VM the backup belongs to."]
    pub vmid: Option<u64>,
    #[doc = "Backup volume ID."]
    pub volid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> PrunebackupsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get prune information for backups. NOTE: this is only a preview and might not be what a subsequent prune call does if backups are removed/added in the meantime."]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
