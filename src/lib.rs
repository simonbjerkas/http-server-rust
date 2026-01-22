mod error;

use anyhow::Result;
use error::ServerError;

use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, BufRead},
    str::FromStr,
};

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

pub struct Request {
    pub protocol: Protocol,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn build(req: impl BufRead) -> Result<Request> {
        let mut iter = req.lines();
        let Some(Ok(req_line)) = iter.next() else {
            return Err(ServerError::BadRequest.into());
        };
        let mut req_line_iter = req_line.split_ascii_whitespace();

        let protocol = match req_line_iter.next() {
            Some(p) => p.parse::<Protocol>()?,
            None => return Err(ServerError::BadRequestLine(req_line).into()),
        };

        let path = match req_line_iter.next() {
            Some(p) => p.to_string(),
            None => return Err(ServerError::BadRequestLine(req_line).into()),
        };

        let mut headers = HashMap::new();
        while let Some(header) = iter.next() {
            let Ok(header) = header else {
                return Err(ServerError::BadRequest.into());
            };
            if header.is_empty() {
                break;
            };

            let Some((key, val)) = header.split_once(':') else {
                return Err(ServerError::BadHeader(header).into());
            };

            headers.insert(key.trim().to_string(), val.trim().to_string());
        }

        let collect_string = || -> Result<String, io::Error> { iter.collect() };

        let body = collect_string()?;

        Ok(Request {
            protocol,
            path,
            headers,
            body,
        })
    }
}
