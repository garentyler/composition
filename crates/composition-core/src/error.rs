/// This type represents all possible errors that can occur when running the server.
#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("the server is not running")]
    NotRunning,
    #[error("an internal channel got disconnected")]
    ChannelDisconnected,
    #[error("the server could not bind to the given address")]
    Bind,
    #[error(transparent)]
    Protocol(composition_protocol::Error),
    /// This error is general purpose.
    /// When possible, other error variants should be used.
    #[error("custom error: {0}")]
    Message(String),
}

/// Alias for a Result with the error type `composition_core::server::Error`.
pub type Result<T> = std::result::Result<T, Error>;
