#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid syntax")]
    Syntax,
    #[error("unexpected end of file")]
    Eof,
    #[error("VarInt was more than 5 bytes")]
    VarIntTooLong,
    #[error(transparent)]
    InvalidJson(#[from] serde_json::Error),
    #[error("custom error: {0}")]
    Message(String),
}

pub type Result<T> = std::result::Result<T, Error>;
pub type ParseResult<'data, T> = Result<(&'data [u8], T)>;
