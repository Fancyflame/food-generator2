pub struct BitReader<'a> {
    current: u8,
    remaining: &'a [u8],
    rest_bits: u8,
    end_token_sent: bool,
}

impl<'a> BitReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        BitReader {
            current: 0,
            remaining: bytes,
            rest_bits: 0,
            end_token_sent: false,
        }
    }

    pub fn get(&mut self) -> bool {
        if self.rest_bits == 0 {
            match self.remaining.split_first() {
                Some((&first, rest)) => {
                    self.remaining = rest;
                    self.current = first;
                    self.rest_bits = 7;
                }
                None => {
                    if self.end_token_sent {
                        return false;
                    } else {
                        self.end_token_sent = true;
                        return true;
                    }
                }
            }
        } else {
            self.rest_bits -= 1;
        }

        let result = self.current & 1 != 0;
        self.current >>= 1;
        result
    }

    pub fn ended(&self) -> bool {
        self.end_token_sent
    }
}
