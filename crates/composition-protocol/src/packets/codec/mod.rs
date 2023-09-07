mod uncompressed;

use crate::{packets::Packet, ClientState};
use std::{ops::Deref, sync::Arc};
use tokio::{runtime::Handle, sync::Mutex};
use tokio_util::codec::{Decoder, Encoder};

#[derive(Clone, Debug)]
pub struct PacketCodec {
    pub client_state: Arc<Mutex<ClientState>>,
    receive_serverbound_packets: bool,
    inner: InnerPacketCodec,
}
impl PacketCodec {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> PacketCodecBuilder {
        PacketCodecBuilder::new()
    }
}
impl Default for PacketCodec {
    fn default() -> Self {
        PacketCodec::new().build()
    }
}

#[derive(Clone, Debug, Default)]
enum InnerPacketCodec {
    #[default]
    Uncompressed,
}

#[derive(Clone, Debug, Default)]
pub struct PacketCodecBuilder {
    client_state: Option<Arc<Mutex<ClientState>>>,
    receive_serverbound_packets: Option<bool>,
    inner: Option<InnerPacketCodec>,
}
impl PacketCodecBuilder {
    pub fn new() -> PacketCodecBuilder {
        PacketCodecBuilder {
            client_state: None,
            receive_serverbound_packets: None,
            inner: None,
        }
    }
    pub fn build(self) -> PacketCodec {
        PacketCodec {
            client_state: self
                .client_state
                .unwrap_or(Arc::new(Mutex::new(ClientState::Handshake))),
            receive_serverbound_packets: self.receive_serverbound_packets.unwrap_or(true),
            inner: self.inner.unwrap_or(InnerPacketCodec::Uncompressed),
        }
    }
    pub fn client(self) -> PacketCodecBuilder {
        PacketCodecBuilder {
            receive_serverbound_packets: Some(false),
            client_state: self.client_state,
            inner: self.inner,
        }
    }
    pub fn server(self) -> PacketCodecBuilder {
        PacketCodecBuilder {
            receive_serverbound_packets: Some(true),
            client_state: self.client_state,
            inner: self.inner,
        }
    }
    pub fn initial_client_state(self, client_state: ClientState) -> PacketCodecBuilder {
        PacketCodecBuilder {
            receive_serverbound_packets: self.receive_serverbound_packets,
            client_state: Some(Arc::new(Mutex::new(client_state))),
            inner: self.inner,
        }
    }
    pub fn client_state(self, client_state: Arc<Mutex<ClientState>>) -> PacketCodecBuilder {
        PacketCodecBuilder {
            receive_serverbound_packets: self.receive_serverbound_packets,
            client_state: Some(client_state),
            inner: self.inner,
        }
    }
    pub fn compression(self, enable_compression: bool) -> PacketCodecBuilder {
        if enable_compression {
            unimplemented!()
        } else {
            PacketCodecBuilder {
                receive_serverbound_packets: self.receive_serverbound_packets,
                client_state: self.client_state,
                inner: Some(InnerPacketCodec::Uncompressed),
            }
        }
    }
}

impl Decoder for PacketCodec {
    type Item = Packet;
    type Error = crate::Error;

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // A little magic to get around the async mutex within sync code
        // https://stackoverflow.com/questions/66035290/how-do-i-await-a-future-inside-a-non-async-method-which-was-called-from-an-async
        let mut client_state = {
            let handle = Handle::try_current().map_err(|_| crate::Error::TokioRuntimeError)?;
            let _ = handle.enter();
            futures::executor::block_on(self.client_state.lock())
        };

        if *client_state == ClientState::Disconnected {
            return Err(crate::Error::Disconnected);
        }

        let packet = match self.inner {
            InnerPacketCodec::Uncompressed => {
                uncompressed::decode(client_state.deref(), self.receive_serverbound_packets, src)
            }
        };

        #[cfg(feature = "tracing")]
        tracing::trace!(
            "PacketCodec ({:?}) decoded packet: {:?}",
            self.inner,
            packet
        );

        if let Ok(Some(packet)) = &packet {
            // State machine logic.
            match packet {
                Packet::SH00Handshake(handshake) => {
                    *client_state = handshake.next_state;
                }
                Packet::CL02LoginSuccess(_) => {
                    *client_state = ClientState::Play;
                }
                Packet::CS01PingResponse(_)
                | Packet::CL00Disconnect(_)
                | Packet::CP17Disconnect(_) => {
                    *client_state = ClientState::Disconnected;
                }
                _ => {}
            }
        }

        packet
    }
}
impl Encoder<Packet> for PacketCodec {
    type Error = crate::Error;

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn encode(&mut self, item: Packet, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "PacketCodec ({:?}) serializing packet: {:?}",
            self.inner,
            item
        );

        match self.inner {
            InnerPacketCodec::Uncompressed => uncompressed::encode(item, dst),
        }
    }
}
