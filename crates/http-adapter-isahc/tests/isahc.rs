use std::error::Error;

use http_adapter::{http::StatusCode, HttpClientAdapter, Request};
use http_adapter_isahc::IsahcAdapter;
use httpmock::Method::GET;
use httpmock::MockServer;

#[tokio::test]
async fn executes_get_request() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET).path("/test");
        then.status(200).body("ok");
    });

    let client = IsahcAdapter::try_new()?;
    let request = Request::get(server.url("/test")).body(Vec::new())?;
    let response = client.execute(request).await?;

    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(b"ok", response.body().as_slice());

    mock.assert();
    Ok(())
}
