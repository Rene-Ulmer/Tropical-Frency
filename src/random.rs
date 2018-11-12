struct Random(u32);

impl Random {
    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1103515245).wrapping_add(12345);
        self.0 & 0x7FFFFFFF
    }
}

static mut RNG: Random = Random(666);
pub fn rand() -> u32 {
    unsafe { RNG.next() }
}

pub fn rand_between(a: u32, b: u32) -> u32 {
    if a >= b {
        unreachable!();
    }

    a + rand() % (b - a)
}
