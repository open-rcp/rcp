use crate::{Error, Result, PROTOCOL_VERSION};
use byteorder::{ByteOrder, LittleEndian};

/// Size of the RCP header in bytes
pub const HEADER_SIZE: usize = 8;

/// RCP Protocol Header
///
/// Every RCP frame begins with a header containing:
/// - Version (1 byte): Always 0x01 for RCP/1.0
/// - Command ID (1 byte): Identifies the frame type
/// - Payload Size (4 bytes): Length of data following the header in little-endian
/// - Flags (2 bytes): Reserved for future use (compression, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    /// Protocol version
    pub version: u8,
    
    /// Command identifier
    pub command: u8,
    
    /// Length of the payload in bytes
    pub payload_size: u32,
    
    /// Flags for additional options
    pub flags: u16,
}

impl Header {
    /// Create a new header with the given command ID and payload size.
    pub fn new(command: u8, payload_size: u32) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            command,
            payload_size,
            flags: 0,
        }
    }
    
    /// Create a new header with flags.
    pub fn with_flags(command: u8, payload_size: u32, flags: u16) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            command,
            payload_size,
            flags,
        }
    }
    
    /// Parse a header from a byte slice.
    pub fn parse(input: &[u8]) -> Result<Self> {
        if input.len() < HEADER_SIZE {
            return Err(Error::InvalidHeader);
        }
        
        let version = input[0];
        if version != PROTOCOL_VERSION {
            return Err(Error::UnsupportedVersion(version));
        }
        
        let command = input[1];
        let payload_size = LittleEndian::read_u32(&input[2..6]);
        let flags = LittleEndian::read_u16(&input[6..8]);
        
        Ok(Self {
            version,
            command,
            payload_size,
            flags,
        })
    }
    
    /// Serialize the header to bytes.
    pub fn serialize(&self) -> [u8; HEADER_SIZE] {
        let mut buffer = [0u8; HEADER_SIZE];
        
        buffer[0] = self.version;
        buffer[1] = self.command;
        LittleEndian::write_u32(&mut buffer[2..6], self.payload_size);
        LittleEndian::write_u16(&mut buffer[6..8], self.flags);
        
        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_header_serialization() {
        let header = Header::new(0x02, 1024);
        let bytes = header.serialize();
        let parsed = Header::parse(&bytes).unwrap();
        
        assert_eq!(header, parsed);
        assert_eq!(parsed.version, PROTOCOL_VERSION);
        assert_eq!(parsed.command, 0x02);
        assert_eq!(parsed.payload_size, 1024);
        assert_eq!(parsed.flags, 0);
    }
    
    #[test]
    fn test_header_with_flags() {
        let header = Header::with_flags(0x03, 512, 0x0001);
        let bytes = header.serialize();
        let parsed = Header::parse(&bytes).unwrap();
        
        assert_eq!(header, parsed);
        assert_eq!(parsed.version, PROTOCOL_VERSION);
        assert_eq!(parsed.command, 0x03);
        assert_eq!(parsed.payload_size, 512);
        assert_eq!(parsed.flags, 0x0001);
    }
    
    #[test]
    fn test_invalid_header() {
        let too_small = [0u8; 7];
        assert!(Header::parse(&too_small).is_err());
    }
    
    #[test]
    fn test_unsupported_version() {
        let mut invalid_version = [0u8; HEADER_SIZE];
        invalid_version[0] = 0xFF; // Invalid version
        
        match Header::parse(&invalid_version) {
            Err(Error::UnsupportedVersion(v)) => assert_eq!(v, 0xFF),
            _ => panic!("Expected UnsupportedVersion error"),
        }
    }
}