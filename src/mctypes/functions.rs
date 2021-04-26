use super::*;

/// Read a single byte from the given `TcpStream`.
pub async fn read_byte(t: &mut TcpStream) -> tokio::io::Result<u8> {
    let mut buffer = [0u8; 1];
    t.read_exact(&mut buffer).await?;
    Ok(buffer[0])
}
/// Read `l` bytes from the given `TcpStream`.
pub async fn read_bytes(t: &mut TcpStream, l: usize) -> tokio::io::Result<Vec<u8>> {
    let mut buffer = vec![];
    for _ in 0..l {
        buffer.push(read_byte(t).await?);
    }
    Ok(buffer)
}
/// Write a single byte to the given `TcpStream`.
pub async fn write_byte(t: &mut TcpStream, value: u8) -> tokio::io::Result<()> {
    t.write(&[value]).await?;
    Ok(())
}
/// Write multiple bytes to the given `TcpStream`.
pub async fn write_bytes(t: &mut TcpStream, bytes: &[u8]) -> tokio::io::Result<()> {
    for b in bytes {
        write_byte(t, *b).await?;
    }
    Ok(())
}
/// Take `l` bytes from the given `Vec<u8>`.
pub fn get_bytes(v: Vec<u8>, l: usize) -> Box<[u8]> {
    use std::collections::VecDeque;
    let mut v = VecDeque::from(v);
    while v.len() > l {
        v.pop_front();
    }
    while v.len() < l {
        v.push_front(0u8);
    }
    let mut a = Vec::new();
    for b in v {
        a.push(b);
    }
    a.into_boxed_slice()
}
/// Makes returning errors shorter.
pub fn io_error(s: &str) -> tokio::io::Error {
    use tokio::io::{Error, ErrorKind};
    Error::new(ErrorKind::Other, s)
}
