use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use crate::CliConfig;
// We need the following imports for the config module
// Even though they're reported as unused, they're needed for config file loading

/// Get the default configuration path
pub fn default_config_path() -> Result<PathBuf> {
    // Platform-specific configuration directory
    let config_dir = if cfg!(windows) {
        // Windows: %APPDATA%\RCP\cli.toml
        dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine configuration directory"))?
            .join("RCP")
    } else {
        // Unix/Linux/macOS: ~/.config/rcp/cli.toml
        dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine configuration directory"))?
            .join("rcp")
    };

    // Create directory if it doesn't exist
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)?;
    }

    Ok(config_dir.join("cli.toml"))
}

/// Get system-wide configuration path
pub fn system_config_path() -> PathBuf {
    if cfg!(windows) {
        // Windows: C:\ProgramData\RCP\cli.toml
        PathBuf::from(r"C:\ProgramData\RCP\cli.toml")
    } else if cfg!(target_os = "macos") {
        // macOS: /Library/Application Support/RCP/cli.toml
        PathBuf::from("/Library/Application Support/RCP/cli.toml")
    } else {
        // Linux/Unix: /etc/rcp/cli.toml
        PathBuf::from("/etc/rcp/cli.toml")
    }
}

/// Search for configuration file in standard locations
pub fn find_config_file() -> Option<PathBuf> {
    // First check in current directory
    let current_dir = std::env::current_dir().ok()?;
    let local_config = current_dir.join("rcp-cli.toml");
    if local_config.exists() {
        return Some(local_config);
    }

    // Then check user config directory
    let user_config = default_config_path().ok()?;
    if user_config.exists() {
        return Some(user_config);
    }

    // Finally check system-wide config
    let system_config = system_config_path();
    if system_config.exists() {
        return Some(system_config);
    }

    None
}

/// Create a new configuration file with default settings
pub fn create_default_config(path: &Path) -> Result<()> {
    let config = crate::cli::CliConfig::default();
    config.to_file(path)?;
    Ok(())
}

/// Load configuration from a file
pub fn load_config(path: &Path) -> Result<CliConfig> {
    let content = fs::read_to_string(path)?;
    let config: CliConfig = toml::from_str(&content)?;
    Ok(config)
}

/// Save configuration to a file
pub fn save_config(config: &CliConfig, path: &Path) -> Result<()> {
    let content = toml::to_string_pretty(config)?;
    fs::write(path, content)?;
    Ok(())
}
