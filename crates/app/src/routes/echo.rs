use http_server::{App, Headers, Request, Response, get};

#[get("/echo/:msg")]
pub fn message(req: Request, _ctx: &App) -> Response {
    let msg = req.param("msg").unwrap_or("");

    let mut headers = Headers::new();
    headers.set_content_type(http_server::headers::ContentType::Text);

    Response::ok(headers, msg)
}
