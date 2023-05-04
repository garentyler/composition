#![deny(clippy::all)]

pub mod error;
pub mod parsable;

pub use error::{Error, ParseResult, Result};
pub use parsable::Parsable;
pub use serde_json;

pub fn take_bytes(num: usize) -> impl Fn(&'_ [u8]) -> ParseResult<'_, &'_ [u8]> {
    move |data| {
        use std::cmp::Ordering;

        match data.len().cmp(&num) {
            Ordering::Greater => Ok((&data[num..], &data[..num])),
            Ordering::Equal => Ok((&[], data)),
            Ordering::Less => Err(Error::Eof),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct VarInt(i32);
impl std::ops::Deref for VarInt {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for VarInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        VarInt(value)
    }
}
impl From<VarInt> for i32 {
    fn from(value: VarInt) -> Self {
        *value
    }
}
impl From<usize> for VarInt {
    fn from(value: usize) -> Self {
        (value as i32).into()
    }
}
impl std::fmt::Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ClientState {
    Handshake,
    Status,
    Login,
    Play,
    Disconnected,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_bytes_works() {
        let data: [u8; 5] = [0, 1, 2, 3, 4];

        assert_eq!(take_bytes(3)(&data).unwrap(), (&data[3..], &data[..3]));
        assert_eq!(take_bytes(1)(&data).unwrap().0.len(), data.len() - 1);
        assert_eq!(take_bytes(1)(&data).unwrap().0[0], 1);
        assert_eq!(take_bytes(1)(&[0, 1]).unwrap().0.len(), 1);
        assert_eq!(take_bytes(1)(&[1]).unwrap().0.len(), 0);
        assert!(take_bytes(1)(&[]).is_err());
    }
}
