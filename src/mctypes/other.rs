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
    unimplemented!()
}
pub fn serialize_json(value: JSON) -> Vec<u8> {
    unimplemented!()
}
