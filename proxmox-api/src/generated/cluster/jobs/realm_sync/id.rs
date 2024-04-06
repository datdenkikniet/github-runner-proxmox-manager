pub struct IdClient<T> {
    client: T,
    path: String,
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, id),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Scope {
    #[serde(rename = "both")]
    Both,
    #[serde(rename = "groups")]
    Groups,
    #[serde(rename = "users")]
    Users,
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete realm-sync job definition."]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
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
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read realm-sync job definition."]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl PostParams {
    pub fn new(schedule: String) -> Self {
        Self {
            schedule,
            comment: Default::default(),
            enable_new: Default::default(),
            enabled: Default::default(),
            realm: Default::default(),
            remove_vanished: Default::default(),
            scope: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description for the Job."]
    pub comment: Option<String>,
    #[serde(rename = "enable-new")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable newly synced users immediately."]
    pub enable_new: Option<()>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Determines if the job is enabled."]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Authentication domain ID"]
    pub realm: Option<String>,
    #[serde(rename = "remove-vanished")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A semicolon-seperated list of things to remove when they or the user vanishes during a sync. The following values are possible: 'entry' removes the user/group when not returned from the sync. 'properties' removes the set properties on existing user/group that do not appear in the source (even custom ones). 'acl' removes acls when the user/group is not returned from the sync. Instead of a list it also can be 'none' (the default)."]
    pub remove_vanished: Option<String>,
    #[doc = "Backup schedule. The format is a subset of `systemd` calendar events."]
    pub schedule: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Select what to sync."]
    pub scope: Option<Scope>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create new realm-sync job."]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl PutParams {
    pub fn new(schedule: String) -> Self {
        Self {
            schedule,
            comment: Default::default(),
            delete: Default::default(),
            enable_new: Default::default(),
            enabled: Default::default(),
            remove_vanished: Default::default(),
            scope: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description for the Job."]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    pub delete: Option<String>,
    #[serde(rename = "enable-new")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable newly synced users immediately."]
    pub enable_new: Option<()>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Determines if the job is enabled."]
    pub enabled: Option<bool>,
    #[serde(rename = "remove-vanished")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A semicolon-seperated list of things to remove when they or the user vanishes during a sync. The following values are possible: 'entry' removes the user/group when not returned from the sync. 'properties' removes the set properties on existing user/group that do not appear in the source (even custom ones). 'acl' removes acls when the user/group is not returned from the sync. Instead of a list it also can be 'none' (the default)."]
    pub remove_vanished: Option<String>,
    #[doc = "Backup schedule. The format is a subset of `systemd` calendar events."]
    pub schedule: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Select what to sync."]
    pub scope: Option<Scope>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update realm-sync job definition."]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
