#[cfg(feature = "api")]
/// API configuration module

use serde::{Deserialize, Serialize};

/// Configuration for the API server component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// The address to bind the API server to
    #[serde(default = "default_api_address")]
    pub address: String,

    /// The port to bind the API server to
    #[serde(default = "default_api_port")]
    pub port: u16,

    /// Database connection string
    #[serde(default = "default_database_url")]
    pub database_url: String,
    
    /// CORS allowed origins
    #[serde(default)]
    pub cors_allowed_origins: Vec<String>,
    
    /// Authentication settings
    #[serde(default)]
    pub auth: ApiAuthConfig,
}

/// Authentication configuration for the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuthConfig {
    /// Whether authentication is required for API access
    #[serde(default = "default_auth_required")]
    pub required: bool,

    /// JWT secret for token-based authentication
    #[serde(default)]
    pub jwt_secret: Option<String>,
    
    /// Token expiration time in seconds
    #[serde(default = "default_token_expiration")]
    pub token_expiration: u64,
}

fn default_api_address() -> String {
    "127.0.0.1".to_string()
}

fn default_api_port() -> u16 {
    8080
}

fn default_database_url() -> String {
    "sqlite:rcpd.db".to_string()
}

fn default_auth_required() -> bool {
    true
}

fn default_token_expiration() -> u64 {
    86400 // 24 hours
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            address: default_api_address(),
            port: default_api_port(),
            database_url: default_database_url(),
            cors_allowed_origins: vec!["http://localhost:3000".to_string()],
            auth: ApiAuthConfig::default(),
        }
    }
}

impl Default for ApiAuthConfig {
    fn default() -> Self {
        Self {
            required: default_auth_required(),
            jwt_secret: None,
            token_expiration: default_token_expiration(),
        }
    }
}
