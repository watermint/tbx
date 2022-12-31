use std::fmt;
use std::fmt::Formatter;
use std::ops::Range;
use regex::Captures as RegexCaptures;
use regex::CaptureMatches as RegexCaptureMatches;
use regex::Match as RegexMatch;
use regex::Matches as RegexMatches;

/// A single match of a regex. This implementation is the wrapper of [`regex::Match`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Match<'a> {
    m: RegexMatch<'a>,
}

impl<'a> Match<'a> {
    /// Create new instance from the [`regex::Match`] instance.
    pub fn new(m: RegexMatch<'a>) -> Self { Self { m } }

    /// Returns the starting byte offset of the match in the haystack.
    pub fn start(&self) -> usize {
        self.m.start()
    }

    /// Returns the ending byte offset of the match in the haystack.
    pub fn end(&self) -> usize {
        self.m.end()
    }

    /// Returns the range over the starting and ending byte offsets of the match in the haystack.
    pub fn range(&self) -> Range<usize> {
        self.m.range()
    }

    /// Returns the matched text.
    pub fn as_str(&self) -> &'a str {
        self.m.as_str()
    }
}

impl<'a> From<Match<'a>> for &'a str {
    fn from(m: Match<'a>) -> Self {
        m.as_str()
    }
}

impl<'a> From<Match<'a>> for Range<usize> {
    fn from(m: Match<'a>) -> Self {
        m.range()
    }
}

/// An iterator over all non-overlapping matches for a particular string.
/// This implementation is the wrapper of [`regex::Matches`].
#[derive(Debug)]
pub struct Matches<'r, 't> {
    m: RegexMatches<'r, 't>,
}

impl<'r, 't> Matches<'r, 't> {
    pub fn new(m: RegexMatches<'r, 't>) -> Self { Self { m } }
}

impl<'r, 't> Iterator for Matches<'r, 't> {
    type Item = Match<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.m.next() {
            Some(m) => Some(Match::new(m)),
            _ => None,
        }
    }
}

pub trait CaptureIndexer<'t, T> {
    /// Returns the match associated with the capture group at index `i`.
    fn get(&self, index: T) -> Option<Match<'t>>;
}

/// Capture represents a group of captured strings for a single match.
/// This implementation is the wrapper of [`regex::Captures`]
pub struct Captures<'t> {
    c: RegexCaptures<'t>,
}

impl<'t> Captures<'t> {
    pub fn new(c: RegexCaptures<'t>) -> Self { Self { c } }
}

impl<'t> CaptureIndexer<'t, usize> for Captures<'t> {
    fn get(&self, i: usize) -> Option<Match<'t>> {
        match self.c.get(i) {
            Some(m) => Some(Match::new(m)),
            _ => None,
        }
    }
}

impl<'t> CaptureIndexer<'t, &str> for Captures<'t> {
    fn get(&self, index: &str) -> Option<Match<'t>> {
        match self.c.name(index) {
            Some(m) => Some(Match::new(m)),
            _ => None,
        }
    }
}

impl<'t> CaptureIndexer<'t, String> for Captures<'t> {
    fn get(&self, index: String) -> Option<Match<'t>> {
        match self.c.name(index.as_str()) {
            Some(m) => Some(Match::new(m)),
            _ => None,
        }
    }
}

impl<'t> fmt::Debug for Captures<'t> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.c.fmt(f)
    }
}

pub struct CaptureMatches<'r, 't> {
    cm: RegexCaptureMatches<'r, 't>,
}

impl<'r, 't> CaptureMatches<'r, 't> {
    pub fn new(cm: RegexCaptureMatches<'r, 't>) -> Self { Self { cm } }
}

impl<'r, 't> Iterator for CaptureMatches<'r, 't> {
    type Item = Captures<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cm.next() {
            Some(c) => Some(Captures::new(c)),
            _ => None
        }
    }
}
