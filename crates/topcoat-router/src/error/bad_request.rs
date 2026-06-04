use http::StatusCode;
use serde_path_to_error::Path;

use crate::Response;

/// Builds a bad-request (HTTP 400) response with a client-safe description.
///
/// Use this when the caller supplied invalid input and the response should
/// explain what was wrong.
///
/// # Examples
///
/// ```rust,ignore
/// use topcoat::Result;
/// use topcoat::router::bad_request;
///
/// async fn update_user(name: String) -> Result {
///     if name.trim().is_empty() {
///         return Err(bad_request("name cannot be empty").into());
///     }
///
///     Ok(())
/// }
/// ```
pub fn bad_request(description: impl Into<String>) -> BadRequestError {
    BadRequestError::new(None, description.into())
}

/// Builds a bad-request (HTTP 400) response whose description includes an
/// input path.
///
/// This is useful for structured request formats where the parser can report
/// the field or element that failed validation.
pub fn bad_request_at(path: impl Into<Path>, description: impl Into<String>) -> BadRequestError {
    BadRequestError::new(Some(path.into()), description.into())
}

/// A bad-request response carried as the `Err` variant of a handler `Result`.
///
/// Construct one with [`bad_request`].
#[derive(Debug)]
pub struct BadRequestError {
    path: Option<Path>,
    description: String,
}

impl BadRequestError {
    fn new(path: Option<Path>, description: String) -> Self {
        Self { description, path }
    }

    /// Returns the path into the request where the error was encountered.
    pub fn path(&self) -> Option<&Path> {
        self.path.as_ref()
    }

    /// Returns the client-safe description of what was wrong with the request.
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl std::fmt::Display for BadRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.path {
            Some(path) => write!(f, "bad request: {} (at `{path}`)", self.description),
            None => write!(f, "bad request: {}", self.description),
        }
    }
}

impl std::error::Error for BadRequestError {}

impl axum::response::IntoResponse for BadRequestError {
    fn into_response(self) -> Response {
        <(StatusCode, String) as axum::response::IntoResponse>::into_response((
            StatusCode::BAD_REQUEST,
            self.to_string(),
        ))
    }
}
