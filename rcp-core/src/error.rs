use std::io;
use thiserror::Error;

/// Result type for RCP operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types that can occur in RCP operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid protocol header")]
    InvalidHeader,

    #[error("Invalid protocol payload")]
    InvalidPayload,

    #[error("Unsupported protocol version: {0}")]
    UnsupportedVersion(u8),

    #[error("Invalid command ID: {0}")]
    InvalidCommand(u8),

    #[error("Authentication error: {0}")]
    AuthenticationFailed(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("Operation timed out")]
    Timeout,

    #[error("Connection closed")]
    ConnectionClosed,

    #[error("Invalid encryption key")]
    InvalidEncryptionKey,

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Unexpected message type")]
    UnexpectedMessageType,

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Operation not supported")]
    NotSupported,

    #[error("{0}")]
    Other(String),
}
