use anyhow::{anyhow, Result};
use std::env::{args, current_dir};
use syntax::SerializeMap;

#[cfg(feature = "compression")]
use flate2::{
    write::{GzDecoder, GzEncoder},
    Compression,
};
#[cfg(feature = "compression")]
use std::io::Write;

pub use self::{decoder::decode, encoder::encode, syntax::parse};

pub mod decoder;
pub mod encoder;
pub mod file;
pub mod share_str;
pub mod syntax;

pub fn decode_mode(map: &SerializeMap, encoded_text: &str) -> Result<()> {
    println!("正在解码：{encoded_text}\n");

    let decoded = decode(map, encoded_text)?;

    #[cfg(feature = "compression")]
    let decoded = {
        let mut gz_decoder = GzDecoder::new(Vec::new());
        gz_decoder.write_all(&decoded).unwrap();
        gz_decoder.finish().unwrap()
    };

    println!("解码完成：\n{}", String::from_utf8(decoded)?);
    Ok(())
}

pub fn encode_mode(map: &SerializeMap, origin_text: &str) {
    println!("正在编码：{origin_text}\n");
    let origin = origin_text.trim().as_bytes();

    #[cfg(feature = "compression")]
    let origin = {
        let mut gz_encoder = GzEncoder::new(Vec::new(), Compression::best());
        gz_encoder.write_all(origin).unwrap();
        gz_encoder.finish().unwrap()
    };

    let encoded = encode(&map, &origin);
    println!("编码完成：\n{encoded}");
}

fn main() -> Result<()> {
    let mut library = current_dir().unwrap_or_default();
    library.push("..");
    library.push("library");
    let cache_path = library.join("cache.fg2");

    println!("加载库文件夹：{}", library.display());
    //let map = syntax::parse(library)?;
    let map = file::read_lib_from_file(cache_path)?;
    println!("库文件夹已加载");
    //file::save_lib_to_file(&map, cache_path)?;

    let mut args = args();
    let mode = args
        .nth(1)
        .ok_or_else(|| anyhow!("必须指定`encode`或者`decode`"))?;
    let text = args
        .next()
        .ok_or_else(|| anyhow!("必须指定要编码或解码的文本"))?;

    match &*mode {
        "e" | "encode" => encode_mode(&map, &text),
        "d" | "decode" => decode_mode(&map, &text)?,
        other => {
            return Err(anyhow!(
                "未知的指令：`{other}`。必须为`encode`, `e`, `decode`, `d`其中之一"
            ))
        }
    };

    Ok(())
}
