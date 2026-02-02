use std::{collections::HashMap, io::BufRead};

use anyhow::Result;

use super::{Method, ServerError, headers::Headers};

pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Headers,
    pub body: Vec<u8>,
    params: HashMap<String, String>,
}

impl Request {
    pub fn build(mut req: impl BufRead) -> Result<Request> {
        let mut req_line = String::new();
        req.read_line(&mut req_line)?;
        let req_line = req_line.trim_end_matches(&['\r', '\n']);

        let mut req_line_iter = req_line.split_ascii_whitespace();

        let method = match req_line_iter.next() {
            Some(p) => p.parse::<Method>()?,
            None => return Err(ServerError::BadRequestLine(req_line.to_string()).into()),
        };

        let path = match req_line_iter.next() {
            Some(p) => p.to_string(),
            None => return Err(ServerError::BadRequestLine(req_line.to_string()).into()),
        };

        let mut line = String::new();
        let mut headers = Headers::new();
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

            headers.insert(key, val);
        }

        let content_length = headers
            .get("Content-Length")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);

        let mut body = vec![0u8; content_length];
        req.read_exact(&mut body)?;

        Ok(Request {
            method,
            path,
            headers,
            body,
            params: HashMap::new(),
        })
    }

    pub fn param(&self, name: &str) -> Option<&str> {
        self.params.get(name).map(|s| s.as_str())
    }

    pub fn set_params(&mut self, params: HashMap<String, String>) {
        self.params = params;
    }
}
