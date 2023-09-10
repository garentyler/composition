use crate::prelude::Chat;

/// This type represents all possible errors that can occur in the Minecraft protocol.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// This error was caused by invalid data.
    #[error("invalid data")]
    InvalidData,
    /// This error was caused by receiving an unexpected packet.
    #[error("received unexpected packet")]
    UnexpectedPacket,
    /// This error was caused by prematurely reaching the end of the input data.
    #[error("unexpected end of file")]
    Eof,
    /// The connection did not receive data and timed out.
    #[error("stream timed out")]
    Timeout,
    /// This error was caused by attempting to send or receive data from a disconnected client.
    #[error("communicating to disconnected client")]
    Disconnected,
    /// This error is a wrapper for `std::io::Error`.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// This error occurs when there is an issue with the tokio runtime.
    #[cfg(feature = "codec")]
    #[error("tokio runtime error")]
    TokioRuntimeError,
}
impl Error {
    pub fn to_disconnection_reason(&self) -> Option<Chat> {
        match self {
            Error::InvalidData => Some(Chat::basic("The server received invalid data.")),
            Error::UnexpectedPacket => {
                Some(Chat::basic("The server received an unexpected packet."))
            }
            Error::Eof => Some(Chat::basic(
                "The server unexpectedly reached the end of the data.",
            )),
            Error::Timeout => Some(Chat::basic("The connection timed out.")),
            _ => None,
        }
    }
}
impl From<composition_parsing::Error> for Error {
    fn from(value: composition_parsing::Error) -> Self {
        use composition_parsing::Error;
        match value {
            Error::InvalidData => Self::InvalidData,
            Error::Eof => Self::Eof,
            Error::VarIntTooLong => Self::InvalidData,
            Error::InvalidJson(_) => Self::InvalidData,
        }
    }
}

/// Alias for a Result with the error type `composition_protocol::Error`.
pub type Result<T> = std::result::Result<T, Error>;
