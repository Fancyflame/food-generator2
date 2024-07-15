use anyhow::{anyhow, Result};
use bits::BitWriter;

use crate::syntax::{Section, Segmentation};

mod bits;

struct Decoder<'a> {
    input: &'a str,
    output: BitWriter,
}

impl Decoder<'_> {
    fn decode(&mut self, section: &Section) -> Result<()> {
        let nth_rule = section
            .str2index
            .try_match(&mut self.input)
            .ok_or_else(|| self.error())?;

        self.write_msg(nth_rule, section.rules.len());

        for seg in &section.rules[nth_rule] {
            match seg {
                Segmentation::Text(txt) => {
                    if self.input.starts_with(txt) {
                        self.input = &self.input[txt.len()..];
                    } else {
                        return Err(self.error());
                    }
                }
                Segmentation::Use(r) => self.decode(&r)?,
            }
        }

        Ok(())
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

pub fn decode(s: &str, section: &Section) -> Result<Vec<u8>> {
    let mut decoder = Decoder {
        input: s,
        output: BitWriter::new(),
    };

    while !decoder.ended() {
        decoder.decode(section)?;
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
