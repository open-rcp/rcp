use std::fmt;
use thiserror::Error;

/// Error type for the WebSocket bridge
#[derive(Error, Debug)]
pub enum Error {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// WebSocket error
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    
    /// RCP client error
    #[error("RCP client error: {0}")]
    RcpClient(#[from] rcp_client::error::Error),
    
    /// JSON serialization error
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),
    
    /// Request validation error
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    /// Service error
    #[error("Service error: {0}")]
    Service(String),
    
    /// Generic error
    #[error("{0}")]
    Other(String),
}

/// Result type alias for WebSocket bridge operations
pub type Result<T> = std::result::Result<T, Error>;

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