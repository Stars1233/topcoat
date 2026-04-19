use axum::routing::get;

use crate::{Layout, Page};

#[derive(Default)]
pub struct Router {
    pages: Vec<Page>,
    layouts: Vec<Layout>,
}

impl Router {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        self.pages.is_empty() && self.layouts.is_empty()
    }

    pub fn page(mut self, page: Page) -> Self {
        self.pages.push(page);
        self
    }

    pub fn layout(mut self, layout: Layout) -> Self {
        self.layouts.push(layout);
        self
    }

    #[cfg(feature = "discover")]
    pub fn discover(mut self) -> Self {
        for page in inventory::iter::<Page>().cloned() {
            self = self.page(page);
        }
        for layout in inventory::iter::<Layout>().cloned() {
            self = self.layout(layout);
        }
        self
    }
}

impl From<Router> for axum::Router {
    fn from(value: Router) -> Self {
        let mut result = axum::Router::new();

        for page in value.pages {
            let layouts: Vec<_> = value
                .layouts
                .iter()
                .filter(|layout| page.path().starts_with(layout.path()))
                .cloned()
                .collect();

            result = result.route(
                &page.path().to_axum_path(),
                get(async move || {
                    let mut result = page.render();
                    for layout in layouts {
                        result = layout.render(result);
                    }
                    result.await
                }),
            );
        }

        result
    }
}
