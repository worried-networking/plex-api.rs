use crate::{
    isahc_compat::StatusCodeExt, myplex::account::RestrictionProfile, url::MYPLEX_INVITES_FRIENDS,
    Error, HttpClient, Result,
};
use http::StatusCode;
use isahc::AsyncReadResponseExt;
use serde::{Deserialize, Serialize};
use serde_plain::derive_display_from_serialize;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum InviteStatus {
    PendingSent,
    Accepted,
    PendingReceived,
    Pending,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_display_from_serialize!(InviteStatus);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Friend {
    /// The global Plex.tv ID of the friend
    pub id: u64,
    /// Plex.tv user's uuid ??
    pub uuid: String,
    /// How the user should be displayed in the interface.
    /// Seems to be generated on the backed from either username or friendly_name.
    pub title: String,
    /// Non-managed user's username
    pub username: Option<String>,
    /// Non-managed user's email
    pub email: Option<String>,
    /// Is it a managed user?
    pub restricted: bool,
    /// Restrictions on the managed user (if any)
    pub restriction_profile: Option<RestrictionProfile>,
    /// Managed user's username
    pub friendly_name: Option<String>,
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub friendship_created_at: Option<OffsetDateTime>,
    /// User's avatar picture URL
    #[serde(with = "http_serde::uri")]
    pub thumb: http::Uri,
    /// Does the user has access to Plex Home?
    pub home: bool,
    /// Status of the friendship
    pub status: Option<InviteStatus>,
    /// Server sharing preferences
    pub sharing_settings: Option<super::server::Settings>,
    /// List of the shared servers
    #[serde(default)]
    pub shared_servers: Vec<super::server::SharedServer>,
    /// No idea what this is
    #[serde(default)]
    pub shared_sources: Vec<String>,
    #[serde(skip)]
    pub(crate) client: Option<HttpClient>,
}

impl Friend {
    /// Accept friendship.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn accept(self) -> Result<Friend> {
        if !matches!(
            self.status,
            Some(InviteStatus::PendingReceived) | Some(InviteStatus::Pending)
        ) {
            return Err(Error::InviteAcceptingNotPendingReceived);
        }

        let mut friend: Friend = self
            .client()
            .post(format!("{}/{}/accept", MYPLEX_INVITES_FRIENDS, self.id))
            .json()
            .await?;
        friend.client.clone_from(&self.client);
        Ok(friend)
    }

    /// Shorthand for self.client.as_ref().unwrap(). Unwrap must be safe at this stage
    /// since the client must be set for a valid invite.
    fn client(&self) -> &HttpClient {
        self.client.as_ref().unwrap()
    }

    /// Delete the friend or friendship request.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn delete(self) -> Result<()> {
        let mut response = self
            .client()
            .delete(format!("{}/{}", MYPLEX_INVITES_FRIENDS, self.id))
            .send()
            .await?;

        match response.status().as_http_status() {
            StatusCode::OK | StatusCode::NO_CONTENT => {
                response.consume().await?;
                Ok(())
            }
            _ => Err(Error::from_response(response).await),
        }
    }
}
