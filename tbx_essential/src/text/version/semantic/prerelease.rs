use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

use crate::text::version::semantic::{compare, parse};
use crate::text::version::semantic::error::{ParseError, ParseErrorReason, ParseInvalidPart};

/// Dot separated pre-release identifies (e.g. `Alpha1`, `Alpha.beta`, `Beta.2`)
#[derive(Debug, Clone)]
pub struct PreRelease<'a> {
    pre_release: Vec<&'a str>,
}

impl<'a> PreRelease<'a> {
    /// Parse pre-release part.
    pub fn parse(pre: &str, strict: bool) -> Result<PreRelease, ParseError> {
        // ```
        // <pre-release> ::= <dot-separated pre-release identifiers>
        //
        // <dot-separated pre-release identifiers> ::= <pre-release identifier>
        //                                           | <pre-release identifier> "." <dot-separated pre-release identifiers>
        // <pre-release identifier> ::= <alphanumeric identifier>
        //                            | <numeric identifier>
        // <alphanumeric identifier> ::= <non-digit>
        //                             | <non-digit> <identifier characters>
        //                             | <identifier characters> <non-digit>
        //                             | <identifier characters> <non-digit> <identifier characters>
        // <numeric identifier> ::= "0"
        //                        | <positive digit>
        //                        | <positive digit> <digits>
        // <identifier characters> ::= <identifier character>
        //                           | <identifier character> <identifier characters>
        // <identifier character> ::= <digit>
        //                          | <non-digit>
        // <non-digit> ::= <letter>
        //               | "-"
        // <digits> ::= <digit>
        //            | <digit> <digits>
        // <digit> ::= "0"
        //           | <positive digit>
        // <positive digit> ::= "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
        // <letter> ::= "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J"
        //            | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T"
        //            | "U" | "V" | "W" | "X" | "Y" | "Z" | "a" | "b" | "c" | "d"
        //            | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n"
        //            | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x"
        //            | "y" | "z"
        // ```
        // CC-BY 3.0, https://semver.org

        match Self::parse_pre_release(pre, strict) {
            Ok(p) => Ok(PreRelease {
                pre_release: p,
            }),
            Err(e) => Err(e),
        }
    }

    fn parse_pre_release_identifier(pre: &str, strict: bool) -> Result<&str, ParseError> {
        // <pre-release identifier> ::= <alphanumeric identifier>
        //                            | <numeric identifier>
        // CC-BY 3.0, https://semver.org

        if let Ok(id) = parse::parse_alphanumeric_identifier(pre, strict) {
            Ok(id)
        } else if let Ok(id) = parse::parse_numeric_identifier(pre, strict) {
            Ok(id)
        } else {
            Err(ParseError::new(ParseInvalidPart::PreRelease, ParseErrorReason::InvalidPattern))
        }
    }

    fn parse_pre_release(pre: &str, strict: bool) -> Result<Vec<&str>, ParseError> {
        // <pre-release> ::= <dot-separated pre-release identifiers>
        //
        // <dot-separated pre-release identifiers> ::= <pre-release identifier>
        //                                           | <pre-release identifier> "." <dot-separated pre-release identifiers>
        // CC-BY 3.0, https://semver.org

        pre.split(".").map(|p| Self::parse_pre_release_identifier(p, strict)).into_iter().collect()
    }
}

impl<'a> fmt::Display for PreRelease<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pre_release.join("."))
    }
}

impl<'a> Eq for PreRelease<'a> {}

impl<'a> PartialEq<Self> for PreRelease<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.pre_release == other.pre_release
    }
}

impl<'a> PartialOrd<Self> for PreRelease<'a> {
    /// Comparison of Pre release.
    ///
    /// > 1. Identifiers consisting of only digits are compared numerically.
    /// > 2. Identifiers with letters or hyphens are compared lexically in ASCII sort order.
    /// > 3. Numeric identifiers always have lower precedence than non-numeric identifiers.
    /// > 4. A larger set of pre-release fields has a higher precedence than a smaller set,
    /// >    if all of the preceding identifiers are equal.
    /// > Example: 1.0.0-alpha < 1.0.0-alpha.1 < 1.0.0-alpha.beta < 1.0.0-beta < 1.0.0-beta.2 < 1.0.0-beta.11 < 1.0.0-rc.1 < 1.0.0.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for (i, vx) in self.pre_release.iter().enumerate() {
            match other.pre_release.get(i) {
                Some(vy) => {
                    let vc = compare::cmp_pre_release(vx, vy);
                    if vc == Ordering::Equal {
                        continue;
                    } else {
                        return Some(vc);
                    }
                }
                None =>
                    return Some(Ordering::Greater)
            }
        }
        if self.pre_release.len() == other.pre_release.len() {
            Some(Ordering::Equal)
        } else if self.pre_release.len() < other.pre_release.len() {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}


