#[cfg(feature = "api")]
use crate::api::ApiConfig;
use crate::server::config::ServerConfig;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub address: String,
    pub port: u16,
    pub tls: TlsConfig,

    /// Integrated server configuration
    #[serde(default)]
    pub server: ServerConfig,

    /// Integrated API configuration (when api feature is enabled)
    #[cfg(feature = "api")]
    pub api: Option<ApiConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub enabled: bool,
    pub cert_path: String,
    pub key_path: String,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        #[cfg(feature = "api")]
        {
            Self {
                address: "127.0.0.1".to_string(),
                port: 8716,
                tls: TlsConfig {
                    enabled: false,
                    cert_path: "cert.pem".to_string(),
                    key_path: "key.pem".to_string(),
                },
                server: ServerConfig::default(),
                api: Some(ApiConfig::default()),
            }
        }

        #[cfg(not(feature = "api"))]
        Self {
            address: "127.0.0.1".to_string(),
            port: 8716,
            tls: TlsConfig {
                enabled: false,
                cert_path: "cert.pem".to_string(),
                key_path: "key.pem".to_string(),
            },
            server: ServerConfig::default(),
        }
    }
}

impl ServiceConfig {
    /// Load configuration from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config_str = std::fs::read_to_string(path)?;
        let config: ServiceConfig = toml::from_str(&config_str)?;
        Ok(config)
    }

    /// Save configuration to a file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let toml = toml::to_string(self)?;
        std::fs::write(path, toml)?;
        Ok(())
    }
}
