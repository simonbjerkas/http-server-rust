use minserver::{Protocol, Request, Response, StatusCode, ThreadPool, headers};

use std::{
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

    let mut write_stream = |mut res: Response| {
        res.finalize();
        stream.write_all(&res.to_bytes()).unwrap();
    };

    match req.protocol {
        Protocol::Get => match req.path.as_str() {
            "/" => write_stream(Response::success()),
            path if path.starts_with("/echo/") => {
                let val = path.strip_prefix("/echo/").unwrap_or("");

                let mut headers = headers::Headers::new();
                headers.set_content_type(headers::ContentType::Text);

                let res = Response::new(StatusCode::Ok, headers, val.to_string());

                write_stream(res)
            }
            path if path.starts_with("/user-agent") => {
                let Some(val) = req.headers.user_agent() else {
                    return write_stream(Response::bad());
                };

                let mut headers = headers::Headers::new();
                headers.set_content_type(headers::ContentType::Text);

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

                let mut headers = headers::Headers::new();
                headers.set_content_type(headers::ContentType::File);

                let res = Response::new(StatusCode::Ok, headers, content);

                write_stream(res)
            }
            _ => write_stream(Response::not_found()),
        },
        Protocol::Post => match req.path.as_str() {
            path if path.starts_with("/files/") => {
                let Some(file_name) = path.strip_prefix("/files/") else {
                    return write_stream(Response::not_found());
                };

                let Some(headers::ContentType::File) = req.headers.content_type() else {
                    return write_stream(Response::bad());
                };

                let mut file_path = PathBuf::from(config.directory.clone());
                file_path.push(file_name);

                let Ok(mut file) = fs::File::create(file_path) else {
                    return write_stream(Response::bad());
                };

                if let Err(_) = file.write_all(&req.body) {
                    return write_stream(Response::bad());
                }

                write_stream(Response::created())
            }
            _ => write_stream(Response::not_found()),
        },
    };
}
