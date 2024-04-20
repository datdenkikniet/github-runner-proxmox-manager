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
impl<'a, T> crate::ProxmoxClient for &'a OptionsClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> OptionsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get datacenter options. Without 'Sys.Audit' on '/' not all options are returned."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> OptionsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Set datacenter options."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.put(&path, &params)
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set I/O bandwidth limit for various operations (in KiB/s)."]
    #[doc = ""]
    pub bwlimit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Select the default Console viewer. You can either use the builtin java applet (VNC; deprecated and maps to html5), an external virt-viewer comtatible application (SPICE), an HTML5 based vnc viewer (noVNC), or an HTML5 based console client (xtermjs). If the selected viewer is not available (e.g. SPICE not activated for the VM), the fallback is noVNC."]
    #[doc = ""]
    pub console: Option<Console>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Cluster resource scheduling settings."]
    #[doc = ""]
    pub crs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Datacenter description. Shown in the web-interface datacenter notes panel. This is saved as comment inside the configuration file."]
    #[doc = ""]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify email address to send notification from (default is root@$hostname)"]
    #[doc = ""]
    pub email_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set the fencing mode of the HA cluster. Hardware mode needs a valid configuration of fence devices in /etc/pve/ha/fence.cfg. With both all two modes are used."]
    #[doc = ""]
    #[doc = "WARNING: 'hardware' and 'both' are EXPERIMENTAL & WIP"]
    #[doc = ""]
    pub fencing: Option<Fencing>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Cluster wide HA settings."]
    #[doc = ""]
    pub ha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify external http proxy which is used for downloads (example: 'http://username:password@host:port/')"]
    #[doc = ""]
    pub http_proxy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Default keybord layout for vnc server."]
    #[doc = ""]
    pub keyboard: Option<Keyboard>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Default GUI language."]
    #[doc = ""]
    pub language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prefix for the auto-generated MAC addresses of virtual guests. The default 'BC:24:11' is the OUI assigned by the IEEE to Proxmox Server Solutions GmbH for a 24-bit large MAC block. You're allowed to use this in local networks, i.e., those not directly reachable by the public (e.g., in a LAN or behind NAT)."]
    #[doc = ""]
    #[doc = "Prefix for the auto-generated MAC addresses of virtual guests. The default `BC:24:11` is the Organizationally Unique Identifier (OUI) assigned by the IEEE to Proxmox Server Solutions GmbH for a MAC Address Block Large (MA-L). You're allowed to use this in local networks, i.e., those not directly reachable by the public (e.g., in a LAN or NAT/Masquerading)."]
    #[doc = ""]
    #[doc = "Note that when you run multiple cluster that (partially) share the networks of their virtual guests, it's highly recommended that you extend the default MAC prefix, or generate a custom (valid) one, to reduce the chance of MAC collisions. For example, add a separate extra hexadecimal to the Proxmox OUI for each cluster, like `BC:24:11:0` for the first, `BC:24:11:1` for the second, and so on."]
    #[doc = ""]
    #[doc = "Alternatively, you can also separate the networks of the guests logically, e.g., by using VLANs."]
    #[doc = ""]
    #[doc = "For publicly accessible guests it's recommended that you get your own https://standards.ieee.org/products-programs/regauth/\\\\[OUI from the IEEE\\\\] registered or coordinate with your, or your hosting providers, network admins."]
    #[doc = ""]
    pub mac_prefix: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Defines how many workers (per node) are maximal started  on actions like 'stopall VMs' or task from the ha-manager."]
    #[doc = ""]
    pub max_workers: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For cluster wide migration settings."]
    #[doc = ""]
    pub migration: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Migration is secure using SSH tunnel by default. For secure private networks you can disable it to speed up migration. Deprecated, use the 'migration' property instead!"]
    #[doc = ""]
    pub migration_unsecure: Option<bool>,
    #[serde(rename = "next-id")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Control the range for the free VMID auto-selection pool."]
    #[doc = ""]
    pub next_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Cluster-wide notification settings."]
    #[doc = ""]
    pub notify: Option<String>,
    #[serde(rename = "registered-tags")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of tags that require a `Sys.Modify` on '/' to set and delete. Tags set here that are also in 'user-tag-access' also require `Sys.Modify`."]
    #[doc = ""]
    pub registered_tags: Option<String>,
    #[serde(rename = "tag-style")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Tag style options."]
    #[doc = ""]
    pub tag_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "u2f"]
    #[doc = ""]
    pub u2f: Option<String>,
    #[serde(rename = "user-tag-access")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Privilege options for user-settable tags"]
    #[doc = ""]
    pub user_tag_access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "webauthn configuration"]
    #[doc = ""]
    pub webauthn: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Select the default Console viewer. You can either use the builtin java applet (VNC; deprecated and maps to html5), an external virt-viewer comtatible application (SPICE), an HTML5 based vnc viewer (noVNC), or an HTML5 based console client (xtermjs). If the selected viewer is not available (e.g. SPICE not activated for the VM), the fallback is noVNC."]
