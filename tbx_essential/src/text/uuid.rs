pub mod error;
pub mod v4;

use std::borrow::Cow;
use crate::text::hex;
use crate::text::hex::Hex;
use crate::text::regex::{Matcher, Regex};
use crate::text::regex::matcher::CaptureIndexer;
use crate::text::uuid::error::ParseError;
use crate::text::uuid::error::ParseError::{InvalidPattern, SystemError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Version {
    /// Version 1: Date-time and MAC address
    Version1,

    /// Version 2: Date-time and MAC address, DCE security version
    Version2,

    /// Version 3: Namespace name-based (MD5)
    Version3,

    /// Version 4: Random UUID
    Version4,

    /// Version 5: Namespace name-based (SHA1)
    Version5,

    /// Version 6 (Draft): UUID version 6 is a field-compatible version of UUIDv1,
    /// reordered for improved DB locality.
    /// <https://datatracker.ietf.org/doc/html/draft-peabody-dispatch-new-uuid-format-04#name-uuid-version-6>
    Version6Draft,

    /// Version 7 (Draft): UUID version 7 features a time-ordered value field derived from the widely
    /// implemented and well known Unix Epoch timestamp source, the number of milliseconds
    /// seconds since midnight 1 Jan 1970 UTC, leap seconds excluded.
    /// <https://datatracker.ietf.org/doc/html/draft-peabody-dispatch-new-uuid-format-04#section-5.2>
    Version7Draft,

    /// Version 8 (Draft): UUID version 8 provides an RFC-compatible format for experimental or
    /// vendor-specific use cases.
    /// <https://datatracker.ietf.org/doc/html/draft-peabody-dispatch-new-uuid-format-04#section-5.3>
    Version8Draft,

    /// Undefined version
    Undefined,
}

/// The variant field determines the layout of the UUID.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Variant {
    /// (0 x x) Reserved, NCS backward compatibility.
    NCS,

    /// (1 0 x) The variant defined in RFC4122.
    RFC4122,

    /// (1 1 0) Reserved, Microsoft Corporation backward compatibility
    Microsoft,

    /// (1 1 1) Reserved for future definition.
    Reserved,
}

/// UUID (A Universally Unique IDentifier).
/// RFC 4122: <https://www.rfc-editor.org/rfc/rfc4122>
pub trait Layout {
    /// Returns UUID format (in lower case) defined in RFC 4122 like `123e4567-e89b-12d3-a456-426655440000`.
    fn uuid_lower<'a>(&self) -> Cow<'a, str>;

    /// Returns UUID format (in upper case) defined in RFC 4122 like `123E4567-E89B-12D3-A456-426655440000`.
    fn uuid_upper<'a>(&self) -> Cow<'a, str>;

    /// Returns UUID format with brace like Microsoft's GUID (e.g. `{123e4567-e89b-12d3-a456-426655440000}`).
    fn uuid_with_brace<'a>(&self) -> Cow<'a, str>;

    /// Returns URN of the UUID like `urn:uuid:123e4567-e89b-12d3-a456-426655440000`.
    fn urn<'a>(&self) -> Cow<'a, str>;

    /// Variant of the UUID.
    /// The variant field determines the layout of the UUID.
    fn variant(&self) -> Variant;

    /// Version of the UUID.
    fn version(&self) -> Version;

    /// Returns true if the UUID is Nil UUID (all zero).
    /// The nil UUID is special form of UUID that is
    /// specified to have all 128 bits set to zero.
    /// RFC 4122 4.1.7 <https://datatracker.ietf.org/doc/html/rfc4122#section-4.1.7>
    fn is_nil(&self) -> bool;

    /// Returns true if the UUID is Max UUID (all one).
    /// The Max UUID is special form of UUID that is
    /// specified to have all 128 bits set to one.
    /// RFC Draft <https://datatracker.ietf.org/doc/html/draft-peabody-dispatch-new-uuid-format-04#name-max-uuid>
    fn is_max(&self) -> bool;
}

/// UUID data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UUID {
    data: [u8; 16],
}

