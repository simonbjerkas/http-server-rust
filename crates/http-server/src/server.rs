pub mod middleware;
pub mod pool;
pub mod scope;

use std::{
    io::{BufReader, Write},
    net::TcpListener,
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
            let mut stream = stream?;
            let app = Arc::clone(&app);

            pool.execute(move || {
                let reader = BufReader::new(&mut stream);
                let request = match Request::build(reader) {
                    Ok(req) => req,
                    Err(_) => {
                        let res = Response::bad();
                        stream.write_all(&res.to_bytes()).unwrap();
                        return;
                    }
                };

                let mut res = app.handle(request);
                res.finalize();

                stream.write_all(&res.to_bytes()).unwrap();
            });
        }
        Ok(())
    }
}
