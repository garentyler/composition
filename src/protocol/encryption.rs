use der::Encode;
use spki::{DecodePublicKey, SubjectPublicKeyInfo};

pub use crate::protocol::parsing::Parsable;
pub use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
pub use generic_array::{
    typenum::{UInt, UTerm, B1},
    GenericArray,
};
pub use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
pub type Aes128Cfb8Encryptor = cfb8::Encryptor<aes::Aes128>;
pub type Aes128Cfb8Decryptor = cfb8::Decryptor<aes::Aes128>;
pub type GenericCFB8BlockArray = GenericArray<u8, UInt<UTerm, B1>>;

impl Parsable for RsaPublicKey {
    fn parse(data: &[u8]) -> nom::IResult<&[u8], Self> {
        Ok((&[], RsaPublicKey::from_public_key_der(data).unwrap()))
    }
    fn serialize(&self) -> Vec<u8> {
        SubjectPublicKeyInfo::from_key(self.clone())
            .unwrap()
            .to_der()
            .unwrap()
    }
}
