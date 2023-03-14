pub mod mctypes;
pub mod packets;

use crate::prelude::*;
use mctypes::*;
pub use packets::Packet;
use std::time::Instant;
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
    pub id: u128,
    pub state: NetworkClientState,
    pub stream: TcpStream,
    pub buffer: VecDeque<u8>,
    pub packet_queue: VecDeque<Packet>,
    pub last_data_time: Instant,
}
impl NetworkClient {
    pub fn new(id: u128, stream: TcpStream) -> NetworkClient {
        NetworkClient {
            id,
            state: NetworkClientState::Handshake,
            stream,
            buffer: VecDeque::new(),
            packet_queue: VecDeque::new(),
            last_data_time: Instant::now(),
        }
    }
    pub async fn read_data(&mut self) -> Result<(), tokio::io::Error> {
        trace!("NetworkClient.read_data() id {}", self.id);
        // Try to read 4kb at a time until there is no more data.
        loop {
            let mut buf = [0; 4096];
            match self.stream.try_read(&mut buf) {
                // There is no data available.
                Ok(0) => break,
                // Data was read.
                Ok(n) => {
                    trace!("Setting last_data_time for client {}", self.id);
                    self.last_data_time = Instant::now();
                    self.buffer.extend(&buf[0..n]);
                    debug!("Read {} bytes from client {}", n, self.id);
                }
                Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => break,
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(())
    }
    pub fn read_packet(&mut self) -> Result<(), ParseError> {
        trace!("NetworkClient.read_packet() id {}", self.id);
        self.buffer.make_contiguous();
        if let (data, &[]) = self.buffer.as_slices() {
            let mut offset = 0;
            let (packet_length, offset_delta) = parse_varint(&data[offset..])?;
            offset += offset_delta;
            let packet_length = packet_length as usize;
            let (packet_id, offset_delta) = parse_varint(&data[offset..])?;
            offset += offset_delta;
            let packet_id = packet_id as usize;
            let (packet, offset_delta) =
                Packet::parse_body(&data[offset..], packet_length, packet_id, self.state, true)?;
            debug!("Got packet {:?} from client {}", packet, self.id);
            offset += offset_delta;
            self.packet_queue.push_back(packet);
            let remaining_data = self.buffer.split_off(offset);
            self.buffer = remaining_data;
        }
        Ok(())
    }
    pub async fn send_packet(&mut self, packet: Packet) -> Result<(), tokio::io::Error> {
        trace!("NetworkClient.send_packet()");
        debug!("Sending packet {:?} to client {}", packet, self.id);
        let bytes = packet.serialize();
        self.stream.write(&bytes).await?;
        Ok(())
    }
    pub async fn disconnect(&mut self, reason: Option<JSON>) {
        if let Some(reason) = reason {
            if self.state == NetworkClientState::Login {
                let _ = self.send_packet(Packet::CL00Disconnect { reason }).await;
            }
        }
        self.state = NetworkClientState::Disconnected;
    }
}
