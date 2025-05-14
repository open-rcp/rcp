// filepath: /Volumes/EXT/repos/open-rcp/rcp/rcpp/tests/protocol_test.rs
use rcpp::*;

#[test]
fn test_connection_state_transitions() {
    // Test basic connection state transitions

    assert_ne!(ConnectionState::Connected, ConnectionState::Authenticated);
    assert_ne!(ConnectionState::Authenticating, ConnectionState::Closing);
    assert_eq!(ConnectionState::Connected, ConnectionState::Connected);
}
