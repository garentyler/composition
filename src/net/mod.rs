pub mod packets;

use crate::mctypes::*;
use log::{debug, error, info, warn};
use packets::*;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};

pub struct NetworkServer {
    pub clients: Vec<NetworkClient>,
    receiver: Receiver<NetworkClient>,
}
impl NetworkServer {
    pub fn new<A: 'static + ToSocketAddrs + Send>(addr: A) -> NetworkServer {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let listener = TcpListener::bind(addr).expect("Could not bind to TCP socket");
            for (id, stream) in listener.incoming().enumerate() {
                if let Ok(s) = stream {
                    tx.send(NetworkClient {
                        id: id as u128,
                        connected: true,
                        stream: s,
                        state: NetworkClientState::Handshake,
                    })
                    .expect("Network receiver disconnected");
                }
            }
        });
        info!("Network server started!");
        NetworkServer {
            clients: vec![],
            receiver: rx,
        }
    }
    pub fn update(&mut self) {
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
            client.update();
        }
    }
}
pub enum NetworkClientState {
    Handshake,
    Status,
    Login,
    Play,
    Disconnected,
}
pub struct NetworkClient {
    pub id: u128,
    pub connected: bool,
    pub stream: TcpStream,
    pub state: NetworkClientState,
}
impl NetworkClient {
    pub fn update(&mut self) {
        match self.state {
            NetworkClientState::Handshake => {
                let (_packet_length, _packet_id) = read_packet_header(&mut self.stream).unwrap();
                let handshake = Handshake::read(&mut self.stream).unwrap();
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
                    logindisconnect.write(&mut self.stream).unwrap();
                    self.state = NetworkClientState::Disconnected;
                }
                println!("{:?}", handshake);
            }
            NetworkClientState::Status => {
                let (_packet_length, _packet_id) = read_packet_header(&mut self.stream).unwrap();
                let statusrequest = StatusRequest::read(&mut self.stream).unwrap();
                println!("{:?}", statusrequest);
                let mut statusresponse = StatusResponse::new();
                statusresponse.json_response = format!(
                    "{{\n\t\"version\": {{\n\t\t\"name\": \"{server_version}\",\n\t\t\"protocol\": {server_protocol}\n\t}},\n\t\"players\": {{\n\t\t\"max\": {max_players},\n\t\t\"online\": {num_players},\n\t\t\"sample\": [\n\t\t\t{{\n\t\t\t\t\"name\": \"ElementG9\",\n\t\t\t\t\"id\": \"e3f58380-60bb-4714-91f2-151d525e64aa\"\n\t\t\t}}\n\t\t]\n\t}},\n\t\"description\": {{\n\t\t\"text\": \"{server_description}\"\n\t}},\n\t\"sample\": \"\"\n}}",
                    server_version = "1.8.9",
                    server_protocol = 47,
                    num_players = 5,
                    max_players = 100,
                    server_description = "Hello world!"
                ).into();
                statusresponse.write(&mut self.stream).unwrap();
                println!("{:?}", statusresponse);
                let (_packet_length, _packet_id) = read_packet_header(&mut self.stream).unwrap();
                let statusping = StatusPing::read(&mut self.stream).unwrap();
                println!("{:?}", statusping);
                let mut statuspong = StatusPong::new();
                statuspong.payload = statusping.payload;
                statuspong.write(&mut self.stream).unwrap();
                self.state = NetworkClientState::Disconnected;
            }
            NetworkClientState::Login => {
                let (_packet_length, _packet_id) = read_packet_header(&mut self.stream).unwrap();
                let loginstart = LoginStart::read(&mut self.stream).unwrap();
                println!("{:?}", loginstart);
                // Offline mode skips encryption and compression.
                let mut loginsuccess = LoginSuccess::new();
                // We're in offline mode, so this is a temporary uuid.
                loginsuccess.uuid = "00000000-0000-3000-0000-000000000000".into();
                loginsuccess.username = loginstart.player_name;
                loginsuccess.write(&mut self.stream).unwrap();
                println!("{:?}", loginsuccess);
                self.state = NetworkClientState::Play;
            }
            NetworkClientState::Play => {}
            NetworkClientState::Disconnected => {
                self.connected = false;
            }
        }
    }
}
