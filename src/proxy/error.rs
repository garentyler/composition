pub use std::io::Error as IoError;
pub use tokio::task::JoinError as TaskError;
pub use crate::net::error::Error as NetworkError;

/// This type represents all possible errors that can occur when running the proxy.
#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(IoError),
    #[error(transparent)]
    Task(TaskError),
    #[error(transparent)]
    Network(NetworkError),
}
