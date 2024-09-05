use anyhow::Result;
use syntax::SerializeMap;

pub use self::{decoder::decode, encoder::encode};

pub mod decoder;
pub mod encoder;
pub mod file;
pub mod share_str;
pub mod syntax;
mod varint;

pub fn decode_mode(map: &SerializeMap, encoded_text: &str) -> Result<String> {
    let decoded = decode(map, encoded_text)?;
    #[cfg(feature = "compression")]
    let decoded = inflate::inflate_bytes(&decoded).map_err(anyhow::Error::msg)?;
    Ok(String::from_utf8(decoded)?)
}

pub fn encode_mode(map: &SerializeMap, decode_text: &str) -> Result<String> {
    let origin = decode_text.as_bytes();
    #[cfg(feature = "compression")]
    let origin = deflate::deflate_bytes(origin);
    Ok(encode(&map, &origin))
}
