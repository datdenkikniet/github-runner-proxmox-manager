pub struct NodeClient<T> {
    client: T,
    path: String,
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, node: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, node),
        }
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Removes a node from the cluster configuration."]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
#[derive(Default)]
struct NumberedLinks;
impl crate::types::multi::NumberedItems for NumberedLinks {
    type Item = String;
    const PREFIX: &'static str = "link";
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The JOIN_API_VERSION of the new node."]
    pub apiversion: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Do not throw error if node already exists."]
    pub force: Option<bool>,
    #[serde(rename = "link[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedLinks, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedLinks, _>"
    )]
    #[serde(
        skip_serializing_if = "::std::collections::BTreeMap::is_empty",
        default
    )]
    #[doc = "Address and priority information of a single corosync link. (up to 8 links supported; link0..link7)"]
    pub links: ::std::collections::BTreeMap<u32, String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IP Address of node to add. Used as fallback if no links are given."]
    pub new_node_ip: Option<::std::net::IpAddr>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Node id for this node."]
    pub nodeid: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of votes for this node"]
    pub votes: Option<u64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostOutput {
    pub fn new(corosync_authkey: String, corosync_conf: String, warnings: Vec<String>) -> Self {
        Self {
            corosync_authkey,
            corosync_conf,
            warnings,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostOutput {
    pub corosync_authkey: String,
    pub corosync_conf: String,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub warnings: Vec<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Adds a node to the cluster configuration. This call is for internal use."]
    pub fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
