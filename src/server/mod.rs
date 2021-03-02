pub mod net;

pub struct GameServer {
    pub network: net::NetworkServer,
}
impl GameServer {
    pub fn update(&mut self) {
        self.network.update();
    }
}
