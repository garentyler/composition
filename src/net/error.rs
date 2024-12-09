pub use std::io::Error as IoError;

/// This type represents all possible errors that can occur in the network.
#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(IoError),
    #[error("There was an error parsing data")]
    Parsing,
    #[error("Internal channel disconnected")]
    ConnectionChannelDisconnnection,
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}
