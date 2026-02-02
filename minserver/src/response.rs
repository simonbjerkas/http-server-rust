use super::{StatusCode, headers::Headers};

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

    ///Prepare rsponse to be sent
    ///
    ///should be called before sending the response
    ///
    /// Adds `Content-Lenght: [body.len()]` header to the response
    pub fn finalize(&mut self) {
        self.headers
            .insert("Content-Length", &self.body.len().to_string());
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

    ///Ok
    ///
    ///creates a default empty `200 Ok` response
    pub fn success() -> Response {
        Response {
            status: StatusCode::Ok,
            headers: Headers::default(),
            body: Vec::new(),
        }
    }

    ///Bad Request
    ///
    ///creates an empty `500 Bad Request` response
    pub fn bad() -> Response {
        Response {
            status: StatusCode::BadRequest,
            headers: Headers::default(),
            body: Vec::new(),
        }
    }

    ///Not found
    ///
    /// creates an empty `404 Not Found`response
    pub fn not_found() -> Response {
        Response {
            status: StatusCode::NotFound,
            headers: Headers::default(),
            body: Vec::new(),
        }
    }

    ///Created
    ///
    /// creates an empty `201 Created` response
    pub fn created() -> Response {
        Response {
            status: StatusCode::Created,
            headers: Headers::default(),
            body: Vec::new(),
        }
    }
}
