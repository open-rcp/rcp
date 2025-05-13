// Import from the library
use rcp_service::config::*;
use rcp_service::server;

#[test]
fn test_default_service_config() {
    // Create a default service configuration
    let config = ServiceConfig::default();

    // Verify default configuration values
    assert_eq!(config.address, "127.0.0.1");
    assert_eq!(config.port, 8716); // Default port for RCP service
    assert!(!config.tls.enabled);
    assert_eq!(config.tls.cert_path, "cert.pem");
    assert_eq!(config.tls.key_path, "key.pem");
}

#[test]
fn test_custom_service_config() {
    // Create a custom TLS configuration
    let tls_config = TlsConfig {
        enabled: true,
        cert_path: "/path/to/cert.pem".to_string(),
        key_path: "/path/to/key.pem".to_string(),
    };

    // Create a custom service configuration
    let config = ServiceConfig {
        address: "0.0.0.0".to_string(),
        port: 9999,
        tls: tls_config,
        server: server::config::ServerConfig::default(),
        #[cfg(feature = "api")]
        api: None,
    };

    // Verify custom configuration values
    assert_eq!(config.address, "0.0.0.0");
    assert_eq!(config.port, 9999);
    assert!(config.tls.enabled);
    assert_eq!(config.tls.cert_path, "/path/to/cert.pem");
    assert_eq!(config.tls.key_path, "/path/to/key.pem");
}

#[test]
fn test_config_serialization() {
    // Create a service configuration
    let config = ServiceConfig::default();

    // Serialize to JSON
    let json = serde_json::to_string(&config).expect("Failed to serialize config to JSON");

    // Verify JSON contains expected values
    assert!(json.contains(r#""address":"127.0.0.1""#));
    assert!(json.contains(r#""port":8716"#));
    assert!(json.contains(r#""enabled":false"#));
    assert!(json.contains(r#""cert_path":"cert.pem""#));
    assert!(json.contains(r#""key_path":"key.pem""#));

    // Deserialize from JSON
    let deserialized: ServiceConfig =
        serde_json::from_str(&json).expect("Failed to deserialize config");

    // Verify deserialized values match original
    assert_eq!(deserialized.address, config.address);
    assert_eq!(deserialized.port, config.port);
    assert_eq!(deserialized.tls.enabled, config.tls.enabled);
    assert_eq!(deserialized.tls.cert_path, config.tls.cert_path);
    assert_eq!(deserialized.tls.key_path, config.tls.key_path);
}

#[test]
fn test_tls_config() {
    // Create a TLS configuration
    let tls_config = TlsConfig {
        enabled: true,
        cert_path: "custom-cert.pem".to_string(),
        key_path: "custom-key.pem".to_string(),
    };

    // Verify TLS configuration values
    assert!(tls_config.enabled);
    assert_eq!(tls_config.cert_path, "custom-cert.pem");
    assert_eq!(tls_config.key_path, "custom-key.pem");

    // Test Debug trait implementation
    let debug_str = format!("{:?}", tls_config);
    assert!(debug_str.contains("enabled: true"));
    assert!(debug_str.contains("cert_path: \"custom-cert.pem\""));
    assert!(debug_str.contains("key_path: \"custom-key.pem\""));
}
