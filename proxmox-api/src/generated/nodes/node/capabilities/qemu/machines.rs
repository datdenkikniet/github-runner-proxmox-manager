pub struct MachinesClient<T> {
    client: T,
    path: String,
}
impl<T> MachinesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/machines"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a MachinesClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> MachinesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get available QEMU/KVM machine types."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), Vec<GetOutputItems>, T::Error>
    for &MachinesClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<Vec<GetOutputItems>, T::Error> {
        self.get()
    }
}
impl GetOutputItems {
    pub fn new(id: String, ty: Type, version: String) -> Self {
        Self {
            id,
            ty,
            version,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "Full name of machine type and version."]
    #[doc = ""]
    pub id: String,
    #[serde(rename = "type")]
    #[doc = "The machine type."]
    #[doc = ""]
    pub ty: Type,
    #[doc = "The machine version."]
    #[doc = ""]
    pub version: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "The machine type."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "i440fx")]
    I440fx,
    #[serde(rename = "q35")]
    Q35,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "i440fx" => Ok(Self::I440fx),
            "q35" => Ok(Self::Q35),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
