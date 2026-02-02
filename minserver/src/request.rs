use std::io::BufRead;

use anyhow::Result;

use super::{Protocol, ServerError, headers::Headers};

#[derive(Debug)]
pub struct Request {
    pub protocol: Protocol,
    pub path: String,
    pub headers: Headers,
    pub body: Vec<u8>,
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
            protocol,
            path,
            headers,
            body,
        })
    }
}
