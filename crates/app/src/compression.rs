use std::io::Write;

use http_server::{
    App, Request, Response,
    middleware::{Next, middleware},
};

use flate2::{Compression, write::GzEncoder};

#[middleware]
pub fn compression(req: Request, app: &App, next: Next) -> Response {
    let Some(encoding) = req.headers.get("accept-encoding") else {
        return next.run(req, app);
    };

    if !encoding
        .split(',')
        .map(|v| v.trim())
        .collect::<Vec<_>>()
        .contains(&"gzip")
    {
        return next.run(req, app);
    }

    let mut res = next.run(req, app);

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    if let Err(e) = encoder.write_all(&mut res.body) {
        eprintln!("Failed to encode body: {e:?}");
        return Response::bad();
    };
    let Ok(compressed) = encoder.finish() else {
        return Response::bad();
    };

    res.body = compressed;
    res.headers.insert("content-encoding", "gzip");

    res
}
