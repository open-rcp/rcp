use thiserror::Error;
use std::io;

/// Result type for RCP server operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types that can occur in RCP server operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
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
    
    #[error("Application error: {0}")]
    Application(String),
    
    #[error("Service error: {0}")]
    Service(String),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("{0}")]
    Other(String),
}