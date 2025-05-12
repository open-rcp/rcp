use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

/// API Server Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Server bind address
    pub bind_address: String,

    /// Server port
    pub port: u16,

    /// Database URL (SQLite)
    pub database_url: String,

    /// RCP Service connection string
    pub service_connection_string: String,

    /// JWT secret for token signing
    pub jwt_secret: String,

    /// JWT token expiration in minutes
    pub jwt_expiration_minutes: u64,

    /// Enable CORS
    pub enable_cors: bool,

    /// CORS allowed origins
    pub cors_origins: Vec<String>,

    /// Enable response compression
    pub enable_compression: bool,

    /// Log level (debug, info, warn, error)
    pub log_level: String,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1".to_string(),
            port: 3000,
            database_url: "sqlite://data/rcp-api.db".to_string(),
            service_connection_string: "tcp://127.0.0.1:9000".to_string(),
            jwt_secret: "change-this-in-production-use-a-strong-secret".to_string(),
            jwt_expiration_minutes: 60,
            enable_cors: true,
            cors_origins: vec!["*".to_string()],
            enable_compression: true,
            log_level: "info".to_string(),
        }
    }
}

impl ApiConfig {
    /// Load configuration from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: ApiConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let contents = toml::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }

    /// Create configuration with environment overrides
    pub fn with_env_overrides(mut self) -> Self {
        // Override config with environment variables if present
        if let Ok(val) = env::var("RCP_API_BIND_ADDRESS") {
            self.bind_address = val;
        }

        if let Ok(val) = env::var("RCP_API_PORT") {
            if let Ok(port) = val.parse::<u16>() {
                self.port = port;
            }
        }

        if let Ok(val) = env::var("RCP_API_DATABASE_URL") {
            self.database_url = val;
        }

        if let Ok(val) = env::var("RCP_API_SERVICE_CONNECTION") {
            self.service_connection_string = val;
        }

        if let Ok(val) = env::var("RCP_API_JWT_SECRET") {
            self.jwt_secret = val;
        }

        if let Ok(val) = env::var("RCP_API_JWT_EXPIRATION") {
            if let Ok(exp) = val.parse::<u64>() {
                self.jwt_expiration_minutes = exp;
            }
        }

        if let Ok(val) = env::var("RCP_API_ENABLE_CORS") {
            if let Ok(enable) = val.parse::<bool>() {
                self.enable_cors = enable;
            }
        }

        if let Ok(val) = env::var("RCP_API_CORS_ORIGINS") {
            self.cors_origins = val.split(',').map(|s| s.trim().to_string()).collect();
        }

        if let Ok(val) = env::var("RCP_API_ENABLE_COMPRESSION") {
            if let Ok(enable) = val.parse::<bool>() {
                self.enable_compression = enable;
            }
        }

        if let Ok(val) = env::var("RCP_API_LOG_LEVEL") {
            self.log_level = val;
        }

        self
    }

    /// Load configuration with fallbacks
    pub fn load() -> Result<Self> {
        // Try to load from config file, fallback to default with env overrides
        let config_path =
            env::var("RCP_API_CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
        let config = if Path::new(&config_path).exists() {
            Self::from_file(&config_path)?.with_env_overrides()
        } else {
            Self::default().with_env_overrides()
        };

        Ok(config)
    }
}
