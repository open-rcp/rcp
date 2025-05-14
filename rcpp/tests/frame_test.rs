// filepath: /Volumes/EXT/repos/open-rcp/rcp/rcpp/tests/frame_test.rs
use rcpp::{CommandId, Frame};

#[test]
fn test_frame_creation() {
    // Test creating a frame with a command ID and empty payload
    let command_id = CommandId::Ping as u8;
    let frame = Frame::new(command_id, Vec::new());

    // Verify the command ID matches
    assert_eq!(frame.header.command, command_id);
}

#[test]
fn test_frame_serialization() {
    // Test serializing a frame to bytes
    let command_id = CommandId::Ping as u8;
    let frame = Frame::new(command_id, Vec::new());

    // Serialize the frame
    let bytes = frame.serialize();

    // Ensure bytes contain at least the header
    assert!(bytes.len() >= 8); // Header is at least 8 bytes
}
