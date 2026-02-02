use std::str::FromStr;

use super::{App, ServerError, request::Request, response::Response};

pub enum StatusCode {
    Ok,
    BadRequest,
    NotFound,
    Created,
}

impl StatusCode {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.phrase().as_bytes().to_vec()
    }

    ///Generate statuscode string
    ///
    /// creates the expected string for the statuscode, i.e. `HTTP 1.1 200 OK`
    fn phrase(&self) -> String {
        let gen_line = |code| format!("HTTP/1.1 {code}");

        match self {
            StatusCode::Ok => format!("{} OK", gen_line(200)),
            StatusCode::BadRequest => format!("{} Bad Request", gen_line(500)),
            StatusCode::NotFound => format!("{} Not Found", gen_line(404)),
            Self::Created => format!("{} Created", gen_line(201)),
        }
    }
}

#[derive(PartialEq)]
pub enum Method {
    Get,
    Post,
}

impl FromStr for Method {
    type Err = ServerError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            _ => Err(ServerError::BadMethod(s.to_string())),
        }
    }
}

pub struct Route {
    pub method: Method,
    pub path: &'static str,
    pub handler: fn(Request, &App) -> Response,
}

pub trait Routable {
    fn route() -> Route;
}
