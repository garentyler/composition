pub use nom::IResult;
use nom::{
    bytes::streaming::{take, take_while_m_n},
    combinator::map_res,
    number::streaming as nom_nums,
    Parser,
};

/// Implementation of the protocol's VarInt type.
///
/// Simple wrapper around an i32, but is parsed and serialized differently.
/// When the original i32 value is needed, simply `Deref` it.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct VarInt(i32);
impl VarInt {
    pub fn parse_usize(data: &[u8]) -> IResult<&[u8], usize> {
        nom::combinator::map_res(Self::parse, usize::try_from)(data)
    }
}
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
impl TryFrom<VarInt> for usize {
    type Error = <usize as TryFrom<i32>>::Error;
    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        usize::try_from(*value)
    }
}
impl std::fmt::Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A structure that can be serialized and deserialized.
///
/// Similar to serde's `Serialize` and `Deserialize` traits.
pub trait Parsable {
    /// Attempt to parse (deserialize) `Self` from the given byte slice.
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;
    /// Serialize `self` into a vector of bytes.
    fn serialize(&self) -> Vec<u8>;

    /// Helper to optionally parse `Self`.
    ///
    /// An `Option<T>` is represented in the protocol as
    /// a boolean optionally followed by `T` if the boolean was true.
    fn parse_optional(data: &[u8]) -> IResult<&[u8], Option<Self>>
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

    /// Helper to parse `num` repetitions of `Self`.
    ///
    /// Useful with an array of known length.
    fn parse_repeated(num: usize, mut data: &[u8]) -> IResult<&[u8], Vec<Self>>
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

    /// Helper to parse an array of `Self`, when the length is unknown.
    ///
    /// In the protocol, arrays are commonly prefixed with their length
    /// as a `VarInt`.
    fn parse_vec(data: &[u8]) -> IResult<&[u8], Vec<Self>>
    where
        Self: Sized,
    {
        let (data, vec_len) = VarInt::parse(data)?;
        Self::parse_repeated(*vec_len as usize, data)
    }
}
impl<T: Parsable + std::fmt::Debug> Parsable for Option<T> {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
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
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
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
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        map_res(String::parse, |json: String| serde_json::from_str(&json))(data)
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        serde_json::to_string(self).expect("valid json").serialize()
    }
}
impl Parsable for VarInt {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let mut output = 0u32;

        // 0-4 bytes with the most significant bit set,
        // followed by one with the bit unset.
        let start_parser = take_while_m_n(0, 4, |byte| byte & 0x80 == 0x80);
        let end_parser = take_while_m_n(1, 1, |byte| byte & 0x80 != 0x80);
        let mut parser = start_parser.and(end_parser);
        let (rest, (start, end)) = parser.parse(data)?;

        for (i, &b) in start.iter().enumerate() {
            output |= ((b & 0x7f) as u32) << (7 * i);
        }
        output |= ((end[0] & 0x7f) as u32) << (7 * start.len());
        Ok((rest, VarInt(output as i32)))
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
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, len) = VarInt::parse(data)?;
        let (data, str_bytes) = take(*len as usize)(data)?;
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
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        nom_nums::u8(data)
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for i8 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        nom_nums::i8(data)
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for u16 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        nom_nums::be_u16(data)
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for i16 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        nom_nums::be_i16(data)
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for u32 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        nom_nums::be_u32(data)
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for i32 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        nom_nums::be_i32(data)
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for u64 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        nom_nums::be_u64(data)
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for i64 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        nom_nums::be_i64(data)
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for u128 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        nom_nums::be_u128(data)
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for i128 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        nom_nums::be_i128(data)
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for f32 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        nom_nums::be_f32(data)
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for f64 {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        nom_nums::be_f64(data)
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Parsable for bool {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        nom_nums::u8(data).map(|(data, num)| (data, num > 0x00))
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
        // Check if the VarInt is too long (>5 bytes).
        assert!(VarInt::parse(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x08]).is_err());
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
