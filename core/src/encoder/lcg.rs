const A: u16 = 110; // 选一个适合u8的乘数
const C: u16 = 13; // 选一个增量
const M: u16 = 256; // 模数为256，因为我们想要u8范围的输出
const W: u8 = 17; // 当出现和初始值相同时添加的扰动

pub(crate) struct LcgU8 {
    state: u8,
    excluded: u8,
}

impl LcgU8 {
    pub fn new(seed: u8) -> Self {
        Self {
            state: seed,
            excluded: seed, // 不允许产生和初始状态相同的值
        }
    }

    pub fn next(&mut self) -> u8 {
        let next_state = (A * self.state as u16 + C) % M;
        self.state = next_state as u8;

        if self.state == self.excluded {
            self.state = self.state.wrapping_add(W);
        }

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
