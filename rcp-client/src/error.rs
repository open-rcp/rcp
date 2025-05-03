use std::fmt;
use std::io;
use std::result;
use thiserror::Error;

/// Result type for RCP client operations
pub type Result<T> = result::Result<T, Error>;

/// Error types for RCP client
#[derive(Error, Debug)]
pub enum Error {
    /// Connection errors
    #[error("Connection error: {0}")]
    Connection(String),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Protocol errors
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// Service-specific errors
    #[error("Service error: {0}")]
    Service(String),

    /// Timeout errors
    #[error("Operation timed out: {0}")]
    Timeout(String),

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// State errors (operation not valid in current state)
    #[error("Invalid state for operation: {0}")]
    InvalidState(String),

    /// Session errors
    #[error("Session error: {0}")]
    Session(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Permission errors
    #[error("Permission denied: {0}")]
    Permission(String),

    /// Frame parsing errors
    #[error("Frame parsing error: {0}")]
    FrameParsing(String),

    /// Other errors
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Create a new connection error
    pub fn connection<S: Into<String>>(msg: S) -> Self {
        Error::Connection(msg.into())
    }

    /// Create a new authentication error
    pub fn authentication<S: Into<String>>(msg: S) -> Self {
        Error::Authentication(msg.into())
    }

    /// Create a new protocol error
    pub fn protocol<S: Into<String>>(msg: S) -> Self {
        Error::Protocol(msg.into())
    }

    /// Create a new service error
    pub fn service<S: Into<String>>(msg: S) -> Self {
        Error::Service(msg.into())
    }

    /// Create a new timeout error
    pub fn timeout<S: Into<String>>(msg: S) -> Self {
        Error::Timeout(msg.into())
    }

    /// Create a new serialization error
    pub fn serialization<S: Into<String>>(msg: S) -> Self {
        Error::Serialization(msg.into())
    }

    /// Create a new invalid state error
    pub fn invalid_state<S: Into<String>>(msg: S) -> Self {
        Error::InvalidState(msg.into())
    }

    /// Create a new session error
    pub fn session<S: Into<String>>(msg: S) -> Self {
        Error::Session(msg.into())
    }

    /// Create a new configuration error
    pub fn config<S: Into<String>>(msg: S) -> Self {
        Error::Config(msg.into())
    }

    /// Create a new permission error
    pub fn permission<S: Into<String>>(msg: S) -> Self {
        Error::Permission(msg.into())
    }

    /// Create a new frame parsing error
    pub fn frame_parsing<S: Into<String>>(msg: S) -> Self {
        Error::FrameParsing(msg.into())
    }

    /// Create a new other error
    pub fn other<S: Into<String>>(msg: S) -> Self {
        Error::Other(msg.into())
    }
}

// Allow conversion from string types to Error
impl<S> From<S> for Error
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        Error::Other(s.into())
    }
}