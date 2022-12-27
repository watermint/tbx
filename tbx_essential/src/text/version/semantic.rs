use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

use build::Build;
use prerelease::PreRelease;

use crate::text::essential::StringEssential;
use crate::text::version::semantic::error::{ParseError, ParseErrorReason, ParseInvalidPart};
use crate::text::version::semantic::error::ParseErrorReason::InvalidPattern;

mod build;
mod prerelease;
mod parse;
mod compare;
mod error;

/// Structure for Semantic versioning elements.
/// see: <https://semver.org> for more detail about semantic versioning.
#[derive(Debug, Clone)]
pub struct Version<'a> {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre_release: Option<PreRelease<'a>>,
    pub build: Option<Build<'a>>,
}

// Constructors
impl<'a> Version<'a> {
    /// Creates version 0.0.0 instance.
    pub fn zero() -> Self {
        Version {
            major: 0,
            minor: 0,
            patch: 0,
            pre_release: None,
            build: None,
        }
    }

    /// Create new version instance with specified version.
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Version {
            major,
            minor,
            patch,
            pre_release: None,
            build: None,
        }
    }
}

// Parsers
impl<'a> Version<'a> {
    /// Parses the string and returns the version.
    pub fn parse(ver: &str, strict: bool) -> Result<Version, ParseError> {
        // <valid semver> ::= <version core>
        //                  | <version core> "-" <pre-release>
        //                  | <version core> "+" <build>
        //                  | <version core> "-" <pre-release> "+" <build>
        // <version core> ::= <major> "." <minor> "." <patch>
        //
        // <major> ::= <numeric identifier>
        // <minor> ::= <numeric identifier>
        // <patch> ::= <numeric identifier>
        // CC-BY 3.0, https://semver.org

        let (major, minor, patch, reminder) = Self::parse_version_core(ver, strict)?;
        match reminder {
            Some(r) => {
                let pb: (Option<PreRelease>, Option<Build>) = Self::parse_pre_release_and_build(r, strict)?;
                Ok(Version {
                    major,
                    minor,
                    patch,
                    pre_release: pb.0,
                    build: pb.1,
                })
            }
            None => Ok(Version {
                major,
                minor,
                patch,
                pre_release: None,
                build: None,
            })
        }
    }

