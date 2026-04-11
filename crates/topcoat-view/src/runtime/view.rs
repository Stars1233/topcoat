use std::borrow::Cow;
use std::fmt;

use crate::runtime::fragment::Fragment;

pub struct View {
    pub(super) buf: Cow<'static, str>,
}

impl View {
    #[inline]
    pub fn new(buf: impl Into<Cow<'static, str>>) -> Self {
        Self { buf: buf.into() }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.buf
    }
}

impl fmt::Display for View {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.buf)
    }
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for View {
    fn into_response(self) -> axum::response::Response {
        axum::response::Html(self.buf.into_owned()).into_response()
    }
}

#[derive(Default)]
pub struct ViewWriter {
    buf: String,
}

impl ViewWriter {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: String::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn push_fragment(&mut self, fragment: impl Fragment) {
        self.buf.push_str(fragment.as_str());
    }

    #[inline]
    pub fn finish(self) -> View {
        View::new(self.buf)
    }
}
