use std::{collections::HashMap, io::BufRead};

use anyhow::Result;

use crate::ContentType;

use super::{Headers, Protocol, ServerError};

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
        let mut others = HashMap::new();
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

            others.insert(key.trim().to_string(), val.trim().to_string());
        }

        let content_length = others
            .remove("Content-Lenght")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);
        let content_type = others
            .remove("Content-Type")
            .and_then(|v| v.parse::<ContentType>().ok());

        let headers = Headers::new(Some(content_length), content_type, others);

        let mut body = Vec::with_capacity(content_length);
        req.read_exact(&mut body)?;

        Ok(Request {
            protocol,
            path,
            headers,
            body,
        })
    }
}
