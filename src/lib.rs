pub mod config;
pub mod net;
pub mod server;

use crate::config::Config;
use once_cell::sync::OnceCell;
use std::time::Instant;

pub static START_TIME: OnceCell<Instant> = OnceCell::new();

/// Start the server.
#[tracing::instrument]
pub async fn start_server() -> (server::Server, tokio_util::sync::CancellationToken) {
    server::Server::new(format!("0.0.0.0:{}", Config::instance().port)).await
}

pub mod prelude {
    pub use crate::config::Config;
    pub use crate::START_TIME;
    pub use composition_protocol::{Chat, Json, Uuid};
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::json;
    pub use std::collections::VecDeque;
    pub use std::io::{Read, Write};
    pub use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
    pub use tracing::{debug, error, info, trace, warn};
    #[derive(Clone, Debug, PartialEq)]
    pub enum ParseError {
        NotEnoughData,
        InvalidData,
        VarIntTooBig,
    }
    pub type ParseResult<T> = Result<(T, usize), ParseError>;
}
