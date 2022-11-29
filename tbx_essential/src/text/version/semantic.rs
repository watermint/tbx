use std::fmt;
use std::fmt::Formatter;

/// Structure for Semantic versioning elements.
/// see: https://semver.org for more detail about semantic versioning.
pub struct Version {
    major: u64,
    minor: u64,
    patch: u64,
    pre_release: Option<String>,
    build: Option<String>,
}

impl Version {
    /// Creates version 0.0.0 instance.
    fn zero() -> Version {
        Version {
            major: 0,
            minor: 0,
            patch: 0,
            pre_release: None,
            build: None,
        }
    }
}

impl fmt::Display for Version {
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

#[cfg(test)]
mod tests {
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
    fn test_fmt() {
        let zero = Version::zero();
        assert_eq!("0.0.0", format!("{zero}"));

        let one_two_three = Version{
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: None,
            build: None,
        };
        assert_eq!("1.2.3", format!("{one_two_three}"));

        let one_two_three_alpha = Version{
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: Some(String::from("alpha")),
            build: None,
        };
        assert_eq!("1.2.3-alpha", format!("{one_two_three_alpha}"));

        let one_two_three_build = Version{
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: None,
            build: Some(String::from("20221130")),
        };
        assert_eq!("1.2.3+20221130", format!("{one_two_three_build}"));

        let one_two_three_beta_build = Version{
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: Some(String::from("beta")),
            build: Some(String::from("20221130")),
        };
        assert_eq!("1.2.3-beta+20221130", format!("{one_two_three_beta_build}"));
    }
}