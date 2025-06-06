/// Connections where we're the server.
mod downstream;
/// Connections where we're the client.
mod upstream;

pub use downstream::{manager::DownstreamConnectionManager, DownstreamConnection};
pub use upstream::UpstreamConnection;

use crate::{
    net::{codec::PacketCodec, error::Error},
    protocol::{
        packets::{Packet, PacketDirection},
        ClientState,
    },
};
use futures::{stream::StreamExt, SinkExt};
use std::time::{Duration, Instant};
use tokio::{io::BufStream, net::TcpStream};
use tokio_util::codec::{Decoder, Framed};
use tracing::trace;

#[derive(Debug)]
pub struct GenericConnection {
    /// The `GenericConnection`'s unique id.
    id: u128,
    stream: Framed<BufStream<TcpStream>, PacketCodec>,
    last_received_data_time: Instant,
    last_sent_data_time: Instant,
}
impl GenericConnection {
    pub fn new(id: u128, receiving_direction: PacketDirection, stream: TcpStream) -> Self {
        let codec = PacketCodec::new(ClientState::Handshake, receiving_direction);

        GenericConnection {
            id,
            stream: codec.framed(BufStream::new(stream)),
            last_received_data_time: Instant::now(),
            last_sent_data_time: Instant::now(),
        }
    }
    pub fn id(&self) -> u128 {
        self.id
    }
    pub fn client_state(&self) -> ClientState {
        self.stream.codec().client_state
    }
    pub fn client_state_mut(&mut self) -> &mut ClientState {
        &mut self.stream.codec_mut().client_state
    }
    pub fn received_elapsed(&self) -> Duration {
        self.last_received_data_time.elapsed()
    }
    pub fn sent_elapsed(&self) -> Duration {
        self.last_sent_data_time.elapsed()
    }
    pub async fn read_packet(&mut self) -> Option<Result<Packet, Error>> {
        let packet = self.stream.next().await.map(|packet| {
            packet.map_err(|mut e| {
                // Set the codec error to something more descriptive.
                if e.to_string() == "bytes remaining on stream" {
                    e = Error::Io(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, e));
                }
                trace!("Error reading packet from connection {}: {:?}", self.id, e);
                e
            })
        });

        if let Some(Ok(ref packet)) = packet {
            trace!("Received packet from connection {}: {:?}", self.id, packet);
            self.last_received_data_time = Instant::now();

            if let Some(next_state) = packet.state_change() {
                *self.client_state_mut() = next_state;
            }
        }

        packet
    }
    pub async fn send_packet<P: Into<Packet>>(&mut self, packet: P) -> Result<(), Error> {
        let packet: Packet = packet.into();
        trace!("Sending packet to connection {}: {:?}", self.id, packet);
        self.stream.send(packet).await.inspect_err(|e| {
            trace!("Error sending packet to connection {}: {:?}", self.id, e);
        })
    }
    pub async fn disconnect(mut self) -> Result<(), Error> {
        trace!("Connection disconnected (id {})", self.id);
        self.stream.flush().await?;
        self.stream.codec_mut().client_state = ClientState::Disconnected;
        Ok(())
    }
}
