use rcp_api::config::ApiConfig;
use tempfile::NamedTempFile;

/// Test default config values
#[test]
fn test_config_defaults() {
    let config = ApiConfig::default();

    // Verify default values
    assert_eq!(config.bind_address, "127.0.0.1");
    assert_eq!(config.port, 3000);
    assert_eq!(config.database_url, "sqlite://data/rcp-api.db");
    assert_eq!(config.service_connection_string, "tcp://127.0.0.1:9000");
    assert_eq!(config.jwt_expiration_minutes, 60);
    assert_eq!(config.enable_cors, true);
    assert!(config.cors_origins.contains(&"*".to_string()));
    assert_eq!(config.enable_compression, true);
}

/// Test save and load config
#[test]
fn test_config_save_load() {
    // Create a custom config
    let mut config = ApiConfig::default();
    config.bind_address = "0.0.0.0".to_string();
    config.port = 8080;
    config.database_url = "sqlite://custom/path.db".to_string();

    // Create a temporary file for the config
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let temp_path = temp_file.path().to_path_buf();

    // Save the config
    config
        .save_to_file(&temp_path)
        .expect("Failed to save config");

    // Load the config back
    let loaded_config = ApiConfig::from_file(&temp_path).expect("Failed to load config");

    // Verify values were preserved
    assert_eq!(loaded_config.bind_address, "0.0.0.0");
    assert_eq!(loaded_config.port, 8080);
    assert_eq!(loaded_config.database_url, "sqlite://custom/path.db");
}

/// Test environment variable overrides
#[test]
fn test_config_env_overrides() {
    // Set environment variables for testing
    std::env::set_var("RCP_API_BIND_ADDRESS", "1.2.3.4");
    std::env::set_var("RCP_API_PORT", "9090");
    std::env::set_var("RCP_API_DATABASE_URL", "sqlite://test/env.db");
    std::env::set_var("RCP_API_SERVICE_CONNECTION", "tcp://localhost:5000");
    std::env::set_var("RCP_API_ENABLE_CORS", "false");
    std::env::set_var("RCP_API_CORS_ORIGINS", "localhost,example.com");

    // Create config with environment overrides
    let config = ApiConfig::default().with_env_overrides();

    // Verify environment overrides were applied
    assert_eq!(config.bind_address, "1.2.3.4");
    assert_eq!(config.port, 9090);
    assert_eq!(config.database_url, "sqlite://test/env.db");
    assert_eq!(config.service_connection_string, "tcp://localhost:5000");
    assert_eq!(config.enable_cors, false);
    assert_eq!(
        config.cors_origins,
        vec!["localhost".to_string(), "example.com".to_string()]
    );

    // Clean up environment variables
    std::env::remove_var("RCP_API_BIND_ADDRESS");
    std::env::remove_var("RCP_API_PORT");
    std::env::remove_var("RCP_API_DATABASE_URL");
    std::env::remove_var("RCP_API_SERVICE_CONNECTION");
    std::env::remove_var("RCP_API_ENABLE_CORS");
    std::env::remove_var("RCP_API_CORS_ORIGINS");
}
