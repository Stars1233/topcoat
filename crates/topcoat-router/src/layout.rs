use std::{borrow::Cow, pin::Pin};

use topcoat_view::runtime::View;

use crate::Path;

pub type Slot = Pin<Box<dyn Future<Output = View> + Send>>;

#[derive(Clone)]
pub struct Layout {
    path: Cow<'static, Path>,
    render: fn(slot: Slot) -> Pin<Box<dyn Future<Output = View> + Send>>,
}

impl Layout {
    pub const fn new(
        path: Cow<'static, Path>,
        render: fn(slot: Slot) -> Pin<Box<dyn Future<Output = View> + Send>>,
    ) -> Self {
        Self { path, render }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn render(&self, slot: Slot) -> Pin<Box<dyn Future<Output = View> + Send>> {
        (self.render)(slot)
    }
}

#[cfg(feature = "discover")]
inventory::collect!(Layout);
