//! Error types returned from page, layout and route handlers.

use http::StatusCode;
use topcoat_core::error::Error;

use crate::{
    ForbiddenError, IntoResponse, NotFoundError, RedirectError, Response, Result, UnauthorizedError,
};

/// A non-success outcome from a handler.
#[derive(Debug)]
pub enum RouterError {
    /// A redirect short-circuiting the request to another URL.
    Redirect(RedirectError),
    /// A not-found response short-circuiting the request.
    NotFound(NotFoundError),
    /// An unauthorized response short-circuiting the request.
    Unauthorized(UnauthorizedError),
    /// A forbidden response short-circuiting the request.
    Forbidden(ForbiddenError),
}

impl From<RedirectError> for RouterError {
    fn from(value: RedirectError) -> Self {
        Self::Redirect(value)
    }
}

impl From<NotFoundError> for RouterError {
    fn from(value: NotFoundError) -> Self {
        Self::NotFound(value)
    }
}

impl From<UnauthorizedError> for RouterError {
    fn from(value: UnauthorizedError) -> Self {
        Self::Unauthorized(value)
    }
}

impl From<ForbiddenError> for RouterError {
    fn from(value: ForbiddenError) -> Self {
        Self::Forbidden(value)
    }
}

impl std::fmt::Display for RouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Redirect(inner) => std::fmt::Display::fmt(inner, f),
            Self::NotFound(inner) => std::fmt::Display::fmt(inner, f),
            Self::Unauthorized(inner) => std::fmt::Display::fmt(inner, f),
            Self::Forbidden(inner) => std::fmt::Display::fmt(inner, f),
        }
    }
}

impl std::error::Error for RouterError {}

impl IntoResponse for RouterError {
    fn into_response(self) -> Response {
        match self {
            Self::Redirect(inner) => inner.into_response(),
            Self::NotFound(inner) => inner.into_response(),
            Self::Unauthorized(inner) => inner.into_response(),
            Self::Forbidden(inner) => inner.into_response(),
        }
    }
}

/// Turns an Error into a response.
///
/// The IntoResponse trait unfortunately cannot be implemented on [`Error`] because it would clash
/// with the axum implementation.
pub(crate) fn error_into_response(error: Error) -> Response {
    match error.downcast::<RouterError>() {
        Ok(error) => error.into_response(),
        Err(error) => InternalServerError::from(error).into_response(),
    }
}

/// Turns a Result into a response.
///
/// The IntoResponse trait unfortunately cannot be implemented on [`Result`] because it would clash
/// with the axum implementation.
pub(crate) fn result_into_response<T: IntoResponse>(result: Result<T>) -> Response {
    match result {
        Ok(value) => value.into_response(),
        Err(error) => error_into_response(error),
    }
}

pub(crate) struct InternalServerError {
    _inner: Error,
}

impl From<Error> for InternalServerError {
    fn from(value: Error) -> Self {
        Self { _inner: value }
    }
}

impl IntoResponse for InternalServerError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error").into_response()
    }
}
