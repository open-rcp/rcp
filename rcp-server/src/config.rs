use crate::error::Result;
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
    /// Map of application configurations by ID
    #[serde(default)]
    pub applications: std::collections::HashMap<String, VirtualAppConfig>,

    /// Default application settings
    #[serde(default)]
    pub defaults: AppDefaults,
}

/// Configuration for a virtual application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualAppConfig {
    /// Unique identifier for the application
    pub id: String,

    /// Display name of the application
    pub name: String,

    /// Path to the executable
    pub executable_path: String,

    /// Working directory for the application
    #[serde(default)]
    pub working_dir: Option<String>,

    /// Launch arguments
    #[serde(default)]
    pub args: Vec<String>,

    /// Environment variables
    #[serde(default)]
    pub env: std::collections::HashMap<String, String>,

    /// Permissions required to use this application
    #[serde(default)]
    pub required_permissions: Vec<String>,

    /// File types this application can handle
    #[serde(default)]
    pub file_associations: Vec<String>,

    /// Whether the application should start maximized
    #[serde(default)]
    pub start_maximized: bool,

    /// Application-specific settings
    #[serde(default)]
    pub settings: serde_json::Value,
}

/// Default application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppDefaults {
    /// Default working directory
    #[serde(default = "default_working_dir")]
    pub working_dir: String,

    /// Default permissions
    #[serde(default)]
    pub permissions: Vec<String>,
}

impl Default for AppDefaults {
    fn default() -> Self {
        Self {
            working_dir: default_working_dir(),
            permissions: Vec::new(),
        }
    }
}

fn default_working_dir() -> String {
    "/tmp".to_string()
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        let mut applications = std::collections::HashMap::new();
        
        // Add default text editor based on platform
        #[cfg(target_os = "macos")]
        applications.insert("textedit".to_string(), VirtualAppConfig {
            id: "textedit".to_string(),
            name: "TextEdit".to_string(),
            executable_path: "/System/Applications/TextEdit.app/Contents/MacOS/TextEdit".to_string(),
            working_dir: None,
            args: vec![],
            env: std::collections::HashMap::new(),
            required_permissions: vec!["app:textedit".to_string()],
            file_associations: vec!["txt".to_string(), "rtf".to_string()],
            start_maximized: false,
            settings: serde_json::json!({}),
        });

        #[cfg(target_os = "windows")]
        applications.insert("notepad".to_string(), VirtualAppConfig {
            id: "notepad".to_string(),
            name: "Notepad".to_string(),
            executable_path: "C:\\Windows\\System32\\notepad.exe".to_string(),
            working_dir: None,
            args: vec![],
            env: std::collections::HashMap::new(),
            required_permissions: vec!["app:notepad".to_string()],
            file_associations: vec!["txt".to_string()],
            start_maximized: false,
            settings: serde_json::json!({}),
        });

        Self {
            applications,
            defaults: AppDefaults {
                working_dir: default_working_dir(),
                permissions: vec!["app:basic".to_string()],
            },
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

    /// Get the server address as a string
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

/// Implementation of Default trait for ServerConfig
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
