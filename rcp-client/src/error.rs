use std::{fmt, io};
use thiserror::Error;

/// Result type for RCP client operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for RCP client
#[derive(Error, Debug)]
pub enum Error {
    /// I/O error
    #[error("I/O error: {0}")]
    IO(#[from] io::Error),

    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Protocol error
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// Service error
    #[error("Service error: {0}")]
    Service(String),

    /// Timeout error
    #[error("Operation timed out: {0}")]
    Timeout(String),

    /// Core library error
    #[error("Core error: {0}")]
    Core(#[from] rcp_core::error::Error),

    /// Websocket protocol error
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialize(String),

    /// Deserialization error
    #[error("Deserialization error: {0}")]
    Deserialize(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::WebSocket(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Deserialize(err.to_string())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::Other(err)
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Self::Other(err.to_string())
    }
}