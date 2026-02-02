use std::{fmt::Display, str::FromStr};

use super::ServerError;

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
