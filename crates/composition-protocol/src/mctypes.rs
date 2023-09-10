use bytes::Bytes;
use composition_parsing::parsable::Parsable;

/// Alias for a u128.
pub type Uuid = u128;
pub use composition_parsing::VarInt;
/// Alias for a `serde_json::Value`.
pub type Json = composition_parsing::serde_json::Value;

/// An implementation of [the protocol's chat](https://wiki.vg/Chat).
#[derive(Debug, Clone, PartialEq)]
pub struct Chat {
    inner: Json,
}
impl Chat {
    pub fn basic<S: Into<String>>(text: S) -> Chat {
        let text: String = text.into();
        serde_json::json!({ "text": text }).into()
    }
}
impl Default for Chat {
    fn default() -> Self {
        Chat::basic("")
    }
}
impl From<Json> for Chat {
    fn from(value: Json) -> Self {
        Chat { inner: value }
    }
}
impl From<Chat> for Json {
    fn from(value: Chat) -> Self {
        value.inner
    }
}
impl Parsable for Chat {
    fn check(data: Bytes) -> composition_parsing::Result<()> {
        Json::check(data)
    }
    fn parse(data: &mut Bytes) -> composition_parsing::Result<Self> {
        Ok(Chat {
            inner: Json::parse(data)?,
        })
    }
    fn serialize(&self) -> Vec<u8> {
        self.inner.serialize()
    }
}

/// An implementation of the protocol's [Position](https://wiki.vg/Protocol#Position) type.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Position { x, y, z }
    }
}
impl Parsable for Position {
    fn check(data: Bytes) -> composition_parsing::Result<()> {
        i64::check(data)
    }
    fn parse(data: &mut Bytes) -> composition_parsing::Result<Self> {
        let i = i64::parse(data)?;

        // x: i26, z: i26, y: i12
        let x = i >> 38;
        let mut y = i & 0xFFF;
        if y >= 0x800 {
            y -= 0x1000;
        }
        let z = i << 26 >> 38;

        Ok(Position::new(x as i32, y as i32, z as i32))
    }
    fn serialize(&self) -> Vec<u8> {
        let i: i64 = ((self.x as i64 & 0x3FF_FFFF) << 38)
            | ((self.z as i64 & 0x3FF_FFFF) << 12)
            | (self.y as i64 & 0xFFF);
        i.serialize()
    }
}

/// An enum of the possible difficulties in Minecraft.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}
impl TryFrom<u8> for Difficulty {
    type Error = ();
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Difficulty::Peaceful),
            1 => Ok(Difficulty::Easy),
            2 => Ok(Difficulty::Normal),
            3 => Ok(Difficulty::Hard),
            _ => Err(()),
        }
    }
}
impl Parsable for Difficulty {
    fn check(data: Bytes) -> composition_parsing::Result<()> {
        u8::check(data)
    }
    fn parse(data: &mut Bytes) -> composition_parsing::Result<Self> {
        let difficulty = u8::parse(data)?;
        let difficulty: Difficulty = difficulty
            .try_into()
            .expect("TODO: handle invalid difficulty");
        Ok(difficulty)
    }
    fn serialize(&self) -> Vec<u8> {
        (*self as u8).serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_positions() -> Vec<(Position, Bytes)> {
        vec![
            // x: 01000110000001110110001100 z: 10110000010101101101001000 y: 001100111111
            (
                Position::new(18357644, 831, -20882616),
                Bytes::from_static(&[
                    0b01000110, 0b00000111, 0b01100011, 0b00101100, 0b00010101, 0b10110100,
                    0b10000011, 0b00111111,
                ]),
            ),
        ]
    }
    #[test]
    fn parse_position_works() {
        for (value, bytes) in get_positions() {
            assert_eq!(value, Position::parse(&mut bytes.clone()).unwrap());
        }
    }
    #[test]
    fn serialize_position_works() {
        for (value, bytes) in get_positions() {
            assert_eq!(bytes, value.serialize());
        }
    }
}
