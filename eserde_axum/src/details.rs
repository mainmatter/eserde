//! Types to represent a problem detail error response.
//!
//! See [RFC 9457](https://www.rfc-editor.org/rfc/rfc9457.html) for more details.
use std::borrow::Cow;

use bytes::{BufMut, BytesMut};
use http::{header::CONTENT_TYPE, HeaderName, HeaderValue, StatusCode};

#[derive(serde::Serialize)]
pub(crate) struct ProblemDetails<Extension> {
    #[serde(rename = "type")]
    pub(crate) type_: Cow<'static, str>,
    pub(crate) status: u16,
    pub(crate) title: Cow<'static, str>,
    pub(crate) detail: Cow<'static, str>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) extensions: Option<Extension>,
}

#[derive(serde::Serialize)]
pub(crate) struct ValidationErrors {
    pub(crate) errors: Vec<ValidationError>,
}

#[derive(serde::Serialize)]
pub(crate) struct ValidationError {
    pub(crate) detail: String,
    #[serde(flatten)]
    pub(crate) source: Source,
}

/// The request part where the problem occurred.
#[derive(serde::Serialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub(crate) enum Source {
    Body {
        /// A [JSON pointer](https://www.rfc-editor.org/info/rfc6901) targeted
        /// at the problematic body property.
        pointer: Option<String>,
    },
    Header {
        /// The name of the problematic header.
        name: Cow<'static, str>,
    },
}

impl<Extension> axum_core::response::IntoResponse for ProblemDetails<Extension>
where
    Extension: serde::Serialize,
{
    fn into_response(self) -> axum_core::response::Response {
        // Use a small initial capacity of 128 bytes like serde_json::to_vec
        // https://docs.rs/serde_json/1.0.82/src/serde_json/ser.rs.html#2189
        let mut buf = BytesMut::with_capacity(128).writer();
        match serde_json::to_writer(&mut buf, &self) {
            Ok(()) => (
                [(CONTENT_TYPE, APPLICATION_PROBLEM_JSON)],
                buf.into_inner().freeze(),
            )
                .into_response(),
            Err(_) => INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

pub(crate) const APPLICATION_PROBLEM_JSON: HeaderValue =
    HeaderValue::from_static("application/problem+json");

pub(crate) const INTERNAL_SERVER_ERROR: (StatusCode, [(HeaderName, HeaderValue); 1], &[u8]) = (
    StatusCode::INTERNAL_SERVER_ERROR,
    [(CONTENT_TYPE, APPLICATION_PROBLEM_JSON)],
    INTERNAL_SERVER_ERROR_PROBLEM,
);

pub(crate) const INTERNAL_SERVER_ERROR_PROBLEM: &[u8] = br#"{
    "type": "internal_server_error",
    "title": "Internal Server Error",
    "detail": "Something went wrong when processing your request. Please try again later."
    "status": 500
}"#;

pub(crate) struct InvalidRequest(ProblemDetails<ValidationErrors>);

impl InvalidRequest {
    pub(crate) fn new(errors: ValidationErrors) -> Self {
        Self(ProblemDetails {
            type_: "invalid_request".into(),
            status: Self::status().as_u16(),
            title: "The request is invalid".into(),
            extensions: Some(errors),
            detail: "The request is either malformed or doesn't match the expected schema".into(),
        })
    }

    pub(crate) fn status() -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    pub(crate) fn into_inner(self) -> ProblemDetails<ValidationErrors> {
        self.0
    }
}

impl axum_core::response::IntoResponse for InvalidRequest {
    fn into_response(self) -> axum_core::response::Response {
        self.into_inner().into_response()
    }
}
