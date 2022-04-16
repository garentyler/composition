pub mod packets;

use crate::prelude::*;
use tokio::net::TcpStream;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum NetworkClientState {
    Disconnected,
    Handshake,
    Status,
    Login,
    Play,
}

pub struct NetworkClient {
    pub status: NetworkClientState,
    pub stream: TcpStream,
    pub buffer: VecDeque<u8>,
}
impl NetworkClient {
    pub async fn read_bytes(&mut self) -> Result<(), ()> {
        unimplemented!()
    }
}
