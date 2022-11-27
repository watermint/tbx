/// Essential functions for str.
trait StringEssential {
    /// Returns substring of this string.
    fn substring(&self, start: usize, finish: usize) -> Option<&str>;

    /// Returns substring of this string to the end.
    fn substring_to_end(&self, start: usize) -> Option<&str>;
}

impl StringEssential for str {
    fn substring(&self, start: usize, finish: usize) -> Option<&str> {
        if finish <= start {
            None
        } else {
            let s = self.chars().take(start).map(|c| c.len_utf8()).sum();
            let f = self.chars().take(finish).map(|c| c.len_utf8()).sum();

            if f <= s || self.chars().count() < finish {
                None
            } else {
                self.get(s..f)
            }
        }
    }

    fn substring_to_end(&self, start: usize) -> Option<&str> {
        if self.chars().count() <= start {
            None
        } else {
            let s = self.chars().take(start).map(|c| c.len_utf8()).sum();
            self.get(s..)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::text::string::StringEssential;

    #[test]
    fn test_substring() {
        assert_eq!("HelloWorld", "HelloWorld".substring(0, 10).unwrap());
        assert_eq!("World", "HelloWorld".substring(5, 10).unwrap());
        assert_eq!("世界", "こんにちは世界".substring(5, 7).unwrap()); // Non ascii
        assert_eq!("🍣", "今日は🍣と🍜".substring(3, 4).unwrap()); // Non plane 0 chars
        assert_eq!(None, "HelloWorld".substring(10, 5));
        assert_eq!(None, "HelloWorld".substring(10, 10));
        assert_eq!(None, "HelloWorld".substring(11, 15));
        assert_eq!(None, "HelloWorld".substring(0, 0));
    }

    #[test]
    fn test_substring_to_end() {
        assert_eq!("HelloWorld", "HelloWorld".substring_to_end(0).unwrap());
        assert_eq!("World", "HelloWorld".substring_to_end(5).unwrap());
        assert_eq!("世界", "こんにちは世界".substring_to_end(5).unwrap()); // Non ascii
        assert_eq!("🍣と🍜", "今日は🍣と🍜".substring_to_end(3).unwrap()); // Non plane 0 chars
        assert_eq!(None, "HelloWorld".substring_to_end(10));
        assert_eq!(None, "HelloWorld".substring_to_end(11));
    }
}