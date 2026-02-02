use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Bad request")]
    BadRequest,

    #[error("not a valid method: {0}")]
    BadMethod(String),

    #[error("not valid requestline: {0}")]
    BadRequestLine(String),

    #[error("not valid header: {0}")]
    BadHeader(String),

    #[error("Worker {0} was poisoned")]
    PoisonedWorker(usize),

    #[error("Encoding not supported: {0}")]
    BadEncoding(String),
}
