use std::io;

// Import the error module
#[path = "../src/error.rs"]
mod error;
use error::Error;

#[test]
fn test_io_error_conversion() {
    // Create an IO error
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");

    // Convert to WebSocket Bridge Error
    let bridge_err: Error = io_err.into();

    // Verify error conversion through Display trait
    let error_message = format!("{}", bridge_err);
    assert!(error_message.contains("IO error"));
    assert!(error_message.contains("file not found"));
}

#[test]
fn test_websocket_error_conversion() {
    // Create a WebSocket error
    let ws_err = tokio_tungstenite::tungstenite::Error::AlreadyClosed;

    // Convert to WebSocket Bridge Error
    let bridge_err: Error = ws_err.into();

    // Verify error conversion through Display trait
    let error_message = format!("{}", bridge_err);
    assert!(error_message.contains("WebSocket error"));
}

#[test]
fn test_json_error_conversion() {
    // Create a JSON error
    let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();

    // Convert to WebSocket Bridge Error
    let bridge_err: Error = json_err.into();

    // Verify error conversion through Display trait
    let error_message = format!("{}", bridge_err);
    assert!(error_message.contains("JSON serialization error"));
}

#[test]
fn test_authentication_error_creation() {
    // Create an authentication error
    let auth_err = Error::Authentication("Invalid token".to_string());

    // Verify error message through Display trait
    let error_message = format!("{}", auth_err);
    assert!(error_message.contains("Authentication error"));
    assert!(error_message.contains("Invalid token"));
}

#[test]
fn test_connection_error_creation() {
    // Create a connection error
    let conn_err = Error::Connection("Connection failed".to_string());

    // Verify error message through Display trait
    let error_message = format!("{}", conn_err);
    assert!(error_message.contains("Connection error"));
    assert!(error_message.contains("Connection failed"));
}

#[test]
fn test_error_debug() {
    // Create an error
    let err = Error::Connection("Test error".to_string());

    // Verify Debug trait implementation works
    let debug_str = format!("{:?}", err);
    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("Connection"));
}