const UUID_REGEX_RFC4122: &str = r"(?P<u0>[0-9a-fA-F]{8})-(?P<u1>[0-9a-fA-F]{4})-(?P<u2>[0-9a-fA-F]{4})-(?P<u3>[0-9a-fA-F]{4})-(?P<u4>[0-9a-fA-F]{12})";
const UUID_REGEX_URN: &str = r"urn:uuid:(?P<u0>[0-9a-fA-F]{8})-(?P<u1>[0-9a-fA-F]{4})-(?P<u2>[0-9a-fA-F]{4})-(?P<u3>[0-9a-fA-F]{4})-(?P<u4>[0-9a-fA-F]{12})";
const UUID_REGEX_MICROSOFT: &str = r"\{(?P<u0>[0-9a-fA-F]{8})-(?P<u1>[0-9a-fA-F]{4})-(?P<u2>[0-9a-fA-F]{4})-(?P<u3>[0-9a-fA-F]{4})-(?P<u4>[0-9a-fA-F]{12})\}";
const UUID_REGEX_NOHYPHEN: &str = r"(?P<u0>[0-9a-fA-F]{8})(?P<u1>[0-9a-fA-F]{4})(?P<u2>[0-9a-fA-F]{4})(?P<u3>[0-9a-fA-F]{4})(?P<u4>[0-9a-fA-F]{12})";

/// Namespace of fully-qualified domain name (for Version 3/5 UUID).
pub const NAMESPACE_DNS: &str = "6ba7b810-9dad-11d1-80b4-00c04fd430c8";

/// Namespace of URL (for Version 3/5 UUID).
pub const NAMESPACE_URL: &str = "6ba7b811-9dad-11d1-80b4-00c04fd430c8";

/// Namespace of ISO OID (for Version 3/5 UUID).
pub const NAMESPACE_OID: &str = "6ba7b812-9dad-11d1-80b4-00c04fd430c8";

/// Namespace of X.500 DN (for Version 3/5 UUID).
pub const NAMESPACE_X500: &str = "6ba7b814-9dad-11d1-80b4-00c04fd430c8";

impl UUID {
    pub fn new(data: [u8; 16]) -> Self { Self { data } }

    pub fn parse(uuid: &str) -> Result<Self, ParseError> {
        let patterns = vec![UUID_REGEX_RFC4122, UUID_REGEX_NOHYPHEN, UUID_REGEX_URN, UUID_REGEX_MICROSOFT];
        for pattern in patterns {
            match Regex::parse(pattern) {
                Ok(re) => match re.capture_first(uuid) {
                    Some(ru) => match (ru.get("u0"), ru.get("u1"), ru.get("u2"), ru.get("u3"), ru.get("u4")) {
                        (Some(u0), Some(u1), Some(u2), Some(u3), Some(u4)) => {
                            return Self::parse_parts(u0.as_str(), u1.as_str(), u2.as_str(), u3.as_str(), u4.as_str());
                        }
                        _ => continue
                    },
                    _ => continue
                },
                _ => return Err(SystemError)
            }
        }
        Err(InvalidPattern)
    }

    pub fn nil_uuid() -> Self {
        Self {
            data: [0; 16]
        }
    }

    pub fn max_uuid() -> Self {
        Self {
            data: [0xff; 16]
        }
    }

    fn parse_parts(p0: &str, p1: &str, p2: &str, p3: &str, p4: &str) -> Result<Self, ParseError> {
        match (hex::parse(p0),
               hex::parse(p1),
               hex::parse(p2),
               hex::parse(p3),
               hex::parse(p4)) {
            (Ok(q0), Ok(q1), Ok(q2), Ok(q3), Ok(q4)) => {
                let mut d: [u8; 16] = [0; 16];
                d[0..4].clone_from_slice(&q0);
                d[4..6].clone_from_slice(&q1);
                d[6..8].clone_from_slice(&q2);
                d[8..10].clone_from_slice(&q3);
                d[10..16].clone_from_slice(&q4);
                Ok(UUID { data: d })
            }
            _ => Err(ParseError::InvalidPattern)
        }
    }
}

impl Layout for UUID {
    fn uuid_lower<'a>(&self) -> Cow<'a, str> {
        self.data[0..4].to_hex_lower() + "-" +
            self.data[4..6].to_hex_lower() + "-" +
            self.data[6..8].to_hex_lower() + "-" +
            self.data[8..10].to_hex_lower() + "-" +
            self.data[10..16].to_hex_lower()
    }

    fn uuid_upper<'a>(&self) -> Cow<'a, str> {
        self.data[0..4].to_hex_upper() + "-" +
            self.data[4..6].to_hex_upper() + "-" +
            self.data[6..8].to_hex_upper() + "-" +
            self.data[8..10].to_hex_upper() + "-" +
            self.data[10..16].to_hex_upper()
    }

    fn uuid_with_brace<'a>(&self) -> Cow<'a, str> {
        Cow::from("{") + self.uuid_lower() + Cow::from("}")
    }

    fn urn<'a>(&self) -> Cow<'a, str> {
        Cow::from("urn:uuid:") + self.uuid_lower()
    }

