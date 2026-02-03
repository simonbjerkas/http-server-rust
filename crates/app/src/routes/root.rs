use http_server::{App, Request, Response, get};

#[get("/")]
pub fn index(_req: Request, _ctx: &App) -> Response {
    Response::success()
}
