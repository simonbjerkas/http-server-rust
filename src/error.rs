use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Bad request")]
    BadRequest,

    #[error("not a valid protocol: {0}")]
    BadProtocol(String),

    #[error("not valid requestline: {0}")]
    BadRequestLine(String),

    #[error("not valid header: {0}")]
    BadHeader(String),
}
