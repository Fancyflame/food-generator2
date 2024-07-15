use crate::syntax::{Section, Segmentation};

use bits::BitReader;

mod bits;

struct Encoder<'a> {
    input: BitReader<'a>,
    output: String,
}

impl<'a> Encoder<'a> {
    fn encode(&mut self, section: &'a Section) {
        let mut range = &*section.rules;
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
                Segmentation::Text(txt) => self.output.push_str(&txt),
                Segmentation::Use(r) => self.encode(&r),
            }
        }
    }
}

pub fn encode(section: &Section, input: &[u8]) -> String {
    let mut encoder = Encoder {
        input: BitReader::new(input),
        output: String::new(),
    };

    while !encoder.input.ended() {
        encoder.encode(section);
    }

    encoder.output
}
