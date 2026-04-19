use std::{borrow::Cow, pin::Pin};

use topcoat_view::runtime::View;

use crate::Path;

#[derive(Clone)]
pub struct Page {
    path: Cow<'static, Path>,
    render: fn() -> Pin<Box<dyn Future<Output = View> + Send>>,
}

impl Page {
    pub const fn new(
        path: Cow<'static, Path>,
        render: fn() -> Pin<Box<dyn Future<Output = View> + Send>>,
    ) -> Self {
        Self { path, render }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn render(&self) -> Pin<Box<dyn Future<Output = View> + Send>> {
        (self.render)()
    }
}

#[cfg(feature = "discover")]
inventory::collect!(Page);
