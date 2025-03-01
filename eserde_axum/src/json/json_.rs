use std::ops::Deref;
use std::ops::DerefMut;

use crate::details::INTERNAL_SERVER_ERROR;

use super::*;
use axum_core::extract::FromRequest;
use axum_core::extract::Request;
use axum_core::response::{IntoResponse, Response};
use bytes::{BufMut, Bytes, BytesMut};
use eserde::EDeserialize;
use http::header::{self, HeaderMap, HeaderValue};
use serde::{de::DeserializeOwned, Serialize};

/// JSON Extractor / Response.
///
/// When used as an extractor, it can deserialize request bodies into some type that
/// implements [`serde::de::DeserializeOwned`] and [`eserde::EDeserialize`].
/// The request will be rejected (and a [`JsonRejection`] will be returned) if:
///
/// - The request doesn't have a `Content-Type: application/json` (or similar) header.
/// - The body doesn't contain syntactically valid JSON or it couldn't be deserialized into the target type.
/// - Buffering the request body fails.
///
/// ⚠️ Since parsing JSON requires consuming the request body, the `Json` extractor must be
/// *last* if there are multiple extractors in a handler.
/// See ["the order of extractors"][order-of-extractors]
///
/// [order-of-extractors]: https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors
///
/// See [`JsonRejection`] for more details.
///
/// # Extractor example
///
/// ```rust,no_run
/// use axum::{routing::post, Router};
/// use eserde_axum::Json;
///
/// #[derive(eserde::Deserialize)]
/// struct CreateUser {
///     email: String,
///     password: String,
/// }
///
/// async fn create_user(Json(payload): Json<CreateUser>) {
///     // payload is a `CreateUser`
/// }
///
/// let app = Router::new().route("/users", post(create_user));
/// # let _: Router = app;
/// ```
///
/// When used as a response, it can serialize any type that implements [`serde::Serialize`] to
/// `JSON`, and will automatically set `Content-Type: application/json` header.
///
/// If the [`Serialize`] implementation decides to fail
/// or if a map with non-string keys is used,
/// a 500 response will be issued.
///
/// # Response example
///
/// ```
/// use axum::{
///     extract::Path,
///     routing::get,
///     Router,
/// };
/// use eserde_axum::Json;
/// use serde::Serialize;
/// use uuid::Uuid;
///
/// #[derive(Serialize)]
/// struct User {
///     id: Uuid,
///     username: String,
/// }
///
/// async fn get_user(Path(user_id) : Path<Uuid>) -> Json<User> {
///     let user = find_user(user_id).await;
///     Json(user)
/// }
///
/// async fn find_user(user_id: Uuid) -> User {
///     // ...
///     # unimplemented!()
/// }
///
/// let app = Router::new().route("/users/{id}", get(get_user));
/// # let _: Router = app;
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
#[must_use]
pub struct Json<T>(pub T);

#[cfg(all(not(feature = "validator"), not(feature = "serde_valid")))]
impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    T: for<'de> EDeserialize<'de>,
    S: Send + Sync,
{
    type Rejection = JsonRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        check_json_content_type(req.headers())?;
        let bytes = Bytes::from_request(req, state).await?;
        Self::from_bytes(&bytes)
    }
}

#[cfg(feature = "validator")]
impl<T, S> FromRequest<S> for Json<T>
where
    T: validator::Validate + DeserializeOwned,
    T: for<'de> EDeserialize<'de>,
    S: Send + Sync,
{
    type Rejection = JsonRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        check_json_content_type(req.headers())?;
        let bytes = Bytes::from_request(req, state).await?;
        let json = Self::from_bytes(&bytes)?;

        json.0.validate().map_err(JsonRejection::ValidationErrors)?;

        Ok(json)
    }
}

#[cfg(feature = "serde_valid")]
impl<T, S> FromRequest<S> for Json<T>
where
    T: serde_valid::Validate + DeserializeOwned,
    T: for<'de> EDeserialize<'de>,
    S: Send + Sync,
{
    type Rejection = JsonRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        check_json_content_type(req.headers())?;
        let bytes = Bytes::from_request(req, state).await?;
        let json = Self::from_bytes(&bytes)?;

        json.0
            .validate()
            .map_err(JsonRejection::SerdeValidRejection)?;

        Ok(json)
    }
}

/// Check that the `Content-Type` header is set to `application/json`, or another
/// `application/*+json` MIME type.
///
/// Return an error otherwise.
fn check_json_content_type(headers: &HeaderMap) -> Result<(), JsonRejection> {
    let Some(content_type) = headers.get(http::header::CONTENT_TYPE) else {
        return Err(MissingJsonContentType.into());
    };
    let Ok(content_type) = content_type.to_str() else {
        return Err(MissingJsonContentType.into());
    };

    let Ok(mime) = content_type.parse::<mime::Mime>() else {
        return Err(JsonContentTypeMismatch {
            actual: content_type.to_string(),
        }
        .into());
    };

    let is_json_content_type = mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().is_some_and(|name| name == "json"));
    if !is_json_content_type {
        return Err(JsonContentTypeMismatch {
            actual: content_type.to_string(),
        }
        .into());
    }
    Ok(())
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Json<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> Json<T>
where
    T: DeserializeOwned,
    T: for<'de> EDeserialize<'de>,
{
    /// Construct a `Json<T>` from a byte slice. Most users should prefer to use the `FromRequest` impl
    /// but special cases may require first extracting a `Request` into `Bytes` then optionally
    /// constructing a `Json<T>`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, JsonRejection> {
        match eserde::json::from_slice(bytes) {
            Ok(value) => Ok(Json(value)),
            Err(errors) => Err(JsonError::new(errors).into()),
        }
    }
}

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        // Use a small initial capacity of 128 bytes like serde_json::to_vec
        // https://docs.rs/serde_json/1.0.82/src/serde_json/ser.rs.html#2189
        let mut buf = BytesMut::with_capacity(128).writer();
        match serde_json::to_writer(&mut buf, &self.0) {
            Ok(()) => (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                )],
                buf.into_inner().freeze(),
            )
                .into_response(),
            Err(_) => INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
