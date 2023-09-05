use crate::{Error, Result, VarInt};
use bytes::{Buf, BufMut, Bytes};

/// A structure that can be serialized and deserialized.
///
/// Similar to serde's `Serialize` and `Deserialize` traits.
pub trait Parsable {
    /// Check if there are enough bytes in `data` to parse Self.
    /// The error will be `Error::Eof` most of the time.
    fn check(data: Bytes) -> Result<()>;
    /// Attempt to parse `Self` from the given bytes.
    fn parse(data: &mut Bytes) -> Result<Self>
    where
        Self: Sized;
    /// Serialize `self` into a vector of bytes.
    fn serialize(&self) -> Vec<u8>;

    /// Helper to optionally parse `Self`.
    ///
    /// An `Option<T>` is represented in the protocol as
    /// a boolean optionally followed by `T` if the boolean was true.
    fn parse_optional(data: &mut Bytes) -> Result<Option<Self>>
    where
        Self: Sized,
    {
        Option::<Self>::parse(data)
    }

    /// Helper to parse `num` repetitions of `Self`.
    ///
    /// Useful with an array of known length.
    fn parse_repeated(num: usize, data: &mut Bytes) -> Result<Vec<Self>>
    where
        Self: Sized,
    {
        let mut output = Vec::with_capacity(num);
        for _ in 0..num {
            output.push(Self::parse(data)?);
        }
        Ok(output)
    }

