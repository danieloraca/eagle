use std::time::{SystemTime, UNIX_EPOCH};
use std::ops::Range;

pub struct SimpleRng(u64);

impl SimpleRng {
    pub fn new() -> Self {
        SimpleRng(simple_seed())
    }

    pub fn next(&mut self) -> u32 {
        // LCG: fast and decent for non-cryptographic randomness
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.0 >> 32) as u32
    }

    pub fn random_range_f32(&mut self, range: Range<f32>) -> f32 {
        let scale = (range.end - range.start) / (u32::MAX as f32 + 1.0);
        range.start + (self.next() as f32) * scale
    }

    pub fn random_range_u32(&mut self, range: Range<u32>) -> u32 {
        let span = range.end - range.start;
        range.start + self.next() % span
    }

    pub fn random_u8(&mut self) -> u8 {
        (self.next() & 0xFF) as u8
    }

    pub fn _random_range_u8(&mut self, range: Range<u8>) -> u8 {
        let span = range.end - range.start;
        range.start + self.random_u8() % span
    }
}

fn simple_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u64
}
