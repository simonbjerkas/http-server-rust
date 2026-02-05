use http_server::{
    App, Request, Response,
    middleware::{Next, middleware},
};

#[middleware]
pub fn compression(req: Request, app: &App, next: Next) -> Response {
    let Some(encoding) = req.headers.get("accept-encoding") else {
        return next.run(req, app);
    };

    if encoding != "gzip" {
        return next.run(req, app);
    }

    let mut res = next.run(req, app);

    res.headers.insert("content-encoding", "gzip");

    res
}
