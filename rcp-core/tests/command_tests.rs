use rcp_core::CommandId;

#[test]
fn test_command_id_constants() {
    // Test that command IDs have the expected values
    assert_eq!(CommandId::LaunchApp as u8, 0x01);
    assert_eq!(CommandId::Heartbeat as u8, 0xFF);

    // Test they are distinct
    assert_ne!(CommandId::LaunchApp as u8, CommandId::Ping as u8);
    assert_ne!(CommandId::Auth as u8, CommandId::Heartbeat as u8);
}

// Add more tests as needed once we understand the actual structure and behavior
// of the Command trait and its implementations
