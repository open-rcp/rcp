use std::io;
use thiserror::Error;

/// Error types specific to the CLI
#[derive(Error, Debug)]
pub enum CliError {
    #[error("Not connected to RCP service")]
    NotConnected,

    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Service error: {0}")]
    ServiceError(String),

    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("Command failed: {0}")]
    CommandFailed(String),

    #[error("Operation timed out")]
    Timeout,

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

// Implement From for other error types
impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> Self {
        CliError::ConfigError(err.to_string())
    }
}

impl From<toml::de::Error> for CliError {
    fn from(err: toml::de::Error) -> Self {
        CliError::ConfigError(err.to_string())
    }
}

impl From<toml::ser::Error> for CliError {
    fn from(err: toml::ser::Error) -> Self {
        CliError::ConfigError(err.to_string())
    }
}

/// Type alias for anyhow::Result with our error type
pub type Result<T> = std::result::Result<T, CliError>;
