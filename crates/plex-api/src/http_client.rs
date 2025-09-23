use crate::{isahc_compat::StatusCodeExt, url::MYPLEX_DEFAULT_API_URL, Result};
use http::{uri::PathAndQuery, StatusCode, Uri};
use isahc::{
    config::{Configurable, RedirectPolicy},
    http::{request::Builder, HeaderValue as IsahcHeaderValue},
    AsyncBody, AsyncReadResponseExt, HttpClient as IsahcHttpClient, Request as HttpRequest,
    Response as HttpResponse,
};
use secrecy::{ExposeSecret, SecretString};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use uuid::Uuid;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone)]
pub struct HttpClient {
    pub api_url: Uri,

    pub http_client: IsahcHttpClient,

    /// `X-Plex-Provides` header value. Comma-separated list.
    ///
    /// Should be one or more of `controller`, `server`, `sync-target`, `player`.
    pub x_plex_provides: String,

    /// `X-Plex-Platform` header value.
    ///
    /// Platform name, e.g. iOS, macOS, etc.
    pub x_plex_platform: String,

    /// `X-Plex-Platform-Version` header value.
    ///
    /// OS version, e.g. 4.3.1
    pub x_plex_platform_version: String,

    /// `X-Plex-Product` header value.
    ///
    /// Application name, e.g. Laika, Plex Media Server, Media Link.
    pub x_plex_product: String,

    /// `X-Plex-Version` header value.
    ///
    /// Application version, e.g. 10.6.7.
    pub x_plex_version: String,

    /// `X-Plex-Device` header value.
    ///
    /// Device name and model number, e.g. iPhone3,2, Motorola XOOM™, LG5200TV.
    pub x_plex_device: String,

    /// `X-Plex-Device-Name` header value.
    ///
    /// Primary name for the device, e.g. "Plex Web (Chrome)".
    pub x_plex_device_name: String,

    /// `X-Plex-Client-Identifier` header value.
    ///
    /// UUID, serial number, or other number unique per device.
    ///
    /// **N.B.** Should be unique for each of your devices.
    pub x_plex_client_identifier: String,

    /// `X-Plex-Token` header value.
    ///
    /// Auth token for Plex.
    x_plex_token: SecretString,

    /// `X-Plex-Sync-Version` header value.
    ///
    /// Not sure what are the valid values, but at the time of writing Plex Web sends `2` here.
    pub x_plex_sync_version: String,

    /// `X-Plex-Model` header value.
    ///
    /// Plex Web sends `hosted`
    pub x_plex_model: String,

    /// `X-Plex-Features` header value.
    ///
    /// Looks like it's a replacement for X-Plex-Provides
    pub x_plex_features: String,

    /// `X-Plex-Target-Client-Identifier` header value.
    ///
    /// Used when proxying a client request via a server.
    pub x_plex_target_client_identifier: String,
}

impl HttpClient {
    fn prepare_request(&self) -> Builder {
        self.prepare_request_min()
            .header("X-Plex-Provides", &self.x_plex_provides)
            .header("X-Plex-Platform", &self.x_plex_platform)
            .header("X-Plex-Platform-Version", &self.x_plex_platform_version)
            .header("X-Plex-Product", &self.x_plex_product)
            .header("X-Plex-Version", &self.x_plex_version)
            .header("X-Plex-Device", &self.x_plex_device)
            .header("X-Plex-Device-Name", &self.x_plex_device_name)
            .header("X-Plex-Sync-Version", &self.x_plex_sync_version)
            .header("X-Plex-Model", &self.x_plex_model)
            .header("X-Plex-Features", &self.x_plex_features)
    }

    fn prepare_request_min(&self) -> Builder {
        let mut request = HttpRequest::builder()
            .header("X-Plex-Client-Identifier", &self.x_plex_client_identifier);

        if !self.x_plex_target_client_identifier.is_empty() {
            request = request.header(
                "X-Plex-Target-Client-Identifier",
                &self.x_plex_target_client_identifier,
            );
        }

        if !self.x_plex_token.expose_secret().is_empty() {
            request = request.header("X-Plex-Token", self.x_plex_token.expose_secret());
        }

        request
    }

