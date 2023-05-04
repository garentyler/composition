#![deny(clippy::all)]

pub mod blocks;
pub mod entities;
pub mod inventory;
pub mod mctypes;
pub mod packets;

use thiserror::Error;

pub use composition_parsing::ClientState;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("invalid data")]
    InvalidData,
    #[error("not enough data")]
    NotEnoughData,
    #[error("stream timed out")]
    Timeout,
    #[error("communicating to disconnected client")]
    Disconnected,
    #[error(transparent)]
    ParseError(#[from] composition_parsing::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
pub type Result<T> = std::result::Result<T, ProtocolError>;
