use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ParseInvalidChar {
    invalid: char,
}

impl ParseInvalidChar {
    pub fn from(invalid_char: char) -> ParseInvalidChar {
        ParseInvalidChar {
            invalid: invalid_char,
        }
    }
}

#[derive(Debug)]
pub struct ParseNonAsciiAlphaNumString<'a> {
    pattern: &'a str,
}

impl<'a> ParseNonAsciiAlphaNumString<'a> {
    pub fn from(pattern: &'a str) -> ParseNonAsciiAlphaNumString<'a> {
        ParseNonAsciiAlphaNumString {
            pattern,
        }
    }
}

#[derive(Debug)]
pub enum ParseInvalidPart {
    Major,
    Minor,
    Patch,
    VersionNumber,
    PreRelease,
    PrereleaseOrBuild,
    Build,
    NumericIdentifier,
    AlphaNumericIdentifier,
    Other,
}

#[derive(Debug)]
pub enum ParseErrorReason<'a> {
    InvalidChar(ParseInvalidChar),
    InvalidPattern,
    NonAsciiAlphaNumString(ParseNonAsciiAlphaNumString<'a>),
    NumberIdentifierShouldNotHaveLeadingZero,
}

impl<'a> Display for ParseErrorReason<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorReason::InvalidChar(c) =>
                write!(f, "invalid character '{}' found", c.invalid),
            ParseErrorReason::InvalidPattern =>
                write!(f, "invalid pattern"),
            ParseErrorReason::NonAsciiAlphaNumString(n) =>
                write!(f, "non ASCII alpha-numeric character '{}' found", n.pattern),
            ParseErrorReason::NumberIdentifierShouldNotHaveLeadingZero =>
                write!(f, "number identifier should not have leading zero"),
        }
    }
}

#[derive(Debug)]
pub struct ParseError<'a> {
    part: ParseInvalidPart,
    reason: ParseErrorReason<'a>,
}

impl<'a> ParseError<'a> {
    pub fn from(part: ParseInvalidPart, reason: ParseErrorReason<'a>) -> ParseError<'a> {
        ParseError {
            part,
            reason,
        }
    }
}

impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.part {
            ParseInvalidPart::Other => write!(f, "{}", self.reason),
            _ => write!(f, "{} in part {:?}", self.reason, self.part),
        }
    }
}

#[cfg(test)]
mod errors {
    use crate::text::version::semantic::error::{ParseError, ParseErrorReason, ParseInvalidChar, ParseInvalidPart};

    #[test]
    fn test_display_parse_error() {
        assert_eq!("invalid character '*' found in part PreRelease",
                   format!("{}",
                           ParseError::from(ParseInvalidPart::PreRelease,
                                            ParseErrorReason::InvalidChar(ParseInvalidChar::from('*')))));
        assert_eq!("invalid character '*' found",
                   format!("{}",
                           ParseError::from(ParseInvalidPart::Other,
                                            ParseErrorReason::InvalidChar(ParseInvalidChar::from('*')))));
    }
}