use secrecy::SecretString;
use serde::Deserialize;
use serde_plain::derive_fromstr_from_deserialize;
use serde_with::{formats::CommaSeparator, serde_as, NoneAsEmptyString, StringWithSeparator};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct DevicesMediaContainer {
    #[serde(rename = "Device", default)]
    pub devices: Vec<Device>,

    // TODO: The rest of the fields should belong to MediaContainer struct, but
    // quick-qml (and serde-xml-rs) has some issues with `flatten`.
    // https://github.com/tafia/quick-xml/issues/226
    #[serde(rename = "@size", default)]
    pub size: Option<i32>,

    #[serde(rename = "@publicAddress")]
    pub public_address: Option<String>,
    // #[serde(flatten)]
    // pub media_container: MediaContainer,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Device {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@product")]
    pub product: String,
    #[serde(rename = "@publicAddress")]
    pub public_address: String,
    #[serde(rename = "@productVersion")]
    pub product_version: String,
    #[serde(rename = "@platform")]
    pub platform: String,
    #[serde(rename = "@platformVersion")]
    pub platform_version: String,
    #[serde(rename = "@device")]
    pub device: String,
    #[serde(rename = "@model")]
    pub model: Option<String>,
    #[serde(rename = "@vendor")]
    pub vendor: Option<String>,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, Feature>")]
    #[serde(rename = "@provides")]
    pub provides: Vec<Feature>,
    #[serde(rename = "@clientIdentifier")]
    pub client_identifier: String,
    #[serde(rename = "@version")]
    pub version: Option<String>,
    #[serde(rename = "@id")]
    pub id: Option<u32>,
    #[serde(rename = "@token")]
    pub token: Option<SecretString>,
    #[serde(rename = "@accessToken")]
    pub access_token: Option<SecretString>,
    #[serde(with = "time::serde::timestamp", rename = "@createdAt")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::timestamp", rename = "@lastSeenAt")]
    pub last_seen_at: OffsetDateTime,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    #[serde(default, rename = "@screenResolution")]
    pub screen_resolution: Vec<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default, rename = "@screenDensity")]
    pub screen_density: Option<u16>,
    #[serde(rename = "Connection", default)]
    pub connections: Vec<Connection>,
    #[serde(rename = "@httpsRequired")]
    pub https_required: Option<bool>,
    #[serde(rename = "@synced")]
    pub synced: Option<bool>,
    #[serde(rename = "@relay")]
    pub relay: Option<bool>,
    #[serde(rename = "@publicAddressMatches")]
    pub public_address_matches: Option<bool>,
    #[serde(rename = "@presence")]
    pub presence: Option<bool>,
    #[serde(rename = "@owned")]
    pub owned: Option<bool>,
    #[serde(rename = "@ownerId")]
    pub owner_id: Option<u64>,
    #[serde(rename = "@home")]
    pub home: Option<bool>,
    #[serde(rename = "@sourceTitle")]
    pub source_title: Option<String>,
    #[serde(rename = "SyncList")]
    pub sync_list: Option<SyncList>,
    #[serde(default = "create_empty_secret_string", rename = "@authToken")]
    pub auth_token: SecretString,
    #[serde(rename = "@dnsRebindingProtection")]
    pub dns_rebinding_protection: Option<bool>,
    #[serde(rename = "@natLoopbackSupported")]
    pub nat_loopback_supported: Option<bool>,
    #[serde(rename = "@searchEnabled")]
    pub search_enabled: Option<bool>,
}

fn create_empty_secret_string() -> SecretString {
    SecretString::new("".into())
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct SyncList {
    #[serde(rename = "@itemsCompleteCount")]
    pub items_complete_count: u32,
    #[serde(rename = "@totalSize")]
    pub total_size: u64,
    #[serde(rename = "@version")]
    pub version: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Connection {
    #[serde(with = "http_serde::uri", rename = "@uri")]
    pub uri: http::Uri,
    #[serde(rename = "@protocol")]
    pub protocol: Option<String>,
    #[serde(rename = "@address")]
    pub address: Option<String>,
    #[serde(rename = "@port")]
    pub port: Option<u32>,
    #[serde(rename = "@local")]
    pub local: Option<bool>,
    #[serde(rename = "@relay")]
    pub relay: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Feature {
    Server,
    Client,
    Controller,
    Player,
    PubsubPlayer,
    ProviderPlayback,
    SyncTarget,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(Feature);
