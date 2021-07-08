/// Definitions for all the packets in the Minecraft protocol.
pub mod packets;

use super::messages::*;
use crate::mctypes::*;
use log::*;
use packets::*;
// use serde_json::json;
use std::{
    collections::VecDeque,
    sync::mpsc::Sender,
    time::{Duration, Instant},
};
use tokio::net::TcpStream;

/// The network client can only be in a few states,
/// this enum keeps track of that.
#[derive(PartialEq, Debug)]
pub enum NetworkClientState {
    Handshake,
    Status,
    Login,
    Play,
    Disconnected,
}

/// A wrapper to contain everything related
/// to networking for the client.
#[derive(Debug)]
pub struct NetworkClient {
    pub id: u128,
    pub connected: bool,
    pub stream: TcpStream,
    pub state: NetworkClientState,
    pub last_keep_alive: Instant,
    pub message_sender: Sender<ServerboundMessage>,
    packets: VecDeque<Packet>,
    pub player: Option<crate::entity::player::Player>,
}
impl NetworkClient {
    /// Create a new `NetworkClient`
    pub fn new(
        stream: TcpStream,
        id: u128,
        message_sender: Sender<ServerboundMessage>,
    ) -> NetworkClient {
        NetworkClient {
            id,
            connected: true,
            stream,
            state: NetworkClientState::Handshake,
            last_keep_alive: Instant::now(),
            message_sender,
            packets: VecDeque::new(),
            player: None,
        }
    }

    /// Try to read a new packet into the processing queue.
    pub async fn update(&mut self) -> tokio::io::Result<()> {
        // Don't try to read packets if disconnected.
        if self.state == NetworkClientState::Disconnected {
            return Ok(());
        }
        if self.stream.peek(&mut [0u8; 4096]).await? > 0 {
            // Read the packet header.
            let (_packet_length, packet_id) = read_packet_header(&mut self.stream).await?;
            // Get the packet based on packet_id.
            let packet = match packet_id.value {
                0x00 => match self.state {
                    NetworkClientState::Handshake => {
                        Some(self.get_wrapped_packet::<Handshake>().await)
                    }
                    NetworkClientState::Status => {
                        Some(self.get_wrapped_packet::<StatusRequest>().await)
                    }
                    NetworkClientState::Login => {
                        Some(self.get_wrapped_packet::<LoginStart>().await)
                    }
                    NetworkClientState::Play => {
                        Some(self.get_wrapped_packet::<KeepAlivePong>().await)
                    }
                    _ => None,
                },
                0x01 => {
                    match self.state {
                        NetworkClientState::Status => {
                            Some(self.get_wrapped_packet::<StatusPing>().await)
                        }
                        NetworkClientState::Login => None, // TODO: 0x01 Encryption Response
                        NetworkClientState::Play => {
                            Some(self.get_wrapped_packet::<ServerboundChatMessage>().await)
                        }
                        _ => None,
                    }
                }
                // The rest of the packets are all always in the play state.
                0x02 => None, // TODO: 0x02 Use Entity
                0x03 => Some(self.get_wrapped_packet::<Player>().await),
                0x04 => Some(self.get_wrapped_packet::<PlayerPosition>().await),
                0x05 => Some(self.get_wrapped_packet::<PlayerLook>().await),
                0x06 => Some(
                    self.get_wrapped_packet::<ServerboundPlayerPositionAndLook>()
                        .await,
                ),
                0x07 => None, // TODO: 0x07 Player Digging
                0x08 => None, // TODO: 0x08 Player Block Placement
                0x09 => None, // TODO: 0x09 Held Item Change
                0x0a => None, // TODO: 0x0a Animation
                0x0b => None, // TODO: 0x0b Entity Action
                0x0c => None, // TODO: 0x0c Steer Vehicle
                0x0d => None, // TODO: 0x0d Close Window
                0x0e => None, // TODO: 0x0e Click Window
                0x0f => None, // TODO: 0x0f Confirm Transaction
                0x10 => None, // TODO: 0x10 Creative Inventory Action
                0x11 => None, // TODO: 0x11 Enchant Item
                0x12 => None, // TODO: 0x12 Update Sign
                0x13 => None, // TODO: 0x13 Player Abilities
                0x14 => None, // TODO: 0x14 Tab-Complete
                0x15 => Some(self.get_wrapped_packet::<ClientSettings>().await),
                0x16 => None, // TODO: 0x16 Client Status
                0x17 => None, // TODO: 0x17 Plugin Message
                0x18 => None, // TODO: 0x18 Spectate
                0x19 => None, // TODO: 0x19 Resource Pack Status
                _ => None,
            };
            if let Some(Ok(packet)) = packet {
                // Add it to the internal queue to be processed.
                self.packets.push_back(packet);
            }
        }
        if self.last_keep_alive.elapsed() > Duration::from_millis(1000) {
            debug!(
                "Sending keep alive, last one was {:?} ago",
                self.last_keep_alive.elapsed()
            );
            self.keep_alive().await?;
        }
        Ok(())
    }

