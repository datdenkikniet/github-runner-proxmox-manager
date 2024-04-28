#[derive(Debug, Clone)]
pub struct ListClient<T> {
    client: T,
    path: String,
}
impl<T> ListClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/list"),
        }
    }
}
impl<T> ListClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List local disks."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(
        devpath: String,
        gpt: bool,
        mounted: bool,
        osdid: i64,
        osdid_list: Vec<i64>,
        size: i64,
    ) -> Self {
        Self {
            devpath,
            gpt,
            mounted,
            osdid,
            osdid_list,
            size,
            health: Default::default(),
            model: Default::default(),
            parent: Default::default(),
            serial: Default::default(),
            used: Default::default(),
            vendor: Default::default(),
            wwn: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "The device path"]
    #[doc = ""]
    pub devpath: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    pub gpt: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub health: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub model: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    pub mounted: bool,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub osdid: i64,
    #[serde(rename = "osdid-list")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub osdid_list: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For partitions only. The device path of the disk the partition resides on."]
    #[doc = ""]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub serial: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub size: i64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub used: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub wwn: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(rename = "include-partitions")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Also include partitions."]
    #[doc = ""]
    pub include_partitions: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Skip smart checks."]
    #[doc = ""]
    pub skipsmart: Option<bool>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list specific types of disks."]
    #[doc = ""]
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Only list specific types of disks."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "journal_disks")]
    JournalDisks,
    #[serde(rename = "unused")]
    Unused,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "journal_disks" => Ok(Self::JournalDisks),
            "unused" => Ok(Self::Unused),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
