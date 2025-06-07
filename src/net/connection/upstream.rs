use crate::{
    net::{connection::GenericConnection, error::Error},
    protocol::{
        encryption::*,
        packets::{self, Packet, PacketDirection},
    },
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct UpstreamConnection {
    inner: GenericConnection,
}
impl UpstreamConnection {
    pub fn new(id: u128, stream: TcpStream) -> Self {
        UpstreamConnection {
            // receiving_direction: PacketDirection::Clientbound
            inner: GenericConnection::new(id, PacketDirection::Clientbound, stream),
        }
    }
    pub async fn read_packet(&mut self) -> Option<Result<Packet, Error>> {
        let packet = self.inner.read_packet().await?.ok()?;

        match packet {
            Packet::EncryptionRequest(ref packet) => {
                // Extract the public key from the packet.
                tracing::trace!(
                    "{}",
                    packet
                        .public_key
                        .iter()
                        .map(|b| format!("{b:02X?}"))
                        .collect::<Vec<String>>()
                        .join("")
                );
                let public_key = rsa::RsaPublicKey::parse(&packet.public_key)
                    .expect("Failed to parse RSA public key from packet")
                    .1;

                // Generate a shared secret.
                let mut rng = StdRng::from_entropy();
                let shared_secret: [u8; 16] = rng.gen();

                // Create the AES stream cipher and initialize it with the shared secret.
                let encryptor =
                    Aes128Cfb8Encryptor::new((&shared_secret).into(), (&shared_secret).into());
                let decryptor =
                    Aes128Cfb8Decryptor::new((&shared_secret).into(), (&shared_secret).into());

                // Send the encryption response packet.
                self.send_packet(packets::login::serverbound::EncryptionResponse {
                    shared_secret: public_key
                        .encrypt(&mut rng, rsa::Pkcs1v15Encrypt, &shared_secret[..])
                        .expect("Failed to encrypt shared secret"),
                    verify_token: public_key
                        .encrypt(&mut rng, rsa::Pkcs1v15Encrypt, &packet.verify_token[..])
                        .expect("Failed to encrypt shared secret"),
                })
                .await
                .expect("Failed to send encryption response");

                // Enable encryption on the connection.
                self.inner.stream.codec_mut().aes_cipher = Some((encryptor, decryptor, 0));
            }
            Packet::SetCompression(_) => todo!(),
            _ => {}
        }

        Some(Ok(packet))
    }
    pub async fn send_packet<P: Into<Packet>>(&mut self, packet: P) -> Result<(), Error> {
        self.inner.send_packet(packet).await
    }
}
impl std::ops::Deref for UpstreamConnection {
    type Target = GenericConnection;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for UpstreamConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<UpstreamConnection> for GenericConnection {
    fn from(value: UpstreamConnection) -> Self {
        value.inner
    }
}
