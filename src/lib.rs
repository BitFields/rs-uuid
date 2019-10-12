use hex::encode;
use rand::Rng;

pub mod iso {
    use super::encode;
    use super::Rng;

    pub fn uuid_v4() -> String {
        let bytes = (
            encode(rand::thread_rng().gen::<[u8; 4]>()),
            encode(rand::thread_rng().gen::<[u8; 2]>()),
            encode(rand::thread_rng().gen::<[u8; 2]>()),
            encode(rand::thread_rng().gen::<[u8; 2]>()),
            encode(rand::thread_rng().gen::<[u8; 6]>()),
        );

        format!(
            "{}-{}-4{}-{}-{}",
            bytes.0,
            bytes.1,
            &bytes.2[1..],
            bytes.3,
            bytes.4
        )
    }
}

pub const NULL_UUID: &str = "00000000-0000-0000-0000-000000000000";

/// Generates 8-byte UUID as a String
pub fn uuid8() -> String {
    let bytes = (
        encode(rand::thread_rng().gen::<[u8; 2]>()),
        encode(rand::thread_rng().gen::<[u8; 1]>()),
        encode(rand::thread_rng().gen::<[u8; 1]>()),
        encode(rand::thread_rng().gen::<[u8; 1]>()),
        encode(rand::thread_rng().gen::<[u8; 3]>()),
    );

    format!(
        "{}-{}-{}-{}-{}",
        bytes.0, bytes.1, bytes.2, bytes.3, bytes.4
    )
}

/// Generates 16-byte UUID as a String
pub fn uuid16() -> String {
    let bytes = (
        encode(rand::thread_rng().gen::<[u8; 4]>()),
        encode(rand::thread_rng().gen::<[u8; 2]>()),
        encode(rand::thread_rng().gen::<[u8; 2]>()),
        encode(rand::thread_rng().gen::<[u8; 2]>()),
        encode(rand::thread_rng().gen::<[u8; 6]>()),
    );

    format!(
        "{}-{}-{}-{}-{}",
        bytes.0, bytes.1, bytes.2, bytes.3, bytes.4
    )
}

/// Generates 32-byte UUID as a String
pub fn uuid32() -> String {
    let bytes = (
        encode(rand::thread_rng().gen::<[u8; 8]>()),
        encode(rand::thread_rng().gen::<[u8; 4]>()),
        encode(rand::thread_rng().gen::<[u8; 4]>()),
        encode(rand::thread_rng().gen::<[u8; 4]>()),
        encode(rand::thread_rng().gen::<[u8; 12]>()),
    );

    format!(
        "{}-{}-{}-{}-{}",
        bytes.0, bytes.1, bytes.2, bytes.3, bytes.4
    )
}

#[cfg(test)]
#[test]
fn test_uuid8() {
    let id: String = uuid8();

    // 8 * 2 bytes + 4 dashes
    assert_eq!(id.len(), 20);
}

#[test]
fn test_uuid16() {
    let id: String = uuid16();

    // 16 * 2 bytes + 4 dashes
    assert_eq!(id.len(), 36);
}

#[test]
fn test_uuid32() {
    let id: String = uuid32();

    // 32 * 2 bytes + 4 dashes
    assert_eq!(id.len(), 68);
}

#[test]
fn test_iso_uuid_v4() {
    use self::iso::uuid_v4;

    let iso_v4_uuid = uuid_v4();

    assert_eq!(iso_v4_uuid.len(), 36);
    assert_eq!(iso_v4_uuid.as_bytes()[14] as char, '4');
}