    /// Parses the string and returns the version.
    /// If an error occurs, return the specified version.
    pub fn parse_or(ver: &'a str, major: u64, minor: u64, patch: u64) -> Version<'a> {
        match Self::parse(ver, false) {
            Ok(v) => v,
            _ => Self::new(major, minor, patch)
        }
    }

    /// Parses the string and returns the version.
    /// If an error occurs, return the zero version.
    pub fn parse_or_zero(ver: &'a str) -> Version<'a> {
        Self::parse_or(ver, 0, 0, 0)
    }

    fn parse_pre_release_and_build(ver_reminder: &str, strict: bool) -> Result<(Option<PreRelease>, Option<Build>), ParseError> {
        let pos_plus = ver_reminder.chars().position(|c| c == '+');
        let first_char = ver_reminder.chars().nth(0);
        match (first_char, pos_plus) {
            (Some('-'), Some(p_plus)) =>
                match (ver_reminder.substring(1, p_plus), ver_reminder.substring_to_end(p_plus + 1)) {
                    (Some(v_pre_release), Some(v_build)) => {
                        let p = PreRelease::parse(v_pre_release, strict)?;
                        let b = Build::parse(v_build, strict)?;
                        Ok((Some(p), Some(b)))
                    }
                    _ => Err(ParseError::new(ParseInvalidPart::PrereleaseOrBuild, ParseErrorReason::InvalidPattern)),
                }
            (Some('-'), None) =>
                match ver_reminder.substring_to_end(1) {
                    Some(v_pre_release) => {
                        let p = PreRelease::parse(v_pre_release, strict)?;
                        Ok((Some(p), None))
                    }
                    _ => Err(ParseError::new(ParseInvalidPart::PreRelease, ParseErrorReason::InvalidPattern))
                },
            (Some('+'), Some(p_plus)) =>
                match ver_reminder.substring_to_end(p_plus + 1) {
                    Some(v_build) => {
                        let b = Build::parse(v_build, strict)?;
                        Ok((None, Some(b)))
                    }
                    _ => Err(ParseError::new(ParseInvalidPart::Build, ParseErrorReason::InvalidPattern))
                },
            _ => Err(ParseError::new(ParseInvalidPart::PrereleaseOrBuild, ParseErrorReason::InvalidPattern))
        }
    }

    /// parse `<version core>` then returns `<major>`, `<minor>`, `<patch>`
    /// and reminder string. Returns `None` when the pattern is not allowed.
    fn parse_version_core(ver: &str, strict: bool) -> Result<(u64, u64, u64, Option<&str>), ParseError> {
        let ver_with_guard = ver.to_owned() + " ";
        let pos_dot1 = ver.chars().position(|c| c == '.').unwrap_or(0);
        let pos_dot2 = ver.chars().skip(pos_dot1 + 1).position(|c| c == '.').unwrap_or(0);
        if pos_dot1 == 0 || pos_dot2 == 0 {
            Err(ParseError::new(ParseInvalidPart::VersionNumber, InvalidPattern))
        } else {
            let pos_reminder = ver_with_guard.chars().skip(pos_dot1 + pos_dot2 + 2).position(|c| !c.is_ascii_digit()).unwrap_or(0);
            let part_major = ver.substring(0, pos_dot1);
            let part_minor = ver.substring(pos_dot1 + 1, pos_dot1 + pos_dot2 + 1);
            let part_patch = ver.substring(pos_dot1 + pos_dot2 + 2, pos_dot1 + pos_dot2 + 2 + pos_reminder);

            match (0 < pos_reminder, part_major, part_minor, part_patch) {
                (true, Some(p_major), Some(p_minor), Some(p_patch)) => {
                    let s_major = parse::parse_numeric_identifier(p_major, strict)?;
                    let s_minor = parse::parse_numeric_identifier(p_minor, strict)?;
                    let s_patch = parse::parse_numeric_identifier(p_patch, strict)?;
                    match (s_major.parse::<u64>(), s_minor.parse::<u64>(), s_patch.parse::<u64>(), ver.substring_to_end(pos_dot1 + pos_dot2 + 2 + pos_reminder)) {
                        (Ok(v_major), Ok(v_minor), Ok(v_patch), Some(s_rem)) =>
                            Ok((v_major, v_minor, v_patch, Some(s_rem))),
                        (Ok(v_major), Ok(v_minor), Ok(v_patch), None) =>
                            Ok((v_major, v_minor, v_patch, None)),
                        _ =>
                            Err(ParseError::new(ParseInvalidPart::VersionNumber, ParseErrorReason::InvalidPattern)),
                    }
                }
                _ => Err(ParseError::new(ParseInvalidPart::VersionNumber, ParseErrorReason::InvalidPattern)),
            }
        }
    }
}

impl<'a> fmt::Display for Version<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match (&self.pre_release, &self.build) {
            (Some(pre), Some(build)) =>
                write!(f, "{}.{}.{}-{}+{}", self.major, self.minor, self.patch, pre, build),
            (Some(pre), None) =>
                write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, pre),
            (None, Some(build)) =>
                write!(f, "{}.{}.{}+{}", self.major, self.minor, self.patch, build),
            _ =>
                write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
        }
    }
}

impl<'a> Eq for Version<'a> {}

impl<'a> PartialEq<Self> for Version<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major &&
            self.minor == other.minor &&
            self.patch == other.patch &&
            self.pre_release == other.pre_release &&
            self.build == other.build
    }
}

