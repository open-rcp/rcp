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