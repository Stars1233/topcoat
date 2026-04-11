use std::fmt::Debug;

use http::Method;

use crate::{Pattern, Route, Routes, TryIntoPattern};

#[derive(Debug, Default, Clone)]
pub struct Router {
    routes: Routes,
}

impl Router {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(&mut self, pattern: impl TryIntoPattern) -> &mut Self {
        self.route(Method::GET, pattern.try_into_pattern().unwrap())
    }

    fn route(&mut self, method: Method, pattern: Pattern) -> &mut Self {
        self.routes.insert(Route::new());
        self
    }
}
