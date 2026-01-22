mod error;

use anyhow::Result;
use error::ServerError;

use std::{fmt::Display, str::FromStr};

pub enum StatusCode {
    Ok,
    BadRequest,
    NotFound,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gen_line = |code| format!("HTTP/1.1 {code}");

        match self {
            StatusCode::Ok => write!(f, "{} OK", gen_line(200)),
            StatusCode::BadRequest => write!(f, "{} Bad Request", gen_line(500)),
            StatusCode::NotFound => write!(f, "{} Not Found", gen_line(404)),
        }
    }
}

#[derive(Clone)]
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

pub struct RequestLine {
    protocol: Protocol,
    path: String,
}

impl RequestLine {
    pub fn build(line: String) -> Result<RequestLine> {
        let mut iter = line.split_ascii_whitespace();

        let protocol = match iter.next() {
            Some(p) => p.parse::<Protocol>()?,
            None => return Err(ServerError::BadRequestLine(line).into()),
        };

        let path = match iter.next() {
            Some(p) => p.to_string(),
            None => return Err(ServerError::BadRequestLine(line).into()),
        };

        Ok(RequestLine { protocol, path })
    }

    pub fn protocol(&self) -> Protocol {
        self.protocol.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }
}
