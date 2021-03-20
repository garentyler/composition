/// The packets that get sent to the client by the server.
pub mod clientbound;
/// The packets that get sent to the server by the client.
pub mod serverbound;

use crate::mctypes::MCVarInt;
pub use clientbound::*;
use core::convert::TryFrom;
pub use serverbound::*;
use tokio::io::{AsyncRead, AsyncWrite};

/// A helper function to read the packet header.
pub async fn read_packet_header<T: AsyncRead + Unpin>(t: &mut T) -> tokio::io::Result<(MCVarInt, MCVarInt)> {
    let length = MCVarInt::read(t).await?;
    let id = MCVarInt::read(t).await?;
    Ok((length, id))
}

/// A way to generically encode a packet.
macro_rules! register_packets {
    ($($name:ident),*) => {
        #[derive(Debug, Clone)]
        pub enum Packet {
            $($name($name),)*
            Null,
        }
        impl Packet {
            pub fn new() -> Packet {
                Packet::Null
            }
            pub async fn write<T: AsyncWrite + Unpin + Send>(&self, t: &mut T) -> tokio::io::Result<()> {
                match self {
                    $(
                        Packet::$name(p) => p.write(t).await,
                    )*
                    Packet::Null => Ok(())
                }
            }
        }
        impl Default for Packet {
            fn default() -> Self {
                Packet::Null
            }
        }
        $(
            impl $name {
                pub fn as_packet(&self) -> Packet {
                    Packet::$name(self.clone())
                }
            }
            impl Into<Packet> for $name {
                fn into(self) -> Packet {
                    Packet::$name(self.clone())
                }
            }
            impl TryFrom<Packet> for $name {
                type Error = &'static str;
                fn try_from(p: Packet) -> Result<Self, Self::Error> {
                    match p {
                        Packet::$name(i) => Ok(i),
                        _ => Err("wrong kind"),
                    }
                }
            }
        )*
    };
}

// Register all the packets.
register_packets!(
    // Clientbound.
    StatusResponse,
    StatusPong,
    LoginSuccess,
    LoginDisconnect,
    JoinGame,
    HeldItemChange,
    EntityStatus,
    ClientboundPlayerPositionAndLook,
    SpawnPosition,
    KeepAlivePing,
    Disconnect,
    ClientboundChatMessage,
    // Serverbound.
    Handshake,
    StatusRequest,
    StatusPing,
    LoginStart,
    ClientSettings,
    KeepAlivePong,
    ServerboundChatMessage,
    Player,
    PlayerPosition,
    PlayerLook,
    ServerboundPlayerPositionAndLook
);

#[async_trait::async_trait]
pub trait PacketCommon: Into<Packet> + core::fmt::Debug
where
    Self: Sized,
{
    fn new() -> Self;
    fn id() -> u8;
    async fn read<T: AsyncRead + Unpin + Send>(t: &'_ mut T) -> tokio::io::Result<Self>;
    async fn write<T: AsyncWrite + Unpin + Send>(&self, t: &'_ mut T) -> tokio::io::Result<()>;
}
