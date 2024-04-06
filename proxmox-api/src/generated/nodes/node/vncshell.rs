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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Cmd {
    #[serde(rename = "ceph_install")]
    CephInstall,
    #[serde(rename = "login")]
    Login,
    #[serde(rename = "upgrade")]
    Upgrade,
}
impl Default for Cmd {
    fn default() -> Self {
        Self::Login
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Run specific command or default to login (requires 'root@pam')"]
    pub cmd: Option<Cmd>,
    #[serde(rename = "cmd-opts")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Add parameters to a command. Encoded as null terminated strings."]
    pub cmd_opts: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "sets the height of the console in pixels."]
    pub height: Option<u64>,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "use websocket instead of standard vnc."]
    pub websocket: Option<bool>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "sets the width of the console in pixels."]
    pub width: Option<u64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> VncshellClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Creates a VNC Shell proxy."]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
