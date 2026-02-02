use std::{fmt::Display, str::FromStr};

use super::ServerError;

pub enum Encoding {
    Gzip,
}

impl FromStr for Encoding {
    type Err = ServerError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gzip" => Ok(Encoding::Gzip),
            other => Err(ServerError::BadEncoding(other.to_string())),
        }
    }
}

impl Display for Encoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Encoding::Gzip => write!(f, "gzip"),
        }
    }
}
