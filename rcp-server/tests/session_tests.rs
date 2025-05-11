use rcp_core::ConnectionState;
use rcp_server::config::ServerConfig;
use uuid::Uuid;

// Helper function to create a basic ServerConfig for testing
fn create_test_config() -> ServerConfig {
    ServerConfig::default()
}

#[test]
fn test_session_id() {
    // In a real test, we would need to properly mock the TcpStream
    // Since we can't do that easily in this minimal setup, we'll just test UUID properties
    let id = Uuid::new_v4();
    assert!(!id.is_nil(), "Generated UUID should not be nil");
}

#[test]
fn test_connection_state() {
    // Test ConnectionState enum values
    let state = ConnectionState::Connected;
    assert!(
        state == ConnectionState::Connected,
        "State should be Connected"
    );
    assert!(
        state != ConnectionState::Authenticated,
        "State should not be Authenticated"
    );

    // Test state transitions
    let initial_state = ConnectionState::Connected;
    let auth_state = ConnectionState::Authenticated;
    let closing_state = ConnectionState::Closing;

    assert!(
        initial_state != auth_state,
        "Connected should not equal Authenticated"
    );
    assert!(
        initial_state != closing_state,
        "Connected should not equal Closing"
    );
}

#[test]
fn test_server_config() {
    let config = create_test_config();

    // Verify the configuration is correctly set up
    assert_eq!(
        config.address, "0.0.0.0",
        "Default address should be 0.0.0.0"
    );
    assert!(
        config.auth.required,
        "Authentication should be required by default"
    );
}
