use std::pin::Pin;

use topcoat_view::runtime::View;

use crate::Slot;

#[derive(Debug, Clone)]
pub struct FileLayout {
    file: &'static str,
    render: fn(slot: Slot) -> Pin<Box<dyn Future<Output = View> + Send>>,
}

impl FileLayout {
    pub const fn new(
        file: &'static str,
        render: fn(slot: Slot) -> Pin<Box<dyn Future<Output = View> + Send>>,
    ) -> Self {
        Self { file, render }
    }
}

#[cfg(feature = "discover")]
inventory::collect!(FileLayout);
