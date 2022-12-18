use std::ops::Range;
use rand::{Rng, RngCore};
use rand::prelude::ThreadRng;

pub trait Generator {
    fn next_bool(&mut self) -> bool;
    fn next_u8(&mut self) -> u8;
    fn next_u32(&mut self) -> u32;
    fn next_u64(&mut self) -> u64;
    fn next_i32(&mut self) -> i32;
    fn next_i64(&mut self) -> u64;
    fn next_f32(&mut self) -> f32;
    fn next_f64(&mut self) -> f64;
    fn next_u8_range(&mut self, range: Range<u8>) -> u8;
    fn next_u32_range(&mut self, range: Range<u32>) -> u32;
    fn next_u64_range(&mut self, range: Range<u64>) -> u64;
    fn next_i32_range(&mut self, range: Range<i32>) -> i32;
    fn next_i64_range(&mut self, range: Range<i64>) -> i64;
}

pub struct Random {
    rng: ThreadRng,
}

impl Random {
    /// Generate new thread local pseudo-Random generator
    pub fn new_thread_local() -> Self {
        Self {
            rng: rand::thread_rng()
        }
    }
}

impl Generator for Random {
    fn next_bool(&mut self) -> bool {
        self.rng.gen()
    }

    fn next_u8(&mut self) -> u8 {
        self.rng.gen()
    }

    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn next_i32(&mut self) -> i32 {
        self.rng.gen()
    }

    fn next_i64(&mut self) -> u64 {
        self.rng.gen()
    }

    fn next_f32(&mut self) -> f32 {
        self.rng.gen()
    }

    fn next_f64(&mut self) -> f64 {
        self.rng.gen()
    }

    fn next_u8_range(&mut self, range: Range<u8>) -> u8 {
        self.rng.gen_range(range)
    }

    fn next_u32_range(&mut self, range: Range<u32>) -> u32 {
        self.rng.gen_range(range)
    }

    fn next_u64_range(&mut self, range: Range<u64>) -> u64 {
        self.rng.gen_range(range)
    }

    fn next_i32_range(&mut self, range: Range<i32>) -> i32 {
        self.rng.gen_range(range)
    }

    fn next_i64_range(&mut self, range: Range<i64>) -> i64 {
        self.rng.gen_range(range)
    }
}

#[cfg(test)]
mod random {
    use crate::number::random::{Generator, Random};

    #[test]
    fn test_next() {
        let mut r = Random::new_thread_local();

        println!("{}", r.next_u32());
    }
}