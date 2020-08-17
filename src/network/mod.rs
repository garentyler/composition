// network/mod.rs
// authors: Garen Tyler
// description:
//   This module contains the network logic.

pub mod packet;

use crate::server::ServerMessage;
use log::{debug, error, info, warn};
use packet::Packet;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};

pub struct NetworkServer {
    receiver: Receiver<NetworkClient>,
    clients: Vec<NetworkClient>,
}
impl NetworkServer {
    pub fn new(port: u16) -> NetworkServer {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || NetworkServer::listen(port, tx));
        NetworkServer {
            receiver: rx,
            clients: Vec::new(),
        }
    }
    fn listen(port: u16, sender: Sender<NetworkClient>) {
        let listener = TcpListener::bind(&format!("0.0.0.0:{}", port)).unwrap();

        for (index, stream) in listener.incoming().enumerate() {
            let stream = stream.unwrap();
            stream.set_nonblocking(true).unwrap();
            sender
                .send(NetworkClient {
                    // The index will increment after each client making it unique. We'll just use this as the id.
                    id: index as u32,
                    stream: BufReader::new(stream),
                    state: NetworkClientState::Handshake,
                    packets: Vec::new(),
                    username: None,
                    alive: true,
                })
                .unwrap();
        }
    }
    pub fn update(&mut self) {
        loop {
            match self.receiver.try_recv() {
                Ok(client) => self.clients.push(client),
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    panic!("Client receiver channel disconnected!")
                }
            }
        }
        // Todo: Update each client
        for client in self.clients.iter_mut() {
            client.update();
        }
    }
}

#[derive(Debug)]
pub struct NetworkClient {
    id: u32,
    state: NetworkClientState,
    alive: bool,
    stream: BufReader<TcpStream>,
    username: Option<String>,
    packets: Vec<Packet>,
}

impl NetworkClient {
    pub fn update(&mut self) {}
}

#[derive(PartialEq, Debug)]
pub enum NetworkClientState {
    Handshake,
    Status,
    Login,
    Play,
}