#[doc = ""]
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
impl TryFrom<&str> for Console {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "applet" => Ok(Self::Applet),
            "html5" => Ok(Self::Html5),
            "vv" => Ok(Self::Vv),
            "xtermjs" => Ok(Self::Xtermjs),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Set the fencing mode of the HA cluster. Hardware mode needs a valid configuration of fence devices in /etc/pve/ha/fence.cfg. With both all two modes are used."]
#[doc = ""]
#[doc = "WARNING: 'hardware' and 'both' are EXPERIMENTAL & WIP"]
#[doc = ""]
pub enum Fencing {
    #[serde(rename = "both")]
    Both,
    #[serde(rename = "hardware")]
    Hardware,
    #[serde(rename = "watchdog")]
    Watchdog,
}
impl TryFrom<&str> for Fencing {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "both" => Ok(Self::Both),
            "hardware" => Ok(Self::Hardware),
            "watchdog" => Ok(Self::Watchdog),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl Default for Fencing {
    fn default() -> Self {
        Self::Watchdog
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Default keybord layout for vnc server."]
#[doc = ""]
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
impl TryFrom<&str> for Keyboard {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "da" => Ok(Self::Da),
            "de" => Ok(Self::De),
            "de-ch" => Ok(Self::DeCh),
            "en-gb" => Ok(Self::EnGb),
            "en-us" => Ok(Self::EnUs),
            "es" => Ok(Self::Es),
            "fi" => Ok(Self::Fi),
            "fr" => Ok(Self::Fr),
            "fr-be" => Ok(Self::FrBe),
            "fr-ca" => Ok(Self::FrCa),
            "fr-ch" => Ok(Self::FrCh),
            "hu" => Ok(Self::Hu),
            "is" => Ok(Self::Is),
            "it" => Ok(Self::It),
            "ja" => Ok(Self::Ja),
            "lt" => Ok(Self::Lt),
            "mk" => Ok(Self::Mk),
            "nl" => Ok(Self::Nl),
            "no" => Ok(Self::No),
            "pl" => Ok(Self::Pl),
            "pt" => Ok(Self::Pt),
            "pt-br" => Ok(Self::PtBr),
            "sl" => Ok(Self::Sl),
            "sv" => Ok(Self::Sv),
            "tr" => Ok(Self::Tr),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Default GUI language."]
#[doc = ""]
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
impl TryFrom<&str> for Language {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "ar" => Ok(Self::Ar),
            "ca" => Ok(Self::Ca),
            "da" => Ok(Self::Da),
            "de" => Ok(Self::De),
            "en" => Ok(Self::En),
            "es" => Ok(Self::Es),
            "eu" => Ok(Self::Eu),
            "fa" => Ok(Self::Fa),
            "fr" => Ok(Self::Fr),
            "he" => Ok(Self::He),
            "hr" => Ok(Self::Hr),
            "it" => Ok(Self::It),
            "ja" => Ok(Self::Ja),
            "ka" => Ok(Self::Ka),
            "kr" => Ok(Self::Kr),
            "nb" => Ok(Self::Nb),
            "nl" => Ok(Self::Nl),
            "nn" => Ok(Self::Nn),
            "pl" => Ok(Self::Pl),
            "pt_BR" => Ok(Self::PtBR),
            "ru" => Ok(Self::Ru),
            "sl" => Ok(Self::Sl),
            "sv" => Ok(Self::Sv),
            "tr" => Ok(Self::Tr),
            "ukr" => Ok(Self::Ukr),
            "zh_CN" => Ok(Self::ZhCN),
            "zh_TW" => Ok(Self::ZhTW),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
