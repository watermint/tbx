
pub trait Bytes<T> {
    /// To byte (unsigned 8-bit integer) vector.
    fn as_bytes(&self) -> Vec<u8>;
}

impl Bytes<u8> for u8 {
    fn as_bytes(&self) -> Vec<u8> {
        vec!(*self)
    }
}

impl Bytes<Vec<u8>> for Vec<u8> {
    fn as_bytes(&self) -> Vec<u8> {
        self.clone()
    }
}

impl Bytes<u16> for u16 {
    fn as_bytes(&self) -> Vec<u8> {
        vec!(
            (*self >> 8) as u8,
            (*self & 0xff) as u8,
        )
    }
}

impl Bytes<Vec<u16>> for Vec<u16> {
    fn as_bytes(&self) -> Vec<u8> {
        self.iter().map(|x| x.as_bytes()).flatten().collect()
    }
}

impl Bytes<u32> for u32 {
    fn as_bytes(&self) -> Vec<u8> {
        vec!(
            (*self >> 24) as u8,
            (*self >> 16) as u8,
            (*self >> 8) as u8,
            (*self & 0xff) as u8,
        )
    }
}

impl Bytes<Vec<u32>> for Vec<u32> {
    fn as_bytes(&self) -> Vec<u8> {
        self.iter().map(|x| x.as_bytes()).flatten().collect()
    }
}

impl Bytes<u64> for u64 {
    fn as_bytes(&self) -> Vec<u8> {
        vec!(
            (*self >> 56) as u8,
            (*self >> 48) as u8,
            (*self >> 40) as u8,
            (*self >> 32) as u8,
            (*self >> 24) as u8,
            (*self >> 16) as u8,
            (*self >> 8) as u8,
            (*self & 0xff) as u8,
        )
    }
}

impl Bytes<Vec<u64>> for Vec<u64> {
    fn as_bytes(&self) -> Vec<u8> {
        self.iter().map(|x| x.as_bytes()).flatten().collect()
    }
}

impl Bytes<u128> for u128 {
    fn as_bytes(&self) -> Vec<u8> {
        vec!(
            (*self >> 120) as u8,
            (*self >> 112) as u8,
            (*self >> 104) as u8,
            (*self >> 96) as u8,
            (*self >> 88) as u8,
            (*self >> 80) as u8,
            (*self >> 72) as u8,
            (*self >> 64) as u8,
            (*self >> 56) as u8,
            (*self >> 48) as u8,
            (*self >> 40) as u8,
            (*self >> 32) as u8,
            (*self >> 24) as u8,
            (*self >> 16) as u8,
            (*self >> 8) as u8,
            (*self & 0xff) as u8,
        )
    }
}

impl Bytes<Vec<u128>> for Vec<u128> {
    fn as_bytes(&self) -> Vec<u8> {
        self.iter().map(|x| x.as_bytes()).flatten().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::number::byte::Bytes;

    #[test]
    fn test_as_bytes() {
        // Scalar
        assert_eq!(vec!(0x12), (0x12 as u8).as_bytes());
        assert_eq!(vec!(0x12, 0x34), (0x1234 as u16).as_bytes());
        assert_eq!(vec!(0x12, 0x34, 0x56, 0x78), (0x1234_5678 as u32).as_bytes());
        assert_eq!(vec!(0x12, 0x34, 0x56, 0x78, 0xab, 0xcd, 0xef, 0xfe),
                   (0x1234_5678_abcd_effe as u64).as_bytes());
        assert_eq!(vec!(0x12, 0x34, 0x56, 0x78,
                        0xab, 0xcd, 0xef, 0xfe,
                        0xdc, 0xba, 0x98, 0x76,
                        0x54, 0x32, 0x10, 0xfe),
                   (0x1234_5678_abcd_effe__dcba_9876_5432_10fe as u128).as_bytes());

        // Vec
        assert_eq!(vec!(0x12, 0x34), (vec!(0x12 as u8, 0x34 as u8).as_bytes()));
        assert_eq!(vec!(0x12, 0x34, 0x56, 0x78),
                   (vec!(0x1234 as u16, 0x5678 as u16).as_bytes()));
        assert_eq!(vec!(0x12, 0x34, 0x56, 0x78, 0xab, 0xcd, 0xef, 0xfe),
                   (vec!(0x1234_5678 as u32, 0xabcdeffe as u32).as_bytes()));
        assert_eq!(vec!(0x12, 0x34, 0x56, 0x78,
                        0xab, 0xcd, 0xef, 0xfe,
                        0xdc, 0xba, 0x98, 0x76,
                        0x54, 0x32, 0x10, 0xfe),
                   vec!(0x1234_5678_abcd_effe as u64, 0xdcba_9876_5432_10fe as u64).as_bytes());
        assert_eq!(vec!(0x12, 0x34, 0x56, 0x78,
                        0xab, 0xcd, 0xef, 0xfe,
                        0xdc, 0xba, 0x98, 0x76,
                        0x54, 0x32, 0x10, 0xfe,
                        0x43, 0x21, 0x56, 0x78,
                        0xab, 0xcd, 0xef, 0xfe,
                        0xdc, 0xba, 0x98, 0x76,
                        0x54, 0x32, 0x10, 0xfe),
                   vec!(0x1234_5678_abcd_effe__dcba_9876_5432_10fe as u128,
                        0x4321_5678_abcd_effe__dcba_9876_5432_10fe as u128).as_bytes());
    }
}