pub mod subnet;
pub struct SubnetsClient<T> {
    client: T,
    path: String,
}
impl<T> SubnetsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/subnets"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Type {
    #[serde(rename = "subnet")]
    Subnet,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Display pending config."]
    pub pending: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Display running config."]
    pub running: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> SubnetsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "SDN subnets index."]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl PostParams {
    pub fn new(subnet: String, ty: Type) -> Self {
        Self {
            subnet,
            ty,
            dhcp_dns_server: Default::default(),
            dhcp_range: Default::default(),
            dnszoneprefix: Default::default(),
            gateway: Default::default(),
            snat: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(rename = "dhcp-dns-server")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IP address for the DNS server"]
    pub dhcp_dns_server: Option<::std::net::IpAddr>,
    #[serde(rename = "dhcp-range")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "A list of DHCP ranges for this subnet"]
    pub dhcp_range: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "dns domain zone prefix  ex: 'adm' -\\> \\<hostname\\>.adm.mydomain.com"]
    pub dnszoneprefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Subnet Gateway: Will be assign on vnet for layer3 zones"]
    pub gateway: Option<::std::net::IpAddr>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "enable masquerade for this subnet if pve-firewall"]
    pub snat: Option<bool>,
    #[doc = "The SDN subnet object identifier."]
    pub subnet: String,
    #[serde(rename = "type")]
    pub ty: Type,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> SubnetsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new sdn subnet object."]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> SubnetsClient<T>
where
    T: crate::client::Client,
{
    pub fn subnet(&self, subnet: &str) -> subnet::SubnetClient<T> {
        subnet::SubnetClient::<T>::new(self.client.clone(), &self.path, subnet)
    }
}
