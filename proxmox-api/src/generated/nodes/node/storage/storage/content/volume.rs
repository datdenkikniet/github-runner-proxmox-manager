#[derive(Debug, Clone)]
pub struct VolumeClient<T> {
    client: T,
    path: String,
}
impl<T> VolumeClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, volume: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, volume),
        }
    }
}
impl<T> VolumeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete volume"]
    #[doc = ""]
    pub fn delete(&self, params: DeleteParams) -> Result<Option<String>, T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &params)
    }
}
impl<T> VolumeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get volume attributes"]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> VolumeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Copy a volume. This is experimental code - do not use."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> VolumeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update volume attributes"]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct DeleteParams {
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Time to wait for the task to finish. We return 'null' if the task finish within that time."]
    #[doc = ""]
    pub delay: Option<i64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutput {
    pub fn new(format: String, path: String, size: i64, used: i64) -> Self {
        Self {
            format,
            path,
            size,
            used,
            notes: Default::default(),
            protected: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[doc = "Format identifier ('raw', 'qcow2', 'subvol', 'iso', 'tgz' ...)"]
    #[doc = ""]
    pub format: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Optional notes."]
    #[doc = ""]
    pub notes: Option<String>,
    #[doc = "The Path"]
    #[doc = ""]
    pub path: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Protection status. Currently only supported for backups."]
    #[doc = ""]
    pub protected: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Volume size in bytes."]
    #[doc = ""]
    pub size: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Used space. Please note that most storage plugins do not report anything useful here."]
    #[doc = ""]
    pub used: i64,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(target: String) -> Self {
        Self {
            target,
            target_node: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "Target volume identifier"]
    #[doc = ""]
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Target node. Default is local node."]
    #[doc = ""]
    pub target_node: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The new notes."]
    #[doc = ""]
    pub notes: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Protection status. Currently only supported for backups."]
    #[doc = ""]
    pub protected: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
