#![allow(dead_code, unused_variables)]

use std::time::{SystemTime, UNIX_EPOCH};

struct Prng {
    seed: u32,
}

impl Prng {
    fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        prng
    }

    fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    // Return a pseudorandom value in the range of [0, 2147483647]
    fn next_u32(&mut self) -> u32 {
        let _ = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        self.seed
    }

    // return a pseudorandom value in the range of [0.0, 1.0]
    fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        f / (2147483647.0 + 1.0)
    }

    // return a pseudorandom value in the range fo [min, max]
    fn next_i32(&mut self, min: i32, max: i32) -> f64 {
        let range = (max - min) as f64;
        min as f64 + range * self.next_f64()
    }
}
fn main() {
    println!("Hello, world!");
}
