/// This type represents all possible errors that can occur in the Minecraft protocol.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// This error was caused by unexpected or invalid data.
    #[error("invalid syntax")]
    Syntax,
    /// This error was caused by prematurely reaching the end of the input data.
    #[error("unexpected end of file")]
    Eof,
    /// The connection did not receive data and timed out.
    #[error("stream timed out")]
    Timeout,
    /// This error was caused by attempting to send or receive data from a disconnected client.
    #[error("communicating to disconnected client")]
    Disconnected,
    /// This error is a wrapper for `crate::parsing::Error`.
    #[error(transparent)]
    ParseError(#[from] crate::protocol::parsing::Error),
    /// This error is general purpose.
    /// When possible, other error variants should be used.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Alias for a Result with the error type `composition_protocol::Error`.
pub type Result<T> = std::result::Result<T, Error>;
