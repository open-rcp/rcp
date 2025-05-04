use crate::error::ApiResult;
use serde::Deserialize;
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

static CONFIG: OnceLock<AppConfig> = OnceLock::new();

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_expiry_hours: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
                workers: num_cpus::get(),
            },
            database: DatabaseConfig {
                url: "sqlite:rcp_management.db".to_string(),
                max_connections: 10,
            },
            auth: AuthConfig {
                jwt_secret: "default_jwt_secret_for_development".to_string(),
                token_expiry_hours: 24,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
            },
        }
    }
}

/// Load the application configuration from environment variables and config file
pub fn load_config() -> &'static AppConfig {
    CONFIG.get_or_init(|| {
        // Try to load configuration from file
        match load_from_file() {
            Ok(config) => config,
            Err(e) => {
                eprintln!(
                    "Failed to load config from file: {}, using default configuration",
                    e
                );
                load_from_env().unwrap_or_default()
            }
        }
    })
}

/// Load configuration from environment variables
fn load_from_env() -> Result<AppConfig, Box<dyn std::error::Error>> {
    // In a real application, we would use a crate like envy to deserialize
    // environment variables directly into our config structure.
    // For now, we'll just create a default config and override specific values.

    let mut config = AppConfig::default();

    if let Ok(port) = std::env::var("RCP_MANAGEMENT_PORT") {
        if let Ok(port) = port.parse::<u16>() {
            config.server.port = port;
        }
    }

    if let Ok(db_url) = std::env::var("DATABASE_URL") {
        config.database.url = db_url;
    }

    if let Ok(jwt_secret) = std::env::var("JWT_SECRET") {
        config.auth.jwt_secret = jwt_secret;
    }

    if let Ok(log_level) = std::env::var("LOG_LEVEL") {
        config.logging.level = log_level;
    }

    Ok(config)
}

/// Load configuration from a TOML file
fn load_from_file() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());

    let config_content = std::fs::read_to_string(config_path)?;
    let config: AppConfig = toml::from_str(&config_content)?;

    Ok(config)
}

#[derive(Default)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub server_handle: Option<Arc<Mutex<rcp_server::server::Server>>>,
    pub jwt_secret: String,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: 8081,
            host: "127.0.0.1".to_string(),
            server_handle: None,
            jwt_secret: "default_secret_change_me".to_string(),
            database_url: "sqlite://rcp_management.db".to_string(),
        }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn with_host(mut self, host: String) -> Self {
        self.host = host;
        self
    }

    pub fn with_server_handle(mut self, handle: Arc<Mutex<rcp_server::server::Server>>) -> Self {
        self.server_handle = Some(handle);
        self
    }

    pub fn with_jwt_secret(mut self, secret: String) -> Self {
        self.jwt_secret = secret;
        self
    }

    pub fn with_database_url(mut self, url: String) -> Self {
        self.database_url = url;
        self
    }
}
