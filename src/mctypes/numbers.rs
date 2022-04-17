use crate::prelude::*;

pub fn parse_byte(data: &[u8]) -> ParseResult<i8> {
    if data.is_empty() {
        Err(ParseError::NotEnoughData)
    } else {
        let value = i8::from_be_bytes([data[0]]);
        Ok((value, 1))
    }
}
pub fn serialize_byte(num: i8) -> [u8; 1] {
    num.to_be_bytes()
}

pub fn parse_short(data: &[u8]) -> ParseResult<i16> {
    if data.len() < 2 {
        Err(ParseError::NotEnoughData)
    } else {
        let value = i16::from_be_bytes([data[0], data[1]]);
        Ok((value, 2))
    }
}
pub fn serialize_short(num: i16) -> [u8; 2] {
    num.to_be_bytes()
}

pub fn parse_int(data: &[u8]) -> ParseResult<i32> {
    if data.len() < 4 {
        Err(ParseError::NotEnoughData)
    } else {
        let value = i32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        Ok((value, 4))
    }
}
pub fn serialize_int(num: i32) -> [u8; 4] {
    num.to_be_bytes()
}

pub fn parse_long(data: &[u8]) -> ParseResult<i64> {
    if data.len() < 8 {
        Err(ParseError::NotEnoughData)
    } else {
        let value = i64::from_be_bytes([
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        ]);
        Ok((value, 8))
    }
}
pub fn serialize_long(num: i64) -> [u8; 8] {
    num.to_be_bytes()
}

pub fn parse_varint(data: &[u8]) -> ParseResult<i32> {
    let mut offset = 0;
    let mut output = 0i32;
    let mut bytes_read = 0i32;

    loop {
        if data.len() <= offset {
            return Err(ParseError::NotEnoughData);
        }

        output |= (((data[offset] & 0x7f) as i32) << bytes_read * 7) as i32;
        bytes_read += 1;
        if data[offset] & 0x80 != 0x80 {
            break;
        }
        offset += 1;
        if bytes_read >= 5 {
            return Err(ParseError::VarIntTooBig);
        }
    }

    Ok((output, bytes_read as usize))
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

pub fn parse_unsigned_byte(data: &[u8]) -> ParseResult<u8> {
    if data.is_empty() {
        Err(ParseError::NotEnoughData)
    } else {
        let value = u8::from_be_bytes([data[0]]);
        Ok((value, 1))
    }
}
pub fn serialize_unsigned_byte(num: u8) -> [u8; 1] {
    num.to_be_bytes()
}

pub fn parse_unsigned_short(data: &[u8]) -> ParseResult<u16> {
    if data.len() < 2 {
        Err(ParseError::NotEnoughData)
    } else {
        let value = u16::from_be_bytes([data[0], data[1]]);
        Ok((value, 2))
    }
}
pub fn serialize_unsigned_short(num: u16) -> [u8; 2] {
    num.to_be_bytes()
}

pub fn parse_unsigned_int(data: &[u8]) -> ParseResult<u32> {
    if data.len() < 4 {
        Err(ParseError::NotEnoughData)
    } else {
        let value = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        Ok((value, 4))
    }
}
pub fn serialize_unsigned_int(num: u32) -> [u8; 4] {
    num.to_be_bytes()
}

pub fn parse_unsigned_long(data: &[u8]) -> ParseResult<u64> {
    if data.len() < 8 {
        Err(ParseError::NotEnoughData)
    } else {
        let value = u64::from_be_bytes([
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        ]);
        Ok((value, 8))
    }
}
pub fn serialize_unsigned_long(num: u64) -> [u8; 8] {
    num.to_be_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_varint_works() {
        let tests = vec![
            (Ok((0, 1)), vec![0x00]),
            (Ok((1, 1)), vec![0x01]),
            (Ok((2, 1)), vec![0x02]),
            (Ok((127, 1)), vec![0x7f]),
            (Ok((128, 2)), vec![0x80, 0x01]),
            (Ok((255, 2)), vec![0xff, 0x01]),
            (Ok((25565, 3)), vec![0xdd, 0xc7, 0x01]),
            (Ok((2097151, 3)), vec![0xff, 0xff, 0x7f]),
            (Ok((2147483647, 5)), vec![0xff, 0xff, 0xff, 0xff, 0x07]),
            (Ok((-1, 5)), vec![0xff, 0xff, 0xff, 0xff, 0x0f]),
            (Ok((-2147483648, 5)), vec![0x80, 0x80, 0x80, 0x80, 0x08]),
        ];
        for test in &tests {
            assert_eq!(test.0, parse_varint(&test.1));
        }
    }
    #[test]
    fn serialize_varint_works() {
        let tests = vec![
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
        ];
        for test in &tests {
            assert_eq!(serialize_varint(test.0), test.1);
        }
    }
}
