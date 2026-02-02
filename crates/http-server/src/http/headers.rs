mod content_type;
mod encoding;

pub use content_type::ContentType;
use encoding::Encoding;

use std::collections::HashMap;

use super::ServerError;

#[derive(Debug, Default)]
pub struct Headers {
    entries: HashMap<String, String>,
}

impl Headers {
    pub fn new() -> Headers {
        Headers {
            entries: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.entries
            .get(&name.trim().to_ascii_lowercase())
            .map(|s| s.as_str())
    }

    pub fn insert(&mut self, k: &str, v: &str) {
        self.entries
            .insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();

        for (key, val) in &self.entries {
            out.extend_from_slice(format!("{key}: {val}\r\n").as_bytes());
        }

        out
    }
}

impl Headers {
    // ----- request helpers -----

    pub fn user_agent(&self) -> Option<&str> {
        self.entries.get("user-agent").map(|s| s.as_str())
    }

    pub fn content_type(&self) -> Option<ContentType> {
        self.entries
            .get("content-type")
            .and_then(|v| v.parse().ok())
    }
}

impl Headers {
    // ----- reponse helpers -----

    pub fn set_content_type(&mut self, content_type: ContentType) {
        let key = "Content-Type";
        match content_type {
            ContentType::File => self.insert(key, &ContentType::File.to_string()),
            ContentType::Text => self.insert(key, &ContentType::Text.to_string()),
        };
    }

    pub fn set_content_encoding(&mut self, enc: Encoding) {
        let key = "Content-Encoding";
        match enc {
            Encoding::Gzip => self.insert(key, &Encoding::Gzip.to_string()),
        }
    }
}
