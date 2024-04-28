pub mod name;
#[derive(Debug, Clone)]
pub struct PoolClient<T> {
    client: T,
    path: String,
}
impl<T> PoolClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/pool"),
        }
    }
}
impl<T> PoolClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List all pools and their settings (which are settable by the POST/PUT endpoints)."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> PoolClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create Ceph pool"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct ApplicationMetadataGetOutputItemsApplicationMetadata {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct AutoscaleStatusGetOutputItemsAutoscaleStatus {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutputItems {
    pub fn new(
        bytes_used: i64,
        crush_rule: i64,
        crush_rule_name: String,
        min_size: i64,
        percent_used: f64,
        pg_num: i64,
        pool: i64,
        pool_name: String,
        size: i64,
        ty: Type,
    ) -> Self {
        Self {
            bytes_used,
            crush_rule,
            crush_rule_name,
            min_size,
            percent_used,
            pg_num,
            pool,
            pool_name,
            size,
            ty,
            application_metadata: Default::default(),
            autoscale_status: Default::default(),
            pg_autoscale_mode: Default::default(),
            pg_num_final: Default::default(),
            pg_num_min: Default::default(),
            target_size: Default::default(),
            target_size_ratio: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub application_metadata: Option<ApplicationMetadataGetOutputItemsApplicationMetadata>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub autoscale_status: Option<AutoscaleStatusGetOutputItemsAutoscaleStatus>,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub bytes_used: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub crush_rule: i64,
    pub crush_rule_name: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub min_size: i64,
    #[serde(
        serialize_with = "crate::types::serialize_number",
        deserialize_with = "crate::types::deserialize_number"
    )]
    pub percent_used: f64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pg_autoscale_mode: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub pg_num: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pg_num_final: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pg_num_min: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub pool: i64,
    pub pool_name: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub size: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target_size: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target_size_ratio: Option<f64>,
    #[serde(rename = "type")]
    pub ty: Type,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(name: String) -> Self {
        Self {
            name,
            add_storages: Default::default(),
            application: Default::default(),
            crush_rule: Default::default(),
            erasure_coding: Default::default(),
            min_size: Default::default(),
            pg_autoscale_mode: Default::default(),
            pg_num: Default::default(),
            pg_num_min: Default::default(),
            size: Default::default(),
            target_size: Default::default(),
            target_size_ratio: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Configure VM and CT storage using the new pool."]
    #[doc = ""]
    pub add_storages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The application of the pool."]
    #[doc = ""]
    pub application: Option<Application>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The rule to use for mapping object placement in the cluster."]
    #[doc = ""]
    pub crush_rule: Option<String>,
    #[serde(rename = "erasure-coding")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Create an erasure coded pool for RBD with an accompaning replicated pool for metadata storage. With EC, the common ceph options 'size', 'min_size' and 'crush_rule' parameters will be applied to the metadata pool."]
    #[doc = ""]
    pub erasure_coding: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Minimum number of replicas per object"]
    #[doc = ""]
    pub min_size: Option<i64>,
    #[doc = "The name of the pool. It must be unique."]
    #[doc = ""]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The automatic PG scaling mode of the pool."]
    #[doc = ""]
    pub pg_autoscale_mode: Option<PgAutoscaleMode>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of placement groups."]
    #[doc = ""]
    pub pg_num: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Minimal number of placement groups."]
    #[doc = ""]
    pub pg_num_min: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of replicas per object"]
    #[doc = ""]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The estimated target size of the pool for the PG autoscaler."]
    #[doc = ""]
    pub target_size: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The estimated target ratio of the pool for the PG autoscaler."]
    #[doc = ""]
    pub target_size_ratio: Option<f64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "The application of the pool."]
#[doc = ""]
pub enum Application {
    #[serde(rename = "cephfs")]
    Cephfs,
    #[serde(rename = "rbd")]
    Rbd,
    #[serde(rename = "rgw")]
    Rgw,
}
impl TryFrom<&str> for Application {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "cephfs" => Ok(Self::Cephfs),
            "rbd" => Ok(Self::Rbd),
            "rgw" => Ok(Self::Rgw),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl Default for Application {
    fn default() -> Self {
        Self::Rbd
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "The automatic PG scaling mode of the pool."]
#[doc = ""]
pub enum PgAutoscaleMode {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "on")]
    On,
    #[serde(rename = "warn")]
    Warn,
}
impl TryFrom<&str> for PgAutoscaleMode {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "off" => Ok(Self::Off),
            "on" => Ok(Self::On),
            "warn" => Ok(Self::Warn),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl Default for PgAutoscaleMode {
    fn default() -> Self {
        Self::Warn
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
pub enum Type {
    #[serde(rename = "erasure")]
    Erasure,
    #[serde(rename = "replicated")]
    Replicated,
    #[serde(rename = "unknown")]
    Unknown,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "erasure" => Ok(Self::Erasure),
            "replicated" => Ok(Self::Replicated),
            "unknown" => Ok(Self::Unknown),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl<T> PoolClient<T>
where
    T: crate::client::Client,
{
    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
