pub struct PasswordClient<T> {
    client: T,
    path: String,
}
impl<T> PasswordClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/password"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a PasswordClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> PasswordClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Change user password."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.put(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PutParams, (), T::Error> for &PasswordClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Put;
    fn exec(&self, params: PutParams) -> Result<(), T::Error> {
        self.put(params)
    }
}
impl PutParams {
    pub fn new(password: String, userid: String) -> Self {
        Self {
            password,
            userid,
            confirmation_password: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[serde(rename = "confirmation-password")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The current password of the user performing the change."]
    #[doc = ""]
    pub confirmation_password: Option<String>,
    #[doc = "The new password."]
    #[doc = ""]
    pub password: String,
    #[doc = "Full User ID, in the `name@realm` format."]
    #[doc = ""]
    pub userid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
