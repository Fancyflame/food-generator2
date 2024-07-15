use anyhow::{anyhow, Result};
use decoder::decode;
use encoder::encode;

#[cfg(feature = "compression")]
use flate2::{
    write::{GzDecoder, GzEncoder},
    Compression,
};
use syntax::Section;

use std::env::{args, current_dir};
#[cfg(feature = "compression")]
use std::io::Write;

mod decoder;
mod encoder;
mod syntax;

fn main() -> Result<()> {
    let library = current_dir().unwrap_or_default().join("library");
    println!("加载库文件夹：{}", library.display());
    let section = syntax::parse(library)?;
    println!("库文件夹已加载");

    let mut args = args();
    let mode = args
        .nth(1)
        .ok_or_else(|| anyhow!("必须指定`encode`或者`decode`"))?;
    let text = args
        .next()
        .ok_or_else(|| anyhow!("必须指定要编码或解码的文本"))?;

    match &*mode {
        "e" | "encode" => encode_mode(&section, &text),
        "d" | "decode" => decode_mode(&section, &text)?,
        other => {
            return Err(anyhow!(
                "未知的指令：`{other}`。必须为`encode`, `e`, `decode`, `d`其中之一"
            ))
        }
    };

    Ok(())
}

fn decode_mode(section: &Section, encoded_text: &str) -> Result<()> {
    println!("正在解码：{encoded_text}\n");

    let decoded = decode(&encoded_text, &section)?;

    #[cfg(feature = "compression")]
    let decoded = {
        let mut gz_decoder = GzDecoder::new(Vec::new());
        gz_decoder.write_all(&decoded).unwrap();
        gz_decoder.finish().unwrap()
    };

    println!("解码完成：\n{}", String::from_utf8(decoded)?);
    Ok(())
}

fn encode_mode(section: &Section, origin_text: &str) {
    println!("正在编码：{origin_text}\n");
    let origin = origin_text.trim().as_bytes();

    #[cfg(feature = "compression")]
    let origin = {
        let mut gz_encoder = GzEncoder::new(Vec::new(), Compression::best());
        gz_encoder.write_all(origin).unwrap();
        gz_encoder.finish().unwrap()
    };

    let encoded = encode(&section, &origin);
    println!("编码完成：\n{encoded}");
}
