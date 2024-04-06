pub struct GroupidClient<T> {
    client: T,
    path: String,
}
impl<T> GroupidClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, groupid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, groupid),
        }
    }
}
impl<T> GroupidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete group."]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl GetOutput {
    pub fn new(members: Vec<String>) -> Self {
        Self {
            members,
            comment: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub members: Vec<String>,
}
impl<T> GroupidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get group configuration."]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> GroupidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update group data."]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
