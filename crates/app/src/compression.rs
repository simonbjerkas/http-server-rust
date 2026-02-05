use std::io::Write;

use http_server::{
    App, Request, Response,
    middleware::{Next, middleware},
};

use flate2::{Compression, write::GzEncoder};

#[middleware]
pub fn compression(req: Request, app: &App, next: Next) -> Response {
    let supports_gzip = req
        .headers
        .get("accept-encoding")
        .map(|v| v.split(',').map(str::trim).any(|enc| enc == "gzip"))
        .unwrap_or(false);

    if !supports_gzip {
        return next.run(req, app);
    }

    let mut res = next.run(req, app);

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    if encoder.write_all(&res.body).is_err() {
        eprintln!("Failed to encode body");
        return Response::bad();
    };

    let Ok(compressed) = encoder.finish() else {
        return Response::bad();
    };

    res.body = compressed;
    res.headers.insert("content-encoding", "gzip");

    res
}
