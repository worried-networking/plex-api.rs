pub mod client;
pub mod myplex;
pub mod server;

use httpmock::MockServer;
use http_adapter::Client;
use plex_api::HttpClientBuilder;
use rstest::fixture;
use std::ops::Deref;

pub struct Mocked<T> {
    inner: T,
    mock_server: MockServer,
}

impl<T> Mocked<T> {
    pub fn new(inner: T, mock_server: MockServer) -> Self {
        Self { inner, mock_server }
    }

    pub fn split(self) -> (T, MockServer) {
        (self.inner, self.mock_server)
    }
}

impl<T> Deref for Mocked<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[fixture]
pub fn mock_server() -> MockServer {
    MockServer::start()
}

#[fixture]
pub fn client_builder(mock_server: MockServer) -> Mocked<HttpClientBuilder> {
    #[cfg(feature = "http-client-isahc")]
    let http_client = {
        use isahc::config::Configurable;
        isahc::HttpClient::builder()
            // We're doing everything locally and using static mocks, no reasons to have big timeouts
            .timeout(std::time::Duration::from_secs(2))
            .connect_timeout(std::time::Duration::from_secs(1))
            // Normally Plex doesn't do redirects and None is the default value in our default client
            .redirect_policy(isahc::config::RedirectPolicy::None)
            // mockito does not support Expect-100 header, so we disabling it here
            .expect_continue(isahc::config::ExpectContinue::disabled())
            .build()
            .expect("failed to create testing http client");
        http_adapter_isahc::from_client(http_client)
    };
    
    #[cfg(all(feature = "http-client-reqwest", not(feature = "http-client-isahc")))]
    let http_client = {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(2))
            .connect_timeout(std::time::Duration::from_secs(1))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("failed to create testing http client");
        http_adapter_reqwest::from_client(client)
    };
    
    let client_builder = HttpClientBuilder::new(mock_server.base_url()).set_http_client(http_client);

    Mocked::new(client_builder, mock_server)
}
