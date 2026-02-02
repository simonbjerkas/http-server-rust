use http_server::{App, Headers, Request, Response, StatusCode, get, headers::ContentType};

#[get("/user-agent")]
pub fn user_agent(req: Request, _ctx: &App) -> Response {
    let Some(val) = req.headers.user_agent() else {
        return Response::bad();
    };

    let mut headers = Headers::new();
    headers.set_content_type(ContentType::Text);

    let mut res = Response::new(StatusCode::Ok, headers, val.to_string());
    res.finalize();

    res
}
