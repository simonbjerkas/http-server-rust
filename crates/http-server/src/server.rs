pub mod middleware;
pub mod pool;
pub mod scope;

use std::{
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use anyhow::Result;
use pool::ThreadPool;

use super::{App, Request, Response, Route, ServerError};

pub struct Server {
    app: App,
}

impl Server {
    pub fn new(app: App) -> Server {
        Server { app }
    }

    pub fn run(self) -> Result<()> {
        let listener = TcpListener::bind("127.0.0.1:4221")?;
        let pool = ThreadPool::new(10);

        let app = Arc::new(self.app);

        for stream in listener.incoming() {
            let stream = stream?;
            let app = Arc::clone(&app);

            pool.execute(move || handle_connection(stream, &app));
        }
        Ok(())
    }
}

fn handle_connection(mut stream: TcpStream, app: &App) {
    if process_request(&mut stream, app).is_err() {
        let res = Response::bad();
        let _ = stream.write_all(&res.to_bytes());
    }
}

fn process_request(stream: &mut TcpStream, app: &App) -> Result<()> {
    loop {
        let reader = BufReader::new(&*stream);
        let req = Request::build(reader)?;

        let close = req.headers.close_connection();
        let mut res = app.handle(req);

        if close {
            res.headers.insert("connection", "close");
        }

        res.finalize();
        stream.write_all(&res.to_bytes())?;

        if close {
            break;
        }
    }

    Ok(())
}
