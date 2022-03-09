use crate::types::{from_hex, from_base58, error};
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter};
use hex_literal::hex;
use crate::types::error::Error;

#[derive(Debug)]
pub struct Address([u8;20]);

const PREFIX:u8 = 0x41;

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