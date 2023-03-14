use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:25566").await?;

    println!("Proxy listening on port 25566, expecting server at 25565");

    loop {
        let (stream, _) = listener.accept().await?;
        process_client(stream).await?;
    }
}

async fn process_client(client: TcpStream) -> anyhow::Result<()> {
    use composition_protocol::packet::GenericPacket;
    client.readable().await?;
    println!("Client connected");
    let server = TcpStream::connect("localhost:25565").await?;
    server.writable().await?;
    println!("Server connected");

    let mut client_state = composition_protocol::ClientState::Handshake;
    let mut last_client_data_time = std::time::Instant::now();
    let mut last_client_data = vec![];
    let mut last_server_data_time = std::time::Instant::now();
    let mut last_server_data = vec![];

    loop {
        let bytes = copy_bytes(&client, &server).await?;
        if !bytes.is_empty() {
            last_client_data_time = std::time::Instant::now();
            last_client_data.extend_from_slice(&bytes);

            if let Ok((d, packet)) =
                GenericPacket::parse_uncompressed(&client_state, true, &last_client_data)
            {
                last_client_data = d.to_vec();
                println!("C -> S: {:?}", packet);
                match packet {
                    GenericPacket::SH00Handshake(handshake) => {
                        client_state = handshake.next_state;
                    }
                    GenericPacket::CP17Disconnect(_) => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        let bytes = copy_bytes(&server, &client).await?;
        if !bytes.is_empty() {
            last_server_data_time = std::time::Instant::now();
            last_server_data.extend_from_slice(&bytes);

            if let Ok((d, packet)) =
                GenericPacket::parse_uncompressed(&client_state, false, &last_server_data)
            {
                last_server_data = d.to_vec();
                println!("S -> C: {:?}", packet);
                match packet {
                    GenericPacket::CS01PingResponse(_) | GenericPacket::CL00Disconnect(_) => {
                        break;
                    }
                    GenericPacket::CL02LoginSuccess(_) => {
                        client_state = composition_protocol::ClientState::Play;
                    }
                    _ => {}
                }
            }
        }

        if last_client_data_time.elapsed() > std::time::Duration::from_secs(10)
            || last_server_data_time.elapsed() > std::time::Duration::from_secs(10)
        {
            println!("timed out");
            break;
        }
    }

    Ok(())
}

async fn copy_bytes(from: &TcpStream, to: &TcpStream) -> anyhow::Result<Vec<u8>> {
    let mut bytes = vec![];

    loop {
        // Read 8kb at a time
        let mut buf = vec![0u8; 8192];

        let num_bytes = match from.try_read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
                break;
            }
            Err(e) => {
                return Err(e.into());
            }
        };

        bytes.extend_from_slice(&buf[0..num_bytes]);

        match to.try_write(&buf[0..num_bytes]) {
            Ok(_n) => {}
            Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
                break;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(bytes)
}
