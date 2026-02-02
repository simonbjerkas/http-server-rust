mod error;
mod http;
mod server;

use error::ServerError;

pub use http::{
    headers::{self, Headers},
    request::Request,
    response::Response,
    types::{Method, Routable, Route, StatusCode},
};
pub use server::Server;

pub use http_server_macros::{get, post};

use std::{collections::HashMap, path::PathBuf, sync::Arc};

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
}

impl App {
    ///Create a new app
    ///
    ///creates a new app with default configs.
    ///
    ///the service containng routing data for your server.
    pub fn new() -> App {
        let config = Config::default();

        App {
            config: Arc::new(config),
            routes: Vec::new(),
        }
    }

    ///Create a new app
    ///
    ///creates a new app with supplied configuration.
    ///
    ///the service containing routing data for your server.
    pub fn with_config(config: Config) -> App {
        App {
            config: Arc::new(config),
            routes: Vec::new(),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    ///Registers a service
    ///
    ///takes your current app with services and returns a new one with the new service attached
    pub fn service<T: Routable>(mut self, _: T) -> App {
        self.routes.push(T::route());
        self
    }

    pub fn handle(&self, mut req: Request) -> Response {
        for route in &self.routes {
            if route.method == req.method {
                if let Some(params) = match_path(route.path, &req.path) {
                    req.set_params(params);
                    return (route.handler)(req, self);
                }
            }
        }

        Response::not_found()
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
