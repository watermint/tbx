use crate::text::essential::StringEssential;

pub trait AlphaNumTokenizer {
    // Split into alpha-numeric tokens.
    // This tokenizer ignores characters except ASCII alpha-numeric.
    // This tokenizer splits token on case change.
    // Token must match regex `[A-Z]*[a-z]*[0-9]*` and is not an empty string.
    // For example,
    // "Powered by RustLang version1.65.0." is tokenized to "Powered", "by", "Rust", "Lang", "version1", "65", and "0".
    fn tokenize_ascii_alpha_num(&self) -> Vec<&str>;
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

impl AlphaNumTokenizer for str {
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
}

#[cfg(test)]
mod tests {
    use crate::text::token::alpha_num::AlphaNumTokenizer;

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
}