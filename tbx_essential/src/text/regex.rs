pub mod error;
pub mod matcher;
pub mod splitter;

use std::borrow::Cow;
use regex::{NoExpand as RegexNoExpand, Regex as RegexRegex};
use crate::text::regex::error::ParseError;
use crate::text::regex::matcher::{CaptureMatches, Captures, Match, Matches};
use crate::text::regex::splitter::Split;

pub trait Matcher {
    /// Returns true when given text matches the regular expression.
    fn is_match(&self, text: &str) -> bool;

    /// Returns true when given text matches the regular expression, search start
    /// from the byte index `offset`.
    fn is_match_at(&self, text: &str, offset: usize) -> bool;

    /// Returns the start and end byte range of the leftmost-first match in text.
    /// If no match exists, then None is returned.
    fn find_first<'t>(&self, text: &'t str) -> Option<Match<'t>>;

    /// Same as [`find`] but start the search at the given offset.
    fn find_at<'t>(&self, text: &'t str, offset: usize) -> Option<Match<'t>>;

    /// Returns an iterator for each successive non-overlapping match in text,
    /// returning the start and end byte indices with respect to `text`.
    fn find_iter<'r, 't>(&'r self, text: &'t str) -> Matches<'r, 't>;

    /// Returns the capture groups corresponding to the leftmost-first match in text.
    /// Capture group 0 always corresponds to the entire match.
    /// If no match is found, then None is returned.
    fn capture_first<'t>(&self, text: &'t str) -> Option<Captures<'t>>;

    /// Returns an iterator over all the non-overlapping capture groups matched in text.
    /// This is operationally the same as find_iter,
    /// except it yields information about capturing group matches.
    fn capture_iter<'r, 't>(&'r self, text: &'t str) -> CaptureMatches<'r, 't>;
}

pub trait Replacer {
    /// Replaces the leftmost-first match with the replacement provided.
    fn replace_first<'t>(&self, text: &'t str, replace: &str) -> Cow<'t, str>;

    /// Same as [`replace_first`] but will not expanding $name to their corresponding capture.
    fn replace_first_noexpansion<'t>(&self, text: &'t str, replace: &str) -> Cow<'t, str>;

    /// Replaces all non-overlapping matches in text with the replacement provided.
    fn replace_all<'t>(&self, text: &'t str, replace: &str) -> Cow<'t, str>;

    /// Same as ['replace_all`] but will not expanding $name to their corresponding capture.
    fn replace_all_noexpansion<'t>(&self, text: &'t str, replace: &str) -> Cow<'t, str>;
}

pub trait Splitter {
    /// Returns an iterator of substrings of text delimited by a match of the regular expression.
    /// Namely, each element of the iterator corresponds to text that isn’t matched by the regular expression.
    fn split<'r, 't>(&'r self, text: &'t str) -> Split<'r, 't>;
}

/// Regular expression matching for Unicode string.
///
/// This library is the wrapper of [`regex::Regex`] with slightly different interfaces.
///
/// This implementation limits some features of [`regex::Regex`] for some reasons.
/// The goal of this wrapper is to reduce supply chain problems in the long term and
/// to maintain utmost compatibility in the short to medium term.
/// If this struct implements original Regex library interfaces as it is,
/// it will be difficult to maintain compatibility.
pub struct Regex {
    re: RegexRegex,
}

impl Regex {
    /// Parse regular expression and returns Regex instance.
    pub fn parse(re: &str) -> Result<Self, ParseError> {
        match RegexRegex::new(re) {
            Ok(rr) => Ok(Self {
                re: rr,
            }),
            Err(err) => Err(ParseError::from(err))
        }
    }
}

impl Matcher for Regex {
    fn is_match(&self, text: &str) -> bool {
        self.re.is_match(text)
    }

    fn is_match_at(&self, text: &str, offset: usize) -> bool {
        self.re.is_match_at(text, offset)
    }

    fn find_first<'t>(&self, text: &'t str) -> Option<Match<'t>> {
        match self.re.find(text) {
            Some(m) => Some(Match::new(m)),
            _ => None,
        }
    }

    fn find_at<'t>(&self, text: &'t str, offset: usize) -> Option<Match<'t>> {
        match self.re.find_at(text, offset) {
            Some(m) => Some(Match::new(m)),
            _ => None,
        }
    }

    fn find_iter<'r, 't>(&'r self, text: &'t str) -> Matches<'r, 't> {
        Matches::new(self.re.find_iter(text))
    }

    fn capture_first<'t>(&self, text: &'t str) -> Option<Captures<'t>> {
        match self.re.captures(text) {
            Some(c) => Some(Captures::new(c)),
            _ => None
        }
    }

    fn capture_iter<'r, 't>(&'r self, text: &'t str) -> CaptureMatches<'r, 't> {
        CaptureMatches::new(self.re.captures_iter(text))
    }
}

#[cfg(test)]
mod tests_matcher {
    use crate::text::regex::{Matcher, Regex};
    use crate::text::regex::matcher::CaptureIndexer;

