use anyhow::{anyhow, Result};
use bits::BitWriter;

use crate::syntax::{Layer, Section, Seg, SerializeMap};

mod bits;

struct Decoder<'a> {
    input: &'a str,
    output: BitWriter,
}

impl Decoder<'_> {
    fn decode(&mut self, map: &SerializeMap, section: &Section) -> Result<()> {
        let nth_rule = self.match_index(section)?;
        self.write_msg(nth_rule, section.encoder.len());

        for seg in &section.encoder[nth_rule] {
            match seg {
                Seg::Text(txt) => {
                    if self.input.starts_with(&**txt) {
                        self.input = &self.input[txt.len()..];
                    } else {
                        return Err(self.error());
                    }
                }
                Seg::Use(r) => self.decode(map, &map[*r as usize])?,
            }
        }

        Ok(())
    }

    fn match_index(&self, section: &Section) -> Result<usize> {
        let mut chars = self.input.chars();
        let mut layer = &section.decoder;

        for ch in &mut chars {
            match layer {
                Layer::Branch(b) => match b.get(&ch) {
                    Some(l) => layer = l,
                    None => {
                        let (display_str, omit) =
                            truncate_str_after_chars(chars.as_str(), 10, "...");
                        return Err(anyhow!("found unexpected character `{ch}` when parsing at `{display_str}{omit}`"));
                    }
                },
                &Layer::Certain(c) => return Ok(c as _),
            }
        }

        return Err(anyhow!("unexpected end of stream"));
    }

    fn error(&self) -> anyhow::Error {
        let (display_str, omit) = truncate_str_after_chars(self.input, 10, "...");
        anyhow!("error when parsing at `{display_str}{omit}`")
    }

    fn write_msg(&mut self, msg: usize, total: usize) {
        let mut start = 0;
        let mut end = total;

        while end - start > 1 {
            let mid = (start + end) / 2;
            if msg < mid {
                self.output.write(false);
                end = mid;
            } else {
                self.output.write(true);
                start = mid;
            }
        }
    }

    fn ended(&self) -> bool {
        self.input.is_empty()
    }

    fn finish(self) -> Vec<u8> {
        assert!(self.ended());
        self.output.finish()
    }
}

pub fn decode(map: &SerializeMap, s: &str) -> Result<Vec<u8>> {
    let mut decoder = Decoder {
        input: s,
        output: BitWriter::new(),
    };

    while !decoder.ended() {
        decoder.decode(map, &map[0])?;
    }

    Ok(decoder.finish())
}

fn truncate_str_after_chars<'a, 'b>(
    s: &'a str,
    chars: usize,
    omit_display: &'b str,
) -> (&'a str, &'b str) {
    match s.char_indices().nth(chars + 1) {
        Some((idx, _)) => (&s[..idx], omit_display),
        None => (s, ""),
    }
}
