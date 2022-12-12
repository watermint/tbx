use std::fmt;
use std::fmt::Formatter;

use crate::text::token::ascii::AsciiMatcher;
use crate::text::version::semantic::error::{ParseError, ParseErrorReason, ParseInvalidPart};
use crate::text::version::semantic::parse;

/// Build metadata.
/// Examples: `1.0.0-alpha+001`, `1.0.0+20130313144700`, `1.0.0-beta+exp.sha.5114f85`, `1.0.0+21AF26D3-117B344092BD`.
#[derive(Debug)]
pub struct Build<'a> {
// ```
// <build> ::= <dot-separated build identifiers>
//
// <dot-separated build identifiers> ::= <build identifier>
//                                     | <build identifier> "." <dot-separated build identifiers>
// <build identifier> ::= <alphanumeric identifier>
//                       | <digits>
// ```

    build: Vec<&'a str>,
}

impl<'a> Build<'a> {
    /// Parse build part.
    pub fn parse(build: &str, strict: bool) -> Result<Build, ParseError> {
        let b = Self::parse_build(build, strict)?;
        Ok(Build {
            build: b,
        })
    }

    fn parse_build_identifier(build: &str, strict: bool) -> Result<&str, ParseError> {
        if let Ok(id) = parse::parse_alphanumeric_identifier(build, strict) {
            Ok(id)
        } else if build.is_ascii_numeric() {
            Ok(build)
        } else {
            Err(ParseError::from(ParseInvalidPart::Build, ParseErrorReason::InvalidPattern))
        }
    }

    fn parse_build(build: &str, strict: bool) -> Result<Vec<&str>, ParseError> {
        // <build> ::= <dot-separated build identifiers>
        //
        // <dot-separated build identifiers> ::= <build identifier>
        //                                     | <build identifier> "." <dot-separated build identifiers>
        // <build identifier> ::= <alphanumeric identifier>
        //                      | <digits>

        build.split(".").map(|p| Self::parse_build_identifier(p, strict)).into_iter().collect()
    }
}

impl<'a> fmt::Display for Build<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build.join("."))
    }
}

impl<'a> Eq for Build<'a> {}

impl<'a> PartialEq<Self> for Build<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.build == other.build
    }
}

#[cfg(test)]
mod build {
    use crate::text::version::semantic::build::Build;

    #[test]
    fn test_parse() {
        let valid_builds = [
            "20130313144700",
        ];
        for b in valid_builds {
            assert_eq!(Build::parse_build_identifier(b, true).unwrap(), b);
        }

        let valid_in_relaxed = [
            "21AF26D3",
        ];
        for b in valid_in_relaxed {
            assert_eq!(Build::parse_build_identifier(b, false).unwrap(), b);
        }
    }
}