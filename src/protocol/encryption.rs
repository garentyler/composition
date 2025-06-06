use der::{
    asn1::{AnyRef, ObjectIdentifier},
    Decode, DecodeValue, Encode, EncodeValue, Header, Reader, Sequence, Tag,
};

pub use crate::protocol::parsing::Parsable;
pub use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
pub use generic_array::{
    typenum::{UInt, UTerm, B1},
    GenericArray,
};
pub use rsa::{RsaPrivateKey, RsaPublicKey};
pub type Aes128Cfb8Encryptor = cfb8::Encryptor<aes::Aes128>;
pub type Aes128Cfb8Decryptor = cfb8::Decryptor<aes::Aes128>;
pub type GenericCFB8BlockArray = GenericArray<u8, UInt<UTerm, B1>>;

impl Parsable for RsaPublicKey {
    fn parse(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let spki = SubjectPublicKeyInfo::from_der(data).unwrap();

        let modulus = rsa::BigUint::from_bytes_be(spki.subject_public_key.modulus.as_bytes());
        let exponent =
            rsa::BigUint::from_bytes_be(spki.subject_public_key.public_exponent.as_bytes());

        Ok((&[], RsaPublicKey::new(modulus, exponent).unwrap()))
    }
    fn serialize(&self) -> Vec<u8> {
        use rsa::traits::PublicKeyParts;
        let algorithm = PublicKeyAlgorithm::default();
        let subject_public_key = SubjectPublicKey {
            modulus: der::asn1::Int::new(&self.n().to_bytes_be()).unwrap(),
            public_exponent: der::asn1::Int::new(&self.e().to_bytes_be()).unwrap(),
        };
        let spki = SubjectPublicKeyInfo {
            algorithm,
            subject_public_key,
        };
        let mut buf = Vec::new();
        spki.encode(&mut buf).unwrap();
        buf
    }
}

// Custom decode implementation for SubjectPublicKeyInfo.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SubjectPublicKeyInfo<'a> {
    algorithm: PublicKeyAlgorithm<'a>,
    subject_public_key: SubjectPublicKey,
}
impl<'a> DecodeValue<'a> for SubjectPublicKeyInfo<'a> {
    fn decode_value<R: Reader<'a>>(reader: &mut R, _header: Header) -> der::Result<Self> {
        let algorithm = reader.decode()?;
        let spk_der: der::asn1::BitString = reader.decode()?;
        let spk_der = spk_der.as_bytes().unwrap();
        let subject_public_key = SubjectPublicKey::from_der(spk_der).unwrap();

        Ok(Self {
            algorithm,
            subject_public_key,
        })
    }
}
impl EncodeValue for SubjectPublicKeyInfo<'_> {
    fn value_len(&self) -> der::Result<der::Length> {
        self.algorithm.value_len()? + self.subject_public_key.value_len()?
    }
    fn encode_value(&self, writer: &mut impl der::Writer) -> der::Result<()> {
        self.algorithm.encode_value(writer)?;
        self.subject_public_key.encode_value(writer)?;
        Ok(())
    }
}
impl<'a> Sequence<'a> for SubjectPublicKeyInfo<'a> {}

#[derive(Debug, Clone, PartialEq, Eq, Sequence)]
struct PublicKeyAlgorithm<'a> {
    pub algorithm: ObjectIdentifier,
    pub parameters: Option<AnyRef<'a>>,
}
impl Default for PublicKeyAlgorithm<'_> {
    fn default() -> Self {
        Self {
            algorithm: ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.1"),
            parameters: Some(AnyRef::new(Tag::Null, &[]).unwrap()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Sequence)]
struct SubjectPublicKey {
    pub modulus: der::asn1::Int,
    pub public_exponent: der::asn1::Int,
}
