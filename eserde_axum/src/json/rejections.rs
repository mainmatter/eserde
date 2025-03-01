use axum_core::extract::rejection::{BytesRejection, FailedToBufferBody};
use eserde::DeserializationErrors;
use http::header::CONTENT_TYPE;

use crate::details::{
    InvalidRequest, ProblemDetails, Source, ValidationError, ValidationErrors,
    INTERNAL_SERVER_ERROR,
};

#[doc(hidden)]
macro_rules! __log_rejection {
    (
        rejection_type = $ty:ident,
        status = $status:expr,
    ) => {
        {
            tracing::event!(
                target: "eserde_axum::json::rejection",
                tracing::Level::TRACE,
                status = $status.as_u16(),
                rejection_type = ::std::any::type_name::<$ty>(),
                "rejecting request",
            );
        }
    };
}

#[derive(Debug)]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
/// Rejection type for [`Json`](super::Json).
///
/// This rejection is used if the request body couldn't be deserialized
/// into the target type.
pub struct JsonError(pub(crate) DeserializationErrors);

impl JsonError {
    pub(crate) fn new(err: DeserializationErrors) -> Self {
        Self(err)
    }
}

impl axum_core::response::IntoResponse for JsonError {
    fn into_response(self) -> axum_core::response::Response {
        let errors = self
            .0
            .iter()
            .map(|e| {
                let pointer = e.path().map(|path| {
                    path.iter().fold(String::new(), |mut acc, part| {
                        acc.push('/');
                        acc.push_str(&part.to_string());
                        acc
                    })
                });
                ValidationError {
                    detail: e.message().into(),
                    source: Source::Body { pointer },
                }
            })
            .collect();
        let response = InvalidRequest::new(ValidationErrors { errors });
        __log_rejection!(
            rejection_type = JsonError,
            status = InvalidRequest::status(),
        );
        response.into_response()
    }
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Failed to deserialize the request JSON body into the target schema:\n")?;
        for e in self.0.iter() {
            writeln!(f, "- {}", e)?;
        }
        Ok(())
    }
}

impl std::error::Error for JsonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
/// Rejection type for [`Json`](super::Json) used if the `Content-Type`
/// header is missing.
pub struct MissingJsonContentType;

impl axum_core::response::IntoResponse for MissingJsonContentType {
    fn into_response(self) -> axum_core::response::Response {
        let error = ValidationError {
                    detail: "Expected request with `Content-Type: application/json`, but no `Content-Type` header was found".into(),
                    source: Source::Header {
                        name: CONTENT_TYPE.as_str().into(),
                    },
                };
        let response = InvalidRequest::new(ValidationErrors {
            errors: vec![error],
        });
        __log_rejection!(
            rejection_type = MissingJsonContentType,
            status = InvalidRequest::status(),
        );
        response.into_response()
    }
}
impl std::fmt::Display for MissingJsonContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected request with `Content-Type: application/json`")
    }
}
impl std::error::Error for MissingJsonContentType {}

impl Default for MissingJsonContentType {
    fn default() -> Self {
        Self
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
/// Rejection type for [`Json`](super::Json) used if the `Content-Type`
/// header has an incorrect value.
pub struct JsonContentTypeMismatch {
    pub(crate) actual: String,
}

impl axum_core::response::IntoResponse for JsonContentTypeMismatch {
    fn into_response(self) -> axum_core::response::Response {
        let error = ValidationError {
            detail: format!(
                "Expected request with `Content-Type: application/json` or `application/*+json`, but found `{}`",
                self.actual
            ),
            source: Source::Header {
                name: CONTENT_TYPE.as_str().into(),
            },
        };
        let response = InvalidRequest::new(ValidationErrors {
            errors: vec![error],
        });
        __log_rejection!(
            rejection_type = JsonContentTypeMismatch,
            status = InvalidRequest::status(),
        );
        response.into_response()
    }
}

impl std::fmt::Display for JsonContentTypeMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Expected request with `Content-Type: application/json` or `application/*+json`, but found `{}`",
            self.actual
        )
    }
}

impl std::error::Error for JsonContentTypeMismatch {}

