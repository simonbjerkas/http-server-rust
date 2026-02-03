use http_server::{App, Headers, Request, Response, get, headers::ContentType};

#[get("/user-agent")]
pub fn agent(req: Request, _ctx: &App) -> Response {
    let Some(val) = req.headers.user_agent() else {
        return Response::bad();
    };

    let mut headers = Headers::new();
    headers.set_content_type(ContentType::Text);

    Response::ok(headers, val)
}
