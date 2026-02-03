use http_server::{App, Request, Response, StatusCode, get, headers, post};

use std::{
    fs,
    io::{Read, Write},
    path::{Component, Path, PathBuf},
};

fn safe_join(base: &Path, rel: &str) -> Option<PathBuf> {
    let rel_path = Path::new(rel);

    if rel_path.is_absolute() {
        return None;
    }

    if rel_path
        .components()
        .any(|c| matches!(c, Component::ParentDir | Component::Prefix(_)))
    {
        return None;
    }

    Some(base.join(rel_path))
}

#[get("/files/:path")]
pub fn read(req: Request, ctx: &App) -> Response {
    let Some(file_name) = req.param("path") else {
        return Response::not_found();
    };

    let base = &ctx.config().directory;
    let Some(file_path) = safe_join(base, file_name) else {
        return Response::bad();
    };

    let Ok(mut file) = fs::File::open(file_path) else {
        return Response::not_found();
    };

    let mut content = Vec::new();
    if let Err(_) = file.read_to_end(&mut content) {
        return Response::bad();
    };

    let mut headers = headers::Headers::new();
    headers.set_content_type(headers::ContentType::File);

    let mut res = Response::new(StatusCode::Ok, headers, content);
    res.finalize();

    res
}

#[post("/files/:path")]
pub fn upload(req: Request, ctx: &App) -> Response {
    let Some(headers::ContentType::File) = req.headers.content_type() else {
        eprintln!("Missing header");
        return Response::bad();
    };
    let Some(file_name) = req.param("path") else {
        return Response::not_found();
    };

    let base = &ctx.config().directory;
    let Some(file_path) = safe_join(base, file_name) else {
        eprintln!("Safe join fails");
        return Response::bad();
    };

    let Ok(mut file) = fs::File::create(file_path) else {
        eprintln!("fails to create file");
        return Response::bad();
    };

    if let Err(_) = file.write_all(&req.body) {
        eprintln!("fails to write file");
        return Response::bad();
    }

    Response::created()
}
