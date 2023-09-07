use composition_protocol::packets::codec::PacketCodec;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
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

async fn handle_client(client_socket: TcpStream) -> composition_protocol::Result<()> {
    let server_socket = TcpStream::connect("127.0.0.1:25565").await?;
    let client_state = Arc::new(Mutex::new(composition_protocol::ClientState::Handshake));

    // Build the two codecs to parse acting as a server and a client, but share the same client state.
    let client_codec = PacketCodec::new()
        .compression(false)
        .server()
        .client_state(client_state.clone())
        .build();
    let server_codec = PacketCodec::new()
        .compression(false)
        .client()
        .client_state(client_state.clone())
        .build();

    let (mut client_sink, mut client_stream) = client_codec.framed(client_socket).split();
    let (mut server_sink, mut server_stream) = server_codec.framed(server_socket).split();

    // Read data from the client and pass it to the server.
    tokio::spawn(async move {
        loop {
            match client_stream.next().await {
                Some(Ok(packet)) => {
                    println!("C->S: {:?}", packet);
                    server_sink.send(packet).await.unwrap();
                }
                Some(Err(composition_protocol::Error::Disconnected)) | None => break,
                Some(Err(e)) => {
                    eprintln!("C->S: {}", e);
                    break;
                }
            }
        }
    });

    // Read data from the client and pass it to the server.
    tokio::spawn(async move {
        loop {
            match server_stream.next().await {
                Some(Ok(packet)) => {
                    println!("S->C: {:?}", packet);
                    client_sink.send(packet).await.unwrap();
                }
                Some(Err(composition_protocol::Error::Disconnected)) | None => break,
                Some(Err(e)) => {
                    eprintln!("S->C: {}", e);
                    break;
                }
            }
        }
    });

    Ok(())
}
