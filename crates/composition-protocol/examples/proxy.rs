use composition_protocol::packets::codec::UncompressedPacketCodec;
use futures::{SinkExt, StreamExt};
use std::sync::{Arc, RwLock};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::Decoder;

#[tokio::main]
async fn main() {
    // Start the listener.
    let listener = TcpListener::bind("127.0.0.1:25566").await.unwrap();
    println!("Listening on port 25566");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            println!("Got client");
            handle_client(socket).await.unwrap();
        });
    }
}

async fn handle_client(socket: TcpStream) -> composition_protocol::Result<()> {
    let client_state = Arc::new(RwLock::new(composition_protocol::ClientState::Handshake));

    let (mut client_sink, mut client_stream) =
        UncompressedPacketCodec::new_server(client_state.clone())
            .framed(socket)
            .split();
    let (mut server_sink, mut server_stream) =
        UncompressedPacketCodec::new_client(client_state.clone())
            .framed(TcpStream::connect("127.0.0.1:25565").await?)
            .split();

    // Read data from the client and pass it to the server.
    tokio::spawn(async move {
        while let Some(value) = client_stream.next().await {
            match value {
                Ok(packet) => {
                    println!("C->S: {:?}", packet);
                    server_sink.send(packet).await.unwrap();
                }
                Err(e) => Err(e).unwrap(),
            }
        }
    });

    // Read data from the server and pass it to the client.
    tokio::spawn(async move {
        while let Some(value) = server_stream.next().await {
            match value {
                Ok(packet) => {
                    println!("S->C: {:?}", packet);
                    client_sink.send(packet).await.unwrap();
                }
                Err(e) => Err(e).unwrap(),
            }
        }
    });

    Ok(())
}
