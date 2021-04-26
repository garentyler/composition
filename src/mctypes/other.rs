use super::*;
use std::convert::{From, Into, TryFrom};
use std::fmt::{Debug, Display};

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
        if bytes.is_empty() {
            Err("Not enough bytes")
        } else if bytes[0] == 1u8 {
            Ok(MCBoolean::True)
        } else {
            Ok(MCBoolean::False)
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
impl MCBoolean {
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<MCBoolean> {
        let b = read_byte(t).await?;
        Ok(MCBoolean::try_from(vec![b]).unwrap())
    }
}

/// The equivalent of a `String`.
#[derive(PartialEq)]
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
        MCString { value: s }
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
impl Debug for MCString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MCString {{ \"{}\" ({} chars) }}",
            self.value,
            self.value.len()
        )
    }
}
impl Display for MCString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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
impl MCString {
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let str_len = MCVarInt::read(t).await?;
        let mut str_bytes = vec![];
        for _ in 0i32..str_len.into() {
            str_bytes.push(read_byte(t).await?);
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
impl From<&str> for MCChat {
    fn from(s: &str) -> MCChat {
        MCChat { text: s.into() }
    }
}
impl From<String> for MCChat {
    fn from(s: String) -> MCChat {
        MCChat { text: s.into() }
    }
}
impl Into<String> for MCChat {
    fn into(self) -> String {
        self.text.value
    }
}
impl PartialEq<&str> for MCChat {
    fn eq(&self, other: &&str) -> bool {
        self.text.value == **other
    }
}
impl PartialEq<String> for MCChat {
    fn eq(&self, other: &String) -> bool {
        self.text.value == *other
    }
}
impl PartialEq<&String> for MCChat {
    fn eq(&self, other: &&String) -> bool {
        self.text.value == **other
    }
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
        Into::<MCString>::into(
            json!({
                "text": self.text.value
            })
            .to_string(),
        )
        .into()
    }
}
impl MCChat {
    pub async fn read(_t: &mut TcpStream) -> tokio::io::Result<Self> {
        Err(io_error("Cannot read MCChat from stream"))
    }
}

// TODO: Actually make the MCPosition work.
#[derive(Debug, PartialEq, Clone)]
pub struct MCPosition {
    x: MCLong,
    y: MCLong,
    z: MCLong,
}
impl Display for MCPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z,)
    }
}
impl TryFrom<Vec<u8>> for MCPosition {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("Cannot read MCPosition from bytes")
    }
}
impl Into<Vec<u8>> for MCPosition {
    fn into(self) -> Vec<u8> {
        // Just output
        // {"text": "<data>"}
        let mut out = vec![];
        let mut temp = vec![];
        temp.extend_from_slice(
            &((((Into::<i64>::into(self.x) & 0x3FFFFFF) << 38)
                | ((Into::<i64>::into(self.y) & 0xFFF) << 26)
                | (Into::<i64>::into(self.z) & 0x3FFFFFF)) as u64)
                .to_be_bytes(),
        );
        // temp.extend_from_slice(&"{\"text\": \"".to_owned().into_bytes());
        // temp.extend_from_slice(&self.text.value.into_bytes());
        // temp.extend_from_slice(&"\"}".to_owned().into_bytes());
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl MCPosition {
    pub fn new() -> MCPosition {
        MCPosition {
            x: 0.into(),
            y: 0.into(),
            z: 0.into(),
        }
    }
    pub async fn read(_t: &mut TcpStream) -> tokio::io::Result<Self> {
        Err(io_error("Cannot read MCPosition from stream"))
    }
}
impl Default for MCPosition {
    fn default() -> Self {
        MCPosition {
            x: 0.into(),
            y: 0.into(),
            z: 0.into(),
        }
    }
}
