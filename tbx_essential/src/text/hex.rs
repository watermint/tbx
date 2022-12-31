pub mod error;

use std::borrow::Cow;
use crate::number::byte::Bytes;
use crate::text::hex::error::ParseError;

const HEX_LOWER: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
const HEX_UPPER: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

/// Hex string converter.
pub trait Hex<T: ?Sized> {
    /// Convert to lower hex string like `01ab`.
    fn to_hex_lower<'a>(&self) -> Cow<'a, str>;

    /// Convert to upper hex string like `01AB`.
    fn to_hex_upper<'a>(&self) -> Cow<'a, str>;
}

fn parse_hex(c: char) -> Result<u8, ParseError> {
    match c.to_digit(16) {
        Some(n) => Ok(n as u8),
        None => Err(ParseError::InvalidChar)
    }
}

fn parse_hex_pair(c0: char, c1: char) -> Result<u8, ParseError> {
    let d0 = parse_hex(c0)?;
    let d1 = parse_hex(c1)?;
    Ok(d0 << 4 | d1)
}

#[cfg(test)]
mod test_parse_hex {
    use crate::text::hex::error::ParseError::InvalidChar;
    use crate::text::hex::parse_hex;

    #[test]
    fn test_parse_hex() {
        assert_eq!(Ok(0), parse_hex('0'));
        assert_eq!(Ok(1), parse_hex('1'));
        assert_eq!(Ok(2), parse_hex('2'));
        assert_eq!(Ok(3), parse_hex('3'));
        assert_eq!(Ok(4), parse_hex('4'));
        assert_eq!(Ok(5), parse_hex('5'));
        assert_eq!(Ok(6), parse_hex('6'));
        assert_eq!(Ok(7), parse_hex('7'));
        assert_eq!(Ok(8), parse_hex('8'));
        assert_eq!(Ok(9), parse_hex('9'));

        assert_eq!(Ok(10), parse_hex('a'));
        assert_eq!(Ok(11), parse_hex('b'));
        assert_eq!(Ok(12), parse_hex('c'));
        assert_eq!(Ok(13), parse_hex('d'));
        assert_eq!(Ok(14), parse_hex('e'));
        assert_eq!(Ok(15), parse_hex('f'));

        assert_eq!(Ok(10), parse_hex('A'));
        assert_eq!(Ok(11), parse_hex('B'));
        assert_eq!(Ok(12), parse_hex('C'));
        assert_eq!(Ok(13), parse_hex('D'));
        assert_eq!(Ok(14), parse_hex('E'));
        assert_eq!(Ok(15), parse_hex('F'));

        assert_eq!(Err(InvalidChar), parse_hex('g'));
        assert_eq!(Err(InvalidChar), parse_hex('h'));
    }
}

/// Parse Hex string
pub fn parse(text: &str) -> Result<Vec<u8>, ParseError> {
    let len = text.chars().count();
    if len & 0x1 == 1 {
        Err(ParseError::LackOfPair)
    } else {
        let chars: Vec<char> = text.chars().into_iter().collect();
        let chunks = chars.chunks(2);

        chunks.map(|pair| parse_hex_pair(pair[0], pair[1])).collect()
    }
}

#[cfg(test)]
mod test_parse {
    use crate::text::hex::error::ParseError::{InvalidChar, LackOfPair};
    use crate::text::hex::parse;

    #[test]
    fn test_parse() {
        assert_eq!(Ok(vec![0x12, 0x34, 0xab, 0xef]), parse("1234abef"));
        assert_eq!(Err(InvalidChar), parse("1234____"));
        assert_eq!(Err(LackOfPair), parse("123"));
    }
}

fn to_indices(x: u8) -> (usize, usize) {
    ((x >> 4) as usize, (x & 0xf) as usize)
}

fn to_hex_lower(x: u8) -> Vec<char> {
    let (h, l) = to_indices(x);
    vec!(HEX_LOWER[h], HEX_LOWER[l])
}

fn to_hex_upper(x: u8) -> Vec<char> {
    let (h, l) = to_indices(x);
    vec!(HEX_UPPER[h], HEX_UPPER[l])
}

impl Hex<u8> for u8 {
    fn to_hex_lower<'a>(&self) -> Cow<'a, str> {
        Cow::Owned(to_hex_lower(*self).iter().collect())
    }

    fn to_hex_upper<'a>(&self) -> Cow<'a, str> {
        Cow::Owned(to_hex_upper(*self).iter().collect())
    }
}

#[cfg(test)]
mod tests_u8 {
    use crate::text::hex::Hex;

