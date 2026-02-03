mod error;
mod http;
mod server;

use error::ServerError;

pub use http::{
    headers::{self, Headers},
    request::Request,
    response::Response,
    types::{IntoRoute, Method, Route, StatusCode},
};
pub use server::{Server, middleware, scope};

pub use http_server_macros::{get, post};

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::server::middleware::ScopedMiddleware;

/// Configuration for the `App`struct.
///
/// holds the directory from where files should be written and read on the server
pub struct Config {
    pub directory: PathBuf,
}

impl Config {
    pub fn new(directory: PathBuf) -> Config {
        Config { directory }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            directory: PathBuf::from("."),
        }
    }
}

pub struct App {
    config: Arc<Config>,
    routes: Vec<Route>,
    middleware: Vec<middleware::ScopedMiddleware>,
}

impl App {
    /// Create a new app
    ///
    /// creates a new app with default configs.
    ///
    /// the service containng routing data for your server.
    pub fn new() -> App {
        let config = Config::default();

        App {
            config: Arc::new(config),
            routes: Vec::new(),
            middleware: Vec::new(),
        }
    }

    /// Create a new app
    ///
    /// creates a new app with supplied configuration.
    ///
    /// the service containing routing data for your server.
    pub fn with_config(config: Config) -> App {
        App {
            config: Arc::new(config),
            routes: Vec::new(),
            middleware: Vec::new(),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn scope(mut self, scope: scope::Scope) -> App {
        for mw in scope.middleware {
            self.middleware.push(ScopedMiddleware {
                middleware: mw,
                scope: middleware::MiddlewareScope::Prefix(scope.prefix.clone()),
            });
        }

        for route in scope.routes {
            self.routes.push(route);
        }

        self
    }

    /// Add global middleware
    ///
    /// adds a middleware for all routes.
    ///
    /// Consumes your current app and returns a new one
    pub fn middleware<M: middleware::Middleware + 'static>(mut self, mw: M) -> App {
        self.middleware.push(ScopedMiddleware {
            middleware: Arc::new(mw),
            scope: middleware::MiddlewareScope::Global,
        });

        self
    }

    /// Scoped middleware
    ///
    /// adds a middleware for routes under the given prefix
    ///
    /// Consumes your current app and returns a new one
    pub fn middleware_for<M: middleware::Middleware + 'static>(
        mut self,
        prefix: impl Into<String>,
        mw: M,
    ) -> App {
        self.middleware.push(ScopedMiddleware {
            middleware: Arc::new(mw),
            scope: middleware::MiddlewareScope::Prefix(prefix.into()),
        });

        self
    }

    /// Registers a service
    ///
    /// takes your current app with services and returns a new one with the new service attached
    pub fn service<R: IntoRoute>(mut self, route: R) -> App {
        self.routes.push(route.into_route());
        self
    }

    pub(crate) fn handle(&self, mut req: Request) -> Response {
        let Some(route) = self.match_route(&mut req) else {
            return Response::not_found();
        };

        let applicable_middleware: Vec<&Arc<dyn middleware::Middleware>> = self
            .middleware
            .iter()
            .filter(|m| m.matches(&req.path))
            .map(|m| &m.middleware)
            .chain(route.middleware.iter())
            .collect();

        self.execute_chain(req, &applicable_middleware, &route.handler)
    }

    fn execute_chain(
        &self,
        req: Request,
        middleware: &[&Arc<dyn middleware::Middleware>],
        handler: &(dyn Fn(Request, &App) -> Response + Send + Sync),
    ) -> Response {
        match middleware.split_first() {
            None => handler(req, self),
            Some((first, rest)) => {
                let rest = rest.to_vec();
                let next = middleware::Next {
                    handler: Box::new(move |req, app: &App| app.execute_chain(req, &rest, handler)),
                };

                first.call(req, self, next)
            }
        }
    }

    fn match_route(&self, req: &mut Request) -> Option<&Route> {
        self.routes.iter().find_map(|route| {
            if route.method != req.method {
                return None;
            }

            let params = match_path(&route.path, &req.path)?;
            req.set_params(params);

            Some(route)
        })
    }
}

pub fn match_path(pattern: &str, path: &str) -> Option<HashMap<String, String>> {
    let pat_segments = pattern.trim_matches('/').split('/');
    let path_segments = path.trim_matches('/').split('/');

    let mut params = HashMap::new();

    for (pat, val) in pat_segments.zip(path_segments) {
        if let Some(name) = pat.strip_prefix(':') {
            params.insert(name.to_string(), val.to_string());
        } else if pat != val {
            return None;
        }
    }

    Some(params)
}
