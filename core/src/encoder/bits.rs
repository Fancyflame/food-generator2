use super::lcg::LcgU8;

pub struct BitReader<'a> {
    current: u8,
    rest_bits: u8,
    source: Source<'a>,
}

enum Source<'a> {
    Data(&'a [u8]),
    EndToken(u8),
    Trailing(LcgU8),
}

impl<'a> BitReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        BitReader {
            current: 0,
            rest_bits: 0,
            source: if bytes.last().is_some() {
                Source::Data(bytes)
            } else {
                Source::Trailing(LcgU8::new(0))
            },
        }
    }

    pub fn get(&mut self) -> bool {
        if self.rest_bits == 0 {
            self.current = self.get_byte_from_source();
            self.rest_bits = 7;
        } else {
            self.rest_bits -= 1;
        }

        let result = self.current & 1 != 0;
        self.current >>= 1;
        result
    }

    fn get_byte_from_source(&mut self) -> u8 {
        match &mut self.source {
            Source::Data(data) => {
                let (&current, rest) = data.split_first().unwrap();
                if rest.is_empty() {
                    self.source = Source::EndToken(!current);
                } else {
                    *data = rest;
                };
                current
            }
            &mut Source::EndToken(token) => {
                self.source = Source::Trailing(LcgU8::new(token));
                token
            }
            Source::Trailing(lcg) => lcg.next(),
        }
    }

    pub fn ended(&self) -> bool {
        matches!(self.source, Source::Trailing(_))
    }
}
