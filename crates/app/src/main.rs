mod compression;
mod routes;

use routes::{echo, files, root, user_agent};

use std::{env, path::PathBuf};

use http_server::{App, Config, Server};

fn main() {
    let mut args = env::args();

    let mut parse_flag = |flag: &str| -> Option<String> {
        while let Some(arg) = args.next() {
            if arg == flag {
                return args.next();
            }
        }
        None
    };

    let config = match parse_flag("--directory") {
        Some(dir) => Config::new(PathBuf::from(dir)),
        None => Config::default(),
    };

    let app = App::with_config(config)
        .service(root::index)
        .service(echo::message)
        .service(user_agent::agent)
        .service(files::read)
        .service(files::upload);

    if let Err(e) = Server::new(app).run() {
        eprintln!("error: {e}");
    }
}