    #[test]
    fn test_to_hex() {
        assert_eq!("01", (0x01 as u8).to_hex_lower());
        assert_eq!("32", (0x32 as u8).to_hex_lower());
        assert_eq!("45", (0x45 as u8).to_hex_lower());
        assert_eq!("67", (0x67 as u8).to_hex_lower());
        assert_eq!("98", (0x98 as u8).to_hex_lower());
        assert_eq!("ab", (0xab as u8).to_hex_lower());
        assert_eq!("cd", (0xcd as u8).to_hex_lower());
        assert_eq!("fe", (0xfe as u8).to_hex_lower());

        assert_eq!("01", (0x01 as u8).to_hex_upper());
        assert_eq!("32", (0x32 as u8).to_hex_upper());
        assert_eq!("45", (0x45 as u8).to_hex_upper());
        assert_eq!("67", (0x67 as u8).to_hex_upper());
        assert_eq!("98", (0x98 as u8).to_hex_upper());
        assert_eq!("AB", (0xab as u8).to_hex_upper());
        assert_eq!("CD", (0xcd as u8).to_hex_upper());
        assert_eq!("FE", (0xfe as u8).to_hex_upper());
    }
}

impl Hex<Vec<u8>> for Vec<u8> {
    fn to_hex_lower<'a>(&self) -> Cow<'a, str> {
        Cow::Owned(self.iter().map(|x| to_hex_lower(*x)).flatten().collect())
    }

    fn to_hex_upper<'a>(&self) -> Cow<'a, str> {
        Cow::Owned(self.iter().map(|x| to_hex_upper(*x)).flatten().collect())
    }
}

impl Hex<[u8]> for [u8] {
    fn to_hex_lower<'a>(&self) -> Cow<'a, str> {
        Cow::Owned(self.iter().map(|x| to_hex_lower(*x)).flatten().collect())
    }

    fn to_hex_upper<'a>(&self) -> Cow<'a, str> {
        Cow::Owned(self.iter().map(|x| to_hex_upper(*x)).flatten().collect())
    }
}

#[cfg(test)]
mod tests_vec_u8 {
    use crate::text::hex::Hex;

    #[test]
    fn test_to_hex() {
        assert_eq!("0123456789abcdef", vec!(0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef).to_hex_lower());
        assert_eq!("0123456789ABCDEF", vec!(0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef).to_hex_upper());

        assert_eq!("0123456789abcdef", [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef].to_hex_lower());
        assert_eq!("0123456789ABCDEF", [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef].to_hex_upper());

        let q = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        assert_eq!("01234567", q[0..=3].to_hex_lower());
        assert_eq!("89ABCDEF", q[4..=7].to_hex_upper());
    }
}

impl Hex<u16> for u16 {
    fn to_hex_lower<'a>(&self) -> Cow<'a, str> {
        self.as_bytes().to_hex_lower()
    }

    fn to_hex_upper<'a>(&self) -> Cow<'a, str> {
        self.as_bytes().to_hex_upper()
    }
}

#[cfg(test)]
mod tests_u16 {
    use crate::text::hex::Hex;

    #[test]
    fn test_to_hex() {
        assert_eq!("0123", (0x0123 as u16).to_hex_lower());
        assert_eq!("0123", (0x0123 as u16).to_hex_upper());

        assert_eq!("abcd", (0xabcd as u16).to_hex_lower());
        assert_eq!("ABCD", (0xABCD as u16).to_hex_upper());
    }
}

impl Hex<u32> for u32 {
    fn to_hex_lower<'a>(&self) -> Cow<'a, str> {
        self.as_bytes().to_hex_lower()
    }

    fn to_hex_upper<'a>(&self) -> Cow<'a, str> {
        self.as_bytes().to_hex_upper()
    }
}


#[cfg(test)]
mod tests_u32 {
    use crate::text::hex::Hex;

    #[test]
    fn test_to_hex() {
        assert_eq!("01234567", (0x01234567 as u32).to_hex_lower());
        assert_eq!("01234567", (0x01234567 as u32).to_hex_upper());

        assert_eq!("ab98cd01", (0xab98cd01 as u32).to_hex_lower());
        assert_eq!("AB98CD01", (0xAB98CD01 as u32).to_hex_upper());
    }
}


impl Hex<u64> for u64 {
    fn to_hex_lower<'a>(&self) -> Cow<'a, str> {
        self.as_bytes().to_hex_lower()
    }

    fn to_hex_upper<'a>(&self) -> Cow<'a, str> {
        self.as_bytes().to_hex_upper()
    }
}

#[cfg(test)]
mod tests_u64 {
    use crate::text::hex::Hex;

    #[test]
    fn test_to_hex() {
        assert_eq!("01234567ab98cd01", (0x01234567_ab98cd01 as u64).to_hex_lower());
        assert_eq!("01234567AB98CD01", (0x01234567_ab98cd01 as u64).to_hex_upper());
    }
}


impl Hex<u128> for u128 {
    fn to_hex_lower<'a>(&self) -> Cow<'a, str> {
        self.as_bytes().to_hex_lower()
    }

    fn to_hex_upper<'a>(&self) -> Cow<'a, str> {
        self.as_bytes().to_hex_upper()
    }
}

#[cfg(test)]
mod tests_u128 {
    use crate::text::hex::Hex;

    #[test]
    fn test_to_hex() {
        assert_eq!("01234567ab98cd0123456789ab01cd23", (0x01234567_ab98cd01_23456789_ab01cd23 as u128).to_hex_lower());
        assert_eq!("01234567AB98CD0123456789AB01CD23", (0x01234567_ab98cd01_23456789_ab01cd23 as u128).to_hex_upper());
    }
}


