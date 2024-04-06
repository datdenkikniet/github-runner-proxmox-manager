pub struct SubnetClient<T> {
    client: T,
    path: String,
}
impl<T> SubnetClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, subnet: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, subnet),
        }
    }
}
impl<T> SubnetClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete sdn subnet object configuration."]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
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
pub struct GetOutput {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> SubnetClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read sdn subnet configuration."]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    pub delete: Option<String>,
    #[serde(rename = "dhcp-dns-server")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IP address for the DNS server"]
    pub dhcp_dns_server: Option<String>,
    #[serde(rename = "dhcp-range")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "A list of DHCP ranges for this subnet"]
    pub dhcp_range: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "dns domain zone prefix  ex: 'adm' -\\> \\<hostname\\>.adm.mydomain.com"]
    pub dnszoneprefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Subnet Gateway: Will be assign on vnet for layer3 zones"]
    pub gateway: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "enable masquerade for this subnet if pve-firewall"]
    pub snat: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> SubnetClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update sdn subnet object configuration."]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
