/// This type represents all possible errors that can occur when serializing or deserializing Minecraft data.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// This error was caused by unexpected or invalid data.
    #[error("invalid syntax")]
    Syntax,
    /// This error was caused by prematurely reaching the end of the input data.
    #[error("unexpected end of file")]
    Eof,
    /// This error was caused by reading a `composition_parsing::VarInt` that was longer than 5 bytes.
    #[error("VarInt was more than 5 bytes")]
    VarIntTooLong,
    /// This error is a wrapper for `serde_json::Error`.
    #[error(transparent)]
    InvalidJson(#[from] serde_json::Error),
    /// This error is general purpose.
    /// When possible, other error variants should be used.
    #[error("custom error: {0}")]
    Message(String),
}

/// Alias for a Result with the error type `composition_parsing::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// Alias for a Result that helps with zero-copy parsing.
///
/// The error type is `composition_parsing::Error`,
/// and the result type is a tuple of the remaining bytes and the parsed item.
pub type ParseResult<'data, T> = Result<(&'data [u8], T)>;
