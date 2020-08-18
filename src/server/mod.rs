use crate::net::{NetworkClient, NetworkClientState, NetworkServer};
use log::{debug, error, info, warn};

pub struct GameServer {
    pub network: NetworkServer,
}
impl GameServer {
    pub fn update(&mut self) {
        self.network.update();
    }
}
