pub struct BitWriter {
    current: u8,
    decoded: Vec<u8>,
    empty_bits: u8,
}

impl BitWriter {
    pub fn new() -> Self {
        BitWriter {
            current: 0,
            decoded: Vec::new(),
            empty_bits: 8,
        }
    }

    pub fn write(&mut self, data: bool) {
        self.empty_bits -= 1;

        self.current >>= 1;
        if data {
            self.current |= 0b1000_0000
        };

        if self.empty_bits == 0 {
            self.decoded.push(self.current);
            self.current = 0;
            self.empty_bits = 8;
        }
    }

    pub fn finish(mut self) -> Vec<u8> {
        if self.empty_bits < 8 {
            self.decoded.push(self.current);
        }

        let mut truncate_len = self.decoded.len();

        for (index, byte) in self.decoded.iter().copied().rev().enumerate() {
            if byte != 0 {
                truncate_len = index + 1;
                break;
            }
        }

        self.decoded.truncate(self.decoded.len() - truncate_len);
        self.decoded
    }
}
