pub struct DirectoriesClient<T> {
    client: T,
    path: String,
}
impl<T> DirectoriesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/directories"),
        }
    }
}
impl GetOutputItems {
    pub fn new(name: String, url: String) -> Self {
        Self { name, url }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub name: String,
    #[doc = "URL of ACME CA directory endpoint."]
    pub url: String,
}
impl<T> DirectoriesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get named known ACME directory endpoints."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
