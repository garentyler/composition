/// Deals with all the network stuff.
pub mod net;

/// The struct containing all the data and running all the updates.
pub struct GameServer {
    pub network: net::NetworkServer,
}
impl GameServer {
    /// Update the game server.
    ///
    /// Start by updating the network.
    pub async fn update(&mut self) {
        self.network.update().await;
    }
}
