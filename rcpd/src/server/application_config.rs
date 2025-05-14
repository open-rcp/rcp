use serde::{Deserialize, Serialize};

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
