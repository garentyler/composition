/// This type represents all possible errors that can occur when running the server.
#[allow(dead_code)]
#[derive(thiserror::Error, Clone, Debug, PartialEq)]
pub enum Error {
    #[error("the server is not running")]
    NotRunning,
}

/// Alias for a Result with the error type `composition_core::server::Error`.
pub type Result<T> = std::result::Result<T, Error>;
