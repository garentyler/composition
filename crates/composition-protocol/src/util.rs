use nom::error::FromExternalError;

pub fn parse_varint(mut data: &[u8]) -> nom::IResult<&[u8], i32> {
    let mut output = 0i32;
    let mut bytes_read = 0i32;

    loop {
        let (d, next_byte) = nom::bytes::streaming::take(1usize)(data)?;
        data = d;

        output |= ((next_byte[0] & 0x7f) as i32) << (bytes_read * 7);
        bytes_read += 1;
        if next_byte[0] & 0x80 != 0x80 {
            break;
        }
        if bytes_read >= 5 {
            break;
        }
    }
    nom::IResult::Ok((data, output))
}
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

pub fn parse_string(data: &[u8]) -> nom::IResult<&[u8], String> {
    let (data, len) = parse_varint(data)?;
    let (data, str_bytes) = nom::bytes::streaming::take(len as usize)(data)?;
    let s = String::from_utf8_lossy(str_bytes).to_string();
    nom::IResult::Ok((data, s))
}
pub fn serialize_string(value: &str) -> Vec<u8> {
    let mut output = vec![];
    output.extend_from_slice(&serialize_varint(value.len() as i32));
    output.extend_from_slice(value.as_bytes());
    output
}

pub fn parse_json(data: &[u8]) -> nom::IResult<&[u8], crate::Json> {
    use nom::error::{Error, ErrorKind};
    let (data, json) = parse_string(data)?;
    let json = serde_json::from_str(&json)
        .map_err(|e| nom::Err::Error(Error::from_external_error(data, ErrorKind::Verify, e)))?;
    Ok((data, json))
}
pub fn serialize_json(value: &crate::Json) -> Vec<u8> {
    serialize_string(&serde_json::to_string(value).expect("valid json"))
}

pub fn parse_chat(data: &[u8]) -> nom::IResult<&[u8], crate::Chat> {
    parse_json(data)
}
pub fn serialize_chat(value: &crate::Chat) -> Vec<u8> {
    serialize_json(value)
}

pub fn parse_uuid(data: &[u8]) -> nom::IResult<&[u8], crate::Uuid> {
    nom::number::streaming::be_u128(data)
}
pub fn serialize_uuid(value: &crate::Uuid) -> Vec<u8> {
    value.to_be_bytes().to_vec()
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}
impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Position { x, y, z }
    }
    pub fn parse(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, i) = nom::number::streaming::be_i64(data)?;

        // x: i26, z: i26, y: i12
        let x = i >> 38;
        let mut y = i & 0xFFF;
        if y >= 0x800 {
            y -= 0x1000;
        }
        let z = i << 26 >> 38;

        Ok((data, Position::new(x as i32, y as i32, z as i32)))
    }
    pub fn serialize(&self) -> Vec<u8> {
        let i: i64 = ((self.x as i64 & 0x3FF_FFFF) << 38)
            | ((self.z as i64 & 0x3FF_FFFF) << 12)
            | (self.y as i64 & 0xFFF);
        i.to_be_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_varints() -> Vec<(i32, Vec<u8>)> {
        vec![
            (0, vec![0x00]),
            (1, vec![0x01]),
            (2, vec![0x02]),
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
