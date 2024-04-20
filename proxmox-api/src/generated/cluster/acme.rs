pub mod account;
pub mod challenge_schema;
pub mod directories;
pub mod meta;
pub mod plugins;
pub mod tos;
pub struct AcmeClient<T> {
    client: T,
    path: String,
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/acme"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a AcmeClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "ACMEAccount index."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), Vec<GetOutputItems>, T::Error>
    for &AcmeClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<Vec<GetOutputItems>, T::Error> {
        self.get()
    }
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
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    pub fn plugins(&self) -> plugins::PluginsClient<T> {
        plugins::PluginsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    pub fn account(&self) -> account::AccountClient<T> {
        account::AccountClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    pub fn tos(&self) -> tos::TosClient<T> {
        tos::TosClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    pub fn meta(&self) -> meta::MetaClient<T> {
        meta::MetaClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    pub fn directories(&self) -> directories::DirectoriesClient<T> {
        directories::DirectoriesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    pub fn challenge_schema(&self) -> challenge_schema::ChallengeSchemaClient<T> {
        challenge_schema::ChallengeSchemaClient::<T>::new(self.client.clone(), &self.path)
    }
}