/// Rejection used for [`Json`](super::Json).
///
/// Contains one variant for each way the [`Json`](super::Json) extractor
/// can fail.
///
/// All error responses follow the problem details specification,
/// as outlined in [RFC 9457](https://www.rfc-editor.org/rfc/rfc9457.html).
#[derive(Debug)]
#[non_exhaustive]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub enum JsonRejection {
    #[allow(missing_docs)]
    JsonError(JsonError),
    #[allow(missing_docs)]
    MissingJsonContentType(MissingJsonContentType),
    #[allow(missing_docs)]
    JsonContentTypeMismatch(JsonContentTypeMismatch),
    #[allow(missing_docs)]
    BytesRejection(BytesRejection),
    #[cfg(feature = "validator")]
    #[allow(missing_docs)]
    ValidationErrors(validator::ValidationErrors),
    #[cfg(feature = "serde_valid")]
    #[allow(missing_docs)]
    SerdeValidRejection(serde_valid::validation::Errors),
}
impl axum_core::response::IntoResponse for JsonRejection {
    fn into_response(self) -> axum_core::response::Response {
        match self {
            Self::JsonError(inner) => inner.into_response(),
            Self::MissingJsonContentType(inner) => inner.into_response(),
            Self::JsonContentTypeMismatch(inner) => inner.into_response(),
            Self::BytesRejection(inner) => {
                let mut response = None;
                #[allow(clippy::single_match)]
                match inner {
                    BytesRejection::FailedToBufferBody(failed_to_buffer_body) => {
                        match failed_to_buffer_body {
                            FailedToBufferBody::LengthLimitError(length_limit_error) => {
                                let details: ProblemDetails<()> = ProblemDetails {
                                    type_: "content_too_large".into(),
                                    status: length_limit_error.status().as_u16(),
                                    title: "The content is too large".into(),
                                    detail: length_limit_error.body_text().into(),
                                    extensions: None,
                                };
                                response = Some(details.into_response());
                            }
                            FailedToBufferBody::UnknownBodyError(unknown_body_error) => {
                                let details: ProblemDetails<()> = ProblemDetails {
                                    type_: "body_buffering_error".into(),
                                    status: unknown_body_error.status().as_u16(),
                                    title: "Failed to buffer the body".into(),
                                    detail: unknown_body_error.body_text().into(),
                                    extensions: None,
                                };
                                response = Some(details.into_response());
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                response.unwrap_or_else(|| INTERNAL_SERVER_ERROR.into_response())
            }
            #[cfg(feature = "validator")]
            Self::ValidationErrors(inner) => {
                let errors = format!("{}", inner)
                    .split("\n")
                    .zip(inner.errors().iter())
                    .map(|(message, (path, _))| ValidationError {
                        detail: message.to_string().replace("\"", "'"),
                        source: Source::Body {
                            pointer: Some(path.to_string()),
                        },
                    })
                    .collect::<Vec<_>>();

                let response = InvalidRequest::new(ValidationErrors { errors });
                __log_rejection!(
                    rejection_type = ValidationErrors,
                    status = InvalidRequest::status(),
                );
                response.into_response()
            }
            #[cfg(feature = "serde_valid")]
            Self::SerdeValidRejection(inner) => serde_valid_error_to_a_response(inner),
        }
    }
}

impl From<JsonError> for JsonRejection {
    fn from(inner: JsonError) -> Self {
        Self::JsonError(inner)
    }
}
impl From<MissingJsonContentType> for JsonRejection {
    fn from(inner: MissingJsonContentType) -> Self {
        Self::MissingJsonContentType(inner)
    }
}
impl From<JsonContentTypeMismatch> for JsonRejection {
    fn from(inner: JsonContentTypeMismatch) -> Self {
        Self::JsonContentTypeMismatch(inner)
    }
}
impl From<BytesRejection> for JsonRejection {
    fn from(inner: BytesRejection) -> Self {
        Self::BytesRejection(inner)
    }
}
impl std::fmt::Display for JsonRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JsonError(inner) => write!(f, "{inner}"),
            Self::MissingJsonContentType(inner) => write!(f, "{inner}"),
            Self::JsonContentTypeMismatch(inner) => write!(f, "{inner}"),
            Self::BytesRejection(inner) => write!(f, "{inner}"),
            #[cfg(feature = "validator")]
            Self::ValidationErrors(inner) => write!(f, "{inner}"),
            #[cfg(feature = "serde_valid")]
            Self::SerdeValidRejection(inner) => write!(f, "{inner}"),
        }
    }
}
impl std::error::Error for JsonRejection {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::JsonError(inner) => inner.source(),
            Self::MissingJsonContentType(inner) => inner.source(),
            Self::JsonContentTypeMismatch(inner) => inner.source(),
            Self::BytesRejection(inner) => inner.source(),
            #[cfg(feature = "validator")]
            Self::ValidationErrors(inner) => inner.source(),
            #[cfg(feature = "serde_valid")]
            Self::SerdeValidRejection(inner) => inner.source(),
        }
    }
}

#[cfg(feature = "serde_valid")]
#[inline]
fn serde_valid_error_to_a_response(
    inner: serde_valid::validation::error::Errors,
) -> axum_core::response::Response {
    use axum_core::response::IntoResponse;
    use serde_valid::validation::Errors;

    match inner.clone() {
        serde_valid::validation::Errors::Object(a) => {
            let errors = a
                .properties
                .iter()
                .map(|(key, value)| ValidationError {
                    detail: match value {
                        Errors::Array(array_errors) => format!("{}", array_errors),
                        Errors::Object(object_errors) => format!("{}", object_errors),
                        Errors::NewType(newtype) => newtype
                            .iter()
                            .map(|e| format!("{}", e))
                            .collect::<Vec<_>>()
                            .join("\n"),
                    },
                    source: Source::Body {
                        pointer: Some(key.to_string()),
                    },
                })
                .collect();
            let response = InvalidRequest::new(ValidationErrors { errors });
            __log_rejection!(
                rejection_type = JsonError,
                status = InvalidRequest::status(),
            );
            return response.into_response();
        }
        Errors::Array(array) => {
            let errors = array
                .errors
                .iter()
                .map(|e| ValidationError {
                    detail: format!("{}", e),
                    source: Source::Body { pointer: None },
                })
                .collect();
            let response = InvalidRequest::new(ValidationErrors { errors });
            __log_rejection!(
                rejection_type = JsonError,
                status = InvalidRequest::status(),
            );
            return response.into_response();
        }
        Errors::NewType(errors) => {
            let errors = errors
                .iter()
                .map(|e| ValidationError {
                    detail: format!("{}", e),
                    source: Source::Body { pointer: None },
                })
                .collect();
            let response = InvalidRequest::new(ValidationErrors { errors });
            __log_rejection!(
                rejection_type = JsonError,
                status = InvalidRequest::status(),
            );
            return response.into_response();
        }
    }
}
