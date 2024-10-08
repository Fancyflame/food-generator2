const A: u16 = 110; // 选一个适合u8的乘数
const C: u16 = 13; // 选一个增量
const M: u16 = 256; // 模数为256，因为我们想要u8范围的输出
pub(crate) struct LcgU8 {
    state: u8,
}

impl LcgU8 {
    pub fn new(seed: u8) -> Self {
        Self { state: seed }
    }

    pub fn next(&mut self) -> u8 {
        let mut patched_state = self.state;
        let next_state = loop {
            let next_state = ((A * patched_state as u16 + C) % M) as u8;

            // 不允许产生和前面字节相同的反码
            if next_state == !self.state {
                patched_state = patched_state.wrapping_add(1);
                continue;
            }

            break next_state;
        };

        self.state = next_state;
        self.state
    }
}

#[test]
fn test() {
    let seed: u8 = 123; // 使用 u8 类型的种子
    let mut lcg = LcgU8::new(seed);
    for _ in 0..10 {
        println!("{}", lcg.next());
    }
}
