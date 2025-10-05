use crate::{
    http_client::HttpClient, url::MYPLEX_WEBHOOKS_PATH, Error, Result,
};
use http::{StatusCode, Uri};
use serde::Deserialize;
use std::fmt::Debug;

pub struct WebhookManager {
    webhooks: Vec<Webhook>,
    client: HttpClient,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Webhook {
    #[serde(with = "http_serde::uri")]
    pub url: Uri,
}

impl WebhookManager {
    #[tracing::instrument(level = "debug", skip(client))]
    pub async fn new(client: HttpClient) -> Result<Self> {
        let mut ret = Self {
            webhooks: vec![],
            client,
        };
        ret.refresh().await?;
        Ok(ret)
    }

    /// Reload the configured webhooks list.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn refresh(&mut self) -> Result {
        self.webhooks = self.client.get(MYPLEX_WEBHOOKS_PATH).json().await?;
        Ok(())
    }

    /// List the current webhooks list.
    pub fn webhooks(&self) -> &[Webhook] {
        &self.webhooks
    }

    /// Set the list of webhook to the provided list.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn set(&mut self, webhooks: Vec<Webhook>) -> Result {
        let webhook_urls = webhooks
            .iter()
            .map(|webhook| webhook.url.to_string())
            .collect::<Vec<_>>();

        let params = if webhooks.is_empty() {
            vec![("urls", "")]
        } else {
            webhook_urls
                .iter()
                .map(|webhook_url| ("urls[]", webhook_url.as_str()))
                .collect()
        };

        let response = self
            .client
            .post(MYPLEX_WEBHOOKS_PATH)
            .form(&params)?
            .send()
            .await?;
        if response.status() == StatusCode::CREATED {
            self.webhooks = webhooks;
            Ok(())
        } else {
            Err(Error::from_response(response).await)
        }
    }

    /// Add a new webhook with the provided URL.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add<U>(&mut self, url: U) -> Result
    where
        U: Debug,
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        let mut webhooks = self.webhooks.clone();
        webhooks.push(Webhook {
            url: Uri::try_from(url).map_err(Into::into)?,
        });
        self.set(webhooks).await
    }

    /// Delete the webhook with the provided URL.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn delete<U>(&mut self, url: U) -> Result
    where
        U: Debug,
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        let original_len = self.webhooks.len();
        let webhook_url = Uri::try_from(url).map_err(Into::into)?;
        let new_webhooks: Vec<Webhook> = self
            .webhooks
            .clone()
            .into_iter()
            .filter(|value| value.url != webhook_url)
            .collect();
        if original_len == new_webhooks.len() {
            Err(Error::WebhookNotFound(format!("{webhook_url}")))
        } else {
            self.set(new_webhooks).await
        }
    }
}
