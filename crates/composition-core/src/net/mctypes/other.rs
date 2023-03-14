use super::*;
use crate::prelude::*;

pub fn parse_bool(data: &[u8]) -> ParseResult<bool> {
    if data.is_empty() {
        Err(ParseError::NotEnoughData)
    } else {
        Ok((data[0] == 1, 1))
    }
}
pub fn serialize_bool(value: bool) -> [u8; 1] {
    if value {
        [0x01]
    } else {
        [0x00]
    }
}

pub fn parse_string(data: &[u8]) -> ParseResult<String> {
    let mut offset = 0;
    let (length, offset_delta) = parse_varint(&data[offset..])?;
    offset += offset_delta;
    let length = length as usize;
    if data.len() < offset + length {
        return Err(ParseError::NotEnoughData);
    }
    let output = String::from_utf8_lossy(&data[offset..offset + length]).to_string();
    offset += length;
    Ok((output, offset))
}
pub fn serialize_string(value: &str) -> Vec<u8> {
    let mut output = vec![];
    output.extend_from_slice(&serialize_varint(value.len() as i32));
    output.extend_from_slice(value.as_bytes());
    output
}

pub fn parse_json(data: &[u8]) -> ParseResult<JSON> {
    let (value_string, offset) = parse_string(data)?;
    if let Ok(value) = serde_json::from_str(&value_string) {
        Ok((value, offset))
    } else {
        Err(ParseError::InvalidData)
    }
}
pub fn serialize_json(value: JSON) -> Vec<u8> {
    serialize_string(&serde_json::to_string(&value).expect("Could not serialize JSON"))
}

pub fn parse_nbt(data: &[u8]) -> ParseResult<NBT> {
    use quartz_nbt::io::{read_nbt, Flavor};
    use std::io::Cursor;
    let mut data = Cursor::new(data);
    // let (value_string, offset) = parse_string(data)?;
    if let Ok(value) = read_nbt(&mut data, Flavor::Uncompressed) {
        Ok((value.0, data.position() as usize))
    } else {
        Err(ParseError::InvalidData)
    }
}
pub fn serialize_nbt(value: NBT) -> Vec<u8> {
    use quartz_nbt::io::{write_nbt, Flavor};
    // serialize_string(&fastnbt::to_string(&value).expect("Could not serialize JSON"))
    let mut out = vec![];
    write_nbt(&mut out, None, &value, Flavor::Uncompressed).expect("Could not serialize NBT");
    out
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}
impl Position {
    pub fn new(x: i32, y: i16, z: i32) -> Position {
        Position { x, y, z }
    }
    pub fn parse(data: &[u8]) -> ParseResult<Position> {
        let (value, offset) = parse_unsigned_long(data)?;
        let x = (value >> 38) as i32;
        let y = (value & 0xFFF) as i16;
        let z = ((value >> 12) & 0x3FFFFFF) as i32;
        Ok((Position::new(x, y, z), offset))
    }
    pub fn serialize(&self) -> [u8; 8] {
        (((self.x as u64 & 0x3FFFFFF) << 38)
            | ((self.z as u64 & 0x3FFFFFF) << 12)
            | (self.y as u64 & 0xFFF))
            .to_be_bytes()
    }
}
