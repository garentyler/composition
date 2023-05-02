pub mod packet;
pub mod util;

use thiserror::Error;

pub type Json = serde_json::Value;
pub type Chat = Json;
pub type Uuid = u128;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ClientState {
    Handshake,
    Status,
    Login,
    Play,
    Disconnected,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}
impl TryFrom<u8> for Difficulty {
    type Error = ();
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Difficulty::Peaceful),
            1 => Ok(Difficulty::Easy),
            2 => Ok(Difficulty::Normal),
            3 => Ok(Difficulty::Hard),
            _ => Err(()),
        }
    }
}

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
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
pub type Result<T> = std::result::Result<T, ProtocolError>;
