use std::sync::Arc;

use super::{App, Request, Response};

pub struct Next<'a> {
    pub handler: Box<dyn Fn(Request, &App) -> Response + 'a>,
}

impl<'a> Next<'a> {
    pub fn run(self, req: Request, app: &App) -> Response {
        (self.handler)(req, app)
    }
}

pub trait Middleware: Send + Sync {
    fn call(&self, req: Request, app: &App, next: Next<'_>) -> Response;
}

impl<F> Middleware for F
where
    F: Fn(Request, &App, Next<'_>) -> Response + Send + Sync,
{
    fn call(&self, req: Request, app: &App, next: Next<'_>) -> Response {
        self(req, app, next)
    }
}

pub enum MiddlewareScope {
    Global,
    Prefix(String),
}

pub(crate) struct ScopedMiddleware {
    pub middleware: Arc<dyn Middleware>,
    pub scope: MiddlewareScope,
}

impl ScopedMiddleware {
    pub fn matches(&self, path: &str) -> bool {
        match &self.scope {
            MiddlewareScope::Global => true,
            MiddlewareScope::Prefix(prefix) => {
                path == prefix.as_str()
                    || path.starts_with(&format!("{}/", prefix.trim_end_matches('/')))
            }
        }
    }
}
