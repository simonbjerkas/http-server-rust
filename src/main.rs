use codecrafters_http_server::{Protocol, Request, StatusCode};

use std::{
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let Ok(req) = Request::build(reader) else {
        return stream
            .write_all(status_string(StatusCode::BadRequest).as_bytes())
            .unwrap();
    };

    let mut write_stream = move |res: String| {
        let res = format!("{}", res);
        stream.write_all(res.as_bytes()).unwrap();
    };

    match req.protocol {
        Protocol::Get => match req.path.as_str() {
            "/" => write_stream(status_string(StatusCode::Ok)),
            path if path.starts_with("/echo/") => {
                let (_, val) = path[1..].split_once('/').unwrap_or(("", ""));
                let res = res_string(val);

                write_stream(res)
            }
            path if path.starts_with("/user-agent") => {
                let Some(header) = req.headers.get("User-Agent") else {
                    return write_stream(status_string(StatusCode::BadRequest));
                };

                write_stream(res_string(header))
            }
            _ => write_stream(status_string(StatusCode::NotFound)),
        },
        Protocol::Post => write_stream(status_string(StatusCode::BadRequest)),
    }
}

fn status_string(status_code: StatusCode) -> String {
    format!("{status_code}\r\n\r\n")
}

fn res_string(body: &str) -> String {
    format!(
        "{}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{body}",
        StatusCode::Ok,
        body.len()
    )
}
