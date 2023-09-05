/// When serializing or deserializing data encounters errors.
pub mod error;
/// The `Parsable` trait, and implementations for useful types.
pub mod parsable;
/// Useful re-exports.
pub mod prelude {
    pub use crate::{parsable::Parsable, VarInt};
    pub use bytes::{Buf, BufMut};
}

pub use error::{Error, Result};
pub use serde_json;

/// Implementation of the protocol's VarInt type.
///
/// Simple wrapper around an i32, but is parsed and serialized differently.
/// When the original i32 value is needed, simply `Deref` it.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct VarInt(i32);
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
impl std::fmt::Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
