use std::borrow::Cow;

use crate::number::random::{Generator, Random};

/// Generate random string of length, by using given chars
pub fn next<'a>(length: usize, chars: Vec<char>) -> Cow<'a, str> {
    let num_chars = chars.len();
    let mut r = Random::new_thread_local();
    let alt_char = '\0';
    assert!(0 < num_chars, "num_chars {}", num_chars);
    assert!(0 < length, "length {}", length);

    Cow::Owned((0..length).map(|_i| *chars.get(r.next_range_usize(0..num_chars)).unwrap_or(&alt_char)).collect())
}

/// Generate random ASCII numeric string of length.
pub fn next_numeric<'a>(length: usize) -> Cow<'a, str> {
    next(length, vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
}

/// Generate random ASCII hex-numeric string (upper case) of length.
pub fn next_hex_upper<'a>(length: usize) -> Cow<'a, str> {
    next(length, vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'])
}

/// Generate random ASCII hex-numeric string (lower case) of length.
pub fn next_hex_lower<'a>(length: usize) -> Cow<'a, str> {
    next(length, vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'])
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::ops::RangeInclusive;

    use crate::text::essential::StringEssential;
    use crate::text::random::ascii::{next, next_hex_lower, next_hex_upper, next_numeric};

    /// Make sure range of number appear at least once in given `q`.
    fn verify_numeric<'a>(range: &RangeInclusive<usize>, q: &Cow<str>, expected_length: usize, formatter: fn(i: usize) -> Cow<'a, str>) -> bool {
        let mut sum_length: usize = 0;
        for i in range.to_owned() {
            let j = formatter(i);
            let c = q.count_char(j.chars().nth(0).unwrap_or('\0'));
            if c < 1 {
                return false;
            }
            sum_length += c;
        }
        sum_length == expected_length
    }

    #[test]
    fn test_next() {
        for i in 1..100 {
            assert_eq!(next(i, vec!['0']), "0".repeat(i).as_str())
        }
        let verify_abc = |q: &Cow<str>, len: usize| {
            let num_a = q.count_char('A');
            let num_b = q.count_char('B');
            let num_c = q.count_char('C');
            assert_eq!(num_a + num_b + num_c, len, "testing string[{}]", q);
        };

        for i in 1..100 {
            let mut last = next(i, vec!['A', 'B', 'C']);
            let mut q = next(i, vec!['A', 'B', 'C']);
            verify_abc(&q, i);

            while last == q {
                last = q;
                q = next(i, vec!['A', 'B', 'C']);
                verify_abc(&q, i);
            }
        }
    }

    fn verify_next_numeric<'a>(range: RangeInclusive<usize>,
                               generator: fn(c: usize) -> Cow<'a, str>,
                               formatter: fn(i: usize) -> Cow<'a, str>) {
        let nn = || -> Cow<str> {
            let expected_length = 1000;
            let abort_limit = expected_length * 100;
            let mut abort_count = 0;
            let mut q = generator(expected_length);
            while !verify_numeric(&range, &q, expected_length, formatter) {
                q = generator(expected_length);
                abort_count += 1;
                assert!(abort_count < abort_limit, "limit {}, count {}", abort_limit, abort_count);
                if abort_limit < abort_count {
                    return Cow::Owned("".to_string());
                }
            }
            q
        };
        let mut last = nn();
        let mut q = nn();

        while last == q {
            last = q;
            q = nn();
        }
    }

    #[test]
    fn test_next_numeric() {
        verify_next_numeric(0..=9 as usize, next_numeric, |i: usize| -> Cow<str> {
            Cow::Owned(format!("{}", i))
        });
    }

    #[test]
    fn test_next_hex_upper() {
        verify_next_numeric(0..=15 as usize, next_hex_upper, |i: usize| -> Cow<str> {
            Cow::Owned(format!("{:X}", i))
        });
    }

    #[test]
    fn test_next_hex_lower() {
        verify_next_numeric(0..=15 as usize, next_hex_lower, |i: usize| -> Cow<str> {
            Cow::Owned(format!("{:x}", i))
        });
    }
}