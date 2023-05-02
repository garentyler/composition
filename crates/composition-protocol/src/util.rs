use crate::ProtocolError;
use byteorder::{BigEndian, ReadBytesExt};
use tracing::trace;

pub type ParseResult<'data, T> = crate::Result<(&'data [u8], T)>;

pub fn take_bytes(num: usize) -> impl Fn(&'_ [u8]) -> ParseResult<'_, &'_ [u8]> {
    move |data| {
        use std::cmp::Ordering;

        match data.len().cmp(&num) {
            Ordering::Greater => Ok((&data[num..], &data[..num])),
            Ordering::Equal => Ok((&[], data)),
            Ordering::Less => Err(ProtocolError::NotEnoughData),
        }
    }
}

#[tracing::instrument]
pub fn parse_varint(data: &[u8]) -> ParseResult<'_, i32> {
    trace!("{:?}", data);
    let mut output = 0u32;
    let mut bytes_read = 0;

    for i in 0..=5 {
        if i == 5 {
            // VarInts can only have 5 bytes maximum.
            return Err(ProtocolError::InvalidData);
        } else if data.len() <= i {
            return Err(ProtocolError::NotEnoughData);
        }

        let byte = data[i];
        output |= ((byte & 0x7f) as u32) << (7 * i);

        if byte & 0x80 != 0x80 {
            // We found the last byte of the VarInt.
            bytes_read = i + 1;
            break;
        }
    }

    Ok((&data[bytes_read..], output as i32))
}
#[tracing::instrument]
pub fn serialize_varint(value: i32) -> Vec<u8> {
    let mut value = value as u32;
    let mut output = vec![];
    loop {
        let data = (value & 0x7f) as u8;
        value >>= 7;

        if value == 0 {
            output.push(data);
            break;
        } else {
            output.push(data | 0x80);
        }
    }
    output
}

#[tracing::instrument]
pub fn parse_string(data: &[u8]) -> ParseResult<'_, String> {
    let (data, len) = parse_varint(data)?;
    let (data, str_bytes) = take_bytes(len as usize)(data)?;
    let s = String::from_utf8_lossy(str_bytes).to_string();
    Ok((data, s))
}
#[tracing::instrument]
pub fn serialize_string(value: &str) -> Vec<u8> {
    let mut output = vec![];
    output.extend_from_slice(&serialize_varint(value.len() as i32));
    output.extend_from_slice(value.as_bytes());
    output
}

#[tracing::instrument]
pub fn parse_json(data: &[u8]) -> ParseResult<'_, crate::Json> {
    trace!("parse_json: {:?}", data);
    let (data, json) = parse_string(data)?;
    let json = serde_json::from_str(&json)?;
    Ok((data, json))
}
#[tracing::instrument]
pub fn serialize_json(value: &crate::Json) -> Vec<u8> {
    trace!("serialize_json: {:?}", value);
    serialize_string(&serde_json::to_string(value).expect("valid json"))
}

#[tracing::instrument]
pub fn parse_chat(data: &[u8]) -> ParseResult<'_, crate::Chat> {
    trace!("parse_chat: {:?}", data);
    parse_json(data)
}
#[tracing::instrument]
pub fn serialize_chat(value: &crate::Chat) -> Vec<u8> {
    trace!("serialize_chat: {:?}", value);
    serialize_json(value)
}

#[tracing::instrument]
pub fn parse_uuid(data: &[u8]) -> ParseResult<'_, crate::Uuid> {
    trace!("parse_uuid: {:?}", data);
    let (data, mut bytes) = take_bytes(16)(data)?;
    let uuid = bytes
        .read_u128::<BigEndian>()
        .map_err(|_| ProtocolError::NotEnoughData)?;
    Ok((data, uuid))
}
#[tracing::instrument]
pub fn serialize_uuid(value: &crate::Uuid) -> Vec<u8> {
    trace!("serialize_uuid: {:?}", value);
    value.to_be_bytes().to_vec()
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl Position {
    #[tracing::instrument]
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Position { x, y, z }
    }
    #[tracing::instrument]
    pub fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        trace!("Position::parse: {:?}", data);
        let (data, mut bytes) = take_bytes(8)(data)?;
        let i = bytes
            .read_i64::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;

        // x: i26, z: i26, y: i12
        let x = i >> 38;
        let mut y = i & 0xFFF;
        if y >= 0x800 {
            y -= 0x1000;
        }
        let z = i << 26 >> 38;

        Ok((data, Position::new(x as i32, y as i32, z as i32)))
    }
    #[tracing::instrument]
    pub fn serialize(&self) -> Vec<u8> {
        trace!("Position::serialize: {:?}", self);
        let i: i64 = ((self.x as i64 & 0x3FF_FFFF) << 38)
            | ((self.z as i64 & 0x3FF_FFFF) << 12)
            | (self.y as i64 & 0xFFF);
        i.to_be_bytes().to_vec()
    }
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

    fn get_varints() -> Vec<(i32, Vec<u8>)> {
        vec![
            (0, vec![0x00]),
            (1, vec![0x01]),
            (2, vec![0x02]),
            (16, vec![0x10]),
            (127, vec![0x7f]),
            (128, vec![0x80, 0x01]),
            (255, vec![0xff, 0x01]),
            (25565, vec![0xdd, 0xc7, 0x01]),
            (2097151, vec![0xff, 0xff, 0x7f]),
            (2147483647, vec![0xff, 0xff, 0xff, 0xff, 0x07]),
            (-1, vec![0xff, 0xff, 0xff, 0xff, 0x0f]),
            (-2147483648, vec![0x80, 0x80, 0x80, 0x80, 0x08]),
        ]
    }
    #[test]
    fn parse_varint_works() {
        for (value, bytes) in get_varints() {
            assert_eq!(value, parse_varint(&bytes).unwrap().1);
        }
    }
    #[test]
    fn serialize_varint_works() {
        for (value, bytes) in get_varints() {
            assert_eq!(bytes, serialize_varint(value));
        }
    }

    fn get_strings() -> Vec<(&'static str, Vec<u8>)> {
        let s_127 = "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456";
        vec![
            ("", vec![0x00]),
            ("A", vec![0x01, 0x41]),
            ("AB", vec![0x02, 0x41, 0x42]),
            (s_127, {
                let mut v = vec![0x7f];
                v.extend_from_slice(s_127.as_bytes());
                v
            }),
        ]
    }
    #[test]
    fn parse_string_works() {
        for (value, bytes) in get_strings() {
            assert_eq!(value, parse_string(&bytes).unwrap().1);
        }
    }
    #[test]
    fn serialize_string_works() {
        for (value, bytes) in get_strings() {
            assert_eq!(bytes, serialize_string(value));
        }
    }

    fn get_positions() -> Vec<(Position, Vec<u8>)> {
        vec![
            // x: 01000110000001110110001100 z: 10110000010101101101001000 y: 001100111111
            (
                Position::new(18357644, 831, -20882616),
                vec![
                    0b01000110, 0b00000111, 0b01100011, 0b00101100, 0b00010101, 0b10110100,
                    0b10000011, 0b00111111,
                ],
            ),
        ]
    }
    #[test]
    fn parse_position_works() {
        for (value, bytes) in get_positions() {
            assert_eq!(value, Position::parse(&bytes).unwrap().1);
        }
    }
    #[test]
    fn serialize_position_works() {
        for (value, bytes) in get_positions() {
            assert_eq!(bytes, value.serialize());
        }
    }
}