    /// Helper to parse an array of `Self`, when the length is unknown.
    ///
    /// In the protocol, arrays are commonly prefixed with their length
    /// as a `VarInt`.
    fn parse_vec(data: &mut Bytes) -> Result<Vec<Self>>
    where
        Self: Sized,
    {
        let vec_len = VarInt::parse(data)?;
        Self::parse_repeated(*vec_len as usize, data)
    }
}
impl<T: Parsable> Parsable for Option<T> {
    fn check(mut data: Bytes) -> Result<()> {
        if bool::parse(&mut data)? {
            T::check(data)
        } else {
            Ok(())
        }
    }
    fn parse(data: &mut Bytes) -> Result<Self> {
        if bool::parse(data)? {
            let thing = T::parse(data)?;
            Ok(Some(thing))
        } else {
            Ok(None)
        }
    }
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
impl<T: Parsable> Parsable for Vec<T> {
    fn check(mut data: Bytes) -> Result<()> {
        T::parse_vec(&mut data).map(|_| ())
    }
    fn parse(data: &mut Bytes) -> Result<Self> {
        T::parse_vec(data)
    }
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
    fn check(data: Bytes) -> Result<()> {
        String::check(data)
    }
    fn parse(data: &mut Bytes) -> Result<Self> {
        let json = String::parse(data)?;
        let json = serde_json::from_str(&json)?;
        Ok(json)
    }
    fn serialize(&self) -> Vec<u8> {
        serde_json::to_string(self).expect("valid json").serialize()
    }
}
impl Parsable for VarInt {
    fn check(mut data: Bytes) -> Result<()> {
        for i in 0..=5 {
            if i == 5 {
                return Err(Error::VarIntTooLong);
            }
            if data.remaining() < 1 {
                // We need to read another byte but it isn't there.
                return Err(Error::Eof);
            }
            if data.get_u8() & 0x80 != 0x80 {
                // We found the last byte of the VarInt.
                break;
            }
        }
        Ok(())
    }
    fn parse(data: &mut Bytes) -> Result<Self> {
        let mut output = 0u32;

        for i in 0..=5 {
            if i == 5 {
                // VarInts can only have 5 bytes maximum.
                return Err(Error::VarIntTooLong);
            }
            if data.remaining() < 1 {
                // We need to read another byte but it isn't there.
                return Err(Error::Eof);
            }
            let byte = data.get_u8();
            output |= ((byte & 0x7f) as u32) << (7 * i);

            if byte & 0x80 != 0x80 {
                // We found the last byte of the VarInt.
                break;
            }
        }

        Ok(VarInt(output as i32))
    }
    fn serialize(&self) -> Vec<u8> {
        let mut value = self.0 as u32;
        let mut output = vec![];

        loop {
            let data = (value & 0x7f) as u8;
            value >>= 7;

            if value == 0 {
                output.put_u8(data);
                break;
            } else {
                output.put_u8(data | 0x80);
            }
        }

        output
    }
}
impl Parsable for String {
    fn check(mut data: Bytes) -> Result<()> {
        let len = *VarInt::parse(&mut data)? as usize;
        // Check that the buffer contains enough data.
        if data.remaining() >= len {
            Ok(())
        } else {
            Err(Error::Eof)
        }
    }
    fn parse(data: &mut Bytes) -> Result<Self> {
        Self::check(data.clone())?;
        let len = *VarInt::parse(data)? as usize;
        let mut str_bytes = vec![];
        str_bytes.put(&mut data.take(len));
        let s = String::from_utf8_lossy(&str_bytes[..]).to_string();
        Ok(s)
    }
    fn serialize(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend(VarInt::from(self.len()).serialize());
        output.extend(self.as_bytes());
        output
    }
}
impl Parsable for u8 {
    fn check(data: Bytes) -> Result<()> {
        if data.remaining() >= 1 {
            Ok(())
        } else {
            Err(Error::Eof)
        }
    }
    fn parse(data: &mut Bytes) -> Result<Self> {
        Self::check(data.clone())?;
        Ok(data.get_u8())
    }
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for i8 {
    fn check(data: Bytes) -> Result<()> {
        if data.remaining() >= 1 {
            Ok(())
        } else {
            Err(Error::Eof)
        }
    }
    fn parse(data: &mut Bytes) -> Result<Self> {
        Self::check(data.clone())?;
        Ok(data.get_i8())
    }
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for u16 {
    fn check(data: Bytes) -> Result<()> {
        if data.remaining() >= 2 {
            Ok(())
        } else {
            Err(Error::Eof)
        }
    }
    fn parse(data: &mut Bytes) -> Result<Self> {
        Self::check(data.clone())?;
        Ok(data.get_u16())
    }
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for i16 {
    fn check(data: Bytes) -> Result<()> {
        if data.remaining() >= 2 {
            Ok(())
        } else {
            Err(Error::Eof)
        }
    }
    fn parse(data: &mut Bytes) -> Result<Self> {
        Self::check(data.clone())?;
        Ok(data.get_i16())
    }
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for u32 {
    fn check(data: Bytes) -> Result<()> {
        if data.remaining() >= 4 {
            Ok(())
        } else {
            Err(Error::Eof)
        }
    }
    fn parse(data: &mut Bytes) -> Result<Self> {
        Self::check(data.clone())?;
        Ok(data.get_u32())
    }
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for i32 {
    fn check(data: Bytes) -> Result<()> {
        if data.remaining() >= 4 {
            Ok(())
        } else {
            Err(Error::Eof)
        }
    }
    fn parse(data: &mut Bytes) -> Result<Self> {
        Self::check(data.clone())?;
        Ok(data.get_i32())
    }
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for u64 {
    fn check(data: Bytes) -> Result<()> {
        if data.remaining() >= 8 {
            Ok(())
        } else {
            Err(Error::Eof)
        }
    }
    fn parse(data: &mut Bytes) -> Result<Self> {
        Self::check(data.clone())?;
        Ok(data.get_u64())
    }
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for i64 {
    fn check(data: Bytes) -> Result<()> {
        if data.remaining() >= 8 {
            Ok(())
        } else {
            Err(Error::Eof)
        }
    }
    fn parse(data: &mut Bytes) -> Result<Self> {
        Self::check(data.clone())?;
        Ok(data.get_i64())
    }
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for u128 {
    fn check(data: Bytes) -> Result<()> {
        if data.remaining() >= 16 {
            Ok(())
        } else {
            Err(Error::Eof)
        }
    }
    fn parse(data: &mut Bytes) -> Result<Self>
    where
        Self: Sized,
    {
        Self::check(data.clone())?;
        Ok(data.get_u128())
    }
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for i128 {
    fn check(data: Bytes) -> Result<()> {
        if data.remaining() >= 16 {
            Ok(())
        } else {
            Err(Error::Eof)
        }
    }
    fn parse(data: &mut Bytes) -> Result<Self>
    where
        Self: Sized,
    {
        Self::check(data.clone())?;
        Ok(data.get_i128())
    }
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for f32 {
    fn check(data: Bytes) -> Result<()> {
        if data.remaining() >= 4 {
            Ok(())
        } else {
            Err(Error::Eof)
        }
    }
    fn parse(data: &mut Bytes) -> Result<Self>
    where
        Self: Sized,
    {
        Self::check(data.clone())?;
        Ok(data.get_f32())
    }
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for f64 {
    fn check(data: Bytes) -> Result<()> {
        if data.remaining() >= 8 {
            Ok(())
        } else {
            Err(Error::Eof)
        }
    }
    fn parse(data: &mut Bytes) -> Result<Self>
    where
        Self: Sized,
    {
        Self::check(data.clone())?;
        Ok(data.get_f64())
    }
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for bool {
    fn check(data: Bytes) -> Result<()> {
        u8::check(data)
    }
    fn parse(data: &mut Bytes) -> Result<Self> {
        Ok(u8::parse(data)? > 0x00)
    }
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
    use crate::{Error, Result};

    fn get_varints() -> Vec<(Result<i32>, Bytes)> {
        vec![
            (Ok(0), Bytes::from_static(&[0x00])),
            (Ok(1), Bytes::from_static(&[0x01])),
            (Ok(2), Bytes::from_static(&[0x02])),
            (Ok(16), Bytes::from_static(&[0x10])),
            (Ok(127), Bytes::from_static(&[0x7f])),
            (Ok(128), Bytes::from_static(&[0x80, 0x01])),
            (Ok(255), Bytes::from_static(&[0xff, 0x01])),
            (Ok(25565), Bytes::from_static(&[0xdd, 0xc7, 0x01])),
            (Ok(2097151), Bytes::from_static(&[0xff, 0xff, 0x7f])),
            (
                Ok(2147483647),
                Bytes::from_static(&[0xff, 0xff, 0xff, 0xff, 0x07]),
            ),
            (Ok(-1), Bytes::from_static(&[0xff, 0xff, 0xff, 0xff, 0x0f])),
            (
                Ok(-2147483648),
                Bytes::from_static(&[0x80, 0x80, 0x80, 0x80, 0x08]),
            ),
            (
                Err(Error::VarIntTooLong),
                Bytes::from_static(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x08]),
            ),
            (Err(Error::Eof), Bytes::from_static(&[0x80])),
        ]
    }
    #[test]
    fn parse_varint_works() {
        for (expected, input) in get_varints() {
            let actual = VarInt::parse(&mut input.clone());
            assert_eq!(actual.is_ok(), expected.is_ok());

            match actual {
                Ok(actual) => assert_eq!(*actual, expected.unwrap()),
                Err(Error::Eof) => assert!(matches!(expected.unwrap_err(), Error::Eof)),
                Err(Error::VarIntTooLong) => {
                    assert!(matches!(expected.unwrap_err(), Error::VarIntTooLong))
                }
                _ => unreachable!(),
            }
        }
    }
    #[test]
    fn serialize_varint_works() {
        for (value, bytes) in get_varints() {
            if let Ok(value) = value {
                assert_eq!(bytes, VarInt::from(value).serialize());
            }
        }
    }

    fn get_strings() -> Vec<(Result<String>, Bytes)> {
        let s_127 = "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456".to_string();
        vec![
            (Ok(String::new()), Bytes::from_static(&[0x00])),
            (Ok("A".to_string()), Bytes::from_static(&[0x01, 0x41])),
            (
                Ok("AB".to_string()),
                Bytes::from_static(&[0x02, 0x41, 0x42]),
            ),
            (Ok(s_127.clone()), {
                let mut v = vec![0x7f];
                v.extend_from_slice(s_127.as_bytes());
                Bytes::from(v)
            }),
            (Err(Error::Eof), Bytes::from_static(&[0x01])),
        ]
    }
    #[test]
    fn parse_string_works() {
        for (expected, input) in get_strings() {
            let actual = String::parse(&mut input.clone());
            assert_eq!(actual.is_ok(), expected.is_ok());

            match actual {
                Ok(actual) => assert_eq!(*actual, expected.unwrap()),
                Err(Error::Eof) => assert!(matches!(expected.unwrap_err(), Error::Eof)),
                _ => unreachable!(),
            }
        }
    }
    #[test]
    fn serialize_string_works() {
        for (value, bytes) in get_strings() {
            if let Ok(value) = value {
                assert_eq!(bytes, value.to_string().serialize());
            }
        }
    }

    #[test]
    fn consecutive_parsing() {
        let mut data = Bytes::from_static(&[0x00, 0x01, 0x02, 0x03, 0x04]);
        assert_eq!(0x00, u8::parse(&mut data).unwrap());
        assert_eq!(0x01, u8::parse(&mut data).unwrap());
        assert_eq!(0x02, u8::parse(&mut data).unwrap());
        assert_eq!(0x0304, u16::parse(&mut data).unwrap());
        assert_eq!(0, data.remaining());
    }

    #[test]
    fn consecutive_checks() {
        let mut data = Bytes::from_static(&[0x00, 0x01, 0x02]);
        assert!(u8::check(data.clone()).is_ok());
        assert_eq!(0x00, u8::parse(&mut data).unwrap());
        assert_eq!(0x01, u8::parse(&mut data).unwrap());
        assert!(u16::check(data.clone()).is_err()); // Error::Eof
    }
}
