#![allow(dead_code)]

pub use functions::*;
pub use numbers::*;
pub use other::*;
use std::convert::{Into, TryFrom};
use std::fmt::Display;
use std::io::prelude::*;
use std::net::TcpStream;

/// Make sure all types can serialize and deserialize to/from `Vec<u8>`.
pub trait MCType: Into<Vec<u8>> + TryFrom<Vec<u8>> + Display {
    fn read(_stream: &mut TcpStream) -> std::io::Result<Self>;
}

/// Helper functions.
pub mod functions {
    use super::*;

    /// Read a single byte from the given `TcpStream`.
    pub fn read_byte(t: &mut TcpStream) -> std::io::Result<u8> {
        let mut buffer = [0u8; 1];
        t.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }
    /// Write a single byte to the given `TcpStream`.
    pub fn write_byte(t: &mut TcpStream, value: u8) -> std::io::Result<u8> {
        t.write(&[value])?;
        Ok(value)
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
    pub fn io_error(s: &str) -> std::io::Error {
        use std::io::{Error, ErrorKind};
        Error::new(ErrorKind::Other, s)
    }
}

/// The other types, (booleans and strings).
pub mod other {
    use super::*;
    use std::convert::{From, Into, TryFrom};
    use std::fmt::Display;

    /// The equivalent of a `bool`.
    #[derive(Debug, Copy, Clone, PartialEq)]
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
    impl Into<bool> for MCBoolean {
        fn into(self) -> bool {
            match self {
                MCBoolean::True => true,
                MCBoolean::False => false,
            }
        }
    }
    impl Display for MCBoolean {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    MCBoolean::True => "true",
                    MCBoolean::False => "false",
                }
            )
        }
    }
    impl TryFrom<Vec<u8>> for MCBoolean {
        type Error = &'static str;
        fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
            if bytes.len() < 1 {
                Err("Not enough bytes")
            } else {
                if bytes[0] == 1u8 {
                    Ok(MCBoolean::True)
                } else {
                    Ok(MCBoolean::False)
                }
            }
        }
    }
    impl Into<Vec<u8>> for MCBoolean {
        fn into(self) -> Vec<u8> {
            match self {
                MCBoolean::True => vec![0x01],
                MCBoolean::False => vec![0x00],
            }
        }
    }
    impl MCType for MCBoolean {
        fn read(t: &mut TcpStream) -> std::io::Result<MCBoolean> {
            let b = read_byte(t)?;
            Ok(MCBoolean::try_from(vec![b]).unwrap())
        }
    }

    /// The equivalent of a `String`.
    #[derive(Debug, PartialEq)]
    pub struct MCString {
        pub value: String,
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
    impl Into<String> for MCString {
        fn into(self) -> String {
            self.value
        }
    }
    impl PartialEq<&str> for MCString {
        fn eq(&self, other: &&str) -> bool {
            self.value == **other
        }
    }
    impl PartialEq<String> for MCString {
        fn eq(&self, other: &String) -> bool {
            self.value == *other
        }
    }
    impl PartialEq<&String> for MCString {
        fn eq(&self, other: &&String) -> bool {
            self.value == **other
        }
    }
    impl Clone for MCString {
        fn clone(&self) -> Self {
            MCString {
                value: self.value.clone(),
            }
        }
    }
    impl Display for MCString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "\"{}\" ({} chars)", self.value, self.value.len())
        }
    }
    impl TryFrom<Vec<u8>> for MCString {
        type Error = &'static str;
        fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
            Err("Cannot read MCString from bytes")
        }
    }
    impl Into<Vec<u8>> for MCString {
        fn into(self) -> Vec<u8> {
            let mut out = vec![];
            let str_len: Vec<u8> = MCVarInt::from(self.value.len() as i32).into();
            for b in str_len {
                out.push(b);
            }
            for b in self.value.into_bytes() {
                out.push(b);
            }
            out
        }
    }
    impl MCType for MCString {
        fn read(t: &mut TcpStream) -> std::io::Result<Self> {
            let str_len = MCVarInt::read(t)?;
            let mut str_bytes = vec![];
            for _ in 0..str_len.into() {
                str_bytes.push(read_byte(t)?);
            }
            Ok(MCString {
                value: String::from_utf8_lossy(&str_bytes).to_string(),
            })
        }
    }

    /// A normal `MCString`, but with extra embedded formatting data.
    #[derive(Debug, PartialEq)]
    pub struct MCChat {
        pub text: MCString,
    }
    impl Clone for MCChat {
        fn clone(&self) -> Self {
            MCChat {
                text: self.text.clone(),
            }
        }
    }
    impl Display for MCChat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "\"{}\" ({} chars)",
                self.text.value,
                self.text.value.len()
            )
        }
    }
    impl TryFrom<Vec<u8>> for MCChat {
        type Error = &'static str;
        fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
            Err("Cannot read MCChat from bytes")
        }
    }
    impl Into<Vec<u8>> for MCChat {
        fn into(self) -> Vec<u8> {
            // Just output
            // {"text": "<data>"}
            let mut out = vec![];
            let mut temp = vec![];
            temp.extend_from_slice(&"{\"text\": \"".to_owned().into_bytes());
            temp.extend_from_slice(&self.text.value.into_bytes());
            temp.extend_from_slice(&"\"}".to_owned().into_bytes());
            out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
            out.extend_from_slice(&temp);
            out
        }
    }
    impl MCType for MCChat {
        fn read(_t: &mut TcpStream) -> std::io::Result<Self> {
            Err(io_error("Cannot read MCChat from stream"))
        }
    }
}

/// All the numbers, from `i8` and `u8` to `i64` and `u64`, plus `VarInt`s.
pub mod numbers {
    use super::*;
    use std::convert::{From, Into, TryFrom};
    use std::fmt::Display;

