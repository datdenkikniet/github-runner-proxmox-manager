pub struct OptionsClient<T> {
    client: T,
    path: String,
}
impl<T> OptionsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/options"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Console {
    #[serde(rename = "applet")]
    Applet,
    #[serde(rename = "html5")]
    Html5,
    #[serde(rename = "vv")]
    Vv,
    #[serde(rename = "xtermjs")]
    Xtermjs,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Fencing {
    #[serde(rename = "both")]
    Both,
    #[serde(rename = "hardware")]
    Hardware,
    #[serde(rename = "watchdog")]
    Watchdog,
}
impl Default for Fencing {
    fn default() -> Self {
        Self::Watchdog
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Keyboard {
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "de-ch")]
    DeCh,
    #[serde(rename = "en-gb")]
    EnGb,
    #[serde(rename = "en-us")]
    EnUs,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "fr-be")]
    FrBe,
    #[serde(rename = "fr-ca")]
    FrCa,
    #[serde(rename = "fr-ch")]
    FrCh,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "mk")]
    Mk,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "pt-br")]
    PtBr,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "tr")]
    Tr,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Language {
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "fa")]
    Fa,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "he")]
    He,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "ka")]
    Ka,
    #[serde(rename = "kr")]
    Kr,
    #[serde(rename = "nb")]
    Nb,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "nn")]
    Nn,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt_BR")]
    PtBR,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "ukr")]
    Ukr,
    #[serde(rename = "zh_CN")]
    ZhCN,
    #[serde(rename = "zh_TW")]
    ZhTW,
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
impl<T> OptionsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get datacenter options. Without 'Sys.Audit' on '/' not all options are returned."]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set I/O bandwidth limit for various operations (in KiB/s)."]
    pub bwlimit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Select the default Console viewer. You can either use the builtin java applet (VNC; deprecated and maps to html5), an external virt-viewer comtatible application (SPICE), an HTML5 based vnc viewer (noVNC), or an HTML5 based console client (xtermjs). If the selected viewer is not available (e.g. SPICE not activated for the VM), the fallback is noVNC."]
    pub console: Option<Console>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Cluster resource scheduling settings."]
    pub crs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Datacenter description. Shown in the web-interface datacenter notes panel. This is saved as comment inside the configuration file."]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify email address to send notification from (default is root@$hostname)"]
    pub email_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set the fencing mode of the HA cluster. Hardware mode needs a valid configuration of fence devices in /etc/pve/ha/fence.cfg. With both all two modes are used.\n\nWARNING: 'hardware' and 'both' are EXPERIMENTAL & WIP"]
    pub fencing: Option<Fencing>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Cluster wide HA settings."]
    pub ha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify external http proxy which is used for downloads (example: 'http://username:password@host:port/')"]
    pub http_proxy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Default keybord layout for vnc server."]
    pub keyboard: Option<Keyboard>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Default GUI language."]
    pub language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prefix for the auto-generated MAC addresses of virtual guests. The default 'BC:24:11' is the OUI assigned by the IEEE to Proxmox Server Solutions GmbH for a 24-bit large MAC block. You're allowed to use this in local networks, i.e., those not directly reachable by the public (e.g., in a LAN or behind NAT)."]
    #[doc = "Prefix for the auto-generated MAC addresses of virtual guests. The default `BC:24:11` is the Organizationally Unique Identifier (OUI) assigned by the IEEE to Proxmox Server Solutions GmbH for a MAC Address Block Large (MA-L). You're allowed to use this in local networks, i.e., those not directly reachable by the public (e.g., in a LAN or NAT/Masquerading).\n \nNote that when you run multiple cluster that (partially) share the networks of their virtual guests, it's highly recommended that you extend the default MAC prefix, or generate a custom (valid) one, to reduce the chance of MAC collisions. For example, add a separate extra hexadecimal to the Proxmox OUI for each cluster, like `BC:24:11:0` for the first, `BC:24:11:1` for the second, and so on.\n Alternatively, you can also separate the networks of the guests logically, e.g., by using VLANs.\n\nFor publicly accessible guests it's recommended that you get your own https://standards.ieee.org/products-programs/regauth/[OUI from the IEEE] registered or coordinate with your, or your hosting providers, network admins."]
    pub mac_prefix: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Defines how many workers (per node) are maximal started  on actions like 'stopall VMs' or task from the ha-manager."]
    pub max_workers: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For cluster wide migration settings."]
    pub migration: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Migration is secure using SSH tunnel by default. For secure private networks you can disable it to speed up migration. Deprecated, use the 'migration' property instead!"]
    pub migration_unsecure: Option<bool>,
    #[serde(rename = "next-id")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Control the range for the free VMID auto-selection pool."]
    pub next_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Cluster-wide notification settings."]
    pub notify: Option<String>,
    #[serde(rename = "registered-tags")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of tags that require a `Sys.Modify` on '/' to set and delete. Tags set here that are also in 'user-tag-access' also require `Sys.Modify`."]
    pub registered_tags: Option<String>,
    #[serde(rename = "tag-style")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Tag style options."]
    pub tag_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "u2f"]
    pub u2f: Option<String>,
    #[serde(rename = "user-tag-access")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Privilege options for user-settable tags"]
    pub user_tag_access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "webauthn configuration"]
    pub webauthn: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> OptionsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Set datacenter options."]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
