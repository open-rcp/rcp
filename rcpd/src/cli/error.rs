//! CLI error types
//!
//! This module defines CLI-specific error types.

#[cfg(feature = "cli")]
use thiserror::Error;

/// CLI-specific error types
#[cfg(feature = "cli")]
#[derive(Debug, Error)]
pub enum CliError {
    /// Error communicating with the daemon
    #[error("Communication error: {0}")]
    CommunicationError(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    /// Authorization error
    #[error("Authorization error: {0}")]
    AuthorizationError(String),

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// File system error
    #[error("File system error: {0}")]
    FileSystemError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Command execution error
    #[error("Command execution error: {0}")]
    CommandExecutionError(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Other error
    #[error("Error: {0}")]
    Other(String),
}

#[cfg(feature = "cli")]
impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::FileSystemError(err.to_string())
    }
}

#[cfg(feature = "cli")]
impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> Self {
        CliError::SerializationError(err.to_string())
    }
}

#[cfg(feature = "cli")]
impl From<anyhow::Error> for CliError {
    fn from(err: anyhow::Error) -> Self {
        CliError::Other(err.to_string())
    }
}
