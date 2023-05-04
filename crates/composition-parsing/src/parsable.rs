use crate::{take_bytes, Error, ParseResult, VarInt};
use byteorder::{BigEndian, ReadBytesExt};

pub trait Parsable {
    fn parse(data: &[u8]) -> ParseResult<'_, Self>
    where
        Self: Sized;
    fn serialize(&self) -> Vec<u8>;

    fn parse_optional(data: &[u8]) -> ParseResult<'_, Option<Self>>
    where
        Self: Sized,
    {
        let (data, exists) = bool::parse(data)?;
        if exists {
            let (data, thing) = Self::parse(data)?;
            Ok((data, Some(thing)))
        } else {
            Ok((data, None))
        }
    }
    fn parse_repeated(num: usize, mut data: &[u8]) -> ParseResult<'_, Vec<Self>>
    where
        Self: Sized,
    {
        let mut output = vec![];
        for _ in 0..num {
            let (d, item) = Self::parse(data)?;
            data = d;
            output.push(item);
        }
        Ok((data, output))
    }
    fn parse_vec(data: &[u8]) -> ParseResult<'_, Vec<Self>>
    where
        Self: Sized,
    {
        let (data, vec_len) = VarInt::parse(data)?;
        Self::parse_repeated(*vec_len as usize, data)
    }
}
impl<T: Parsable + std::fmt::Debug> Parsable for Option<T> {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, exists) = bool::parse(data)?;
        if exists {
            let (data, thing) = T::parse(data)?;
            Ok((data, Some(thing)))
        } else {
            Ok((data, None))
        }
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        match self {
            Some(t) => {
                let mut output = vec![];
                output.extend(true.serialize());
                output.extend(t.serialize());
                output
            }
            None => false.serialize(),
        }
    }
}
impl<T: Parsable + std::fmt::Debug> Parsable for Vec<T> {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        T::parse_vec(data)
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend(VarInt::from(self.len()).serialize());
        for item in self {
            output.extend(item.serialize());
        }
        output
    }
}

impl Parsable for serde_json::Value {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, json) = String::parse(data)?;
        let json = serde_json::from_str(&json)?;
        Ok((data, json))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        serde_json::to_string(self).expect("valid json").serialize()
    }
}
impl Parsable for VarInt {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let mut output = 0u32;
        let mut bytes_read = 0;

        for i in 0..=5 {
            if i == 5 {
                // VarInts can only have 5 bytes maximum.
                return Err(Error::VarIntTooLong);
            } else if data.len() <= i {
                return Err(Error::Eof);
            }

            let byte = data[i];
            output |= ((byte & 0x7f) as u32) << (7 * i);

            if byte & 0x80 != 0x80 {
                // We found the last byte of the VarInt.
                bytes_read = i + 1;
                break;
            }
        }

        Ok((&data[bytes_read..], VarInt(output as i32)))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        let mut value = self.0 as u32;
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
}
impl Parsable for String {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, len) = VarInt::parse(data)?;
        let (data, str_bytes) = take_bytes(*len as usize)(data)?;
        let s = String::from_utf8_lossy(str_bytes).to_string();
        Ok((data, s))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend(VarInt::from(self.len()).serialize());
        output.extend(self.as_bytes());
        output
    }
}
impl Parsable for u8 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, mut bytes) = take_bytes(1)(data)?;
        let i = bytes.read_u8().map_err(|_| Error::Eof)?;
        Ok((data, i))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for i8 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, mut bytes) = take_bytes(1)(data)?;
        let i = bytes.read_i8().map_err(|_| Error::Eof)?;
        Ok((data, i))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for u16 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, mut bytes) = take_bytes(2)(data)?;
        let i = bytes.read_u16::<BigEndian>().map_err(|_| Error::Eof)?;
        Ok((data, i))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for i16 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, mut bytes) = take_bytes(2)(data)?;
        let i = bytes.read_i16::<BigEndian>().map_err(|_| Error::Eof)?;
        Ok((data, i))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for u32 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, mut bytes) = take_bytes(4)(data)?;
        let i = bytes.read_u32::<BigEndian>().map_err(|_| Error::Eof)?;
        Ok((data, i))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for i32 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, mut bytes) = take_bytes(4)(data)?;
        let i = bytes.read_i32::<BigEndian>().map_err(|_| Error::Eof)?;
        Ok((data, i))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for u64 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, mut bytes) = take_bytes(8)(data)?;
        let i = bytes.read_u64::<BigEndian>().map_err(|_| Error::Eof)?;
        Ok((data, i))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for i64 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, mut bytes) = take_bytes(8)(data)?;
        let i = bytes.read_i64::<BigEndian>().map_err(|_| Error::Eof)?;
        Ok((data, i))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for u128 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, mut bytes) = take_bytes(16)(data)?;
        let i = bytes.read_u128::<BigEndian>().map_err(|_| Error::Eof)?;
        Ok((data, i))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for i128 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, mut bytes) = take_bytes(16)(data)?;
        let i = bytes.read_i128::<BigEndian>().map_err(|_| Error::Eof)?;
        Ok((data, i))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for f32 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, mut bytes) = take_bytes(4)(data)?;
        let i = bytes.read_f32::<BigEndian>().map_err(|_| Error::Eof)?;
        Ok((data, i))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for f64 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, mut bytes) = take_bytes(8)(data)?;
        let i = bytes.read_f64::<BigEndian>().map_err(|_| Error::Eof)?;
        Ok((data, i))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for bool {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, bytes) = take_bytes(1)(data)?;
        Ok((data, bytes[0] > 0x00))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        if *self {
            vec![0x01]
        } else {
            vec![0x00]
        }
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
            assert_eq!(value, *VarInt::parse(&bytes).unwrap().1);
        }
    }
    #[test]
    fn serialize_varint_works() {
        for (value, bytes) in get_varints() {
            assert_eq!(bytes, VarInt::from(value).serialize());
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
            assert_eq!(value, String::parse(&bytes).unwrap().1);
        }
    }
    #[test]
    fn serialize_string_works() {
        for (value, bytes) in get_strings() {
            assert_eq!(bytes, value.to_string().serialize());
        }
    }
}
