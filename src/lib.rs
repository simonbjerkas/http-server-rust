mod error;
mod pool;
mod request;
mod response;

pub use pool::ThreadPool;
pub use request::Request;
pub use response::Response;

use std::{collections::HashMap, fmt::Display, str::FromStr};

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

#[derive(Debug, Clone)]
pub enum ContentType {
    Text,
    File,
}

impl FromStr for ContentType {
    type Err = ServerError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "application/octet-stream" => Ok(ContentType::File),
            "text/plain" => Ok(ContentType::Text),
            other => Err(ServerError::BadHeader(other.to_string())),
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::File => write!(f, "application/octet-stream"),
            ContentType::Text => write!(f, "text/plain"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Headers {
    pub content_length: Option<usize>,
    pub content_type: Option<ContentType>,
    others: HashMap<String, String>,
}

impl Headers {
    pub fn new(
        content_length: Option<usize>,
        content_type: Option<ContentType>,
        others: HashMap<String, String>,
    ) -> Headers {
        Headers {
            content_length,
            content_type,
            others,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();

        let mut headers = self.others.clone();
        if let Some(content_lenght) = self.content_length {
            headers.insert(String::from("Content-Length"), content_lenght.to_string());
        }
        if let Some(content_type) = self.content_type.as_ref() {
            headers.insert(
                String::from("Content-Type"),
                content_type.clone().to_string(),
            );
        }

        for (key, val) in headers {
            out.extend_from_slice(format!("{key}: {val}\r\n").as_bytes());
        }

        out
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.others.get(key)
    }
}
