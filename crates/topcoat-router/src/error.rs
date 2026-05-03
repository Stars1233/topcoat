use axum::response::IntoResponse;
use http::StatusCode;

use crate::RedirectError;

pub type Result<T = topcoat_view::runtime::View, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Redirect(RedirectError),
    InternalServer(InternalServerError),
}

impl From<RedirectError> for Error {
    fn from(value: RedirectError) -> Self {
        Self::Redirect(value)
    }
}

#[derive(Debug)]
pub struct InternalServerError {
    _inner: Box<dyn std::error::Error + Send + Sync>,
}

impl From<InternalServerError> for Error {
    fn from(value: InternalServerError) -> Self {
        Self::InternalServer(value)
    }
}

impl IntoResponse for InternalServerError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "internal sever error").into_response()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Redirect(inner) => inner.into_response(),
            Self::InternalServer(inner) => inner.into_response(),
        }
    }
}
