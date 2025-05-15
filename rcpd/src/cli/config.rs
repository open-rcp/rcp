//! CLI configuration module
//!
//! This module contains the configuration types for the CLI.

use serde::{Deserialize, Serialize};
use std::default::Default;

/// CLI configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CliConfig {
    /// Global configuration
    pub global: GlobalConfig,

    /// Service configuration
    pub service: ServiceConfig,
}

/// Global CLI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// Enable color output
    pub color: bool,

    /// Enable JSON output
    pub json: bool,

    /// Quiet mode
    pub quiet: bool,

    /// Default output format
    pub format: OutputFormat,
}

/// Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    /// Service host
    pub host: String,

    /// Service port
    pub port: u16,

    /// Service timeout in seconds
    pub timeout: u64,

    /// Use TLS
    pub use_tls: bool,

    /// Skip TLS verification
    pub skip_verify: bool,
}

/// Output format options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutputFormat {
    /// Text output
    Text,

    /// JSON output
    Json,

    /// YAML output
    Yaml,
}

// Default implementation is now derived

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            color: true,
            json: false,
            quiet: false,
            format: OutputFormat::Text,
        }
    }
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5000,
            timeout: 30,
            use_tls: false,
            skip_verify: false,
        }
    }
}
