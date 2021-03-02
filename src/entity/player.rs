use crate::world::location::Location;
use crate::server::NetworkClient;

pub struct Player {
    position: Location,
    display_name: String,
    connection: NetworkClient,
}
