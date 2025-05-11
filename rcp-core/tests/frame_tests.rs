use bytes::{BufMut, BytesMut};
use rcp_core::*;

#[test]
fn test_frame_creation() {
    // Test creating a frame with a command ID and empty payload
    let command_id = CommandId::Ping as u8;
    let frame = Frame::new(command_id, Vec::new());

    // Verify frame properties
    assert_eq!(frame.command_id(), command_id);
    assert_eq!(frame.header.command, command_id);
}

#[test]
fn test_frame_serialization() {
    // Test serializing a frame to bytes
    let command_id = CommandId::Ping as u8;
    let frame = Frame::new(command_id, Vec::new());

    // Convert to bytes
    let bytes = frame.serialize();

    // Ensure bytes are not empty
    assert!(!bytes.is_empty());
    assert!(bytes.len() >= 8); // Header is at least 8 bytes
}

#[test]
fn test_frame_deserialization() {
    // Create a frame with a LaunchApp command
    let command_id = CommandId::LaunchApp as u8;

    // Create a serialized LaunchAppCommand
    let launch_command = LaunchAppCommand {
        flags: 0,
        application_path: "test-app".to_string(),
        args: Some("--arg1 --arg2=value".to_string()),
    };

    // We would normally serialize the command here, but for the test
    // we'll just use a placeholder payload
    let payload = serde_json::to_vec(&launch_command).unwrap_or_else(|_| b"test payload".to_vec());
    let original_frame = Frame::new(command_id, payload.clone());
    let bytes = original_frame.serialize();

    // Parse bytes back into a frame
    let mut buffer = BytesMut::from(&bytes[..]);
    let parsed_frame = Frame::parse(&mut buffer).unwrap().unwrap();

    // Verify the command ID matches
    assert_eq!(parsed_frame.command_id(), command_id);
    assert_eq!(parsed_frame.payload(), payload);
}

#[test]
fn test_frame_with_large_payload() {
    // Test handling of frames with large payloads
    let mut large_data = BytesMut::with_capacity(1024 * 1024); // 1MB
    for i in 0..1024 * 1024 / 8 {
        large_data.put_u64(i as u64);
    }

    // Create a frame with a large payload
    let payload = large_data.freeze().to_vec();
    let frame = Frame::new(CommandId::StreamFrame as u8, payload.clone());

    // Serialize and deserialize the frame
    let bytes = frame.serialize();
    let mut buffer = BytesMut::from(&bytes[..]);
    let parsed_frame = Frame::parse(&mut buffer).unwrap().unwrap();

    // Verify the deserialized frame matches the original
    assert_eq!(parsed_frame.command_id(), CommandId::StreamFrame as u8);
    assert_eq!(parsed_frame.payload().len(), payload.len());
}

#[test]
fn test_frame_with_multiple_chunks() {
    // Test parsing a frame that arrives in multiple chunks
    let command_id = CommandId::Ping as u8;
    let payload = b"test-payload".to_vec();
    let frame = Frame::new(command_id, payload.clone());
    let serialized = frame.serialize();

    // Split the serialized data into header and payload parts - header size is 8 bytes
    let header_size = 8; // Use direct constant instead of accessing internal module
    let header_part = &serialized[..header_size];
    let payload_part = &serialized[header_size..];

    // Parse in chunks
    let mut buffer = BytesMut::new();

    // Add just the header first
    buffer.extend_from_slice(header_part);

    // Should not be able to parse yet because we're missing the payload
    let result = Frame::parse(&mut buffer).unwrap();
    assert!(result.is_none());

    // Now add the payload
    buffer.extend_from_slice(payload_part);

    // Now we should be able to parse the complete frame
    let parsed_frame = Frame::parse(&mut buffer).unwrap().unwrap();
    assert_eq!(parsed_frame.command_id(), command_id);
    assert_eq!(parsed_frame.payload(), payload);
}

#[test]
fn test_invalid_frame_header() {
    // Test handling of invalid frame headers
    let mut invalid_header = BytesMut::with_capacity(8);

    // Create an invalid header with invalid protocol version
    invalid_header.put_u8(0xFF); // Invalid protocol version (should be 0x01)
    invalid_header.put_u8(0x01); // Command ID
    invalid_header.put_u16(0); // Reserved bytes
    invalid_header.put_u32(10); // Payload size

    // Add some payload data to meet the size requirements
    invalid_header.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    // Attempt to parse the invalid header
    let result = Frame::parse(&mut invalid_header);

    // Should result in an error because of invalid protocol version
    assert!(result.is_err());
}
