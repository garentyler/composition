use super::*;
use std::convert::{From, Into, TryFrom};
use std::fmt::Display;

/// The equivalent of an `i8`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MCByte {
    pub value: i8, // -128 to 127
}
impl MCByte {
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<MCByte> {
        Ok(MCByte::from_bytes(vec![read_byte(t).await?]))
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
impl From<i8> for MCByte {
    fn from(v: i8) -> MCByte {
        MCByte { value: v }
    }
}
impl Into<i8> for MCByte {
    fn into(self) -> i8 {
        self.value
    }
}
impl PartialEq<i8> for MCByte {
    fn eq(&self, other: &i8) -> bool {
        self.value == *other
    }
}
impl Display for MCByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl TryFrom<Vec<u8>> for MCByte {
    type Error = &'static str;
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.is_empty() {
            Err("Not enough bytes")
        } else {
            let mut a = [0u8; 1];
            a.copy_from_slice(&get_bytes(bytes, 1));
            Ok(MCByte {
                value: i8::from_be_bytes(a),
            })
        }
    }
}
impl Into<Vec<u8>> for MCByte {
    fn into(self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

/// The equivalent of a `u8`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MCUnsignedByte {
    pub value: u8, // 0 to 255
}
impl MCUnsignedByte {
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<MCUnsignedByte> {
        Ok(MCUnsignedByte::from_bytes(vec![read_byte(t).await?]))
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
impl From<u8> for MCUnsignedByte {
    fn from(v: u8) -> MCUnsignedByte {
        MCUnsignedByte { value: v }
    }
}
impl Into<u8> for MCUnsignedByte {
    fn into(self) -> u8 {
        self.value
    }
}
impl PartialEq<u8> for MCUnsignedByte {
    fn eq(&self, other: &u8) -> bool {
        self.value == *other
    }
}
impl Display for MCUnsignedByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl TryFrom<Vec<u8>> for MCUnsignedByte {
    type Error = &'static str;
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.is_empty() {
            Err("Not enough bytes")
        } else {
            let mut a = [0u8; 1];
            a.copy_from_slice(&get_bytes(bytes, 1));
            Ok(MCUnsignedByte {
                value: u8::from_be_bytes(a),
            })
        }
    }
}
impl Into<Vec<u8>> for MCUnsignedByte {
    fn into(self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

/// The equivalent of an `i16`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MCShort {
    pub value: i16, // -32768 to 32767
}
impl MCShort {
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<MCShort> {
        let mut bytes = Vec::new();
        bytes.push(read_byte(t).await?); // MSD
        bytes.push(read_byte(t).await?); // LSD
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
impl From<i16> for MCShort {
    fn from(v: i16) -> MCShort {
        MCShort { value: v }
    }
}
impl Into<i16> for MCShort {
    fn into(self) -> i16 {
        self.value
    }
}
impl PartialEq<i16> for MCShort {
    fn eq(&self, other: &i16) -> bool {
        self.value == *other
    }
}
impl Display for MCShort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl TryFrom<Vec<u8>> for MCShort {
    type Error = &'static str;
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < 2 {
            Err("Not enough bytes")
        } else {
            let mut a = [0u8; 2];
            a.copy_from_slice(&get_bytes(bytes, 2));
            Ok(MCShort {
                value: i16::from_be_bytes(a),
            })
        }
    }
}
impl Into<Vec<u8>> for MCShort {
    fn into(self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

/// The equivalent of a `u16`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MCUnsignedShort {
    pub value: u16, // 0 to 65535
}
impl MCUnsignedShort {
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<MCUnsignedShort> {
        let mut bytes = Vec::new();
        bytes.push(read_byte(t).await?); // MSD
        bytes.push(read_byte(t).await?); // LSD
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
impl From<u16> for MCUnsignedShort {
    fn from(v: u16) -> MCUnsignedShort {
        MCUnsignedShort { value: v }
    }
}
impl Into<u16> for MCUnsignedShort {
    fn into(self) -> u16 {
        self.value
    }
}
impl PartialEq<u16> for MCUnsignedShort {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
impl Display for MCUnsignedShort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl TryFrom<Vec<u8>> for MCUnsignedShort {
    type Error = &'static str;
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < 2 {
            Err("Not enough bytes")
        } else {
            let mut a = [0u8; 2];
            a.copy_from_slice(&get_bytes(bytes, 2));
            Ok(MCUnsignedShort {
                value: u16::from_be_bytes(a),
            })
        }
    }
}
impl Into<Vec<u8>> for MCUnsignedShort {
    fn into(self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

/// The equivalent of an `i32`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MCInt {
    pub value: i32, // -2147483648 to 2147483647
}
impl MCInt {
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<MCInt> {
        let mut bytes = Vec::new();
        for _ in 0..4 {
            bytes.push(read_byte(t).await?);
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
impl From<i32> for MCInt {
    fn from(v: i32) -> MCInt {
        MCInt { value: v }
    }
}
impl Into<i32> for MCInt {
    fn into(self) -> i32 {
        self.value
    }
}
impl PartialEq<i32> for MCInt {
    fn eq(&self, other: &i32) -> bool {
        self.value == *other
    }
}
impl Display for MCInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl TryFrom<Vec<u8>> for MCInt {
    type Error = &'static str;
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < 4 {
            Err("Not enough bytes")
        } else {
            let mut a = [0u8; 4];
            a.copy_from_slice(&get_bytes(bytes, 4));
            Ok(MCInt {
                value: i32::from_be_bytes(a),
            })
        }
    }
}
impl Into<Vec<u8>> for MCInt {
    fn into(self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

/// The equivalent of a `u32`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MCUnsignedInt {
    pub value: u32, // 0 to 4294967295
}
impl MCUnsignedInt {
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<MCUnsignedInt> {
        let mut bytes = Vec::new();
        for _ in 0..4 {
            bytes.push(read_byte(t).await?);
        }
        Ok(MCUnsignedInt::from_bytes(bytes))
    }
    pub fn from_bytes(v: Vec<u8>) -> MCUnsignedInt {
        let mut a = [0u8; 4];
        a.copy_from_slice(&get_bytes(v, 4));
        MCUnsignedInt {
            value: u32::from_be_bytes(a),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}
impl From<u32> for MCUnsignedInt {
    fn from(v: u32) -> MCUnsignedInt {
        MCUnsignedInt { value: v }
    }
}
impl Into<u32> for MCUnsignedInt {
    fn into(self) -> u32 {
        self.value
    }
}
impl PartialEq<u32> for MCUnsignedInt {
    fn eq(&self, other: &u32) -> bool {
        self.value == *other
    }
}
impl Display for MCUnsignedInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl TryFrom<Vec<u8>> for MCUnsignedInt {
    type Error = &'static str;
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < 4 {
            Err("Not enough bytes")
        } else {
            let mut a = [0u8; 4];
            a.copy_from_slice(&get_bytes(bytes, 4));
            Ok(MCUnsignedInt {
                value: u32::from_be_bytes(a),
            })
        }
    }
}
impl Into<Vec<u8>> for MCUnsignedInt {
    fn into(self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

/// The equivalent of an `864`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MCLong {
    pub value: i64, // -9223372036854775808 to 9223372036854775807
}
impl MCLong {
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<MCLong> {
        let mut bytes = Vec::new();
        for _ in 0..8 {
            bytes.push(read_byte(t).await?);
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
impl From<i64> for MCLong {
    fn from(v: i64) -> MCLong {
        MCLong { value: v }
    }
}
impl Into<i64> for MCLong {
    fn into(self) -> i64 {
        self.value
    }
}
impl PartialEq<i64> for MCLong {
    fn eq(&self, other: &i64) -> bool {
        self.value == *other
    }
}
impl Display for MCLong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl TryFrom<Vec<u8>> for MCLong {
    type Error = &'static str;
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < 8 {
            Err("Not enough bytes")
        } else {
            let mut a = [0u8; 8];
            a.copy_from_slice(&get_bytes(bytes, 8));
            Ok(MCLong {
                value: i64::from_be_bytes(a),
            })
        }
    }
}
impl Into<Vec<u8>> for MCLong {
    fn into(self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

/// The equivalent of a `u64`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MCUnsignedLong {
    pub value: u64, // 0 to 18446744073709551615
}
impl MCUnsignedLong {
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<MCUnsignedLong> {
        let mut bytes = Vec::new();
        for _ in 0..8 {
            bytes.push(read_byte(t).await?);
        }
        Ok(MCUnsignedLong::from_bytes(bytes))
    }
    pub fn from_bytes(v: Vec<u8>) -> MCUnsignedLong {
        let mut a = [0u8; 8];
        a.copy_from_slice(&get_bytes(v, 8));
        MCUnsignedLong {
            value: u64::from_be_bytes(a),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}
impl From<u64> for MCUnsignedLong {
    fn from(v: u64) -> MCUnsignedLong {
        MCUnsignedLong { value: v }
    }
}
impl Into<u64> for MCUnsignedLong {
    fn into(self) -> u64 {
        self.value
    }
}
impl PartialEq<u64> for MCUnsignedLong {
    fn eq(&self, other: &u64) -> bool {
        self.value == *other
    }
}
impl Display for MCUnsignedLong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl TryFrom<Vec<u8>> for MCUnsignedLong {
    type Error = &'static str;
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < 8 {
            Err("Not enough bytes")
        } else {
            let mut a = [0u8; 8];
            a.copy_from_slice(&get_bytes(bytes, 8));
            Ok(MCUnsignedLong {
                value: u64::from_be_bytes(a),
            })
        }
    }
}
impl Into<Vec<u8>> for MCUnsignedLong {
    fn into(self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

/// The equivalent of a `f32`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MCFloat {
    pub value: f32, // 32-bit floating point number
}
impl MCFloat {
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<MCFloat> {
        let mut bytes = Vec::new();
        for _ in 0..4 {
            bytes.push(read_byte(t).await?);
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
impl From<f32> for MCFloat {
    fn from(v: f32) -> MCFloat {
        MCFloat { value: v }
    }
}
impl Into<f32> for MCFloat {
    fn into(self) -> f32 {
        self.value
    }
}
impl PartialEq<f32> for MCFloat {
    fn eq(&self, other: &f32) -> bool {
        self.value == *other
    }
}
impl Display for MCFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl TryFrom<Vec<u8>> for MCFloat {
    type Error = &'static str;
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < 4 {
            Err("Not enough bytes")
        } else {
            let mut a = [0u8; 4];
            a.copy_from_slice(&get_bytes(bytes, 4));
            Ok(MCFloat {
                value: f32::from_be_bytes(a),
            })
        }
    }
}
impl Into<Vec<u8>> for MCFloat {
    fn into(self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

/// The equivalent of a `f64`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MCDouble {
    pub value: f64, // 64-bit floating point number
}
impl MCDouble {
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<MCDouble> {
        let mut bytes = Vec::new();
        for _ in 0..8 {
            bytes.push(read_byte(t).await?);
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
impl From<f64> for MCDouble {
    fn from(v: f64) -> MCDouble {
        MCDouble { value: v }
    }
}
impl Into<f64> for MCDouble {
    fn into(self) -> f64 {
        self.value
    }
}
impl PartialEq<f64> for MCDouble {
    fn eq(&self, other: &f64) -> bool {
        self.value == *other
    }
}
impl Display for MCDouble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl TryFrom<Vec<u8>> for MCDouble {
    type Error = &'static str;
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < 8 {
            Err("Not enough bytes")
        } else {
            let mut a = [0u8; 8];
            a.copy_from_slice(&get_bytes(bytes, 8));
            Ok(MCDouble {
                value: f64::from_be_bytes(a),
            })
        }
    }
}
impl Into<Vec<u8>> for MCDouble {
    fn into(self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

/// A variable-length integer.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MCVarInt {
    pub value: i32, // Variable length 32-bit integer
}
impl MCVarInt {
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<MCVarInt> {
        let mut num_read = 0;
        let mut result = 0i128;
        let mut read = 0u8;
        let mut run_once = false;
        while (read & 0b10000000) != 0 || !run_once {
            run_once = true;
            read = read_byte(t).await?;
            let value = (read & 0b01111111) as i128;
            result |= value << (7 * num_read);
            num_read += 1;
            if num_read > 5 {
                return Err(io_error("MCVarInt is too big"));
            }
        }
        Ok(MCVarInt {
            value: result as i32,
        })
    }
    pub fn from_bytes(_v: Vec<u8>) -> MCVarInt {
        unimplemented!()
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        Into::<Vec<u8>>::into(*self)
    }
}
impl From<u8> for MCVarInt {
    fn from(v: u8) -> MCVarInt {
        MCVarInt { value: v as i32 }
    }
}
impl Into<u8> for MCVarInt {
    fn into(self) -> u8 {
        self.value as u8
    }
}
impl PartialEq<u8> for MCVarInt {
    fn eq(&self, other: &u8) -> bool {
        self.value == *other as i32
    }
}
impl From<i32> for MCVarInt {
    fn from(v: i32) -> MCVarInt {
        MCVarInt { value: v }
    }
}
impl Into<i32> for MCVarInt {
    fn into(self) -> i32 {
        self.value
    }
}
impl PartialEq<i32> for MCVarInt {
    fn eq(&self, other: &i32) -> bool {
        self.value == *other
    }
}
impl Display for MCVarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl TryFrom<Vec<u8>> for MCVarInt {
    type Error = &'static str;
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let mut num_read = 0;
        let mut result: i32 = 0;
        loop {
            let value = bytes[num_read] & 0b01111111;
            result |= (value << (7 * num_read)) as i32;
            if bytes[num_read] & 0b10000000 == 0x00 {
                break;
            }
            num_read += 1;
            if num_read == bytes.len() {
                return Err("Not enough bytes");
            }
            if num_read > 5 {
                return Err("VarInt is too big");
            }
        }
        Ok(MCVarInt { value: result })
    }
}
impl Into<Vec<u8>> for MCVarInt {
    fn into(self) -> Vec<u8> {
        let mut value = self.value;
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
        out
    }
}
