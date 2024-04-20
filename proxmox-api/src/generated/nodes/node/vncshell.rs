pub struct VncshellClient<T> {
    client: T,
    path: String,
}
impl<T> VncshellClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/vncshell"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a VncshellClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> VncshellClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Creates a VNC Shell proxy."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PostParams, (), T::Error> for &VncshellClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: PostParams) -> Result<(), T::Error> {
        self.post(params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Run specific command or default to login (requires 'root@pam')"]
    #[doc = ""]
    pub cmd: Option<Cmd>,
    #[serde(rename = "cmd-opts")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Add parameters to a command. Encoded as null terminated strings."]
    #[doc = ""]
    pub cmd_opts: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "sets the height of the console in pixels."]
    #[doc = ""]
    pub height: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "use websocket instead of standard vnc."]
    #[doc = ""]
    pub websocket: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "sets the width of the console in pixels."]
    #[doc = ""]
    pub width: Option<u64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Run specific command or default to login (requires 'root@pam')"]
#[doc = ""]
pub enum Cmd {
    #[serde(rename = "ceph_install")]
    CephInstall,
    #[serde(rename = "login")]
    Login,
    #[serde(rename = "upgrade")]
    Upgrade,
}
impl TryFrom<&str> for Cmd {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "ceph_install" => Ok(Self::CephInstall),
            "login" => Ok(Self::Login),
            "upgrade" => Ok(Self::Upgrade),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl Default for Cmd {
    fn default() -> Self {
        Self::Login
    }
}
