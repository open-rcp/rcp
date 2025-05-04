use thiserror::Error;

/// Error types for the WebSocket Bridge
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

/// Result type for WebSocket Bridge operations
pub type Result<T> = std::result::Result<T, Error>;

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Self::Other(err.to_string())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::Other(err)
    }
}
