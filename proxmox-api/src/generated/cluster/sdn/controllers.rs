pub mod controller;
pub struct ControllersClient<T> {
    client: T,
    path: String,
}
impl<T> ControllersClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/controllers"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Type {
    #[serde(rename = "bgp")]
    Bgp,
    #[serde(rename = "evpn")]
    Evpn,
    #[serde(rename = "faucet")]
    Faucet,
    #[serde(rename = "isis")]
    Isis,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Display pending config."]
    pub pending: Option<bool>,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Display running config."]
    pub running: Option<bool>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list sdn controllers of specific type"]
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutputItems {
    pub fn new(controller: String, ty: String) -> Self {
        Self {
            controller,
            ty,
            pending: Default::default(),
            state: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub controller: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pending: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub state: Option<String>,
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> ControllersClient<T>
where
    T: crate::client::Client,
{
    #[doc = "SDN controllers index."]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl PostParams {
    pub fn new(controller: String, ty: Type) -> Self {
        Self {
            controller,
            ty,
            asn: Default::default(),
            bgp_multipath_as_path_relax: Default::default(),
            ebgp: Default::default(),
            ebgp_multihop: Default::default(),
            isis_domain: Default::default(),
            isis_ifaces: Default::default(),
            isis_net: Default::default(),
            loopback: Default::default(),
            node: Default::default(),
            peers: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "autonomous system number"]
    pub asn: Option<()>,
    #[serde(rename = "bgp-multipath-as-path-relax")]
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bgp_multipath_as_path_relax: Option<bool>,
    #[doc = "The SDN controller object identifier."]
    pub controller: String,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable ebgp. (remote-as external)"]
    pub ebgp: Option<bool>,
    #[serde(rename = "ebgp-multihop")]
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ebgp_multihop: Option<u64>,
    #[serde(rename = "isis-domain")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ISIS domain."]
    pub isis_domain: Option<String>,
    #[serde(rename = "isis-ifaces")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ISIS interface."]
    pub isis_ifaces: Option<String>,
    #[serde(rename = "isis-net")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ISIS network entity title."]
    pub isis_net: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "source loopback interface."]
    pub loopback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The cluster node name."]
    pub node: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "peers address list."]
    pub peers: Option<String>,
    #[serde(rename = "type")]
    #[doc = "Plugin type."]
    pub ty: Type,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> ControllersClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new sdn controller object."]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> ControllersClient<T>
where
    T: crate::client::Client,
{
    pub fn controller(&self, controller: &str) -> controller::ControllerClient<T> {
        controller::ControllerClient::<T>::new(self.client.clone(), &self.path, controller)
    }
}
