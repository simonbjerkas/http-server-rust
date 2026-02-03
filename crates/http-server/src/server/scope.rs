use std::sync::Arc;

use super::{Route, middleware::Middleware};

pub struct Scope {
    pub(crate) prefix: String,
    pub(crate) routes: Vec<Route>,
    pub(crate) middleware: Vec<Arc<dyn Middleware>>,
}

impl Scope {
    pub fn new(prefix: impl Into<String>) -> Scope {
        Scope {
            prefix: prefix.into(),
            routes: Vec::new(),
            middleware: Vec::new(),
        }
    }

    /// Add middlewarew to this scope
    pub fn middleware<M: Middleware + 'static>(mut self, mw: M) -> Scope {
        self.middleware.push(Arc::new(mw));
        self
    }

    /// Add a route to this scope
    ///
    /// path is prefixed automatically
    pub fn service(mut self, mut route: Route) -> Scope {
        route.path = format!(
            "{}/{}",
            self.prefix.trim_end_matches('/'),
            route.path.trim_start_matches('/')
        );
        self.routes.push(route);

        self
    }
}
