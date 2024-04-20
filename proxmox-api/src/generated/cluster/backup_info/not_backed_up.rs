pub struct NotBackedUpClient<T> {
    client: T,
    path: String,
}
impl<T> NotBackedUpClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/not-backed-up"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a NotBackedUpClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> NotBackedUpClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Shows all guests which are not covered by any backup job."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(ty: Type, vmid: u64) -> Self {
        Self {
            ty,
            vmid,
            name: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Name of the guest"]
    #[doc = ""]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[doc = "Type of the guest."]
    #[doc = ""]
    pub ty: Type,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "VMID of the guest."]
    #[doc = ""]
    pub vmid: u64,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Type of the guest."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "lxc")]
    Lxc,
    #[serde(rename = "qemu")]
    Qemu,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "lxc" => Ok(Self::Lxc),
            "qemu" => Ok(Self::Qemu),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
