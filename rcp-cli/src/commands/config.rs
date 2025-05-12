use crate::error::CliError;
use crate::{config::{load_config, save_config}, CliConfig};
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Helper function for tests to check if this module is properly loaded
#[cfg(test)]
pub fn is_module_loaded() -> bool {
    true
}

pub async fn handle_config_command() -> Result<(), CliError> {
    // TODO: Implement config command handling
    Ok(())
}

/// Set a configuration value - Helper for testing
#[cfg(test)]
pub fn set_config_value(
    key: &str,
    value: &str,
    config_path: Option<impl AsRef<Path>>
) -> Result<()> {
    // Get the config path
    let config_path = config_path.map(|p| p.as_ref().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("config.toml"));
    
    // Load existing config or create new one
    let mut config = load_config(&config_path).unwrap_or_else(|_| CliConfig::default());
    
    // Set the value based on the key
    match key {
        "log_level" => config.log_level = value.to_string(),
        "format" => config.format = value.to_string(),
        "color" => config.color = value.parse().unwrap_or(true),
        "timeout_seconds" => config.timeout_seconds = value.parse().unwrap_or(30),
        "socket" => config.connection.socket = value.to_string(),
        _ => return Err(anyhow::anyhow!("Unknown config key: {}", key)),
    }
    
    // Save the config
    save_config(&config, &config_path)?;
    
    Ok(())
}

/// Get a configuration value - Helper for testing
#[cfg(test)]
pub fn get_config_value(
    key: &str,
    config_path: Option<impl AsRef<Path>>
) -> Result<String> {
    // Get the config path
    let config_path = config_path.map(|p| p.as_ref().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("config.toml"));
    
    // Load the config
    let config = load_config(&config_path)?;
    
    // Get the value based on the key
    let value = match key {
        "log_level" => config.log_level.clone(),
        "format" => config.format.clone(),
        "color" => config.color.to_string(),
        "timeout_seconds" => config.timeout_seconds.to_string(),
        "socket" => config.connection.socket.clone(),
        _ => return Err(anyhow::anyhow!("Unknown config key: {}", key)),
    };
    
    Ok(value)
}
