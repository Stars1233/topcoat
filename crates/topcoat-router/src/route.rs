use http::Method;

use crate::{Pattern, dynamic_routes::DynamicRoutes, static_routes::StaticRoutes};

#[derive(Debug, Clone)]
pub struct Route {
    method: Method,
    pattern: Pattern,
    handler: HandlerFn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RouteId(usize);

impl RouteId {
    fn new(inner: usize) -> Self {
        Self(inner)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Routes {
    routes: Vec<Route>,
    static_routes: StaticRoutes,
    dynamic_routes: DynamicRoutes,
}

impl Routes {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, route: Route) {
        let route_id = RouteId::new(self.routes.len());
        if let Some(path) = route.pattern.to_path() {
            self.static_routes
                .insert(route.method.clone(), path, route_id);
        } else {
            self.dynamic_routes
                .insert(route.method.clone(), &route.pattern, route_id);
        }
        self.routes.push(route);
    }
}
