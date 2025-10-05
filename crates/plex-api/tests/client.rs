mod fixtures;

mod offline {
    use super::fixtures::offline::mock_server;
    use httpmock::{Method::GET, MockServer};
    use http_adapter::Client;
    use plex_api::HttpClientBuilder;
    use std::time::Duration;

    #[plex_api_test_helper::offline_test]
    async fn default_client(mock_server: MockServer) {
        let client = HttpClientBuilder::new(mock_server.base_url())
            .build()
            .expect("failed to build default client");

        let m = mock_server.mock(|when, then| {
            when.method(GET)
                .path("/")
                .is_true(|req| {
                    let mut found = false;
                    for (header, _) in req.headers().iter() {
                        if header.as_str() == "x-plex-token" {
                            found = true;
                            break;
                        }
                    }

                    !found
                })
                .header_exists("X-Plex-Platform")
                .header_exists("X-Plex-Platform-Version")
                .header_exists("X-Plex-Device")
                .header_exists("X-Plex-Device-Name")
                .header_exists("X-Plex-Sync-Version")
                .header_exists("X-Plex-Client-Identifier")
                .header("X-Plex-Product", env!("CARGO_PKG_NAME"))
                .header("X-Plex-Version", env!("CARGO_PKG_VERSION"))
                .header("X-Plex-Provides", "controller");
            then.status(200).body("");
        });

        let get_result = client.get("/").send().await;

        m.assert();

        get_result.expect("failed to perform first http request");
    }

    #[plex_api_test_helper::offline_test]
    async fn customized_client(mock_server: MockServer) {
        let client = HttpClientBuilder::new(mock_server.base_url())
            .set_x_plex_token("auth_token".to_owned())
            .set_x_plex_client_identifier("client_id".to_owned())
            .set_x_plex_provides(&["provides1", "provides2"])
            .set_x_plex_platform("platform".to_owned())
            .set_x_plex_platform_version("platform_version".to_owned())
            .set_x_plex_product("product".to_owned())
            .set_x_plex_version("product_version".to_owned())
            .set_x_plex_device("device".to_owned())
            .set_x_plex_device_name("device_name".to_owned())
            .build()
            .expect("failed to build custom client");

        let m = mock_server.mock(|when, then| {
            when.method(GET)
                .path("/")
                .header("X-Plex-Provides", "provides1,provides2")
                .header("X-Plex-Platform", "platform")
                .header("X-Plex-Platform-Version", "platform_version")
                .header("X-Plex-Product", "product")
                .header("X-Plex-Version", "product_version")
                .header("X-Plex-Device", "device")
                .header("X-Plex-Device-Name", "device_name")
                .header("X-Plex-Sync-Version", "2")
                .header("X-Plex-Client-Identifier", "client_id")
                .header("X-Plex-Token", "auth_token");
            then.status(200).body("");
        });

        let get_result = client.get("/").send().await;

        m.assert();

        get_result.expect("failed to perform first http request");
    }

    #[plex_api_test_helper::offline_test]
    async fn auth_token_updated_after_build(mock_server: MockServer) {
        let client = HttpClientBuilder::new(mock_server.base_url())
            .build()
            .expect("failed to build default client")
            .set_x_plex_token("auth_token".to_owned());

        let m = mock_server.mock(|when, then| {
            when.method(GET)
                .path("/")
                .header_exists("X-Plex-Platform")
                .header_exists("X-Plex-Platform-Version")
                .header_exists("X-Plex-Device")
                .header_exists("X-Plex-Device-Name")
                .header_exists("X-Plex-Sync-Version")
                .header_exists("X-Plex-Client-Identifier")
                .header("X-Plex-Product", env!("CARGO_PKG_NAME"))
                .header("X-Plex-Version", env!("CARGO_PKG_VERSION"))
                .header("X-Plex-Provides", "controller")
                .header("X-Plex-Token", "auth_token");
            then.status(200).body("");
        });

        let get_result = client.get("/").send().await;

        m.assert();

        get_result.expect("failed to perform first http request");
    }

    #[plex_api_test_helper::offline_test]
    async fn custom_http_client(mock_server: MockServer) {
        #[cfg(feature = "http-client-isahc")]
        let http_client = {
            use isahc::config::Configurable as _;
            let client = isahc::HttpClient::builder()
                .timeout(Duration::from_secs(1))
                .connect_timeout(Duration::from_secs(1))
                .proxy(Some(mock_server.base_url().parse().unwrap()))
                .build()
                .expect("failed to create testing http client");
            http_adapter_isahc::from_client(client)
        };
        
        #[cfg(all(feature = "http-client-reqwest", not(feature = "http-client-isahc")))]
        let http_client = {
            let client = reqwest::Client::builder()
                .timeout(Duration::from_secs(1))
                .connect_timeout(Duration::from_secs(1))
                .proxy(reqwest::Proxy::all(mock_server.base_url()).expect("failed to create proxy"))
                .build()
                .expect("failed to create testing http client");
            http_adapter_reqwest::from_client(client)
        };

        let client = HttpClientBuilder::default()
            .set_http_client(http_client)
            .set_api_url("http://plex.tv")
            .set_x_plex_token("auth_token".to_owned())
            .set_x_plex_client_identifier("client_id".to_owned())
            .set_x_plex_provides(&["provides1", "provides2"])
            .set_x_plex_platform("platform".to_owned())
            .set_x_plex_platform_version("platform_version".to_owned())
            .set_x_plex_product("product".to_owned())
            .set_x_plex_version("product_version".to_owned())
            .set_x_plex_device("device".to_owned())
            .set_x_plex_device_name("device_name".to_owned())
            .build()
            .expect("failed to build client");

        let m = mock_server.mock(|when, then| {
            when.method(GET)
                .path("/index.html")
                .header_exists("Proxy-connection")
                .header("X-Plex-Provides", "provides1,provides2")
                .header("X-Plex-Platform", "platform")
                .header("X-Plex-Platform-Version", "platform_version")
                .header("X-Plex-Product", "product")
                .header("X-Plex-Version", "product_version")
                .header("X-Plex-Device", "device")
                .header("X-Plex-Device-Name", "device_name")
                .header("X-Plex-Sync-Version", "2")
                .header("X-Plex-Client-Identifier", "client_id")
                .header("X-Plex-Token", "auth_token");
            then.status(200).body("");
        });

        let get_result = client.get("/index.html").send().await;

        m.assert();

        get_result.expect("failed to perform first http request");
    }
}
