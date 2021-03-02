/// Definitions for all the packets in the Minecraft protocol.
pub mod packets;

use crate::mctypes::*;
use log::{debug, info};
use packets::*;
use serde_json::json;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

/// The part of the server that handles
/// connecting clients and receiving/sending packets.
pub struct NetworkServer {
    pub clients: Vec<NetworkClient>,
    receiver: Receiver<NetworkClient>,
}
impl NetworkServer {
    /// Create a thread for listening to new clients.
    /// Use `std::sync::mpsc::channel()` to send the new clients across threads,
    /// then hold that in a queue for processing on an update.
    pub fn new<A: 'static + ToSocketAddrs + Send>(addr: A) -> NetworkServer {
        let (tx, rx) = mpsc::channel();
        tokio::task::spawn(async move {
            let listener = TcpListener::bind(addr)
                .await
                .expect("Could not bind to TCP socket");
            let mut id = 0;
            loop {
                let (stream, _) = listener
                    .accept()
                    .await
                    .expect("Network receiver disconnected");
                tx.send(NetworkClient {
                    id: id as u128,
                    connected: true,
                    stream,
                    state: NetworkClientState::Handshake,
                })
                .expect("Network receiver disconnected");
                id += 1;
            }
        });
        info!("Network server started!");
        NetworkServer {
            clients: vec![],
            receiver: rx,
        }
    }
    /// Update each client in `self.clients`.
    pub async fn update(&mut self) {
        loop {
            match self.receiver.try_recv() {
                Ok(client) => {
                    info!(
                        "Got client at {}",
                        client.stream.peer_addr().expect("Could not get peer addr")
                    );
                    self.clients.push(client)
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => panic!("Network sender disconnected"),
            }
        }
        for client in self.clients.iter_mut() {
            client.update().await;
        }
    }
}

/// The network client can only be in a few states,
/// this enum keeps track of that.
pub enum NetworkClientState {
    Handshake,
    Status,
    Login,
    Play,
    Disconnected,
}

/// A wrapper to contain everything related
/// to networking for the client.
pub struct NetworkClient {
    pub id: u128,
    pub connected: bool,
    pub stream: TcpStream,
    pub state: NetworkClientState,
}
impl NetworkClient {
    /// Update the client.
    ///
    /// Updating could mean connecting new clients, reading packets,
    /// writing packets, or disconnecting clients.
    pub async fn update(&mut self) {
        match self.state {
            NetworkClientState::Handshake => {
                let (_packet_length, _packet_id) =
                    read_packet_header(&mut self.stream).await.unwrap();
                let handshake = Handshake::read(&mut self.stream).await.unwrap();
                // Minecraft versions 1.8 - 1.8.9 use protocol version 47.
                let compatible_versions = handshake.protocol_version == 47;
                let next_state = match handshake.next_state.into() {
                    1 => NetworkClientState::Status,
                    2 => NetworkClientState::Login,
                    _ => NetworkClientState::Disconnected,
                };
                self.state = next_state;
                // If incompatible versions or wrong next state
                if !compatible_versions {
                    let mut logindisconnect = LoginDisconnect::new();
                    logindisconnect.reason = MCChat {
                        text: MCString::from("Incompatible client! Server is on 1.8.9"),
                    };
                    logindisconnect.write(&mut self.stream).await.unwrap();
                    self.state = NetworkClientState::Disconnected;
                }
                debug!("Got handshake: {:?}", handshake);
            }
            NetworkClientState::Status => {
                let (_packet_length, _packet_id) =
                    read_packet_header(&mut self.stream).await.unwrap();
                let statusrequest = StatusRequest::read(&mut self.stream).await.unwrap();
                debug!("Got status request: {:?}", statusrequest);
                let mut statusresponse = StatusResponse::new();
                statusresponse.json_response = json!({
                    "version": {
                        "name": "1.8.9",
                        "protocol": 47,
                    },
                    "players": {
                        "max": 100,
                        "online": 5,
                        "sample": [
                            {
                                "name": "shvr",
                                "id": "e3f58380-60bb-4714-91f2-151d525e64aa"
                            }
                        ]
                    },
                    "description": {
                        "text": "Hello world!"
                    },
                    // TODO: Dynamically send the icon instead of linking statically.
                    "favicon": format!("data:image/png;base64,{}", radix64::STD.encode(include_bytes!("../../server-icon.png")))
                })
                .to_string()
                .into();
                statusresponse.write(&mut self.stream).await.unwrap();
                debug!("Sending status response: StatusResponse");
                let (_packet_length, _packet_id) =
                    read_packet_header(&mut self.stream).await.unwrap();
                let statusping = StatusPing::read(&mut self.stream).await.unwrap();
                debug!("Got status ping: {:?}", statusping);
                let mut statuspong = StatusPong::new();
                statuspong.payload = statusping.payload;
                statuspong.write(&mut self.stream).await.unwrap();
                debug!("Sending status pong: {:?}", statuspong);
                self.state = NetworkClientState::Disconnected;
            }
            NetworkClientState::Login => {
                let (_packet_length, _packet_id) =
                    read_packet_header(&mut self.stream).await.unwrap();
                let loginstart = LoginStart::read(&mut self.stream).await.unwrap();
                debug!("{:?}", loginstart);
                // Offline mode skips encryption and compression.
                let mut loginsuccess = LoginSuccess::new();
                // We're in offline mode, so this is a temporary uuid.
                loginsuccess.uuid = "00000000-0000-3000-0000-000000000000".into();
                loginsuccess.username = loginstart.player_name;
                loginsuccess.write(&mut self.stream).await.unwrap();
                debug!("{:?}", loginsuccess);
                self.state = NetworkClientState::Play;
            }
            NetworkClientState::Play => {}
            NetworkClientState::Disconnected => {
                self.connected = false;
            }
        }
    }
}
