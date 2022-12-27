use crate::text::token::ascii::AsciiMatcher;
use crate::text::version::semantic::error::{ParseError, ParseErrorReason, ParseInvalidChar, ParseInvalidPart, ParseNonAsciiAlphaNumString};

fn parse_is_non_digit(c: char) -> bool {
    // <non-digit> ::= <letter>
    //               | "-"
    // CC-BY 3.0, https://semver.org

    match c {
        'a'..='z' => true,
        'A'..='Z' => true,
        '-' => true,
        _ => false,
    }
}

fn parse_is_identifier_character(c: char) -> bool {
    // <identifier character> ::= <digit>
    //                          | <non-digit>
    // CC-BY 3.0, https://semver.org

    match c {
        '0'..='9' => true,
        _ => parse_is_non_digit(c),
    }
}

pub fn parse_numeric_identifier(pre: &str, strict: bool) -> Result<&str, ParseError> {
    // <numeric identifier> ::= "0"
    //                        | <positive digit>
    //                        | <positive digit> <digits>

    if strict {
        if pre.eq("0") {
            Ok(pre)
        } else {
            // Reject leading 0
            match pre.chars().nth(0) {
                None => Err(ParseError::new(
                    ParseInvalidPart::NumericIdentifier,
                    ParseErrorReason::InvalidPattern,
                )),
                Some(f) =>
                    if f == '0' {
                        Err(ParseError::new(
                            ParseInvalidPart::NumericIdentifier,
                            ParseErrorReason::NumberIdentifierShouldNotHaveLeadingZero,
                        ))
                    } else if pre.is_ascii_numeric() {
                        Ok(pre)
                    } else {
                        Err(ParseError::new(
                            ParseInvalidPart::NumericIdentifier,
                            ParseErrorReason::InvalidChar(ParseInvalidChar::from(f)),
                        ))
                    }
            }
        }
    } else {
        // Accept leading zeros.
        if pre.is_ascii_numeric() {
            Ok(pre)
        } else {
            Err(ParseError::new(
                ParseInvalidPart::NumericIdentifier,
                ParseErrorReason::NonAsciiAlphaNumString(ParseNonAsciiAlphaNumString::from(pre)),
            ))
        }
    }
}

pub fn parse_alphanumeric_identifier(pre: &str, strict: bool) -> Result<&str, ParseError> {
    // <alphanumeric identifier> ::= <non-digit>
    //                             | <non-digit> <identifier characters>
    //                             | <identifier characters> <non-digit>
    //                             | <identifier characters> <non-digit> <identifier characters>
    // CC-BY 3.0, https://semver.org

    let pre_len = pre.chars().count();
    let pre_with_guard = pre.to_owned() + " ";

    if strict {
        let pos_non_digit = pre_with_guard.chars().nth(0).map_or(0, |c| { parse_is_non_digit(c).then(|| 1).unwrap_or(0) });
        if pos_non_digit == 1 {
            let pos_identifier_char = pre_with_guard.chars().skip(pos_non_digit).position(|c| !parse_is_identifier_character(c)).unwrap_or(0);
            if pos_non_digit + pos_identifier_char == pre_len {
                Ok(pre)
            } else {
                Err(ParseError::new(ParseInvalidPart::AlphaNumericIdentifier, ParseErrorReason::InvalidPattern))
            }
        } else {
            let pos_identifier1 = pre_with_guard.chars().position(|c| !parse_is_identifier_character(c)).unwrap_or(0);
            let pos_non_digit1 = pre_with_guard.chars().nth(pos_identifier1).map_or(0, |c| { parse_is_non_digit(c).then(|| 1).unwrap_or(0) });
            let pos_identifier2 = pre_with_guard.chars().skip(pos_identifier1 + pos_non_digit1).position(|c| !parse_is_identifier_character(c)).unwrap_or(0);

            if pos_identifier1 == 0 {
                Err(ParseError::new(ParseInvalidPart::AlphaNumericIdentifier, ParseErrorReason::InvalidPattern))
            } else if pos_identifier1 + pos_non_digit1 == pre_len && pos_non_digit1 == 1 {
                Ok(pre)
            } else if pos_identifier1 + pos_non_digit1 + pos_identifier2 == pre_len && pos_non_digit1 == 1 {
                Ok(pre)
            } else {
                Err(ParseError::new(ParseInvalidPart::AlphaNumericIdentifier, ParseErrorReason::InvalidPattern))
            }
        }
    } else {
        // Validate whether the str consist of legal chars
        let pos_non_id = pre_with_guard.chars().position(|c| !parse_is_identifier_character(c)).unwrap_or(0);
        if pos_non_id == pre_len {
            Ok(pre)
        } else {
            Err(ParseError::new(ParseInvalidPart::AlphaNumericIdentifier,
                                ParseErrorReason::NonAsciiAlphaNumString(ParseNonAsciiAlphaNumString::from(pre))))
        }
    }
}
