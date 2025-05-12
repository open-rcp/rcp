use rcp_cli::CliError;
use std::io;
use tokio::test;
use anyhow;

/// Test timeout error
#[test]
async fn test_timeout_error() {
    let error = CliError::Timeout;
    
    // Verify error display
    assert_eq!(error.to_string(), "Operation timed out");
    
    // Verify error compatibility with anyhow
    let anyhow_error = anyhow::Error::from(error);
    assert!(anyhow_error.to_string().contains("Operation timed out"));
}

/// Test service error
#[test]
async fn test_service_error() {
    let error = CliError::ServiceError("Service failed to start".to_string());
    
    // Verify error display
    assert_eq!(error.to_string(), "Service error: Service failed to start");
    
    // Verify error compatibility with anyhow
    let anyhow_error = anyhow::Error::from(error);
    assert!(anyhow_error.to_string().contains("Service error"));
    assert!(anyhow_error.to_string().contains("Service failed to start"));
}

/// Test config error
#[test]
async fn test_config_error() {
    let error = CliError::ConfigError("Failed to parse config".to_string());
    
    // Verify error display
    assert_eq!(error.to_string(), "Configuration error: Failed to parse config");
    
    // Verify error compatibility with anyhow
    let anyhow_error = anyhow::Error::from(error);
    assert!(anyhow_error.to_string().contains("Configuration error"));
    assert!(anyhow_error.to_string().contains("Failed to parse config"));
}

/// Test from IO error
#[test]
async fn test_from_io_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let error = CliError::from(io_error);
    
    // Verify conversion to CliError
    match error {
        CliError::IoError(_) => {}
        _ => panic!("Expected IoError variant"),
    }
    
    // Verify error display
    assert!(error.to_string().contains("IO error"));
    assert!(error.to_string().contains("File not found"));
}
