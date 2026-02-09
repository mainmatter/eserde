//!  [`Form`] extractor.

use axum::extract::{rejection::RawFormRejection, RawForm};
use axum_core::{
    extract::{FromRequest, Request},
    RequestExt,
};
use eserde::DeserializationErrors;
use serde::de::DeserializeOwned;

use crate::details::{InvalidRequest, Source, ValidationError, ValidationErrors};

#[doc(hidden)]
macro_rules! __log_rejection {
    (
        rejection_type = $ty:ident,
        status = $status:expr,
    ) => {
        {
            tracing::event!(
                target: "eserde_axum::html_form::rejection",
                tracing::Level::TRACE,
                status = $status.as_u16(),
                rejection_type = ::std::any::type_name::<$ty>(),
                "rejecting request",
            );
        }
    };
}

/// Extractor that deserializes `application/x-www-form-urlencoded` requests
/// into some type.
///
/// `T` is expected to implement [`serde::Deserialize`].
///
/// # Differences from `axum::extract::Form`
///
/// This extractor uses [`serde_html_form`] under-the-hood which supports multi-value items. These
/// are sent by multiple `<input>` attributes of the same name (e.g. checkboxes) and `<select>`s
/// with the `multiple` attribute. Those values can be collected into a `Vec` or other sequential
/// container.
///
/// # Example
///
/// ```rust,no_run
/// use axum_extra::extract::Form;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Payload {
///     #[serde(rename = "value")]
///     values: Vec<String>,
/// }
///
/// async fn accept_form(Form(payload): Form<Payload>) {
///     // ...
/// }
/// ```
///
/// [`serde_html_form`]: https://crates.io/crates/serde_html_form
#[derive(Debug, Clone, Copy, Default)]
pub struct Form<T>(pub T);

impl<T> std::ops::Deref for Form<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> std::ops::DerefMut for Form<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, S> FromRequest<S> for Form<T>
where
    T: DeserializeOwned,
    T: for<'de> eserde::EDeserialize<'de>,
    S: Send + Sync,
{
    type Rejection = FormRejection;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let is_get_or_head =
            req.method() == http::Method::GET || req.method() == http::Method::HEAD;

        let RawForm(bytes) = req.extract().await?;

        eserde::html_form::from_bytes(&bytes)
            .map(Self)
            .map_err(|err| {
                if is_get_or_head {
                    FailedToDeserializeForm::from(err).into()
                } else {
                    FailedToDeserializeForm::from(err).into()
                }
            })
    }
}

#[doc = r" Rejection type used if the [`Form`](Form) extractor is unable to"]
#[doc = r" deserialize the form into the target type."]
#[derive(Debug)]
pub struct FailedToDeserializeForm(DeserializationErrors);

impl From<DeserializationErrors> for FailedToDeserializeForm {
    fn from(value: DeserializationErrors) -> Self {
        Self(value)
    }
}

impl axum_core::response::IntoResponse for FailedToDeserializeForm {
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
            rejection_type = FailedToDeserializeForm,
            status = InvalidRequest::status(),
        );
        response.into_response()
    }
}
impl std::fmt::Display for FailedToDeserializeForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Failed to deserialize form:\n")?;
        for e in self.0.iter() {
            writeln!(f, "- {}", e)?;
        }
        Ok(())
    }
}
impl std::error::Error for FailedToDeserializeForm {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

#[doc = r" Rejection used for [`Form`]."]
#[doc = r""]
#[doc = r" Contains one variant for each way the [`Form`] extractor can fail."]
#[derive(Debug)]
#[non_exhaustive]
pub enum FormRejection {
    #[allow(missing_docs)]
    RawFormRejection(RawFormRejection),
    #[allow(missing_docs)]
    FailedToDeserializeForm(FailedToDeserializeForm),
    #[allow(missing_docs)]
    FailedToDeserializeFormBody(FailedToDeserializeForm),
}
impl axum_core::response::IntoResponse for FormRejection {
    fn into_response(self) -> axum_core::response::Response {
        match self {
            Self::RawFormRejection(inner) => inner.into_response(),
            Self::FailedToDeserializeForm(inner) => inner.into_response(),
            Self::FailedToDeserializeFormBody(inner) => inner.into_response(),
        }
    }
}

impl From<RawFormRejection> for FormRejection {
    fn from(inner: RawFormRejection) -> Self {
        Self::RawFormRejection(inner)
    }
}
impl From<FailedToDeserializeForm> for FormRejection {
    fn from(inner: FailedToDeserializeForm) -> Self {
        Self::FailedToDeserializeForm(inner)
    }
}

impl std::fmt::Display for FormRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RawFormRejection(inner) => write!(f, "{inner}"),
            Self::FailedToDeserializeForm(inner) => write!(f, "{inner}"),
            Self::FailedToDeserializeFormBody(inner) => write!(f, "{inner}"),
        }
    }
}
impl std::error::Error for FormRejection {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::RawFormRejection(inner) => inner.source(),
            Self::FailedToDeserializeForm(inner) => inner.source(),
            Self::FailedToDeserializeFormBody(inner) => inner.source(),
        }
    }
}
