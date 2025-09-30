use crate::{
    isahc_compat::StatusCodeExt,
    url::{MYPLEX_PINS, MYPLEX_PINS_LINK},
    Error, HttpClient, Result,
};
use http::StatusCode;
use isahc::AsyncReadResponseExt;
use serde::Deserialize;
use time::OffsetDateTime;

pub struct PinManager {
    client: HttpClient,
}

impl PinManager {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn link(&self, code: &str) -> Result {
        if !self.client.is_authenticated() {
            return Err(Error::ClientNotAuthenticated);
        }

        let response = self
            .client
            .putm(MYPLEX_PINS_LINK)
            .header("X-Plex-Product", "Plex SSO")
            .form(&[("code", code)])?
            .send()
            .await?;

        if response.status().as_http_status() == StatusCode::NO_CONTENT {
            Ok(())
        } else {
            Err(Error::from_response(response).await)
        }
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn pin(&self) -> Result<Pin<'_>> {
        if self.client.is_authenticated() {
            return Err(Error::ClientAuthenticated);
        }

        let mut response = self
            .client
            .post(MYPLEX_PINS)
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status().as_http_status() == StatusCode::CREATED {
            let pin = response.json::<PinInfo>().await?;
            Ok(Pin {
                client: &self.client,
                pin,
            })
        } else {
            Err(Error::from_response(response).await)
        }
    }
}

#[derive(Debug)]
pub struct Pin<'a> {
    client: &'a HttpClient,
    pub pin: PinInfo,
}

impl<'a> Pin<'a> {
    /// Returns the code that should be displayed to a user.
    pub fn code(&self) -> &str {
        &self.pin.code
    }

    /// Checks if the pin is still valid.
    pub fn is_expired(&self) -> bool {
        self.pin.expires_at < OffsetDateTime::now_utc()
    }

    /// Check if the pin was linked by a user.
    #[tracing::instrument(level = "debug", skip(self), fields(self.pin.id = self.pin.id))]
    pub async fn check(&self) -> Result<PinInfo> {
        if self.is_expired() {
            return Err(Error::PinExpired);
        }

        let url = format!("{}/{}", MYPLEX_PINS, self.pin.id);
        let pin: PinInfo = self.client.get(url).json().await?;

        if pin.auth_token.is_some() {
            Ok(pin)
        } else {
            Err(Error::PinNotLinked)
        }
    }
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct PinInfo {
    pub id: u32,
    pub code: String,
    pub product: String,
    pub trusted: bool,
    pub client_identifier: String,
    pub location: Location,
    pub expires_in: u32,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub expires_at: OffsetDateTime,
    pub auth_token: Option<String>,
    pub new_registration: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Location {
    pub code: String,
    pub european_union_member: bool,
    pub continent_code: String,
    pub country: String,
    pub city: String,
    pub time_zone: String,
    pub postal_code: String,
    pub in_privacy_restricted_country: bool,
    pub subdivisions: String,
    pub coordinates: String,
}
