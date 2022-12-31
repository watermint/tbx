use std::borrow::Cow;

use crate::text::essential::StringEssential;

pub trait AsciiTokenizer {
    /// Split into alpha-numeric tokens.
    /// This tokenizer ignores characters except ASCII alpha-numeric.
    /// This tokenizer splits token on case change.
    /// Token must match regex `[A-Z]*[a-z]*[0-9]*` and is not an empty string.
    /// For example,
    /// `"Powered by RustLang version1.65.0."` is tokenized to
    /// `["Powered"`, `"by"`, `"Rust"`, `"Lang"`, `"version1"`, `"65"`, `"0"]`.
    fn tokenize_ascii_alpha_num(&self) -> Vec<&str>;

    /// Split into alpha-numeric tokens, then change all cases to capital.
    /// Other behavior is same as [`Self::tokenize_ascii_alpha_num`]
    /// Example: `"Powered by RustLang"` -> `["POWERED", "BY", "RUST", "LANG"]`
    fn tokenize_ascii_alpha_num_to_capital<'a>(&self) -> Vec<Cow<'a, str>>;

    /// Split into alpha-numeric tokens, then change all cases to upper case for the first char, then
    /// rest of characters to lower case.
    /// Other behavior is same as [`Self::tokenize_ascii_alpha_num`]
    /// Example: `"Powered by RustLang"` -> `["Powered", "By", "Rust", "Lang"]`
    fn tokenize_ascii_alpha_num_to_first_upper<'a>(&self) -> Vec<Cow<'a, str>>;

    /// Split into alpha-numeric tokens, then change all cases to lower case.
    /// Other behavior is same as [`Self::tokenize_ascii_alpha_num`]
    /// Example: `"Powered by RustLang"` -> `["powered", "by", "rust", "lang"]`
    fn tokenize_ascii_alpha_num_to_lower<'a>(&self) -> Vec<Cow<'a, str>>;
}

pub trait AsciiMatcher {
    /// Returns true when the string is ASCII numeric string.
    fn is_ascii_numeric(&self) -> bool;

    /// Returns true when the string is ASCII alphabetic string.
    fn is_ascii_alphabetic(&self) -> bool;

    /// Returns true when the string is ASCII alpha-numeric string.
    fn is_ascii_alphanumeric(&self) -> bool;
}

fn next_alpha_num_token(s: &str) -> Option<(usize, usize, &str)> {
    match s.chars().position(|c| c.is_ascii_alphanumeric()) {
        None => None,
        Some(start) => {
            match s.substring_to_end(start) {
                Some(reminder) => {
                    let reminder_with_guard = reminder.to_string() + " ";
                    let upper = reminder_with_guard.chars().position(|c| !(c.is_ascii_uppercase())).unwrap_or(0);
                    let lower = reminder_with_guard.chars().skip(upper).position(|c| !(c.is_ascii_lowercase())).unwrap_or(0);
                    let num = reminder_with_guard.chars().skip(upper + lower).position(|c| !(c.is_ascii_digit())).unwrap_or(0);

                    match reminder.substring(0, upper + lower + num) {
                        Some(token) => Some((start, start + upper + lower + num, token)),
                        _ => Some((start, start + 1, s.substring(start, start + 1).unwrap_or("")))
                    }
                }
                _ => None,
            }
        }
    }
}

impl AsciiTokenizer for str {
    fn tokenize_ascii_alpha_num(&self) -> Vec<&str> {
        let mut tokens: Vec<&str> = Vec::new();
        let mut offset: usize = 0;

        while let Some(reminder) = self.substring_to_end(offset) {
            match next_alpha_num_token(reminder) {
                Some((_s, f, token)) => {
                    tokens.push(token);
                    offset += f;
                }
                _ => break
            }
        }

        tokens
    }

    fn tokenize_ascii_alpha_num_to_capital<'a>(&self) -> Vec<Cow<'a, str>> {
        self.tokenize_ascii_alpha_num().iter().map(|token| {
            Cow::Owned(token.to_string().to_uppercase())
        }).collect()
    }

    fn tokenize_ascii_alpha_num_to_first_upper<'a>(&self) -> Vec<Cow<'a, str>> {
        self.tokenize_ascii_alpha_num().iter().map(|token| {
            match (token.substring(0, 1), token.substring_to_end(1)) {
                (Some(h), None) => Cow::Owned(h.to_uppercase().to_string()),
                (Some(h), Some(r)) => Cow::Owned(h.to_uppercase() + &r.to_lowercase()),
                _ => Cow::Owned(String::from("")),
            }
        }).collect()
    }

