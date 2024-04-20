pub mod current;
pub mod reboot;
pub mod resume;
pub mod shutdown;
pub mod start;
pub mod stop;
pub mod suspend;
pub struct StatusClient<T> {
    client: T,
    path: String,
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/status"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a StatusClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index"]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), Vec<GetOutputItems>, T::Error>
    for &StatusClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<Vec<GetOutputItems>, T::Error> {
        self.get()
    }
}
impl GetOutputItems {
    pub fn new(subdir: String) -> Self {
        Self {
            subdir,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub subdir: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    pub fn current(&self) -> current::CurrentClient<T> {
        current::CurrentClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    pub fn start(&self) -> start::StartClient<T> {
        start::StartClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    pub fn stop(&self) -> stop::StopClient<T> {
        stop::StopClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    pub fn shutdown(&self) -> shutdown::ShutdownClient<T> {
        shutdown::ShutdownClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    pub fn suspend(&self) -> suspend::SuspendClient<T> {
        suspend::SuspendClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    pub fn resume(&self) -> resume::ResumeClient<T> {
        resume::ResumeClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    pub fn reboot(&self) -> reboot::RebootClient<T> {
        reboot::RebootClient::<T>::new(self.client.clone(), &self.path)
    }
}