    /// Verifies that this client has an authentication token.
    pub fn is_authenticated(&self) -> bool {
        !self.x_plex_token.expose_secret().is_empty()
    }

    /// Begins building a request using the HTTP POST method.
    pub fn post<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request().method("POST"),
            timeout: Some(DEFAULT_TIMEOUT),
        }
    }

    /// Does the same as HttpClient::post(), but appends only bare minimum
    /// headers: `X-Plex-Client-Identifier` and `X-Plex-Token`.
    pub fn postm<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request_min().method("POST"),
            timeout: Some(DEFAULT_TIMEOUT),
        }
    }

    /// Begins building a request using the HTTP HEAD method.
    pub fn head<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request().method("HEAD"),
            timeout: Some(DEFAULT_TIMEOUT),
        }
    }

    /// Begins building a request using the HTTP GET method.
    pub fn get<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request().method("GET"),
            timeout: Some(DEFAULT_TIMEOUT),
        }
    }

    /// Does the same as HttpClient::get(), but appends only bare minimum
    /// headers: `X-Plex-Client-Identifier` and `X-Plex-Token`.
    pub fn getm<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request_min().method("GET"),
            timeout: Some(DEFAULT_TIMEOUT),
        }
    }

    /// Begins building a request using the HTTP PUT method.
    pub fn put<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request().method("PUT"),
            timeout: Some(DEFAULT_TIMEOUT),
        }
    }

    /// Does the same as HttpClient::put(), but appends only bare minimum
    /// headers: `X-Plex-Client-Identifier` and `X-Plex-Token`.
    pub fn putm<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request_min().method("PUT"),
            timeout: Some(DEFAULT_TIMEOUT),
        }
    }

    /// Begins building a request using the HTTP DELETE method.
    pub fn delete<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request().method("DELETE"),
            timeout: Some(DEFAULT_TIMEOUT),
        }
    }

    /// Does the same as HttpClient::delete(), but appends only bare minimum
    /// headers: `X-Plex-Client-Identifier` and `X-Plex-Token`.
    pub fn deletem<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request_min().method("DELETE"),
            timeout: Some(DEFAULT_TIMEOUT),
        }
    }

    /// Set the client's authentication token.
    pub fn set_x_plex_token<T>(self, x_plex_token: T) -> Self
    where
        T: Into<SecretString>,
    {
        Self {
            x_plex_token: x_plex_token.into(),
            ..self
        }
    }

    /// Get a reference to the client's authentication token.
    pub fn x_plex_token(&self) -> &str {
        self.x_plex_token.expose_secret()
    }
}

impl From<&HttpClient> for HttpClient {
    fn from(value: &HttpClient) -> Self {
        value.to_owned()
    }
}

pub struct RequestBuilder<'a, P>
where
    PathAndQuery: TryFrom<P>,
    <PathAndQuery as TryFrom<P>>::Error: Into<http::Error>,
{
    http_client: &'a IsahcHttpClient,
    base_url: Uri,
    path_and_query: P,
    request_builder: Builder,
    timeout: Option<Duration>,
}

