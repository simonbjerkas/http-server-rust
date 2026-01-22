mod error;

use anyhow::Result;
use error::ServerError;

use std::{collections::HashMap, fmt::Display, io::BufRead, str::FromStr};

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

#[derive(Debug)]
pub struct Request {
    pub protocol: Protocol,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn build(mut req: impl BufRead) -> Result<Request> {
        let mut req_line = String::new();
        req.read_line(&mut req_line)?;
        let req_line = req_line.trim_end_matches(&['\r', '\n']);

        let mut req_line_iter = req_line.split_ascii_whitespace();

        let protocol = match req_line_iter.next() {
            Some(p) => p.parse::<Protocol>()?,
            None => return Err(ServerError::BadRequestLine(req_line.to_string()).into()),
        };

        let path = match req_line_iter.next() {
            Some(p) => p.to_string(),
            None => return Err(ServerError::BadRequestLine(req_line.to_string()).into()),
        };

        let mut line = String::new();
        let mut headers = HashMap::new();
        loop {
            line.clear();
            req.read_line(&mut line)?;

            let line = line.trim_end_matches(['\r', '\n']);
            if line.is_empty() {
                break;
            };

            let (key, val) = line
                .split_once(':')
                .ok_or(ServerError::BadHeader(line.to_string()))?;

            headers.insert(key.trim().to_string(), val.trim().to_string());
        }

        let content_length = headers
            .get("Content-Lenght")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);

        let mut body = Vec::with_capacity(content_length);
        req.read_exact(&mut body)?;

        let body = String::from_utf8(body)?;

        Ok(Request {
            protocol,
            path,
            headers,
            body,
        })
    }
}
