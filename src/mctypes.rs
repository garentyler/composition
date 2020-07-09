use std::io::prelude::*;
use std::net::TcpStream;

pub fn read_byte(t: &mut TcpStream) -> std::io::Result<u8> {
    let mut buffer = [0u8; 1];
    t.read_exact(&mut buffer)?;
    Ok(buffer[0])
}
pub fn write_byte(t: &mut TcpStream, value: u8) -> std::io::Result<()> {
    t.write(&[value])?;
    Ok(())
}
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
pub fn io_error(s: &str) -> std::io::Error {
    use std::io::{Error, ErrorKind};
    Error::new(ErrorKind::Other, s)
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum MCBoolean {
    True,
    False,
}
impl From<bool> for MCBoolean {
    fn from(v: bool) -> MCBoolean {
        if v {
            MCBoolean::True
        } else {
            MCBoolean::False
        }
    }
}
#[allow(dead_code)]
impl MCBoolean {
    pub fn from_stream(t: &mut TcpStream) -> std::io::Result<MCBoolean> {
        let b = read_byte(t)?;
        Ok(MCBoolean::from_bytes(vec![b]))
    }
    pub fn from_bytes(v: Vec<u8>) -> MCBoolean {
        if get_bytes(v, 1)[0] == 0x01 {
            MCBoolean::True
        } else {
            MCBoolean::False
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            MCBoolean::True => vec![0x01],
            MCBoolean::False => vec![0x00],
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct MCByte {
    pub value: i8, // -128 to 127
}
impl From<i8> for MCByte {
    fn from(v: i8) -> MCByte {
        MCByte { value: v }
    }
}
#[allow(dead_code)]
impl MCByte {
    pub fn from_stream(t: &mut TcpStream) -> std::io::Result<MCByte> {
        Ok(MCByte::from_bytes(vec![read_byte(t)?]))
    }
    pub fn from_bytes(v: Vec<u8>) -> MCByte {
        MCByte {
            value: get_bytes(v, 1)[0] as i8,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct MCUnsignedByte {
    pub value: u8, // 0 to 255
}
impl From<u8> for MCUnsignedByte {
    fn from(v: u8) -> MCUnsignedByte {
        MCUnsignedByte { value: v }
    }
}
#[allow(dead_code)]
impl MCUnsignedByte {
    pub fn from_stream(t: &mut TcpStream) -> std::io::Result<MCUnsignedByte> {
        Ok(MCUnsignedByte::from_bytes(vec![read_byte(t)?]))
    }
    pub fn from_bytes(v: Vec<u8>) -> MCUnsignedByte {
        MCUnsignedByte {
            value: get_bytes(v, 1)[0],
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct MCShort {
    pub value: i16, // -32768 to 32767
}
impl From<i16> for MCShort {
    fn from(v: i16) -> MCShort {
        MCShort { value: v }
    }
}
#[allow(dead_code)]
impl MCShort {
    pub fn from_stream(t: &mut TcpStream) -> std::io::Result<MCShort> {
        let mut bytes = Vec::new();
        bytes.push(read_byte(t)?); // MSD
        bytes.push(read_byte(t)?); // LSD
        Ok(MCShort::from_bytes(bytes))
    }
    pub fn from_bytes(v: Vec<u8>) -> MCShort {
        let mut a = [0u8; 2];
        a.copy_from_slice(&get_bytes(v, 2));
        MCShort {
            value: i16::from_be_bytes(a),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct MCUnsignedShort {
    pub value: u16, // 0 to 65535
}
impl From<u16> for MCUnsignedShort {
    fn from(v: u16) -> MCUnsignedShort {
        MCUnsignedShort { value: v }
    }
}
#[allow(dead_code)]
impl MCUnsignedShort {
    pub fn from_stream(t: &mut TcpStream) -> std::io::Result<MCUnsignedShort> {
        let mut bytes = Vec::new();
        bytes.push(read_byte(t)?); // MSD
        bytes.push(read_byte(t)?); // LSD
        Ok(MCUnsignedShort::from_bytes(bytes))
    }
    pub fn from_bytes(v: Vec<u8>) -> MCUnsignedShort {
        let mut a = [0u8; 2];
        a.copy_from_slice(&get_bytes(v, 2));
        MCUnsignedShort {
            value: u16::from_be_bytes(a),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct MCInt {
    pub value: i32, // -2147483648 to 2147483647
}
impl From<i32> for MCInt {
    fn from(v: i32) -> MCInt {
        MCInt { value: v }
    }
}
#[allow(dead_code)]
impl MCInt {
    pub fn from_stream(t: &mut TcpStream) -> std::io::Result<MCInt> {
        let mut bytes = Vec::new();
        for _ in 0..4 {
            bytes.push(read_byte(t)?);
        }
        Ok(MCInt::from_bytes(bytes))
    }
    pub fn from_bytes(v: Vec<u8>) -> MCInt {
        let mut a = [0u8; 4];
        a.copy_from_slice(&get_bytes(v, 4));
        MCInt {
            value: i32::from_be_bytes(a),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct MCLong {
    pub value: i64, // -9223372036854775808 to 9223372036854775807
}
impl From<i64> for MCLong {
    fn from(v: i64) -> MCLong {
        MCLong { value: v }
    }
}
#[allow(dead_code)]
impl MCLong {
    pub fn from_stream(t: &mut TcpStream) -> std::io::Result<MCLong> {
        let mut bytes = Vec::new();
        for _ in 0..8 {
            bytes.push(read_byte(t)?);
        }
        Ok(MCLong::from_bytes(bytes))
    }
    pub fn from_bytes(v: Vec<u8>) -> MCLong {
        let mut a = [0u8; 8];
        a.copy_from_slice(&get_bytes(v, 8));
        MCLong {
            value: i64::from_be_bytes(a),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct MCFloat {
    pub value: f32, // 32-bit floating point number
}
impl From<f32> for MCFloat {
    fn from(v: f32) -> MCFloat {
        MCFloat { value: v }
    }
}
#[allow(dead_code)]
impl MCFloat {
    pub fn from_stream(t: &mut TcpStream) -> std::io::Result<MCFloat> {
        let mut bytes = Vec::new();
        for _ in 0..4 {
            bytes.push(read_byte(t)?);
        }
        Ok(MCFloat::from_bytes(bytes))
    }
    pub fn from_bytes(v: Vec<u8>) -> MCFloat {
        let mut a = [0u8; 4];
        a.copy_from_slice(&get_bytes(v, 4));
        MCFloat {
            value: f32::from_be_bytes(a),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct MCDouble {
    pub value: f64, // 64-bit floating point number
}
impl From<f64> for MCDouble {
    fn from(v: f64) -> MCDouble {
        MCDouble { value: v }
    }
}
#[allow(dead_code)]
impl MCDouble {
    pub fn from_stream(t: &mut TcpStream) -> std::io::Result<MCDouble> {
        let mut bytes = Vec::new();
        for _ in 0..8 {
            bytes.push(read_byte(t)?);
        }
        Ok(MCDouble::from_bytes(bytes))
    }
    pub fn from_bytes(v: Vec<u8>) -> MCDouble {
        let mut a = [0u8; 8];
        a.copy_from_slice(&get_bytes(v, 8));
        MCDouble {
            value: f64::from_be_bytes(a),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct MCVarInt {
    pub value: i32, // Variable length 32-bit integer
}
impl From<i32> for MCVarInt {
    fn from(v: i32) -> MCVarInt {
        MCVarInt { value: v }
    }
}
#[allow(dead_code)]
impl MCVarInt {
    pub fn new(i: i32) -> MCVarInt {
        MCVarInt { value: i }
    }
    pub fn from_stream(t: &mut TcpStream) -> std::io::Result<MCVarInt> {
        let mut numRead = 0;
        let mut result = 0i32;
        let mut read = 0u8;
        let mut run_once = false;
        while (read & 0b10000000) != 0 || !run_once {
            run_once = true;
            read = read_byte(t)?;
            let value = (read & 0b01111111) as i32;
            result |= value << (7 * numRead);
            numRead += 1;
            if numRead > 5 {
                return Err(io_error("MCVarInt is too big"));
            }
        }
        Ok(MCVarInt { value: result })
    }
    pub fn from_bytes(_: Vec<u8>) -> MCVarInt {
        panic!("Cannot construct MCVarInt from raw bytes");
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut value = self.value.clone();
        let mut run_once = false;
        let mut out: Vec<u8> = Vec::new();
        while value != 0 || !run_once {
            run_once = true;
            let mut temp: u8 = (value & 0b01111111) as u8;
            value >>= 7;
            if value != 0 {
                temp |= 0b10000000;
            }
            out.push(temp);
        }
        return out;
    }
}
impl std::fmt::Display for MCVarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct MCString {
    pub value: String,
}
#[allow(dead_code)]
impl MCString {
    pub fn from_stream(t: &mut TcpStream) -> std::io::Result<MCString> {
        let length = MCVarInt::from_stream(t)?.value as u32;
        let mut bytes = Vec::new();
        for _ in 0..length {
            bytes.push(read_byte(t)?);
        }
        let value = String::from_utf8(bytes);
        if value.is_ok() {
            Ok(MCString {
                value: value.unwrap(),
            })
        } else {
            return Err(io_error("MCString contains invalid utf-8"));
        }
    }
    pub fn from_bytes(_: Vec<u8>) -> MCString {
        panic!("Cannot construct MCVarInt from raw bytes");
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let length = MCVarInt {
            value: self.value.len() as i32,
        };
        for b in length.to_bytes() {
            out.push(b);
        }
        for b in self.value.clone().into_bytes() {
            out.push(b);
        }
        out
    }
}
impl From<&str> for MCString {
    fn from(s: &str) -> MCString {
        MCString {
            value: s.to_owned(),
        }
    }
}
impl From<String> for MCString {
    fn from(s: String) -> MCString {
        MCString { value: s.clone() }
    }
}
impl Clone for MCString {
    fn clone(&self) -> Self {
        MCString {
            value: self.value.clone(),
        }
    }
}
impl std::fmt::Display for MCString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\" ({} chars)", self.value, self.value.len())
    }
}
