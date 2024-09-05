use core::str;
use std::{collections::HashMap, io::Read};

use bytes::Buf;
use inflate::inflate_bytes_zlib;

use crate::{
    share_str::ShareStr,
    syntax::{Layer, Section, Seg, SerializeMap},
};

pub fn read_lib(bytes: &[u8]) -> Option<SerializeMap> {
    let decompressed = inflate_bytes_zlib(bytes).ok()?;
    let mut bytes = &decompressed[..];

    let mut sections = Vec::new();
    for _ in 0..get_varint(&mut bytes)? {
        let mut rules = Vec::new();
        for _ in 0..get_varint(&mut bytes)? {
            let mut rule = Vec::new();
            for _ in 0..get_varint(&mut bytes)? {
                rule.push(get_seg(&mut bytes)?);
            }
            rules.push(rule);
        }

        let table = get_layer(&mut bytes)?;
        sections.push(Section {
            encoder: rules,
            decoder: table,
        });
    }
    Some(sections)
}

fn get_seg(data: &mut &[u8]) -> Option<Seg> {
    match data.get_u8() {
        0 => {
            let txt_len = get_varint(data)? as usize;
            if txt_len > data.len() {
                return None;
            }
            let (text, rest) = data.split_at(txt_len);
            let text = str::from_utf8(text).ok()?;
            *data = rest;
            Some(Seg::Text(ShareStr::new(text)))
        }
        1 => {
            let id = get_varint(data)?;
            Some(Seg::Use(id))
        }
        _ => None,
    }
}

fn get_layer(data: &mut &[u8]) -> Option<Layer> {
    match data.get_u8() {
        0 => Some(Layer::Certain(get_varint(data)?)),
        1 => {
            let mut branch = HashMap::new();
            let len = get_varint(data)?;
            for _ in 0..len {
                let ch = char::from_u32(get_varint(data)?)?;
                let layer = get_layer(data)?;
                if branch.insert(ch, layer).is_some() {
                    return None;
                }
            }
            Some(Layer::Branch(branch))
        }
        _ => None,
    }
}

pub(super) fn get_varint(data: &mut &[u8]) -> Option<u32> {
    let header = data.get_u8();
    let used_bytes = ((header & 0b1100_0000) >> 6) as usize + 1;

    let mut buffer = [0u8; 4];
    buffer[0] = header & 0b0011_1111;
    data.read_exact(&mut buffer[1..used_bytes]).ok()?;

    let num = u32::from_be_bytes(buffer);
    Some(num >> (4 - used_bytes) * 8)
}
