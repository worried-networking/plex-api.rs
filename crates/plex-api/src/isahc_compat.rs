use http::StatusCode;
use isahc::http as isahc_http;

pub(crate) trait StatusCodeExt {
    fn as_http_status(&self) -> StatusCode;
}

impl StatusCodeExt for isahc_http::StatusCode {
    fn as_http_status(&self) -> StatusCode {
        // `StatusCode::from_u16` only fails for values outside the valid HTTP
        // status code range. `isahc` already validates status codes before
        // constructing them, so this conversion cannot fail in practice.
        StatusCode::from_u16(self.as_u16()).expect("isahc provided an invalid HTTP status code")
    }
}