#[cfg(test)]
mod pre_release {
    use std::cmp::Ordering;

    use crate::text::version::semantic::prerelease::PreRelease;

    #[test]
    fn test_parse_pre_release_identifier() {
        let valid_pre_release = [
            "-", "-0-0-0-", "123", "Alpha1", "alpha-1", "ALPHA-1",
            "Alpha1Beta2", "Alpha-1-Beta-2-Theta-3", "alpha"
        ];
        for p in valid_pre_release {
            assert_eq!(PreRelease::parse_pre_release_identifier(p, true).unwrap(), p);
        }

        let invalid_pre_release = [
            "_", "ABC_123", "-ABC_123-", // invalid chars
            "12-34-56", "100-Alpha1", "0-", // invalid patterns
        ];
        for p in invalid_pre_release {
            assert_eq!(PreRelease::parse_pre_release_identifier(p, true).unwrap_or("ERR"), "ERR");
        }

        let relaxed_pre_release = [
            "12-34-56", "100-Alpha1", "0-", // invalid patterns
        ];
        for p in relaxed_pre_release {
            assert_eq!(PreRelease::parse_pre_release_identifier(p, false).unwrap(), p);
        }
    }

    #[test]
    fn test_eq() {
        let x_alpha1 = PreRelease::parse("alpha1", true).unwrap();
        let y_alpha1 = PreRelease::parse("alpha1", true).unwrap();
        let x_alpha1_beta2 = PreRelease::parse("alpha1-beta2", true).unwrap();

        assert!(x_alpha1.eq(&y_alpha1));
        assert_eq!(x_alpha1, y_alpha1);
        assert!(!x_alpha1.eq(&x_alpha1_beta2));
    }

    #[test]
    fn test_partial_cmp() {
        let x_alpha = PreRelease::parse("alpha", true).unwrap();
        let x_alpha_1 = PreRelease::parse("alpha.1", true).unwrap();
        let x_alpha_beta = PreRelease::parse("alpha.beta", true).unwrap();
        let x_beta = PreRelease::parse("beta", true).unwrap();
        let x_beta_2 = PreRelease::parse("beta.2", true).unwrap();
        let x_beta_11 = PreRelease::parse("beta.11", true).unwrap();
        let x_rc_1 = PreRelease::parse("rc.1", true).unwrap();

        // 1.0.0-alpha < 1.0.0-alpha.1
        assert_eq!(x_alpha.partial_cmp(&x_alpha).unwrap(), Ordering::Equal);
        assert_eq!(x_alpha.partial_cmp(&x_alpha_1).unwrap(), Ordering::Less);
        assert_eq!(x_alpha_1.partial_cmp(&x_alpha).unwrap(), Ordering::Greater);

        // 1.0.0-alpha.1 < 1.0.0-alpha.beta
        assert_eq!(x_alpha_1.partial_cmp(&x_alpha_1).unwrap(), Ordering::Equal);
        assert_eq!(x_alpha_1.partial_cmp(&x_alpha_beta).unwrap(), Ordering::Less);
        assert_eq!(x_alpha_beta.partial_cmp(&x_alpha_1).unwrap(), Ordering::Greater);

        // 1.0.0-alpha.beta < 1.0.0-beta
        assert_eq!(x_alpha_beta.partial_cmp(&x_alpha_beta).unwrap(), Ordering::Equal);
        assert_eq!(x_alpha_beta.partial_cmp(&x_beta).unwrap(), Ordering::Less);
        assert_eq!(x_beta.partial_cmp(&x_alpha_beta).unwrap(), Ordering::Greater);

        // 1.0.0-beta < 1.0.0-beta.2
        assert_eq!(x_beta.partial_cmp(&x_beta).unwrap(), Ordering::Equal);
        assert_eq!(x_beta.partial_cmp(&x_beta_2).unwrap(), Ordering::Less);
        assert_eq!(x_beta_2.partial_cmp(&x_beta).unwrap(), Ordering::Greater);

        // 1.0.0-beta.2 < 1.0.0-beta.11
        assert_eq!(x_beta_2.partial_cmp(&x_beta_2).unwrap(), Ordering::Equal);
        assert_eq!(x_beta_2.partial_cmp(&x_beta_11).unwrap(), Ordering::Less);
        assert_eq!(x_beta_11.partial_cmp(&x_beta_2).unwrap(), Ordering::Greater);

        // 1.0.0-beta.11 < 1.0.0-rc.1
        assert_eq!(x_beta_11.partial_cmp(&x_beta_11).unwrap(), Ordering::Equal);
        assert_eq!(x_beta_11.partial_cmp(&x_rc_1).unwrap(), Ordering::Less);
        assert_eq!(x_rc_1.partial_cmp(&x_beta_11).unwrap(), Ordering::Greater);
    }
}
