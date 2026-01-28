use codecrafters_http_server::{
    ContentType, Headers, Protocol, Request, Response, StatusCode, ThreadPool,
};

use std::{
    collections::HashMap,
    env, fs,
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    sync::Arc,
};

struct Config {
    directory: PathBuf,
}

impl Config {
    fn new(mut iter: impl Iterator<Item = String>) -> Config {
        let mut parse_flag = |flag: &str| -> Option<String> {
            while let Some(arg) = iter.next() {
                if arg == flag {
                    return iter.next();
                }
            }
            None
        };

        let path = parse_flag("--directory").unwrap_or(String::from("."));
        let directory = PathBuf::from(path);

        Config { directory }
    }
}

fn main() {
    let args = env::args();
    let config = Config::new(args);
    let config = Arc::new(config);

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let config = Arc::clone(&config);

                pool.execute(|| {
                    handle_connection(stream, config);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, config: Arc<Config>) {
    let reader = BufReader::new(&mut stream);
    let Ok(req) = Request::build(reader) else {
        return stream.write_all(&Response::bad().to_bytes()).unwrap();
    };

    let mut write_stream = |res: Response| {
        stream.write_all(&res.to_bytes()).unwrap();
    };

    match req.protocol {
        Protocol::Get => match req.path.as_str() {
            "/" => write_stream(Response::empty()),
            path if path.starts_with("/echo/") => {
                let val = path.strip_prefix("/echo/").unwrap_or("");
                let headers =
                    Headers::new(Some(val.len()), Some(ContentType::Text), HashMap::new());
                let res = Response::new(StatusCode::Ok, headers, val.to_string());

                write_stream(res)
            }
            path if path.starts_with("/user-agent") => {
                let Some(val) = req.headers.get("User-Agent") else {
                    return write_stream(Response::bad());
                };

                let headers =
                    Headers::new(Some(val.len()), Some(ContentType::Text), HashMap::new());
                let res = Response::new(StatusCode::Ok, headers, val.to_string());

                write_stream(res)
            }
            path if path.starts_with("/files/") => {
                let Some(file_name) = path.strip_prefix("/files/") else {
                    return write_stream(Response::not_found());
                };
                let mut file_path = PathBuf::from(config.directory.clone());
                file_path.push(file_name);

                let Ok(mut file) = fs::File::open(file_path) else {
                    return write_stream(Response::not_found());
                };

                let mut content = Vec::new();
                if let Err(_) = file.read_to_end(&mut content) {
                    return write_stream(Response::bad());
                };

                let headers =
                    Headers::new(Some(content.len()), Some(ContentType::File), HashMap::new());
                let res = Response::new(StatusCode::Ok, headers, content);

                write_stream(res)
            }
            _ => write_stream(Response::not_found()),
        },
        Protocol::Post => write_stream(Response::bad()),
    };
}
