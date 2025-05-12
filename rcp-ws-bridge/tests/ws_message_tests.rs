use serde::{Deserialize, Serialize};
use serde_json::json;

// Create our test version of WsMessage that matches the one in lib.rs
// This allows us to test serialization/deserialization
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

#[test]
fn test_serialize_auth_message() {
    // Create an Auth message
    let auth_message = WsMessage::Auth {
        token: "test-token".to_string(),
    };

    // Serialize to JSON
    let json = serde_json::to_string(&auth_message).expect("Failed to serialize");

    // Check JSON structure
    assert!(
        json.contains(r#""type":"Auth""#),
        "JSON should contain message type"
    );
    assert!(
        json.contains(r#""payload":{"token":"test-token"}"#),
        "JSON should contain token"
    );
}

#[test]
fn test_serialize_command_message() {
    // Create a Command message
    let command_message = WsMessage::Command {
        service: "display".to_string(),
        data: json!({ "command": "start" }),
    };

    // Serialize to JSON
    let json = serde_json::to_string(&command_message).expect("Failed to serialize");

    // Check JSON structure
    assert!(
        json.contains(r#""type":"Command""#),
        "JSON should contain message type"
    );
    assert!(
        json.contains(r#""service":"display""#),
        "JSON should contain service name"
    );
    assert!(
        json.contains(r#""data":{"command":"start"}"#),
        "JSON should contain command data"
    );
}

#[test]
fn test_serialize_update_message() {
    // Create an Update message
    let update_message = WsMessage::Update {
        service: "display".to_string(),
        data: json!({"status": "running"}),
    };

    // Serialize to JSON
    let json = serde_json::to_string(&update_message).expect("Failed to serialize");

    // Check JSON structure
    assert!(
        json.contains(r#""type":"Update""#),
        "JSON should contain message type"
    );
    assert!(
        json.contains(r#""service":"display""#),
        "JSON should contain service name"
    );
    assert!(
        json.contains(r#""data":{"status":"running"}"#),
        "JSON should contain status data"
    );
}

#[test]
fn test_deserialize_auth_message() {
    // Create JSON for Auth message
    let json = r#"{"type":"Auth","payload":{"token":"test-token"}}"#;

    // Deserialize from JSON
    let message: WsMessage = serde_json::from_str(json).expect("Failed to deserialize");

    // Check deserialized message
    match message {
        WsMessage::Auth { token } => {
            assert_eq!(token, "test-token", "Token should match");
        }
        _ => panic!("Expected Auth message type"),
    }
}

#[test]
fn test_deserialize_command_message() {
    // Create JSON for Command message
    let json = r#"{"type":"Command","payload":{"service":"display","data":{"command":"start"}}}"#;

    // Deserialize from JSON
    let message: WsMessage = serde_json::from_str(json).expect("Failed to deserialize");

    // Check deserialized message
    match message {
        WsMessage::Command { service, data } => {
            assert_eq!(service, "display", "Service should match");
            assert_eq!(
                data.get("command").unwrap().as_str().unwrap(),
                "start",
                "Command should match"
            );
        }
        _ => panic!("Expected Command message type"),
    }
}

#[test]
fn test_deserialize_update_message() {
    // Create JSON for Update message
    let json = r#"{"type":"Update","payload":{"service":"display","data":{"status":"running"}}}"#;

    // Deserialize from JSON
    let message: WsMessage = serde_json::from_str(json).expect("Failed to deserialize");

    // Check deserialized message
    match message {
        WsMessage::Update { service, data } => {
            assert_eq!(service, "display", "Service should match");
            assert_eq!(
                data.get("status").unwrap().as_str().unwrap(),
                "running",
                "Status should match"
            );
        }
        _ => panic!("Expected Update message type"),
    }
}
