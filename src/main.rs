use anyhow::Result;
use decoder::decode;
use encoder::encode;

#[cfg(feature = "compression")]
use flate2::{
    write::{GzDecoder, GzEncoder},
    Compression,
};

#[cfg(feature = "compression")]
use std::io::Write;
//use parser::MyParser;

mod decoder;
mod encoder;
mod syntax;

fn main() -> Result<()> {
    let section = syntax::parse(r"D:\FancyFlame\food-generator2\library")?;
    let origin_text = "大家好啊，我是说的道理，今天来到大家想看的东西啊";
    println!("{origin_text}\n");
    let origin = origin_text.as_bytes();

    #[cfg(feature = "compression")]
    let origin = {
        let mut gz_encoder = GzEncoder::new(Vec::new(), Compression::best());
        gz_encoder.write_all(origin).unwrap();
        gz_encoder.finish().unwrap()
    };

    let encoded = encode(&section, &origin);
    println!("{encoded}\n");

    let decoded = decode(&encoded, &section)?;

    #[cfg(feature = "compression")]
    let decoded = {
        let mut gz_decoder = GzDecoder::new(Vec::new());
        gz_decoder.write_all(&decoded).unwrap();
        gz_decoder.finish().unwrap()
    };

    println!("{}", String::from_utf8(decoded)?);
    Ok(())
}
