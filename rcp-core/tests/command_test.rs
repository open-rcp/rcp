// filepath: /Volumes/EXT/repos/open-rcp/rcp/rcp-core/tests/command_test.rs
use rcp_core::CommandId;

#[test]
fn test_command_id_constants() {
    // Test that command IDs are properly defined with distinct values
    assert_ne!(CommandId::LaunchApp as u8, CommandId::Ping as u8);
    assert_ne!(CommandId::Auth as u8, CommandId::Heartbeat as u8);
    
    // Test conversion to u8
    assert_eq!(CommandId::LaunchApp as u8, 0x01);
    assert_eq!(CommandId::Heartbeat as u8, 0xFF);
}

