use thiserror::Error;

/// Result type for service operations
pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ServiceError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Service error: {0}")]
    Service(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Server error: {0}")]
    Server(String),

    #[error("Database error: {0}")]
    Database(String),
}
