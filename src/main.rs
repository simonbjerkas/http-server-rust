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

    let response = match reader.lines().next() {
        Some(Ok(line)) => {
            if line == "GET / HTTP 1.1" {
                String::from("HTTP/1.1 200 OK\r\n\r\n")
            } else {
                String::from("HTTP/1.1 404 Not Found\r\n\r\n")
            }
        }
        _ => String::from("HTTP/1.1 500 Bad Request\r\n\r\n"),
    };

    stream.write_all(response.as_bytes()).unwrap();
}
