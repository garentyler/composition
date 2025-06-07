pub use crate::net::error::Error as NetworkError;
pub use std::io::Error as IoError;
pub use tokio::task::JoinError as TaskError;

/// This type represents all possible errors that can occur when running the server.
#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] IoError),
    #[error(transparent)]
    Task(#[from] TaskError),
    #[error(transparent)]
    Network(#[from] NetworkError),
}
