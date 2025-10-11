//! # HTTP adapter implementation for [`isahc`](https://crates.io/crates/isahc)
//!
//! For more details refer to [`http-adapter`](https://crates.io/crates/http-adapter)

use core::fmt::Debug;
use std::io;

pub use isahc;

use http_adapter::async_trait::async_trait;
use http_adapter::http::{self as http1, Request, Response};
use http_adapter::HttpClientAdapter;
use isahc::http as isahc_http;
use isahc::AsyncReadResponseExt;

#[derive(Clone, Debug)]
pub struct IsahcAdapter {
    client: isahc::HttpClient,
}

impl IsahcAdapter {
    #[must_use]
    pub fn new(client: isahc::HttpClient) -> Self {
        Self { client }
    }

    pub fn try_new() -> Result<Self, isahc::Error> {
        isahc::HttpClient::new().map(Self::new)
    }
}

fn to_isahc_request(
    request: Request<Vec<u8>>,
) -> Result<isahc_http::Request<Vec<u8>>, isahc::Error> {
    let (parts, body) = request.into_parts();
    let mut builder = isahc_http::Request::builder()
        .method(parts.method.as_str())
        .version(to_isahc_version(parts.version))
        .uri(parts.uri.to_string());

    if let Some(headers) = builder.headers_mut() {
        *headers = to_isahc_headers(parts.headers)?;
    }

    builder.body(body).map_err(isahc::Error::from)
}

fn to_isahc_headers(headers: http1::HeaderMap) -> Result<isahc_http::HeaderMap, isahc::Error> {
    let mut converted = isahc_http::HeaderMap::with_capacity(headers.len());
    let mut current_name = None;

    for (name, value) in headers.into_iter() {
        if let Some(name) = name {
            current_name = Some(name.as_str().to_owned());
        }

        let header_name = current_name.as_ref().ok_or_else(|| {
            isahc::Error::from(io::Error::new(
                io::ErrorKind::InvalidData,
                "missing header name",
            ))
        })?;

        let name = isahc_http::HeaderName::from_bytes(header_name.as_bytes())
            .map_err(|error| isahc::Error::from(isahc_http::Error::from(error)))?;
        let value = isahc_http::HeaderValue::from_bytes(value.as_bytes())
            .map_err(|error| isahc::Error::from(isahc_http::Error::from(error)))?;
        converted.append(name, value);
    }

    Ok(converted)
}

fn to_http_headers(headers: isahc_http::HeaderMap) -> Result<http1::HeaderMap, isahc::Error> {
    let mut converted = http1::HeaderMap::with_capacity(headers.len());
    let mut current_name = None;

    for (name, value) in headers.into_iter() {
        if let Some(name) = name {
            current_name = Some(name.as_str().to_owned());
        }

        let header_name = current_name.as_ref().ok_or_else(|| {
            isahc::Error::from(io::Error::new(
                io::ErrorKind::InvalidData,
                "missing header name",
            ))
        })?;

        let name = http1::HeaderName::from_bytes(header_name.as_bytes()).map_err(|error| {
            isahc::Error::from(io::Error::new(io::ErrorKind::InvalidData, error))
        })?;
        let value = http1::HeaderValue::from_bytes(value.as_bytes()).map_err(|error| {
            isahc::Error::from(io::Error::new(io::ErrorKind::InvalidData, error))
        })?;
        converted.append(name, value);
    }

    Ok(converted)
}

fn to_http_version(version: isahc_http::Version) -> http1::Version {
    match version {
        isahc_http::Version::HTTP_09 => http1::Version::HTTP_09,
        isahc_http::Version::HTTP_10 => http1::Version::HTTP_10,
        isahc_http::Version::HTTP_11 => http1::Version::HTTP_11,
        isahc_http::Version::HTTP_2 => http1::Version::HTTP_2,
        isahc_http::Version::HTTP_3 => http1::Version::HTTP_3,
        _ => http1::Version::HTTP_11,
    }
}

fn to_isahc_version(version: http1::Version) -> isahc_http::Version {
    match version {
        http1::Version::HTTP_09 => isahc_http::Version::HTTP_09,
        http1::Version::HTTP_10 => isahc_http::Version::HTTP_10,
        http1::Version::HTTP_11 => isahc_http::Version::HTTP_11,
        http1::Version::HTTP_2 => isahc_http::Version::HTTP_2,
        http1::Version::HTTP_3 => isahc_http::Version::HTTP_3,
        _ => isahc_http::Version::HTTP_11,
    }
}

async fn to_response(
    mut response: isahc::Response<isahc::AsyncBody>,
) -> Result<Response<Vec<u8>>, isahc::Error> {
    let body = response.bytes().await.map_err(isahc::Error::from)?;
    let (parts, _) = response.into_parts();

    let status = http1::StatusCode::from_u16(parts.status.as_u16())
        .map_err(|error| isahc::Error::from(io::Error::new(io::ErrorKind::InvalidData, error)))?;

    let mut builder = Response::builder()
        .status(status)
        .version(to_http_version(parts.version));

    if let Some(headers) = builder.headers_mut() {
        *headers = to_http_headers(parts.headers)?;
    }

    builder
        .body(body)
        .map_err(|error| isahc::Error::from(io::Error::new(io::ErrorKind::InvalidData, error)))
}

#[async_trait]
impl HttpClientAdapter for IsahcAdapter {
    type Error = isahc::Error;

    async fn execute(&self, request: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Self::Error> {
        let request = to_isahc_request(request)?;
        let response = self.client.send_async(request).await?;
        to_response(response).await
    }
}
