pub mod id;
pub struct RealmSyncClient<T> {
    client: T,
    path: String,
}
impl<T> RealmSyncClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/realm-sync"),
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
impl GetOutputItems {
    pub fn new(enabled: bool, id: String, realm: String, schedule: String) -> Self {
        Self {
            enabled,
            id,
            realm,
            schedule,
            comment: Default::default(),
            last_run: Default::default(),
            next_run: Default::default(),
            remove_vanished: Default::default(),
            scope: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A comment for the job."]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    #[doc = "If the job is enabled or not."]
    pub enabled: bool,
    #[doc = "The ID of the entry."]
    pub id: String,
    #[serde(rename = "last-run")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Last execution time of the job in seconds since the beginning of the UNIX epoch"]
    pub last_run: Option<u64>,
    #[serde(rename = "next-run")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Next planned execution time of the job in seconds since the beginning of the UNIX epoch."]
    pub next_run: Option<u64>,
    #[doc = "Authentication domain ID"]
    pub realm: String,
    #[serde(rename = "remove-vanished")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A semicolon-seperated list of things to remove when they or the user vanishes during a sync. The following values are possible: 'entry' removes the user/group when not returned from the sync. 'properties' removes the set properties on existing user/group that do not appear in the source (even custom ones). 'acl' removes acls when the user/group is not returned from the sync. Instead of a list it also can be 'none' (the default)."]
    pub remove_vanished: Option<String>,
    #[doc = "The configured sync schedule."]
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
impl<T> RealmSyncClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List configured realm-sync-jobs."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> RealmSyncClient<T>
where
    T: crate::client::Client,
{
    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
