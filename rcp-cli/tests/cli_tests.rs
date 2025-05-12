// filepath: /Volumes/EXT/repos/open-rcp/rcp/rcp-cli/tests/cli_tests.rs
use rcp_cli::{Cli, CliConfig, ConnectionConfig};
use tokio::test;

/// Create a test CLI instance
async fn create_test_cli() -> Cli {
    // Create a basic config for testing
    let config = CliConfig::default();
    Cli::new(config)
}

/// Test CLI creation with default configuration
#[test]
async fn test_cli_creation() {
    let cli = create_test_cli().await;

    // Verify CLI was created with default config
    let config = cli.get_config();
    assert!(config.color);
    assert_eq!(config.log_level, "info");
    assert_eq!(config.format, "table");
}

/// Test CLI configuration from custom values
#[test]
async fn test_cli_custom_config() {
    // Create custom config
    let mut config = CliConfig::default();
    config.color = false;
    config.log_level = "debug".to_string();
    config.format = "json".to_string();
    config.connection.socket = "/tmp/custom-socket".to_string();

    let cli = Cli::new(config);

    // Verify custom config was applied
    let cli_config = cli.get_config();
    assert!(!cli_config.color);
    assert_eq!(cli_config.log_level, "debug");
    assert_eq!(cli_config.format, "json");
    assert_eq!(cli_config.connection.socket, "/tmp/custom-socket");
}

/// Test connection configuration
#[test]
async fn test_connection_config() {
    // Test custom connection config
    let mut conn_config = ConnectionConfig::default();
    conn_config.socket = "/tmp/test-socket".to_string();

    // Verify connection config
    assert_eq!(conn_config.socket, "/tmp/test-socket");

    // Apply to CLI config
    let mut config = CliConfig::default();
    config.connection = conn_config;

    let cli = Cli::new(config);

    // Verify connection config was applied to CLI
    let cli_config = cli.get_config();
    assert_eq!(cli_config.connection.socket, "/tmp/test-socket");
}
