pub mod layout;
pub mod page;

use axum::routing::get;

use crate::page::Page;

#[derive(Default)]
pub struct Router {
    pages: Vec<Page>,
}

impl Router {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn page(mut self, page: Page) -> Self {
        self.pages.push(page);
        self
    }
}

impl From<Router> for axum::Router {
    fn from(value: Router) -> Self {
        let mut result = axum::Router::new();

        for page in value.pages {
            result = result.route(page.path(), get(async move || page.render().await));
        }

        result
    }
}
