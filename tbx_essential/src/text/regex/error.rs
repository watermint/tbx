use std::fmt;
use std::fmt::Formatter;
use regex::Error as RegexError;

pub struct ParseError {
    re_err: RegexError,
}

impl ParseError {
    pub fn from(re_err: RegexError) -> Self {
        Self {
            re_err,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // just delegate to [`Regex::Error`]
        self.re_err.fmt(f)
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // just delegate to [`Regex::Error`]
        self.re_err.fmt(f)
    }
}