    fn tokenize_ascii_alpha_num_to_lower<'a>(&self) -> Vec<Cow<'a, str>> {
        self.tokenize_ascii_alpha_num().iter().map(|token| {
            Cow::Owned(token.to_string().to_lowercase())
        }).collect()
    }
}

impl AsciiMatcher for str {
    fn is_ascii_numeric(&self) -> bool {
        self.chars().all(|c| c.is_ascii_digit())
    }

    fn is_ascii_alphabetic(&self) -> bool {
        self.chars().all(|c| c.is_ascii_alphabetic())
    }

    fn is_ascii_alphanumeric(&self) -> bool {
        self.chars().all(|c| c.is_ascii_alphanumeric())
    }
}

#[cfg(test)]
mod tests {
    use crate::text::token::ascii::{AsciiMatcher, AsciiTokenizer};

    #[test]
    fn test_tokenize_alpha_num() {
        assert_eq!(vec!["Powered", "by", "Rust", "Lang", "version1", "65", "0"],
                   "  Powered by RustLang version1.65.0".tokenize_ascii_alpha_num());
        assert_eq!(vec!["X", "XX", "XXx", "XXxx", "X1", "XX1", "Xx1", "Xxx1"],
                   "  X XX XXx XXxx X1 XX1 Xx1 Xxx1".tokenize_ascii_alpha_num());
        assert_eq!(vec!["X", "XX", "XXx", "XXxx", "X1", "XX1", "Xx1", "XXX"],
                   "  X XX XXx XXxx X1 XX1 Xx1 XXX".tokenize_ascii_alpha_num());
        assert_eq!(vec!["x", "xx", "xx1", "xx11"],
                   "  x xx xx1 xx11".tokenize_ascii_alpha_num());
        assert_eq!(vec!["1", "12", "123"],
                   "  1 12 123".tokenize_ascii_alpha_num());
        assert_eq!(vec!["Ver1", "b"],
                   "Ver1b".tokenize_ascii_alpha_num());
        assert_eq!(vec!["RAMEN123", "123", "RAMEN"],
                   " RAMEN123 123RAMEN".tokenize_ascii_alpha_num());

        // full-width alpha-num will be eliminated
        assert_eq!(vec!["Somen"],
                   "  Somen ＲＡＭＥＮ１２３　１２３ＵＤＯＮ".tokenize_ascii_alpha_num());
        assert_eq!(vec!["789"],
                   "  789 １   １２　１２３".tokenize_ascii_alpha_num());
    }

    #[test]
    fn test_tokenize_ascii_alpha_num_to_capital() {
        assert_eq!(vec!["POWERED", "BY", "RUST", "LANG", "VERSION1", "65", "0"],
                   "  Powered by RustLang version1.65.0".tokenize_ascii_alpha_num_to_capital());
    }

    #[test]
    fn test_tokenize_ascii_alpha_num_to_first_upper() {
        assert_eq!(vec!["Powered", "By", "Rust", "Lang", "Version1", "65", "0"],
                   "  Powered by RustLang version1.65.0".tokenize_ascii_alpha_num_to_first_upper());
    }

    #[test]
    fn test_tokenize_ascii_alpha_num_to_lower() {
        assert_eq!(vec!["powered", "by", "rust", "lang", "version1", "65", "0"],
                   "  Powered by RustLang version1.65.0".tokenize_ascii_alpha_num_to_lower());
    }

    #[test]
    fn test_is_ascii_numeric() {
        assert!("1234".is_ascii_numeric());
        assert!("0".is_ascii_numeric());
        assert!(!"abc".is_ascii_numeric());
        assert!(!"abc123".is_ascii_numeric());
        assert!(!"１２３".is_ascii_numeric());
    }

    #[test]
    fn test_is_ascii_alphabetic() {
        assert!("abc".is_ascii_alphabetic());
        assert!("ABC".is_ascii_alphabetic());
        assert!("Abc".is_ascii_alphabetic());
        assert!("aBC".is_ascii_alphabetic());
        assert!("a".is_ascii_alphabetic());
        assert!(!"a123".is_ascii_alphabetic());
        assert!(!"123".is_ascii_alphabetic());
    }

    #[test]
    fn test_is_ascii_alphanumeric() {
        assert!("abc".is_ascii_alphanumeric());
        assert!("ABC".is_ascii_alphanumeric());
        assert!("Abc".is_ascii_alphanumeric());
        assert!("aBC".is_ascii_alphanumeric());
        assert!("a".is_ascii_alphanumeric());
        assert!("a123".is_ascii_alphanumeric());
        assert!("123".is_ascii_alphanumeric());
        assert!(!"１２３".is_ascii_alphanumeric());
        assert!(!"エービーシー".is_ascii_alphanumeric());
    }
}