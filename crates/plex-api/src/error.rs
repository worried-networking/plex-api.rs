use crate::media_container::server::Feature;
use http_adapter::Body;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Authenticated client must be provided.")]
    ClientNotAuthenticated,
    #[error("Non-authenticated client must be provided.")]
    ClientAuthenticated,
    #[error("Unable to deserialize JSON: {source}.")]
    JsonDeserealiseError {
        #[from]
        source: serde_json::Error,
    },
    #[error("Unable to deserialize XML: {source}.")]
    XmlDeserealiseError {
        #[from]
        source: quick_xml::de::DeError,
    },
    #[error("{source}")]
    UrlencodingError {
        #[from]
        source: serde_urlencoded::ser::Error,
    },
    #[error("{source}")]
    HttpError {
        #[from]
        source: http::Error,
    },
    #[error("{0}")]
    HttpAdapterError(#[from] http_adapter::Error),
    #[error("{source}")]
    StdIoError {
        #[from]
        source: std::io::Error,
    },
    #[error("{source}")]
    Utf8Error {
        #[from]
        source: std::string::FromUtf8Error,
    },
    #[error("Error while communicating with MyPlexApi: {errors:?}.")]
    MyPlexErrorResponse { errors: Vec<Self> },
    #[error("Error occurred while communicating to MyPlex API: #{code} - {message}.")]
    MyPlexApiError { code: i32, message: String },
    #[error("Failed to get claim token: {0}.")]
    FailedToGetClaimToken(String),
    #[error("Unexpected API response: HTTP {status_code}, content: {content}.")]
    UnexpectedApiResponse { status_code: u16, content: String },
    #[error("The requested webhook wasn't found: {0}.")]
    WebhookNotFound(String),
    #[error("The mandatory feature is not available: {0}.")]
    SubscriptionFeatureNotAvailable(Feature),
    #[error("OTP is required for the authentication.")]
    OtpRequired,
    #[error("OTP is provided, but no username/password.")]
    UselessOtp,
    #[error("Connecting to the device is not supported.")]
    DeviceConnectionNotSupported,
    #[error("Device doesn't have any exposed connection endpoints.")]
    DeviceConnectionsIsEmpty,
    #[error("Requested unknown setting: {0}.")]
    RequestedSettingNotFound(String),
    #[error("You can't set setting to a value of a different type.")]
    IncompatibleSettingValues,
    #[error("Provided pin is already expired.")]
    PinExpired,
    #[error("Provided pin is not linked yet.")]
    PinNotLinked,
    #[error("Item requested was not found on the server.")]
    ItemNotFound,
    #[error("The requested transcode parameters were invalid.")]
    InvalidTranscodeSettings,
    #[error("The transcode request failed: {0}.")]
    TranscodeError(String),
    #[error("The server thinks the client should just play the original media.")]
    TranscodeRefused,
    #[error("Only invites with status pending_received can be accepted.")]
    InviteAcceptingNotPendingReceived,
    #[error("Unexpected error. Please create a bug report.")]
    UnexpectedError,
}

const PLEX_API_ERROR_CODE_AUTH_OTP_REQUIRED: i32 = 1029;

impl Error {
    pub async fn from_response(response: http::Response<Body>) -> Self {
        let status_code = response.status().as_u16();
        let headers = response.headers().clone();
        
        let response_body = match response.into_body().into_bytes().await {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(body) => body,
                Err(err) => {
                    return err.into();
                }
            },
            Err(err) => {
                return err.into();
            }
        };

        let err: Result<MyPlexApiErrorResponse, Error>;
        if let Some(content_type) = headers.get("Content-type") {
            match content_type.to_str().unwrap().split("; ").next().unwrap() {
                "application/xml" => {
                    err = quick_xml::de::from_str::<MyPlexApiErrorResponse>(&response_body)
                        .map_err(|e| e.into())
                }
                _ => {
                    err = serde_json::from_str::<MyPlexApiErrorResponse>(&response_body)
                        .map_err(|e| e.into())
                }
            }
        } else {
            err = serde_json::from_str::<MyPlexApiErrorResponse>(&response_body)
                .map_err(|e| e.into());
        }

        match err {
            Ok(r) => {
                if r.errors.len() == 1 && r.errors[0].code == PLEX_API_ERROR_CODE_AUTH_OTP_REQUIRED
                {
                    Self::OtpRequired
                } else {
                    r.into()
                }
            }
            Err(_) => Self::UnexpectedApiResponse {
                status_code,
                content: response_body,
            },
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
struct MyPlexApiError {
    code: i32,
    message: String,

    /// HTTP status code for the error. Not used, keeping it here to reflect the complete API response.
    #[allow(dead_code)]
    status: u16,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub(crate) struct MyPlexApiErrorResponse {
    errors: Vec<MyPlexApiError>,
}

impl From<MyPlexApiError> for Error {
    fn from(error: MyPlexApiError) -> Self {
        Self::MyPlexApiError {
            code: error.code,
            message: error.message,
        }
    }
}

impl From<MyPlexApiErrorResponse> for Error {
    fn from(r: MyPlexApiErrorResponse) -> Self {
        Self::MyPlexErrorResponse {
            errors: r.errors.into_iter().map(|e| e.into()).collect(),
        }
    }
}