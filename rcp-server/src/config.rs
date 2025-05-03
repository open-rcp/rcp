use serde::{Deserialize, Serialize};
use crate::error::Result;
use std::path::Path;
use rcp_core::DEFAULT_PORT;

/// Configuration for the RCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server address to bind to
    #[serde(default = "default_address")]
    pub address: String,
    
    /// Server port to listen on
    #[serde(default = "default_port")]
    pub port: u16,
    
    /// TLS configuration
    #[serde(default)]
    pub tls: TlsConfig,
    
    /// Authentication configuration
    #[serde(default)]
    pub auth: AuthConfig,
    
    /// Session configuration
    #[serde(default)]
    pub session: SessionConfig,
    
    /// Application configuration
    #[serde(default)]
    pub application: ApplicationConfig,
}

/// Default address to bind to
fn default_address() -> String {
    "0.0.0.0".to_string()
}

/// Default port to listen on
fn default_port() -> u16 {
    DEFAULT_PORT
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Whether TLS is enabled
    #[serde(default)]
    pub enabled: bool,
    
    /// Path to the certificate file
    #[serde(default)]
    pub cert_path: String,
    
    /// Path to the key file
    #[serde(default)]
    pub key_path: String,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cert_path: "cert.pem".to_string(),
            key_path: "key.pem".to_string(),
        }
    }
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Whether authentication is required
    #[serde(default = "default_auth_required")]
    pub required: bool,
    
    /// Pre-shared key for authentication
    #[serde(default)]
    pub psk: Option<String>,
    
    /// Allowed client IDs
    #[serde(default)]
    pub allowed_clients: Vec<String>,
}

fn default_auth_required() -> bool {
    true
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            required: true,
            psk: None,
            allowed_clients: Vec::new(),
        }
    }
}

/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Maximum number of concurrent sessions
    #[serde(default = "default_max_sessions")]
    pub max_sessions: usize,
    
    /// Session timeout in seconds
    #[serde(default = "default_session_timeout")]
    pub timeout_secs: u64,
    
    /// Whether to allow session resumption
    #[serde(default = "default_allow_resumption")]
    pub allow_resumption: bool,
}

fn default_max_sessions() -> usize {
    10
}

fn default_session_timeout() -> u64 {
    3600 // 1 hour
}

fn default_allow_resumption() -> bool {
    true
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            max_sessions: default_max_sessions(),
            timeout_secs: default_session_timeout(),
            allow_resumption: default_allow_resumption(),
        }
    }
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    /// Allowed applications
    #[serde(default)]
    pub allowed_apps: Vec<String>,
    
    /// Denied applications
    #[serde(default)]
    pub denied_apps: Vec<String>,
    
    /// Whether to allow elevated privileges
    #[serde(default)]
    pub allow_elevated: bool,
    
    /// Working directory for applications
    #[serde(default = "default_work_dir")]
    pub work_dir: String,
}

fn default_work_dir() -> String {
    "./apps".to_string()
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            allowed_apps: Vec::new(),
            denied_apps: Vec::new(),
            allow_elevated: false,
            work_dir: default_work_dir(),
        }
    }
}

impl ServerConfig {
    /// Load configuration from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::from(path.as_ref()))
            .add_source(config::Environment::with_prefix("RCP"))
            .build()?
            .try_deserialize()?;
            
        Ok(config)
    }
    
    /// Create a default configuration
    pub fn default() -> Self {
        Self {
            address: default_address(),
            port: default_port(),
            tls: TlsConfig::default(),
            auth: AuthConfig::default(),
            session: SessionConfig::default(),
            application: ApplicationConfig::default(),
        }
    }
    
    /// Get the server address as a string
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}