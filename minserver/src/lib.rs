mod error;
mod pool;
mod request;
mod response;

pub mod headers;

pub use pool::ThreadPool;
pub use request::Request;
pub use response::Response;

use std::str::FromStr;

use anyhow::Result;
use error::ServerError;

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

#[derive(Clone, Debug)]
pub enum Protocol {
    Get,
    Post,
}

impl FromStr for Protocol {
    type Err = ServerError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Protocol::Get),
            "POST" => Ok(Protocol::Post),
            _ => Err(ServerError::BadProtocol(s.to_string())),
        }
    }
}
