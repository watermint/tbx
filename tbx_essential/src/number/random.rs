use std::ops::Range;
use rand::{Rng};
use rand::prelude::ThreadRng;

/// Pseudo-random number generator.
///
/// This trait defines functions to generate pseudo-random numbers
/// for each integer and floating point types.
///
/// Functions named `next_<type name>` generates pseudo-random numbers
/// in the specified range for each type, and functions named `next_range_<type name>`
/// generates random numbers in the specified range.
///
/// Design considerations: The implementation of this trait wraps the trait [`rand::Rng`].
/// [`rand::Rng`] provides an interface to generate pseudo-random numbers
/// for each type via [`rand::Rng::gen`]. In order to clarify the random number
/// scope/distribution of each type, functions are prepared for all applicable
/// types without relying on generics.
pub trait Generator {
    /// Generate next random boolean value.
    fn next_bool(&mut self) -> bool;

    /// Generate next boolean with a probability of numerator/denominator of being true.
    /// `next_ratio(2, 3)` has chance of 2 in 3 of true.
    fn next_ratio(&mut self, numerator: u32, denominator: u32) -> bool;

    /// Generate next integer value in the interval `[0, 255]`.
    fn next_u8(&mut self) -> u8;

    /// Generate next integer value in the interval `[0, u16::MAX]`
    fn next_u16(&mut self) -> u16;

    /// Generate next integer value in the interval `[0, u32::MAX]`
    fn next_u32(&mut self) -> u32;

    /// Generate next integer value in the interval `[0, u64::MAX]`
    fn next_u64(&mut self) -> u64;

    /// Generate next integer value in the interval `[0, u128::MAX]`
    fn next_u128(&mut self) -> u128;

    /// Generate next integer value in the interval `[0, usize::MAX]`
    fn next_usize(&mut self) -> usize;

    /// Generate next integer value in the interval `[i8::MIN, i8::MAX]`
    fn next_i8(&mut self) -> i8;

    /// Generate next integer value in the interval `[i16::MIN, i16::MAX]`
    fn next_i16(&mut self) -> i16;

    /// Generate next integer value in the interval `[i32::MIN, i32::MAX]`
    fn next_i32(&mut self) -> i32;

    /// Generate next integer value in the interval `[i64::MIN, i64::MAX]`
    fn next_i64(&mut self) -> i64;

    /// Generate next integer value in the interval `[i128::MIN, i128::MAX]`
    fn next_i128(&mut self) -> i128;

    /// Generate next integer value in the interval `[isize::MIN, isize::MAX]`
    fn next_isize(&mut self) -> isize;

    /// Generate next random number in the interval `[0, 1)`
    /// including 0 but not 1.
    fn next_f32(&mut self) -> f32;

    /// Generate next random number in the interval `[0, 1)`
    /// including 0 but not 1.
    fn next_f64(&mut self) -> f64;

    /// Generate pseudo-random numbers within the specified scope.
    fn next_range_u8(&mut self, range: Range<u8>) -> u8;

    /// Generate pseudo-random numbers within the specified scope.
    fn next_range_u16(&mut self, range: Range<u16>) -> u16;

    /// Generate pseudo-random numbers within the specified scope.
    fn next_range_u32(&mut self, range: Range<u32>) -> u32;

    /// Generate pseudo-random numbers within the specified scope.
    fn next_range_u64(&mut self, range: Range<u64>) -> u64;

    /// Generate pseudo-random numbers within the specified scope.
    fn next_range_u128(&mut self, range: Range<u128>) -> u128;

    /// Generate pseudo-random numbers within the specified scope.
    fn next_range_usize(&mut self, range: Range<usize>) -> usize;

    /// Generate pseudo-random numbers within the specified scope.
    fn next_range_i8(&mut self, range: Range<i8>) -> i8;

    /// Generate pseudo-random numbers within the specified scope.
    fn next_range_i16(&mut self, range: Range<i16>) -> i16;

    /// Generate pseudo-random numbers within the specified scope.
    fn next_range_i32(&mut self, range: Range<i32>) -> i32;

    /// Generate pseudo-random numbers within the specified scope.
    fn next_range_i64(&mut self, range: Range<i64>) -> i64;

    /// Generate pseudo-random numbers within the specified scope.
    fn next_range_i128(&mut self, range: Range<i128>) -> i128;

    /// Generate pseudo-random numbers within the specified scope.
    fn next_range_isize(&mut self, range: Range<isize>) -> isize;
}

pub struct Random {
    rng: ThreadRng,
}

impl Random {
    /// Generate new thread local pseudo-Random generator.
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

    fn next_ratio(&mut self, numerator: u32, denominator: u32) -> bool {
        self.rng.gen_ratio(numerator, denominator)
    }

    fn next_u8(&mut self) -> u8 {
        self.rng.gen()
    }

    fn next_u16(&mut self) -> u16 {
        self.rng.gen()
    }

    fn next_u32(&mut self) -> u32 {
        self.rng.gen()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.gen()
    }

    fn next_u128(&mut self) -> u128 {
        self.rng.gen()
    }

    fn next_usize(&mut self) -> usize {
        self.rng.gen()
    }

    fn next_i8(&mut self) -> i8 {
        self.rng.gen()
    }

    fn next_i16(&mut self) -> i16 {
        self.rng.gen()
    }

    fn next_i32(&mut self) -> i32 {
        self.rng.gen()
    }

    fn next_i64(&mut self) -> i64 {
        self.rng.gen()
    }

