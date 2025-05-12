use rcp_cli::CliConfig;
use rcp_cli::config::{load_config, save_config};
use tokio::test;
use std::fs;

/// Test default configuration
#[test]
async fn test_config_defaults() {
    let config = CliConfig::default();
    
    // Verify default values
    assert_eq!(config.log_level, "info");
    assert_eq!(config.format, "table");
    assert_eq!(config.color, true);
    assert_eq!(config.timeout_seconds, 5);
    assert_eq!(config.json_output, false);
    assert_eq!(config.quiet, false);
}

/// Test loading configuration from file
#[test]
async fn test_config_load() {
    // Create a temporary file for testing
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("rcp-cli-test-config.toml");
    
    // Create a test config file
    let config_content = r#"
    log_level = "debug"
    format = "json"
    color = false
    timeout_seconds = 60
    json_output = true
    quiet = true

    [connection]
    socket = "/tmp/test-socket"

    [auth]
    token_path = "/tmp/test-token"
    save_token = true
    "#;
    
    fs::write(&config_path, config_content).expect("Failed to write test config file");
    
    // Load the config
    let result = load_config(&config_path);
    assert!(result.is_ok());
    
    let config = result.unwrap();
    
    // Verify loaded values
    assert_eq!(config.log_level, "debug");
    assert_eq!(config.format, "json");
    assert_eq!(config.color, false);
    assert_eq!(config.timeout_seconds, 60);
    assert_eq!(config.json_output, true);
    assert_eq!(config.quiet, true);
    assert_eq!(config.connection.socket, "/tmp/test-socket");
    assert_eq!(config.auth.token_path, "/tmp/test-token");
    
    // Clean up
    fs::remove_file(config_path).ok();
}

/// Test saving configuration to file
#[test]
async fn test_config_save() {
    // Create a temporary file for testing
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("rcp-cli-test-config-save.toml");
    
    // Create a config to save
    let mut config = CliConfig::default();
    config.log_level = "debug".to_string();
    config.format = "json".to_string();
    config.color = false;
    config.timeout_seconds = 60;
    config.json_output = true;
    config.quiet = true;
    config.connection.socket = "/tmp/test-socket".to_string();
    config.auth.token_path = "/tmp/test-token".to_string();
    
    // Save the config
    let result = save_config(&config, &config_path);
    assert!(result.is_ok());
    
    // Reload the config and verify
    let loaded_result = load_config(&config_path);
    assert!(loaded_result.is_ok());
    
    let loaded_config = loaded_result.unwrap();
    
    assert_eq!(loaded_config.log_level, "debug");
    assert_eq!(loaded_config.format, "json");
    assert_eq!(loaded_config.color, false);
    assert_eq!(loaded_config.timeout_seconds, 60);
    assert_eq!(loaded_config.json_output, true);
    assert_eq!(loaded_config.quiet, true);
    assert_eq!(loaded_config.connection.socket, "/tmp/test-socket");
    assert_eq!(loaded_config.auth.token_path, "/tmp/test-token");
    
    // Clean up
    fs::remove_file(config_path).ok();
}
