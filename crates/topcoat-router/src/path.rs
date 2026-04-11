use std::{borrow::Cow, ops::Deref};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path<'a> {
    inner: Cow<'a, str>,
}

impl<'a> Path<'a> {
    pub fn new(path: impl Into<Cow<'a, str>>) -> Self {
        Self { inner: path.into() }
    }
}

impl Deref for Path<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
