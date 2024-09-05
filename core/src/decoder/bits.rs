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
        // we ignore `empty_bits`

        let mut resized_len = self.decoded.len();
        for window in self.decoded.windows(2).rev() {
            resized_len -= 1;

            let &[front, back] = window else {
                unreachable!();
            };

            if back == !front {
                break;
            }
        }

        self.decoded.truncate(resized_len);
        self.decoded
    }
}
