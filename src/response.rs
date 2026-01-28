use super::{Headers, StatusCode};

pub struct Response {
    status: StatusCode,
    headers: Headers,
    body: Vec<u8>,
}

impl Response {
    pub fn new<B>(status: StatusCode, headers: Headers, body: B) -> Response
    where
        B: Into<Vec<u8>>,
    {
        Response {
            status,
            headers,
            body: body.into(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend(self.status.to_bytes());
        out.extend_from_slice(b"\r\n");

        out.extend(self.headers.to_bytes());

        out.extend_from_slice(b"\r\n");

        out.extend(self.body.clone());

        out
    }

    pub fn empty() -> Response {
        Response {
            status: StatusCode::Ok,
            headers: Headers::default(),
            body: Vec::new(),
        }
    }

    pub fn bad() -> Response {
        Response {
            status: StatusCode::BadRequest,
            headers: Headers::default(),
            body: Vec::new(),
        }
    }

    pub fn not_found() -> Response {
        Response {
            status: StatusCode::NotFound,
            headers: Headers::default(),
            body: Vec::new(),
        }
    }
}
