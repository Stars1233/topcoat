use std::borrow::Cow;
use std::fmt;

pub struct View {
    buf: Cow<'static, str>,
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
    pub fn push_str(&mut self, string: &str) {
        self.buf.push_str(string);
    }

    #[inline]
    pub fn finish(self) -> View {
        View::new(self.buf)
    }
}

impl fmt::Write for ViewWriter {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buf.push_str(s);
        Ok(())
    }
}