impl<'a> PartialOrd<Self> for Version<'a> {
    /// Compare versions.
    /// ---
    /// Precedence for two pre-release versions with the same major, minor, and patch version MUST be determined by comparing each dot separated identifier from left to right until a difference is found as follows:
    /// Identifiers consisting of only digits are compared numerically.
    /// Identifiers with letters or hyphens are compared lexically in ASCII sort order.
    /// Numeric identifiers always have lower precedence than non-numeric identifiers.
    /// A larger set of pre-release fields has a higher precedence than a smaller set, if all of the preceding identifiers are equal.
    /// Example: 1.0.0-alpha < 1.0.0-alpha.1 < 1.0.0-alpha.beta < 1.0.0-beta < 1.0.0-beta.2 < 1.0.0-beta.11 < 1.0.0-rc.1 < 1.0.0.
    /// (description CC-BY 3.0, <https://semver.org>)
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp_major = self.major.cmp(&other.major);
        if cmp_major != Ordering::Equal {
            Some(cmp_major)
        } else {
            let cmp_minor = self.minor.cmp(&other.minor);
            if cmp_minor != Ordering::Equal {
                Some(cmp_minor)
            } else {
                let cmp_patch = self.patch.cmp(&other.patch);
                if cmp_patch != Ordering::Equal {
                    Some(cmp_patch)
                } else {
                    match (&self.pre_release, &other.pre_release) {
                        (Some(sp), Some(op)) => sp.partial_cmp(&op),
                        (Some(_), None) => Some(Ordering::Less),
                        (None, Some(_)) => Some(Ordering::Greater),
                        (None, None) => Some(Ordering::Equal),
                    }
                }
            }
        }
    }
}

/// Parse & return package version.
/// This function will return additional information in the future,
/// such as build numbers from CI.
///
/// ```rust
/// use tbx_essential::text::version::semantic;
///
/// let v = semantic::package_version(option_env!("CARGO_PKG_VERSION"));
/// ```
pub fn package_version(v: Option<&str>) -> Version {
    match v {
        None => Version::zero(),
        Some(v) => Version::parse_or_zero(v),
    }
}

#[cfg(test)]
mod version {
    use crate::text::version::semantic::build::Build;
    use crate::text::version::semantic::prerelease::PreRelease;
    use crate::text::version::semantic::Version;

    #[test]
    fn test_zero() {
        let z = Version::zero();

        assert_eq!(0, z.major);
        assert_eq!(0, z.minor);
        assert_eq!(0, z.patch);
        assert_eq!(None, z.pre_release);
        assert_eq!(None, z.build);
    }

    #[test]
    fn test_parse_version_core() {
        let valid_version = [
            "1.2.3", "12.34.56", "100.0.3840",
            "1.0.0-alpha", "1.0.0-alpha.1", "1.0.0-alpha.beta",
            "1.0.0-beta", "1.0.0-beta.2", "1.0.0-beta.11", "1.0.0-rc.1", "1.0.0",
        ];

        for v in valid_version {
            assert!(Version::parse_version_core(v, true).is_ok(), "{}", &v)
        }

        assert_eq!(Version::parse_version_core("1.2.3", true).unwrap(), (1, 2, 3, None));
        assert_eq!(Version::parse_version_core("12.34.56", true).unwrap(), (12, 34, 56, None));
        assert_eq!(Version::parse_version_core("100.0.3840", true).unwrap(), (100, 0, 3840, None));
        assert_eq!(Version::parse_version_core("1.0.0-alpha", true).unwrap(), (1, 0, 0, Some("-alpha")));
        assert_eq!(Version::parse_version_core("1.0.0-alpha.1", true).unwrap(), (1, 0, 0, Some("-alpha.1")));
    }

