use crate::{Error, Result};
use rand::{rngs::OsRng, Rng};

/// Generate random bytes of the specified length
pub fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; length];
    OsRng.fill(&mut bytes[..]);
    bytes
}

/// Convert a byte slice to a hexadecimal string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

/// Convert a hexadecimal string to a byte vector
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return Err(Error::Other("Invalid hex string length".to_string()));
    }

    let mut bytes = Vec::with_capacity(hex.len() / 2);
    let mut chars = hex.chars();

    while let (Some(a), Some(b)) = (chars.next(), chars.next()) {
        let high = char_to_nibble(a)?;
        let low = char_to_nibble(b)?;
        bytes.push((high << 4) | low);
    }

    Ok(bytes)
}

/// Convert a character to its nibble value
fn char_to_nibble(c: char) -> Result<u8> {
    match c {
        '0'..='9' => Ok(c as u8 - b'0'),
        'a'..='f' => Ok(c as u8 - b'a' + 10),
        'A'..='F' => Ok(c as u8 - b'A' + 10),
        _ => Err(Error::Other(format!("Invalid hex character: {}", c))),
    }
}

/// Pad a buffer to a multiple of the specified block size
pub fn pad_to_block_size(data: &[u8], block_size: usize) -> Vec<u8> {
    let remainder = data.len() % block_size;
    if remainder == 0 {
        return data.to_vec();
    }

    let padding_needed = block_size - remainder;
    let mut padded = Vec::with_capacity(data.len() + padding_needed);
    padded.extend_from_slice(data);
    padded.extend(std::iter::repeat_n(0, padding_needed));
    padded
}

/// Convert a struct to bytes using bincode serialization
pub fn to_bytes<T: serde::Serialize>(value: &T) -> Result<Vec<u8>> {
    bincode::serialize(value).map_err(Error::SerializationError)
}

/// Convert bytes to a struct using bincode deserialization
pub fn from_bytes<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    bincode::deserialize(bytes).map_err(Error::SerializationError)
}

/// Get current time in seconds since UNIX epoch
pub fn current_time_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_hex() {
        let bytes = vec![0x12, 0xAB, 0xCD, 0xEF];
        assert_eq!(bytes_to_hex(&bytes), "12abcdef");
    }

    #[test]
    fn test_hex_to_bytes() {
        let hex = "12AbCdEf";
        assert_eq!(hex_to_bytes(hex).unwrap(), vec![0x12, 0xAB, 0xCD, 0xEF]);
    }

    #[test]
    fn test_invalid_hex_string() {
        assert!(hex_to_bytes("invalid").is_err());
        assert!(hex_to_bytes("123").is_err()); // Odd length
    }

    #[test]
    fn test_pad_to_block_size() {
        let data = vec![1, 2, 3, 4, 5];
        let padded = pad_to_block_size(&data, 8);
        assert_eq!(padded.len(), 8);
        assert_eq!(&padded[0..5], &[1, 2, 3, 4, 5]);
        assert_eq!(&padded[5..], &[0, 0, 0]);
    }

    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct TestStruct {
        field1: u32,
        field2: String,
    }

    #[test]
    fn test_bincode_serialization() {
        let test_struct = TestStruct {
            field1: 42,
            field2: "test".to_string(),
        };

        let bytes = to_bytes(&test_struct).unwrap();
        let deserialized: TestStruct = from_bytes(&bytes).unwrap();

        assert_eq!(test_struct, deserialized);
    }
}