impl<'a, P> RequestBuilder<'a, P>
where
    PathAndQuery: TryFrom<P>,
    <PathAndQuery as TryFrom<P>>::Error: Into<http::Error>,
{
    /// Sets the maximum timeout for this request or disables timeouts.
    #[must_use]
    pub fn timeout(self, timeout: Option<Duration>) -> Self {
        Self {
            http_client: self.http_client,
            base_url: self.base_url,
            path_and_query: self.path_and_query,
            request_builder: self.request_builder,
            timeout,
        }
    }

    /// Adds a body to the request.
    pub fn body<B>(self, body: B) -> Result<Request<'a, B>>
    where
        B: Into<AsyncBody>,
    {
        let path_and_query = PathAndQuery::try_from(self.path_and_query).map_err(Into::into)?;
        let mut uri_parts = self.base_url.into_parts();
        uri_parts.path_and_query = Some(path_and_query);
        let uri = Uri::from_parts(uri_parts).map_err(Into::<http::Error>::into)?;
        let uri_string = uri.to_string();

        let mut builder = self.request_builder.uri(uri_string);
        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }

        Ok(Request {
            http_client: self.http_client,
            request: builder.body(body)?,
        })
    }

    /// Serializes the provided struct as json and adds it as a body for the request.
    /// Header "Content-type: application/json" will be added along the way.
    pub fn json_body<B>(self, body: &B) -> Result<Request<'a, String>>
    where
        B: ?Sized + Serialize,
    {
        self.header("Content-type", "application/json")
            .body(serde_json::to_string(body)?)
    }

    /// Adds a form encoded parameters to the request body.
    pub fn form(self, params: &[(&str, &str)]) -> Result<Request<'a, String>> {
        let body = serde_urlencoded::to_string(params)?;
        self.header("Content-type", "application/x-www-form-urlencoded")
            .header("Content-Length", body.len().to_string())
            .body(body)
    }

    /// Adds a request header.
    #[must_use]
    pub fn header<K, V>(self, key: K, value: V) -> Self
    where
        isahc::http::header::HeaderName: TryFrom<K>,
        <isahc::http::header::HeaderName as TryFrom<K>>::Error: Into<isahc::http::Error>,
        isahc::http::header::HeaderValue: TryFrom<V>,
        <isahc::http::header::HeaderValue as TryFrom<V>>::Error: Into<isahc::http::Error>,
    {
        Self {
            http_client: self.http_client,
            base_url: self.base_url,
            path_and_query: self.path_and_query,
            request_builder: self.request_builder.header(key, value),
            timeout: self.timeout,
        }
    }

    /// Sends this request generating a response.
    pub async fn send(self) -> Result<HttpResponse<AsyncBody>> {
        self.body(())?.send().await
    }

    /// Sends this request and attempts to decode the response as JSON.
    pub async fn json<T: DeserializeOwned + Unpin>(self) -> Result<T> {
        self.body(())?.json().await
    }

    /// Sends this request and attempts to decode the response as XML.
    pub async fn xml<T: DeserializeOwned + Unpin>(self) -> Result<T> {
        self.body(())?.xml().await
    }

    /// Sends this request, verifies success and then consumes any response.
    pub async fn consume(self) -> Result<()> {
        let mut response = self.header("Accept", "application/json").send().await?;

        match response.status().as_http_status() {
            StatusCode::OK => {
                response.consume().await?;
                Ok(())
            }
            _ => Err(crate::Error::from_response(response).await),
        }
    }
}

pub struct Request<'a, T> {
    http_client: &'a IsahcHttpClient,
    request: HttpRequest<T>,
}

impl<'a, T> Request<'a, T>
where
    T: Into<AsyncBody>,
{
    /// Sends this request generating a response.
    pub async fn send(self) -> Result<HttpResponse<AsyncBody>> {
        Ok(self.http_client.send_async(self.request).await?)
    }

    /// Sends this request and attempts to decode the response as JSON.
    pub async fn json<R: DeserializeOwned + Unpin>(mut self) -> Result<R> {
        let headers = self.request.headers_mut();
        headers.insert("Accept", IsahcHeaderValue::from_static("application/json"));

        let mut response = self.send().await?;

        match response.status().as_http_status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => {
                let body = response.text().await?;
                match serde_json::from_str(&body) {
                    Ok(response) => Ok(response),
                    Err(error) => {
                        #[cfg(feature = "tests_deny_unknown_fields")]
                        // We're in tests, so it's fine to print
                        #[allow(clippy::print_stdout)]
                        {
                            println!("Received body: {body}");
                        }
                        Err(error.into())
                    }
                }
            }
            _ => Err(crate::Error::from_response(response).await),
        }
    }

    /// Sends this request and attempts to decode the response as XML.
    pub async fn xml<R: DeserializeOwned + Unpin>(mut self) -> Result<R> {
        let headers = self.request.headers_mut();
        headers.insert("Accept", IsahcHeaderValue::from_static("application/xml"));

        let mut response = self.send().await?;

        match response.status().as_http_status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => {
                let body = response.text().await?;
                match quick_xml::de::from_str(&body) {
                    Ok(response) => Ok(response),
                    Err(error) => {
                        #[cfg(feature = "tests_deny_unknown_fields")]
                        // We're in tests, so it's fine to print
                        #[allow(clippy::print_stdout)]
                        {
                            println!("Received body: {body}");
                        }
                        Err(error.into())
                    }
                }
            }
            _ => Err(crate::Error::from_response(response).await),
        }
    }
}