    /// The equivalent of an `i8`
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct MCByte {
        pub value: i8, // -128 to 127
    }
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
            if bytes.len() < 1 {
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
    impl MCType for MCByte {
        fn read(t: &mut TcpStream) -> std::io::Result<Self> {
            let mut bytes = Vec::new();
            for _ in 0..1 {
                bytes.push(read_byte(t)?);
            }
            Ok(MCByte::try_from(bytes).unwrap())
        }
    }

    /// The equivalent of a `u8`
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct MCUnsignedByte {
        pub value: u8, // 0 to 255
    }
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
            if bytes.len() < 1 {
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
    impl MCType for MCUnsignedByte {
        fn read(t: &mut TcpStream) -> std::io::Result<Self> {
            let mut bytes = Vec::new();
            for _ in 0..1 {
                bytes.push(read_byte(t)?);
            }
            Ok(MCUnsignedByte::try_from(bytes).unwrap())
        }
    }

    /// The equivalent of an `i16`
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct MCShort {
        pub value: i16, // -32768 to 32767
    }
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
    impl MCType for MCShort {
        fn read(t: &mut TcpStream) -> std::io::Result<Self> {
            let mut bytes = Vec::new();
            for _ in 0..2 {
                bytes.push(read_byte(t)?);
            }
            Ok(MCShort::try_from(bytes).unwrap())
        }
    }

    /// The equivalent of a `u16`
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct MCUnsignedShort {
        pub value: u16, // 0 to 65535
    }
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
    impl MCType for MCUnsignedShort {
        fn read(t: &mut TcpStream) -> std::io::Result<Self> {
            let mut bytes = Vec::new();
            for _ in 0..2 {
                bytes.push(read_byte(t)?);
            }
            Ok(MCUnsignedShort::try_from(bytes).unwrap())
        }
    }

    /// The equivalent of an `i32`
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct MCInt {
        pub value: i32, // -2147483648 to 2147483647
    }
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
    impl MCType for MCInt {
        fn read(t: &mut TcpStream) -> std::io::Result<Self> {
            let mut bytes = Vec::new();
            for _ in 0..4 {
                bytes.push(read_byte(t)?);
            }
            Ok(MCInt::try_from(bytes).unwrap())
        }
    }

    /// The equivalent of a `u32`
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct MCUnsignedInt {
        pub value: u32, // 0 to 4294967295
    }
    impl MCUnsignedInt {
        pub fn from_stream(t: &mut TcpStream) -> std::io::Result<MCUnsignedInt> {
            let mut bytes = Vec::new();
            for _ in 0..4 {
                bytes.push(read_byte(t)?);
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
    impl MCType for MCUnsignedInt {
        fn read(t: &mut TcpStream) -> std::io::Result<Self> {
            let mut bytes = Vec::new();
            for _ in 0..4 {
                bytes.push(read_byte(t)?);
            }
            Ok(MCUnsignedInt::try_from(bytes).unwrap())
        }
    }

    /// The equivalent of an `864`
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct MCLong {
        pub value: i64, // -9223372036854775808 to 9223372036854775807
    }
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
    impl MCType for MCLong {
        fn read(t: &mut TcpStream) -> std::io::Result<Self> {
            let mut bytes = Vec::new();
            for _ in 0..8 {
                bytes.push(read_byte(t)?);
            }
            Ok(MCLong::try_from(bytes).unwrap())
        }
    }

    /// The equivalent of a `u64`
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct MCUnsignedLong {
        pub value: u64, // 0 to 18446744073709551615
    }
    impl MCUnsignedLong {
        pub fn from_stream(t: &mut TcpStream) -> std::io::Result<MCUnsignedLong> {
            let mut bytes = Vec::new();
            for _ in 0..8 {
                bytes.push(read_byte(t)?);
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
    impl MCType for MCUnsignedLong {
        fn read(t: &mut TcpStream) -> std::io::Result<Self> {
            let mut bytes = Vec::new();
            for _ in 0..8 {
                bytes.push(read_byte(t)?);
            }
            Ok(MCUnsignedLong::try_from(bytes).unwrap())
        }
    }

    /// The equivalent of a `f32`
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct MCFloat {
        pub value: f32, // 32-bit floating point number
    }
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
    impl MCType for MCFloat {
        fn read(t: &mut TcpStream) -> std::io::Result<Self> {
            let mut bytes = Vec::new();
            for _ in 0..4 {
                bytes.push(read_byte(t)?);
            }
            Ok(MCFloat::try_from(bytes).unwrap())
        }
    }

    /// The equivalent of a `f64`
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct MCDouble {
        pub value: f64, // 64-bit floating point number
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
    impl MCType for MCDouble {
        fn read(t: &mut TcpStream) -> std::io::Result<Self> {
            let mut bytes = Vec::new();
            for _ in 0..8 {
                bytes.push(read_byte(t)?);
            }
            Ok(MCDouble::try_from(bytes).unwrap())
        }
    }

    /// A variable-length integer.
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct MCVarInt {
        pub value: i32, // Variable length 32-bit integer
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
            return out;
        }
    }
    impl MCType for MCVarInt {
        fn read(t: &mut TcpStream) -> std::io::Result<Self> {
            let mut num_read = 0;
            let mut result = 0i32;
            let mut read = 0u8;
            let mut run_once = false;
            while (read & 0b10000000) != 0 || !run_once {
                run_once = true;
                read = read_byte(t)?;
                let value = (read & 0b01111111) as i32;
                result |= value << (7 * num_read);
                num_read += 1;
                if num_read > 5 {
                    return Err(io_error("MCVarInt is too big"));
                }
            }
            Ok(MCVarInt { value: result })
        }
    }
}
