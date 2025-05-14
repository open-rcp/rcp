use rcpc::{Client, ClientState};
use rcpp::AuthMethod;
use tokio::test;
use uuid::Uuid;

/// Test client builder with default values
#[test]
async fn test_client_builder_defaults() {
    let client = Client::builder().build();

    // Test that state is initially disconnected
    assert_eq!(client.state().await, ClientState::Disconnected);

    // Since we can't easily inspect private fields, we'll rely on behavior testing
    // or any public accessors in a real implementation
}

/// Test client builder with custom configuration
#[test]
async fn test_client_builder_custom() {
    let client_id = Uuid::new_v4();
    let client = Client::builder()
        .host("test-host")
        .port(12345)
        .client_name("Test Client")
        .client_id(client_id)
        .auth_method(AuthMethod::PreSharedKey)
        .auth_psk("test-psk")
        .auto_reconnect(false)
        .reconnect_delay(500)
        .keep_alive_interval(60)
        .connection_timeout(15)
        .build();

    assert_eq!(client.state().await, ClientState::Disconnected);

    // Note: In a real test with access to the struct fields, we could verify each value was set correctly
}

/// Test client builder with connection string
#[test]
async fn test_client_builder_connection_string() {
    // Build client from connection string
    let client = Client::builder()
        .connection_string("rcp://user:pass@example.com:8888")
        .unwrap()
        .build();

    assert_eq!(client.state().await, ClientState::Disconnected);

    // Note: In a real test with access to the struct fields, we could verify each value from
    // the connection string was set correctly (host, port, etc.)
}

/// Test client state transitions
/// This is a more complex test that would ideally use a mock server
/// For now, we just validate that connect attempts fail as expected with no server
#[test]
async fn test_client_connection_failure() {
    let client = Client::builder()
        .host("non-existent-host") // This host doesn't exist
        .connection_timeout(1) // Short timeout for faster test
        .build();

    // Attempt to connect should fail since there's no server
    let result = client.connect().await;
    assert!(result.is_err());

    // State should be Disconnected
    assert_eq!(client.state().await, ClientState::Disconnected);
}