    /// Pop a packet from the queue.
    pub fn read_packet(&mut self) -> Option<Packet> {
        self.packets.pop_front()
    }

    /// Send a generic packet to the client.
    pub async fn send_packet<P: PacketCommon>(&mut self, packet: P) -> tokio::io::Result<()> {
        debug!("Sent {:?} {:#04X?} {:?}", self.state, P::id(), packet);
        packet.write(&mut self.stream).await
    }

    /// Read a generic packet from the network.
    async fn get_packet<T: PacketCommon>(&mut self) -> tokio::io::Result<T> {
        let packet = T::read(&mut self.stream).await?;
        debug!("Got {:?} {:#04X?} {:?}", self.state, T::id(), packet);
        Ok(packet)
    }

    /// Read a generic packet from the network and wrap it in `Packet`.
    async fn get_wrapped_packet<T: PacketCommon>(&mut self) -> tokio::io::Result<Packet> {
        let packet = T::read(&mut self.stream).await?;
        debug!("Got {:?} {:#04X?} {:?}", self.state, T::id(), packet);
        Ok(packet.as_packet())
    }

    /// Send the client a message in chat.
    pub async fn send_chat_message<C: Into<MCChat>>(
        &mut self,
        message: C,
    ) -> tokio::io::Result<()> {
        let mut chatmessage = ClientboundChatMessage::new();
        chatmessage.text = message.into();
        self.send_packet(chatmessage).await?;
        Ok(())
    }

    /// Disconnect the client.
    ///
    /// Sends `0x40 Disconnect` then waits 10 seconds before forcing the connection closed.
    pub async fn disconnect<S: Into<MCString>>(&mut self, reason: S) -> tokio::io::Result<()> {
        let mut disconnect = Disconnect::new();
        disconnect.reason.text = reason.into();
        self.send_packet(disconnect).await?;
        self.force_disconnect();
        Ok(())
    }

    /// Force disconnect the client by marking it for cleanup as disconnected.
    pub fn force_disconnect(&mut self) {
        self.connected = false;
        self.state = NetworkClientState::Disconnected;
    }

    /// Send a keep alive packet to the client.
    pub async fn keep_alive(&mut self) -> tokio::io::Result<()> {
        if cfg!(debug_assertions) {
            // self.send_chat_message("keep alive").await?;
        }
        // Keep alive ping to client.
        self.send_packet(KeepAlivePing::new()).await?;
        // Keep alive pong to server.
        let (_packet_length, _packet_id) = read_packet_header(&mut self.stream).await?;
        let _ = self.get_packet::<KeepAlivePong>().await?;
        self.last_keep_alive = Instant::now();
        Ok(())
    }

    /// Receives messages from the server.
    pub async fn handle_message(&mut self, message: ClientboundMessage) -> tokio::io::Result<()> {
        use ClientboundMessage::*;
        match message {
            Chat(s) => self.send_chat_message(s).await?,
            Disconnect(reason) => self.disconnect(reason).await?,
        }
        Ok(())
    }
}
