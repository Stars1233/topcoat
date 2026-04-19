use std::pin::Pin;

use topcoat_view::runtime::View;

#[derive(Debug, Clone)]
pub struct FilePage {
    file: &'static str,
    render: fn() -> Pin<Box<dyn Future<Output = View> + Send>>,
}

impl FilePage {
    pub const fn new(
        file: &'static str,
        render: fn() -> Pin<Box<dyn Future<Output = View> + Send>>,
    ) -> Self {
        Self { file, render }
    }
}

#[cfg(feature = "discover")]
inventory::collect!(FilePage);
