use crate::server::net::NetworkClient;
use crate::world::location::Location;

pub struct Player {
    position: Location,
    display_name: String,
    connection: NetworkClient,
}
