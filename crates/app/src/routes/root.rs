use http_server::{App, Request, Response, get};

#[get("/")]
pub fn root(_req: Request, _ctx: &App) -> Response {
    Response::success()
}
