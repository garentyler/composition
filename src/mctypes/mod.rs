#![allow(dead_code)]

/// Helper functions.
pub mod functions;
/// All the numbers, from `i8` and `u8` to `i64` and `u64`, plus `VarInt`s.
pub mod numbers;
/// The other types, (booleans and strings).
pub mod other;

pub use functions::*;
pub use numbers::*;
pub use other::*;
use serde_json::json;
use std::{
    clone::Clone,
    convert::{Into, TryFrom},
    fmt::{Debug, Display},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

// /// Make sure all types can serialize and deserialize to/from `Vec<u8>`.
// pub trait MCType: Into<Vec<u8>> + TryFrom<Vec<u8>> + Display {
//     pub async fn read(_stream: &mut TcpStream) -> tokio::io::Result<Self>;
// }

pub enum MCTypeError {
    NetworkError(tokio::io::Error),
    NotEnoughData,
    ParseError,
}

#[async_trait::async_trait]
pub trait MCType: Into<Vec<u8>> + TryFrom<Vec<u8>> + Clone + Debug + Display {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, MCTypeError>;
    async fn read(t: &mut TcpStream) -> Result<Self, MCTypeError>;
    async fn write(t: &mut TcpStream) -> tokio::io::Result<()>;
}