    fn next_i128(&mut self) -> i128 {
        self.rng.gen()
    }

    fn next_isize(&mut self) -> isize {
        self.rng.gen()
    }

    fn next_f32(&mut self) -> f32 {
        self.rng.gen()
    }

    fn next_f64(&mut self) -> f64 {
        self.rng.gen()
    }

    fn next_range_u8(&mut self, range: Range<u8>) -> u8 {
        self.rng.gen_range(range)
    }

    fn next_range_u16(&mut self, range: Range<u16>) -> u16 {
        self.rng.gen_range(range)
    }

    fn next_range_u32(&mut self, range: Range<u32>) -> u32 {
        self.rng.gen_range(range)
    }

    fn next_range_u64(&mut self, range: Range<u64>) -> u64 {
        self.rng.gen_range(range)
    }

    fn next_range_u128(&mut self, range: Range<u128>) -> u128 {
        self.rng.gen_range(range)
    }

    fn next_range_usize(&mut self, range: Range<usize>) -> usize {
        self.rng.gen_range(range)
    }

    fn next_range_i8(&mut self, range: Range<i8>) -> i8 {
        self.rng.gen_range(range)
    }

    fn next_range_i16(&mut self, range: Range<i16>) -> i16 {
        self.rng.gen_range(range)
    }

    fn next_range_i32(&mut self, range: Range<i32>) -> i32 {
        self.rng.gen_range(range)
    }

    fn next_range_i64(&mut self, range: Range<i64>) -> i64 {
        self.rng.gen_range(range)
    }

    fn next_range_i128(&mut self, range: Range<i128>) -> i128 {
        self.rng.gen_range(range)
    }

    fn next_range_isize(&mut self, range: Range<isize>) -> isize {
        self.rng.gen_range(range)
    }
}


#[cfg(test)]
mod random {
    use crate::number::random::{Generator, Random};

    fn verify_next<T: PartialEq>(r: &mut Random, f: fn(r: &mut Random) -> T) {
        let mut last: T = f(r);
        let mut next: T = f(r);
        while last == next {
            last = next;
            next = f(r);
        }
    }

    #[test]
    fn test_next() {
        let mut r = Random::new_thread_local();

        verify_next(&mut r, |r| { r.next_bool() });
        verify_next(&mut r, |r| { r.next_ratio(2, 3) });

        // unsigned
        verify_next(&mut r, |r| { r.next_u8() });
        verify_next(&mut r, |r| { r.next_u16() });
        verify_next(&mut r, |r| { r.next_u32() });
        verify_next(&mut r, |r| { r.next_u64() });
        verify_next(&mut r, |r| { r.next_u128() });
        verify_next(&mut r, |r| { r.next_usize() });

        // signed
        verify_next(&mut r, |r| { r.next_i8() });
        verify_next(&mut r, |r| { r.next_i16() });
        verify_next(&mut r, |r| { r.next_i32() });
        verify_next(&mut r, |r| { r.next_i64() });
        verify_next(&mut r, |r| { r.next_i128() });
        verify_next(&mut r, |r| { r.next_isize() });

        // unsigned range
        verify_next(&mut r, |r| { r.next_range_u8(10..20) });
        verify_next(&mut r, |r| { r.next_range_u16(10..20) });
        verify_next(&mut r, |r| { r.next_range_u32(10..20) });
        verify_next(&mut r, |r| { r.next_range_u64(10..20) });
        verify_next(&mut r, |r| { r.next_range_u128(10..20) });
        verify_next(&mut r, |r| { r.next_range_usize(10..20) });

        // signed range
        verify_next(&mut r, |r| { r.next_range_i8(-10..10) });
        verify_next(&mut r, |r| { r.next_range_i16(-10..10) });
        verify_next(&mut r, |r| { r.next_range_i32(-10..10) });
        verify_next(&mut r, |r| { r.next_range_i64(-10..10) });
        verify_next(&mut r, |r| { r.next_range_i128(-10..10) });
        verify_next(&mut r, |r| { r.next_range_isize(-10..10) });

        // test types
        let _r: bool = r.next_bool();
        let _r: bool = r.next_ratio(2, 3);

        // unsigned
        let _r: u8 = r.next_u8();
        let _r: u16 = r.next_u16();
        let _r: u32 = r.next_u32();
        let _r: u64 = r.next_u64();
        let _r: u128 = r.next_u128();
        let _r: usize = r.next_usize();

        // signed
        let _r: i8 = r.next_i8();
        let _r: i16 = r.next_i16();
        let _r: i32 = r.next_i32();
        let _r: i64 = r.next_i64();
        let _r: i128 = r.next_i128();
        let _r: isize = r.next_isize();

        // unsigned range
        let _r: u8 = r.next_range_u8(10..20);
        let _r: u16 = r.next_range_u16(10..20);
        let _r: u32 = r.next_range_u32(10..20);
        let _r: u64 = r.next_range_u64(10..20);
        let _r: u128 = r.next_range_u128(10..20);
        let _r: usize = r.next_range_usize(10..20);

        // signed range
        let _r: i8 = r.next_range_i8(-10..10);
        let _r: i16 = r.next_range_i16(-10..10);
        let _r: i32 = r.next_range_i32(-10..10);
        let _r: i64 = r.next_range_i64(-10..10);
        let _r: i128 = r.next_range_i128(-10..10);
        let _r: isize = r.next_range_isize(-10..10);
    }
}