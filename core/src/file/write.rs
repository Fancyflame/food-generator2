use bytes::BufMut;
use deflate::deflate_bytes;

use crate::syntax::{Layer, Section, Seg};

pub fn save_lib(secs: &Vec<Section>) -> Vec<u8> {
    let mut data = Vec::new();

    put_varint(&mut data, secs.len() as _);
    for sec in secs {
        put_varint(&mut data, sec.encoder.len() as u32);
        for rule in &sec.encoder {
            put_varint(&mut data, rule.len() as u32);
            for seg in rule {
                put_seg(&mut data, seg);
            }
        }

        put_layer(&mut data, &sec.decoder);
    }

    deflate_bytes(&data)
}

fn put_seg(data: &mut Vec<u8>, seg: &Seg) {
    match seg {
        Seg::Text(txt) => {
            data.put_u8(0);
            put_varint(data, txt.len() as _);
            data.put(txt.as_bytes());
        }
        &Seg::Use(u) => {
            data.put_u8(1);
            put_varint(data, u);
        }
    }
}

fn put_layer(data: &mut Vec<u8>, layer: &Layer) {
    match layer {
        &Layer::Certain(value) => {
            data.put_u8(0);
            put_varint(data, value);
        }
        Layer::Branch(branch) => {
            data.put_u8(1);
            put_varint(data, branch.len() as _);
            for (&ch, value) in branch.iter() {
                put_varint(data, ch as u32);
                put_layer(data, value);
            }
        }
    }
}

pub(super) fn put_varint(vec: &mut Vec<u8>, value: u32) {
    let used_bits = 34 - value.leading_zeros(); // 2 bit to store length
    let used_bytes = used_bits.div_ceil(8);
    let skip = 4u32
        .checked_sub(used_bytes)
        .expect("number cannot greater than `2 ^ 30 - 1`") as usize;
    let mut out = value.to_be_bytes();
    out[skip] |= (used_bytes as u8 - 1) << 6; // xx000000, xx is length
    vec.put(&out[skip..]);
}