    fn variant(&self) -> Variant {
        let x = self.data[8] >> 4;
        if x & 0b1000 == 0 {
            Variant::NCS
        } else if x & 0b1100 == 0b1000 {
            Variant::RFC4122
        } else if x & 0b1110 == 0b1100 {
            Variant::Microsoft
        } else {
            Variant::Reserved
        }
    }

    fn version(&self) -> Version {
        match self.data[6] >> 4 {
            1 => Version::Version1,
            2 => Version::Version2,
            3 => Version::Version3,
            4 => Version::Version4,
            5 => Version::Version5,
            6 => Version::Version6Draft,
            7 => Version::Version7Draft,
            8 => Version::Version8Draft,
            _ => Version::Undefined
        }
    }

    fn is_nil(&self) -> bool {
        self.data.iter().all(|x| *x == 0)
    }

    fn is_max(&self) -> bool {
        self.data.iter().all(|x| *x == 0xff)
    }
}

#[cfg(test)]
mod tests {
    use crate::text::uuid::UUID;
    use crate::text::uuid::Layout;
    use crate::text::uuid::Variant::RFC4122;
    use crate::text::uuid::Version::{Version1, Version3, Version4, Version5, Version6Draft, Version7Draft, Version8Draft};

    #[test]
    fn test_nil() {
        let n = UUID::nil_uuid();
        assert!(n.is_nil());
        assert_eq!("00000000-0000-0000-0000-000000000000", n.uuid_lower());
        assert_eq!("00000000-0000-0000-0000-000000000000", n.uuid_upper());
        assert_eq!("urn:uuid:00000000-0000-0000-0000-000000000000", n.urn());
    }

    #[test]
    fn test_max() {
        let m = UUID::max_uuid();
        assert!(m.is_max());
        assert_eq!("ffffffff-ffff-ffff-ffff-ffffffffffff", m.uuid_lower());
        assert_eq!("FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF", m.uuid_upper());
    }

    #[test]
    fn test_parse() {
        let u0 = UUID::parse("00000000-0000-0000-0000-000000000000").unwrap();
        assert!(u0.is_nil());

        let v1 = UUID::parse("{C232AB00-9414-11EC-B3C8-9E6BDECED846}").unwrap();
        assert_eq!(v1.version(), Version1);
        assert_eq!(v1.variant(), RFC4122);
        assert_eq!("C232AB00-9414-11EC-B3C8-9E6BDECED846", v1.uuid_upper());

        let v3 = UUID::parse("375dde34-fc9b-3822-8191-6b2199358695").unwrap();
        assert_eq!("375dde34-fc9b-3822-8191-6b2199358695", v3.uuid_lower());
        assert_eq!(v3.version(), Version3);
        assert_eq!(v3.variant(), RFC4122);

        let v4 = UUID::parse("urn:uuid:f07535d3-228a-4ac3-a900-57081609572e").unwrap();
        assert_eq!("f07535d3-228a-4ac3-a900-57081609572e", v4.uuid_lower());
        assert_eq!(v4.version(), Version4);
        assert_eq!(v4.variant(), RFC4122);

        let v5 = UUID::parse("6063c8c3-aaf6-5e9e-837e-468182f5f55a").unwrap();
        assert_eq!("6063c8c3-aaf6-5e9e-837e-468182f5f55a", v5.uuid_lower());
        assert_eq!(v5.version(), Version5);
        assert_eq!(v5.variant(), RFC4122);

        let v6 = UUID::parse("1EC9414C-232A-6B00-B3C8-9E6BDECED846").unwrap();
        assert_eq!(v6.version(), Version6Draft);
        assert_eq!(v6.variant(), RFC4122);
        assert_eq!("1EC9414C-232A-6B00-B3C8-9E6BDECED846", v6.uuid_upper());

        let v7 = UUID::parse("017F22E2-79B0-7CC3-98C4-DC0C0C07398F").unwrap();
        assert_eq!(v7.version(), Version7Draft);
        assert_eq!(v7.variant(), RFC4122);
        assert_eq!("017F22E2-79B0-7CC3-98C4-DC0C0C07398F", v7.uuid_upper());

        let v8 = UUID::parse("320C3D4D-CC00-875B-8EC9-32D5F69181C0").unwrap();
        assert_eq!(v8.version(), Version8Draft);
        assert_eq!(v8.variant(), RFC4122);
        assert_eq!("320C3D4D-CC00-875B-8EC9-32D5F69181C0", v8.uuid_upper());
    }

    #[test]
    fn test_versions() {}
}