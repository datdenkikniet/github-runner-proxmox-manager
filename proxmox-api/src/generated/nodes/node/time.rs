#[derive(Debug, Clone)]
pub struct TimeClient<T> {
    client: T,
    path: String,
}
impl<T> TimeClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/time"),
        }
    }
}
impl<T> TimeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read server time and time zone settings."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> TimeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Set time zone."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
impl GetOutput {
    pub fn new(localtime: i64, time: i64, timezone: String) -> Self {
        Self {
            localtime,
            time,
            timezone,
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Seconds since 1970-01-01 00:00:00 (local time)"]
    #[doc = ""]
    pub localtime: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Seconds since 1970-01-01 00:00:00 UTC."]
    #[doc = ""]
    pub time: i64,
    #[doc = "Time zone"]
    #[doc = ""]
    pub timezone: String,
}
impl PutParams {
    pub fn new(timezone: String) -> Self {
        Self {
            timezone,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[doc = "Time zone. The file '/usr/share/zoneinfo/zone.tab' contains the list of valid names."]
    #[doc = ""]
    pub timezone: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
