use crate::server::error::Result;
use rcp_core::DEFAULT_PORT;
use serde::{Deserialize, Serialize};
use std::path::Path;

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
    /// Maximum number of sessions
    #[serde(default = "default_max_sessions")]
    pub max_sessions: usize,

    /// Session timeout in seconds
    #[serde(default = "default_session_timeout")]
    pub timeout: u64,
}

fn default_max_sessions() -> usize {
    100
}

fn default_session_timeout() -> u64 {
    3600
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            max_sessions: default_max_sessions(),
            timeout: default_session_timeout(),
        }
    }
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    /// Whether to enable application management
    #[serde(default)]
    pub enabled: bool,

    /// Application directory
    #[serde(default)]
    pub app_dir: String,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            app_dir: "apps".to_string(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            address: default_address(),
            port: default_port(),
            tls: TlsConfig::default(),
            auth: AuthConfig::default(),
            session: SessionConfig::default(),
            application: ApplicationConfig::default(),
        }
    }
}

impl ServerConfig {
    /// Load configuration from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let builder = config::Config::builder()
            .add_source(config::File::with_name(path.as_ref().to_str().unwrap()))
            .add_source(config::Environment::with_prefix("RCP_SERVER"));
            
        let config = builder.build()
            .map_err(|e| crate::server::error::Error::Config(e))?;
            
        let server_config: ServerConfig = config.try_deserialize()
            .map_err(|e| crate::server::error::Error::Config(e))?;
            
        Ok(server_config)
    }

    /// Save configuration to a file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let toml = toml::to_string(self)
            .map_err(|e| crate::server::error::Error::Other(e.to_string()))?;
            
        std::fs::write(path, toml)
            .map_err(|e| e.into())
    }
}
