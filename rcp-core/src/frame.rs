use crate::{Error, Header, Result};
use bytes::{Buf, BytesMut};

/// Represents a complete RCP protocol frame with header and payload
#[derive(Debug, Clone)]
pub struct Frame {
    /// Frame header
    pub header: Header,
    
    /// Frame payload
    pub payload: Vec<u8>,
}

impl Frame {
    /// Create a new frame with the given command ID and payload
    pub fn new(command: u8, payload: Vec<u8>) -> Self {
        let header = Header::new(command, payload.len() as u32);
        Self { header, payload }
    }
    
    /// Create a new frame with the given header and payload
    pub fn with_header(header: Header, payload: Vec<u8>) -> Self {
        Self { header, payload }
    }
    
    /// Serialize the frame to a byte vector
    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.header.serialize().len() + self.payload.len());
        bytes.extend_from_slice(&self.header.serialize());
        bytes.extend_from_slice(&self.payload);
        bytes
    }
    
    /// Parse a frame from a buffer
    pub fn parse(buffer: &mut BytesMut) -> Result<Option<Self>> {
        // Ensure we have enough bytes for the header
        if buffer.len() < crate::header::HEADER_SIZE {
            return Ok(None);
        }
        
        // Parse the header but don't consume it yet
        let header = Header::parse(buffer.chunk())?;
        
        // Check if we have the full payload
        let frame_size = crate::header::HEADER_SIZE + header.payload_size as usize;
        if buffer.len() < frame_size {
            return Ok(None);
        }
        
        // Now we can consume the header and payload
        let _ = buffer.advance(crate::header::HEADER_SIZE);
        let payload = buffer.split_to(header.payload_size as usize).to_vec();
        
        Ok(Some(Self { header, payload }))
    }
    
    /// Get a reference to the payload as a byte slice
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    
    /// Take ownership of the payload
    pub fn into_payload(self) -> Vec<u8> {
        self.payload
    }
    
    /// Get the command ID from the header
    pub fn command_id(&self) -> u8 {
        self.header.command
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;
    
    #[test]
    fn test_frame_serialization() {
        let payload = b"Hello, RCP!".to_vec();
        let frame = Frame::new(0x01, payload.clone());
        let serialized = frame.serialize();
        
        let mut buffer = BytesMut::from(&serialized[..]);
        let parsed = Frame::parse(&mut buffer).unwrap().unwrap();
        
        assert_eq!(parsed.command_id(), 0x01);
        assert_eq!(parsed.payload(), payload);
        assert_eq!(parsed.header.payload_size as usize, payload.len());
    }
    
    #[test]
    fn test_incomplete_frame() {
        let payload = b"Hello, RCP!".to_vec();
        let frame = Frame::new(0x01, payload);
        let serialized = frame.serialize();
        
        // Only provide part of the frame
        let partial_length = serialized.len() - 5;
        let mut buffer = BytesMut::from(&serialized[..partial_length]);
        
        // Should return None as the frame is incomplete
        assert!(Frame::parse(&mut buffer).unwrap().is_none());
    }
    
    #[test]
    fn test_multiple_frames() {
        let frame1 = Frame::new(0x01, b"Frame1".to_vec());
        let frame2 = Frame::new(0x02, b"Frame2".to_vec());
        
        let mut buffer = BytesMut::new();
        buffer.extend_from_slice(&frame1.serialize());
        buffer.extend_from_slice(&frame2.serialize());
        
        // Should be able to parse both frames
        let parsed1 = Frame::parse(&mut buffer).unwrap().unwrap();
        assert_eq!(parsed1.command_id(), 0x01);
        assert_eq!(parsed1.payload(), b"Frame1");
        
        let parsed2 = Frame::parse(&mut buffer).unwrap().unwrap();
        assert_eq!(parsed2.command_id(), 0x02);
        assert_eq!(parsed2.payload(), b"Frame2");
        
        // Buffer should be empty now
        assert!(buffer.is_empty());
    }
}