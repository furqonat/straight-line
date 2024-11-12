use rand::Rng;

pub fn uuid_v4() -> String {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 16];

    // Fill the array with random bytes
    rng.fill(&mut bytes);

    // Set the version to 4 (UUIDv4)
    bytes[6] = (bytes[6] & 0x0f) | 0x40;

    // Set the variant to 1 (RFC4122 variant)
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    // Format the UUID in the standard 8-4-4-4-12 format
    format!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), // 4 bytes for the first group
        u16::from_be_bytes([bytes[4], bytes[5]]), // 2 bytes for the second group
        u16::from_be_bytes([bytes[6], bytes[7]]), // 2 bytes for the third group
        u16::from_be_bytes([bytes[8], bytes[9]]), // 2 bytes for the fourth group
        u64::from_be_bytes([
            0, 0, bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
        ]), // 8 bytes, padded with two leading zeros
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_uuid_v4_length() {
        let uuid = uuid_v4();
        // The UUID should be 36 characters long (32 hex characters + 4 dashes)
        assert_eq!(uuid.len(), 36, "UUID length is incorrect");
    }

    #[test]
    fn test_generate_uuid_v4_format() {
        let uuid = uuid_v4();

        // Ensure the UUID has the correct hyphen placement
        assert_eq!(&uuid[8..9], "-", "First dash is misplaced");
        assert_eq!(&uuid[13..14], "-", "Second dash is misplaced");
        assert_eq!(&uuid[18..19], "-", "Third dash is misplaced");
        assert_eq!(&uuid[23..24], "-", "Fourth dash is misplaced");
    }

    #[test]
    fn test_generate_uuid_v4_version() {
        let uuid = uuid_v4();

        // The version number should be '4', which is the 13th character (index 12)
        assert_eq!(&uuid[14..15], "4", "UUID version is not 4");
    }

    #[test]
    fn test_generate_uuid_v4_variant() {
        let uuid = uuid_v4();

        // The variant is defined by the 17th character (index 16).
        // It must be one of the following: 8, 9, a, or b.
        let variant_char = &uuid[19..20];
        assert!(
            variant_char == "8"
                || variant_char == "9"
                || variant_char == "a"
                || variant_char == "b",
            "UUID variant is not RFC4122 compliant"
        );
    }
}
