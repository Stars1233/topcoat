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

pub trait Fragment {
    fn as_str(&self) -> &str;
}

impl<T> Fragment for T
where
    T: AsRef<str>,
{
    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl Fragment for &View {
    fn as_str(&self) -> &str {
        &self.buf
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
