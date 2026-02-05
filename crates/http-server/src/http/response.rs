use super::{StatusCode, headers::Headers};

pub struct Response {
    status: StatusCode,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Response {
    ///Create a new Response
    ///
    ///takes statuscode, headers and a body, where `Into<Vec<u8>>`is required on the body.
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
    pub(crate) fn finalize(&mut self) {
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
    ///creates a new response with a `200 OK`.
    ///
    ///Rquire a body where `Into<Vec<u8>>` is satisfied, as well as the headers expected for the response.
    pub fn ok<B>(headers: Headers, body: B) -> Response
    where
        B: Into<Vec<u8>>,
    {
        Response {
            status: StatusCode::Ok,
            headers,
            body: body.into(),
        }
    }

    ///Sucess
    ///
    ///creates a default empty `200 OK` response
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
