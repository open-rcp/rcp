use std::io;

// Import the error module
#[path = "../src/error.rs"]
mod error;
use error::ServiceError;

#[test]
fn test_io_error_conversion() {
    // Create an IO error
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");

    // Convert to ServiceError
    let service_err: ServiceError = io_err.into();

    // Verify error conversion through Display trait
    let error_message = format!("{}", service_err);
    assert!(error_message.contains("IO error"));
    assert!(error_message.contains("file not found"));
}

#[test]
fn test_config_error_creation() {
    // Create a config error
    let config_err = ServiceError::Config("Invalid configuration".to_string());

    // Verify error message through Display trait
    let error_message = format!("{}", config_err);
    assert!(error_message.contains("Config error"));
    assert!(error_message.contains("Invalid configuration"));
}

#[test]
fn test_service_error_creation() {
    // Create a service error
    let service_err = ServiceError::Service("Service failed to start".to_string());

    // Verify error message through Display trait
    let error_message = format!("{}", service_err);
    assert!(error_message.contains("Service error"));
    assert!(error_message.contains("Service failed to start"));
}

#[test]
fn test_error_debug() {
    // Create an error
    let err = ServiceError::Service("Test error".to_string());

    // Verify Debug trait implementation works
    let debug_str = format!("{:?}", err);
    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("Service"));
}
