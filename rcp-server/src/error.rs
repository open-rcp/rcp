use std::io;
use thiserror::Error;

/// Result type for RCP server operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for RCP server
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Core protocol error: {0}")]
    Core(#[from] rcp_core::Error),

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("TLS error: {0}")]
    Tls(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Session error: {0}")]
    Session(String),

    #[error("Operation not permitted: {0}")]
    PermissionDenied(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),
    
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Application error: {0}")]
    Application(String),

    #[error("Service error: {0}")]
    Service(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("{0}")]
    Other(String),
}
