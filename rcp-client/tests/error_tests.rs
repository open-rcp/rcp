use rcp_client::{Error, Result};
use std::io;
use tokio::test;

/// Test error conversion from io::Error
#[test]
async fn test_from_io_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let rcp_error: Error = io_error.into();
    
    match rcp_error {
        Error::IO(_) => (), // Successfully converted
        _ => panic!("Expected IO error variant"),
    }
}

/// Test error conversion from rcp_core::Error
#[test]
async fn test_from_core_error() {
    let core_error = rcp_core::Error::InvalidPayload;
    let rcp_error: Error = core_error.into();
    
    match rcp_error {
        Error::Core(_) => (), // Successfully converted
        _ => panic!("Expected Core error variant"),
    }
}

/// Test result type with error
#[test]
async fn test_result_with_error() {
    let result: Result<()> = Err(Error::Connection("test connection error".to_string()));
    assert!(result.is_err());
    
    match result {
        Err(Error::Connection(msg)) => assert_eq!(msg, "test connection error"),
        _ => panic!("Expected Connection error with correct message"),
    }
}

/// Test Display trait implementation for Error
#[test]
async fn test_error_display() {
    let error = Error::Auth("authentication failed".to_string());
    let display_string = format!("{}", error);
    
    assert_eq!(display_string, "Authentication error: authentication failed");
}

/// Test Debug trait implementation for Error
#[test]
async fn test_error_debug() {
    let error = Error::Timeout("operation timed out".to_string());
    let debug_string = format!("{:?}", error);
    
    // Just check that it contains something sensible
    assert!(debug_string.contains("Timeout"));
    assert!(debug_string.contains("operation timed out"));
}
