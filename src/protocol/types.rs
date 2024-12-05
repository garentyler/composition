use crate::protocol::parsing::{IResult, Parsable};

/// Alias for a u128.
pub type Uuid = u128;
pub use crate::protocol::parsing::VarInt;
/// Alias for a `serde_json::Value`.
pub type Json = serde_json::Value;
/// Alias for a `Json`.
pub type Chat = Json;

/// An implementation of the protocol's [Position](https://wiki.vg/Protocol#Position) type.
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
}
impl Parsable for Position {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, i) = i64::parse(data)?;

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
    #[tracing::instrument]
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        nom::combinator::map_res(u8::parse, Difficulty::try_from)(data)
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
