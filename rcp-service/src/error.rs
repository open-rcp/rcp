use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ServiceError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Service error: {0}")]
    Service(String),
}