pub struct HttpClientBuilder {
    client: Result<HttpClient>,
}

impl Default for HttpClientBuilder {
    fn default() -> Self {
        let sys_platform = sysinfo::System::name().unwrap_or("unknown".to_string());
        let sys_version = sysinfo::System::os_version().unwrap_or("unknown".to_string());
        let sys_hostname = sysinfo::System::host_name().unwrap_or("unknown".to_string());

        let random_uuid = Uuid::new_v4();

        let client = HttpClient {
            api_url: Uri::from_static(MYPLEX_DEFAULT_API_URL),
            http_client: IsahcHttpClient::builder()
                .connect_timeout(DEFAULT_CONNECTION_TIMEOUT)
                .redirect_policy(RedirectPolicy::None)
                .build()
                .expect("failed to create default http client"),
            x_plex_provides: String::from("controller"),
            x_plex_product: option_env!("CARGO_PKG_NAME")
                .unwrap_or("plex-api")
                .to_string(),
            x_plex_platform: sys_platform.clone(),
            x_plex_platform_version: sys_version,
            x_plex_version: option_env!("CARGO_PKG_VERSION")
                .unwrap_or("unknown")
                .to_string(),
            x_plex_device: sys_platform,
            x_plex_device_name: sys_hostname,
            x_plex_client_identifier: random_uuid.to_string(),
            x_plex_sync_version: String::from("2"),
            x_plex_token: SecretString::new("".into()),
            x_plex_model: String::from("hosted"),
            x_plex_features: String::from("external-media,indirect-media,hub-style-list"),
            x_plex_target_client_identifier: String::from(""),
        };

        Self { client: Ok(client) }
    }
}

impl HttpClientBuilder {
    /// Creates a client that maps to Plex's Generic profile which has no
    /// particular settings defined for transcoding.
    pub fn generic() -> Self {
        Self::default().set_x_plex_platform("Generic")
    }

    pub fn build(self) -> Result<HttpClient> {
        self.client
    }

    pub fn set_http_client(self, http_client: IsahcHttpClient) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.http_client = http_client;
                client
            }),
        }
    }

    pub fn from(client: HttpClient) -> Self {
        Self { client: Ok(client) }
    }

    pub fn new<U>(api_url: U) -> Self
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        Self::default().set_api_url(api_url)
    }

    pub fn set_api_url<U>(self, api_url: U) -> Self
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        Self {
            client: self.client.and_then(move |mut client| {
                client.api_url = Uri::try_from(api_url).map_err(Into::into)?;
                Ok(client)
            }),
        }
    }

    pub fn set_x_plex_token<S: Into<SecretString>>(self, token: S) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_token = token.into();
                client
            }),
        }
    }

    pub fn set_x_plex_client_identifier<S: Into<String>>(self, client_identifier: S) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_client_identifier = client_identifier.into();
                client
            }),
        }
    }

    pub fn set_x_plex_provides(self, x_plex_provides: &[&str]) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_provides = x_plex_provides.join(",");
                client
            }),
        }
    }

    pub fn set_x_plex_platform<S: Into<String>>(self, platform: S) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_platform = platform.into();
                client
            }),
        }
    }

    pub fn set_x_plex_platform_version<S: Into<String>>(self, platform_version: S) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_platform_version = platform_version.into();
                client
            }),
        }
    }

    pub fn set_x_plex_product<S: Into<String>>(self, product: S) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_product = product.into();
                client
            }),
        }
    }

    pub fn set_x_plex_version<S: Into<String>>(self, version: S) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_version = version.into();
                client
            }),
        }
    }

    pub fn set_x_plex_device<S: Into<String>>(self, device: S) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_device = device.into();
                client
            }),
        }
    }

    pub fn set_x_plex_device_name<S: Into<String>>(self, device_name: S) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_device_name = device_name.into();
                client
            }),
        }
    }

    pub fn set_x_plex_model<S: Into<String>>(self, model: S) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_model = model.into();
                client
            }),
        }
    }

    pub fn set_x_plex_features(self, features: &[&str]) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_features = features.join(",");
                client
            }),
        }
    }
}
