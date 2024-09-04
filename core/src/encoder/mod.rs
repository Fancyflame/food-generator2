use bits::BitReader;

use crate::syntax::{Section, Seg, SerializeMap};

mod bits;

struct Encoder<'a> {
    input: BitReader<'a>,
    output: String,
}

impl<'a> Encoder<'a> {
    fn encode(&mut self, map: &SerializeMap, section: &Section) {
        let mut range = &section.encoder[..];
        assert!(!range.is_empty());

        let rule = loop {
            if let [rule] = range {
                break rule;
            }

            let (small, large) = range.split_at(range.len() / 2);
            range = if self.input.get() { large } else { small };
        };

        for seg in rule {
            match seg {
                Seg::Text(txt) => self.output.push_str(&txt),
                Seg::Use(r) => self.encode(map, &map[*r as usize]),
            }
        }
    }
}

pub fn encode(map: &SerializeMap, input: &[u8]) -> String {
    let mut encoder = Encoder {
        input: BitReader::new(input),
        output: String::new(),
    };

    while !encoder.input.ended() {
        encoder.encode(map, &map[0]);
    }

    encoder.output
}