    #[test]
    fn test_is_match() {
        let re = Regex::parse(r"\d{4}").unwrap();

        assert!(re.is_match("1234"));
        assert!(!re.is_match("789"));
        assert!(re.is_match_at("01234", 1));
        assert!(!re.is_match_at("01234", 2));
    }

    #[test]
    fn test_find() {
        let re = Regex::parse(r"[A-Z][a-z]{3}").unwrap();

        assert_eq!(re.find_first("Rust").unwrap().as_str(), "Rust");
        assert_eq!(re.find_first("Rust").unwrap().range(), 0..4 as usize);

        assert!(re.find_first("RUST").is_none());
        assert!(re.find_at("Rust", 1).is_none());
    }

    #[test]
    fn test_find_iter() {
        let re = Regex::parse(r"\d{4}").unwrap();

        assert_eq!(re.find_iter("2022-2023-2024").count(), 3);
        assert_eq!(re.find_iter("2022-2023-2024").nth(1).unwrap().as_str(), "2023");
    }

    #[test]
    fn test_captures() {
        let re = Regex::parse(r"(\d{4})-(\d{2})-(\d{2})").unwrap();

        let da = re.capture_first("Departure: 2022-12-27, Arrival: 2022-12-28").unwrap();
        assert_eq!(da.get(0).unwrap().as_str(), "2022-12-27");
        assert_eq!(da.get(1).unwrap().as_str(), "2022");
        assert_eq!(da.get(2).unwrap().as_str(), "12");
        assert_eq!(da.get(3).unwrap().as_str(), "27");
    }

    #[test]
    fn test_captures_name() {
        let re = Regex::parse(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();

        let da = re.capture_first("Departure: 2022-12-27, Arrival: 2022-12-28").unwrap();
        assert_eq!(da.get(0).unwrap().as_str(), "2022-12-27");
        assert_eq!(da.get("year").unwrap().as_str(), "2022");
        assert_eq!(da.get("month").unwrap().as_str(), "12");
        assert_eq!(da.get("day").unwrap().as_str(), "27");
    }

    #[test]
    fn test_captures_iter_name() {
        let re = Regex::parse(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();

        let mut da = re.capture_iter("Departure: 2022-12-27, Arrival: 2022-12-28");
        let d0 = da.next().unwrap();
        let d1 = da.next().unwrap();

        assert_eq!(d0.get(0).unwrap().as_str(), "2022-12-27");
        assert_eq!(d0.get("year").unwrap().as_str(), "2022");
        assert_eq!(d0.get("month").unwrap().as_str(), "12");
        assert_eq!(d0.get("day").unwrap().as_str(), "27");

        assert_eq!(d1.get(0).unwrap().as_str(), "2022-12-28");
        assert_eq!(d1.get("year").unwrap().as_str(), "2022");
        assert_eq!(d1.get("month").unwrap().as_str(), "12");
        assert_eq!(d1.get("day").unwrap().as_str(), "28");
    }
}

impl Replacer for Regex {
    fn replace_first<'t>(&self, text: &'t str, replace: &str) -> Cow<'t, str> {
        self.re.replace(text, replace)
    }

    fn replace_first_noexpansion<'t>(&self, text: &'t str, replace: &str) -> Cow<'t, str> {
        self.re.replace(text, RegexNoExpand(replace))
    }

    fn replace_all<'t>(&self, text: &'t str, replace: &str) -> Cow<'t, str> {
        self.re.replace_all(text, replace)
    }

    fn replace_all_noexpansion<'t>(&self, text: &'t str, replace: &str) -> Cow<'t, str> {
        self.re.replace_all(text, RegexNoExpand(replace))
    }
}

#[cfg(test)]
mod tests_replacer {
    use crate::text::regex::{Regex, Replacer};

    #[test]
    fn test_replace_first() {
        let re = Regex::parse(r"\d+").unwrap();

        assert_eq!(re.replace_first("ABC_123_DEF_789", "QQQ"), "ABC_QQQ_DEF_789");
        assert_eq!(re.replace_first_noexpansion("ABC_123_DEF_789", "QQQ"), "ABC_QQQ_DEF_789");

        let re2 = Regex::parse(r"(?P<Num>\d+)").unwrap();

        assert_eq!(re2.replace_first("ABC_123_DEF_789", "[$Num]"), "ABC_[123]_DEF_789");
        assert_eq!(re2.replace_first_noexpansion("ABC_123_DEF_789", "[$Num]"), "ABC_[$Num]_DEF_789");
    }

    #[test]
    fn test_replace_all() {
        let re = Regex::parse(r"\d+").unwrap();

        assert_eq!(re.replace_all("ABC_123_DEF_789", "QQQ"), "ABC_QQQ_DEF_QQQ");
        assert_eq!(re.replace_all_noexpansion("ABC_123_DEF_789", "QQQ"), "ABC_QQQ_DEF_QQQ");

        let re2 = Regex::parse(r"(?P<Num>\d+)").unwrap();

        assert_eq!(re2.replace_all("ABC_123_DEF_789", "[$Num]"), "ABC_[123]_DEF_[789]");
        assert_eq!(re2.replace_all_noexpansion("ABC_123_DEF_789", "[$Num]"), "ABC_[$Num]_DEF_[$Num]");
    }
}