    #[test]
    fn test_ord() {
        // Example: 1.0.0-alpha < 1.0.0-alpha.1 < 1.0.0-alpha.beta < 1.0.0-beta < 1.0.0-beta.2 < 1.0.0-beta.11 < 1.0.0-rc.1 < 1.0.0.
        let v1_0_0_alpha = Version::parse("1.0.0-alpha", true).unwrap();
        let v1_0_0_alpha_1 = Version::parse("1.0.0-alpha.1", true).unwrap();
        let v1_0_0_alpha_beta = Version::parse("1.0.0-alpha.beta", true).unwrap();
        let v1_0_0_beta = Version::parse("1.0.0-beta", true).unwrap();
        let v1_0_0_beta_2 = Version::parse("1.0.0-beta.2", true).unwrap();
        let v1_0_0_beta_11 = Version::parse("1.0.0-beta.11", true).unwrap();
        let v1_0_0_rc_1 = Version::parse("1.0.0-rc.1", true).unwrap();
        let v1_0_0 = Version::parse("1.0.0", true).unwrap();
        let v1_0_1 = Version::parse("1.0.1", true).unwrap();
        let v1_1_0 = Version::parse("1.1.0", true).unwrap();
        let v2_0_0 = Version::parse("2.0.0", true).unwrap();

        assert!(v1_0_0_alpha.partial_cmp(&v1_0_0_alpha_1).unwrap().is_le());
        assert!(v1_0_0_alpha_1.partial_cmp(&v1_0_0_alpha_beta).unwrap().is_le());
        assert!(v1_0_0_beta.partial_cmp(&v1_0_0_beta_2).unwrap().is_le());
        assert!(v1_0_0_beta_2.partial_cmp(&v1_0_0_beta_11).unwrap().is_le());
        assert!(v1_0_0_beta_11.partial_cmp(&v1_0_0_rc_1).unwrap().is_le());
        assert!(v1_0_0_rc_1.partial_cmp(&v1_0_0).unwrap().is_le());
        assert!(v1_0_0.partial_cmp(&v1_0_1).unwrap().is_le());
        assert!(v1_0_1.partial_cmp(&v1_1_0).unwrap().is_le());
        assert!(v1_1_0.partial_cmp(&v2_0_0).unwrap().is_le());

        assert!(v2_0_0.partial_cmp(&v1_1_0).unwrap().is_ge());
        assert!(v1_1_0.partial_cmp(&v1_0_1).unwrap().is_ge());
        assert!(v1_0_1.partial_cmp(&v1_0_0).unwrap().is_ge());
        assert!(v1_0_0_rc_1.partial_cmp(&v1_0_0_beta_11).unwrap().is_ge());
        assert!(v1_0_0_beta_11.partial_cmp(&v1_0_0_beta_2).unwrap().is_ge());
        assert!(v1_0_0_beta_2.partial_cmp(&v1_0_0_beta).unwrap().is_ge());
        assert!(v1_0_0_beta.partial_cmp(&v1_0_0_alpha_beta).unwrap().is_ge());
        assert!(v1_0_0_alpha_beta.partial_cmp(&v1_0_0_alpha_1).unwrap().is_ge());
        assert!(v1_0_0_alpha_1.partial_cmp(&v1_0_0_alpha).unwrap().is_ge());

        let v1_0_0_build_20221208 = Version::parse("1.0.0+20221208", true).unwrap();
        assert!(v1_0_0.partial_cmp(&v1_0_0_build_20221208).unwrap().is_eq());
    }

    #[test]
    fn test_eq() {
        let z = Version::zero();
        let one_zero_zero = Version {
            major: 1,
            minor: 0,
            patch: 0,
            pre_release: None,
            build: None,
        };
        assert!(z.eq(&z));
        assert!(!z.eq(&one_zero_zero));
        assert!(one_zero_zero.eq(&one_zero_zero));
        assert!(!one_zero_zero.eq(&z));
    }

    #[test]
    fn test_parse() {
        let one_two_three = Version::parse("1.2.3", true).unwrap();
        assert_eq!("1.2.3", format!("{one_two_three}"));

        let one_two_three_alpha = Version::parse("1.2.3-alpha", true).unwrap();
        assert_eq!("1.2.3-alpha", format!("{one_two_three_alpha}"));

        let one_two_three_alpha_beta = Version::parse("1.2.3-alpha+beta", true).unwrap();
        assert_eq!("1.2.3-alpha+beta", format!("{one_two_three_alpha_beta}"));
    }

    #[test]
    fn test_fmt() {
        let zero = Version::zero();
        assert_eq!("0.0.0", format!("{zero}"));

        let one_two_three = Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: None,
            build: None,
        };
        assert_eq!("1.2.3", format!("{one_two_three}"));

        let one_two_three_alpha = Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: Some(PreRelease::parse("alpha", true).unwrap()),
            build: None,
        };
        assert_eq!("1.2.3-alpha", format!("{one_two_three_alpha}"));

        let one_two_three_build = Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: None,
            build: Some(Build::parse("20221130", true).unwrap()),
        };
        assert_eq!("1.2.3+20221130", format!("{one_two_three_build}"));

        let one_two_three_beta_build = Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: Some(PreRelease::parse("beta", true).unwrap()),
            build: Some(Build::parse("20221130", true).unwrap()),
        };
        assert_eq!("1.2.3-beta+20221130", format!("{one_two_three_beta_build}"));
    }
}
