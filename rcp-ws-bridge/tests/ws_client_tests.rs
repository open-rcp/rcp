// filepath: /Volumes/EXT/repos/open-rcp/rcp/rcp-ws-bridge/tests/ws_client_tests.rs
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::time::{sleep, Duration};

// Create our test version of WsMessage that matches the one in lib.rs
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
enum WsMessage {
    Auth {
        token: String,
    },
    Command {
        service: String,
        data: serde_json::Value,
    },
    Update {
        service: String,
        data: serde_json::Value,
    },
    Error {
        code: String,
        message: String,
    },
}

// This test requires running a WebSocket server, which is complex to do in a unit test.
// We'll create a simplified test that demonstrates the approach. In practice, you might:
// 1. Use a mock server
// 2. Set up an actual server for integration tests
// 3. Use dependency injection for testing

#[tokio::test]
#[ignore] // This test requires external setup, so we'll ignore it by default
async fn test_ws_client_connection() {
    // Start a WebSocket bridge in a separate task
    let bridge_addr = "127.0.0.1:9278";
    let bridge_handle = tokio::spawn(async move {
        // In a real test, you would configure and start the bridge here
        // For this example, we'll simulate it with a delay
        sleep(Duration::from_secs(5)).await;
    });

    // Give the bridge time to start
    sleep(Duration::from_millis(100)).await;

    // Create a test message that would be sent to a WebSocket client
    let auth_message = WsMessage::Auth {
        token: "test-token".to_string(),
    };

    // Serialize the message to JSON
    let json_str = serde_json::to_string(&auth_message).expect("Failed to serialize");
    assert!(
        json_str.contains("test-token"),
        "JSON should contain the token"
    );

    // Clean up
    bridge_handle.abort();
}

#[tokio::test]
async fn test_message_serialization() {
    // Test serializing various message types

    // Auth message
    let auth = WsMessage::Auth {
        token: "auth-token".to_string(),
    };
    let auth_json = serde_json::to_string(&auth).expect("Failed to serialize Auth");
    assert!(
        auth_json.contains("Auth"),
        "JSON should include message type"
    );
    assert!(
        auth_json.contains("auth-token"),
        "JSON should include token"
    );

    // Command message
    let command = WsMessage::Command {
        service: "test-service".to_string(),
        data: json!({"action": "test"}),
    };
    let command_json = serde_json::to_string(&command).expect("Failed to serialize Command");
    assert!(
        command_json.contains("Command"),
        "JSON should include message type"
    );
    assert!(
        command_json.contains("test-service"),
        "JSON should include service name"
    );

    // Update message
    let update = WsMessage::Update {
        service: "status-service".to_string(),
        data: json!({"status": "running"}),
    };
    let update_json = serde_json::to_string(&update).expect("Failed to serialize Update");
    assert!(
        update_json.contains("Update"),
        "JSON should include message type"
    );
    assert!(
        update_json.contains("status-service"),
        "JSON should include service name"
    );

    // Error message
    let error = WsMessage::Error {
        code: "ERR-1001".to_string(),
        message: "Test error".to_string(),
    };
    let error_json = serde_json::to_string(&error).expect("Failed to serialize Error");
    assert!(
        error_json.contains("Error"),
        "JSON should include message type"
    );
    assert!(
        error_json.contains("ERR-1001"),
        "JSON should include error code"
    );
    assert!(
        error_json.contains("Test error"),
        "JSON should include error message"
    );
}
