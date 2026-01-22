use codecrafters_http_server::{Protocol, RequestLine, StatusCode};

use std::{
    io::{BufRead, BufReader, Write},
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
    let request_line = reader.lines().next();

    let mut write_stream = move |res: String| {
        let res = format!("{}", res);
        stream.write_all(res.as_bytes()).unwrap();
    };

    let Some(Ok(line)) = request_line else {
        return write_stream(StatusCode::BadRequest.to_string());
    };

    let Ok(request_line) = RequestLine::build(line) else {
        return write_stream(StatusCode::BadRequest.to_string());
    };

    match request_line.protocol() {
        Protocol::Get => match request_line.path().as_str() {
            "/" => write_stream(status_string(StatusCode::Ok)),
            path if path.starts_with("/echo/") => {
                let (_, val) = path[1..].split_once('/').unwrap_or(("", ""));
                let res = format!(
                    "{}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{val}",
                    StatusCode::Ok,
                    val.len()
                );

                write_stream(res)
            }
            _ => write_stream(status_string(StatusCode::NotFound)),
        },
        Protocol::Post => write_stream(status_string(StatusCode::BadRequest)),
    }
}

fn status_string(status_code: StatusCode) -> String {
    format!("{status_code}\r\n\r\n")
}
