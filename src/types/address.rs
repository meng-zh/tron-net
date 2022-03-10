use crate::types::{from_hex, from_base58};
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use secp256k1::{SecretKey,All, Secp256k1, PublicKey};
use once_cell::sync::Lazy;

use crate::types::error::Error;
use crate::types::signing::keccak256;


#[derive(Debug)]
pub struct Address([u8;20]);

const PREFIX:u8 = 0x41;
static CONTEXT: Lazy<Secp256k1<All>> = Lazy::new(Secp256k1::new);

impl Address {
    pub const fn new(inner: [u8; 20]) -> Self {
        Self(inner)
    }

    pub fn to_base58(&self)->String{
        from_hex(self.0.as_slice(),PREFIX)
    }
    pub fn from_base58(address:&str)->Self{
        let bytes = from_base58(address).unwrap();
        bytes[1..].try_into().unwrap()
    }
    pub fn from_hex(hex_address:&str)->Self{
        let hex_address = hex_address.trim_start_matches("0x");
        hex_address.as_bytes().try_into().unwrap()
    }
    pub fn from_private_key(private:&str)->Self{
        let prvk = SecretKey::from_str(private).unwrap();
        let secp = &*CONTEXT;
        let public_key = PublicKey::from_secret_key(secp, &prvk);
        let public_key = public_key.serialize_uncompressed();
        debug_assert_eq!(public_key[0], 0x04);
        let hash = keccak256(&public_key[1..]);
        hash[12..].try_into().unwrap()
    }
}
impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}
impl From<[u8; 20]> for Address {
    fn from(x: [u8; 20]) -> Self {
        Self::new(x)
    }
}
impl TryFrom<&[u8]> for Address{

    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self,Error> {
        if value.len() < 20{
            Err(Error::IllegalInput)
        }else{
            let value = value[value.len() - 20..].try_into().map_err(|_| Error::IllegalInput)?;
            Ok(Self::new(value))
        }
    }
}
impl Display for Address{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s =  from_hex(self.0.as_slice(),PREFIX);
        write!(f,"{}",s)
    }
}