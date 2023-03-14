#[derive(Debug, PartialEq, Clone)]
pub enum ServerboundMessage {
    Chat(String),               // The chat message.
    PlayerJoin(String, String), // UUID, then username
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClientboundMessage {
    Chat(String),       // The chat message.
    Disconnect(String), // The reason for disconnecting.
}
