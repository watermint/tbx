use std::borrow::Cow;
use crate::number::random::{Generator, Random};
use crate::text::uuid::{Layout, UUID};

/// Create new UUID version 4 (randomly generated UUID) with given random generator.
pub fn new_with_rand(r: &mut Random) -> UUID {
    let mut data: [u8; 16] = [0; 16];

    for i in 0..16 {
        data[i] = r.next_u8();
    }
    data[6] = (data[6] & 0x0f) | 0x40; // Version 4
    data[8] = (data[8] & 0x3f) | 0x80; // RFC 4122 Variant

    UUID::new(data)
}

/// Create new UUID version 4 (randomly generated UUID) by default random generator.
pub fn new() -> UUID {
    new_with_rand(&mut Random::new_thread_local())
}


/// Create new UUID version 4 string.
pub fn new_str<'a>() -> Cow<'a, str> {
    new().uuid_lower()
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use crate::text::uuid::{Layout, UUID, Variant, Version};
    use crate::text::uuid::v4::{new, new_str};

    #[test]
    fn test_v4() {
        let v4 = new();

        assert_eq!(v4.variant(), Variant::RFC4122);
        assert_eq!(v4.version(), Version::Version4);

        let v4b = new();

        assert_ne!(v4, v4b);

        let v4s = new_str();
        let v4p = UUID::parse(v4s.borrow()).unwrap();

        assert_eq!(v4s.borrow(), v4p.uuid_lower());
        assert_eq!(v4p.variant(), Variant::RFC4122);
        assert_eq!(v4p.version(), Version::Version4);
    }
}