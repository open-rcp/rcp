// Import the lib module to access BridgeConfig and other types
#[path = "../src/lib.rs"]
mod lib;
use lib::BridgeConfig;

#[test]
fn test_bridge_config_creation() {
    // Create a bridge configuration
    let config = BridgeConfig {
        ws_host: "127.0.0.1".to_string(),
        ws_port: 9000,
        rcp_host: "localhost".to_string(),
        rcp_port: 8716,
    };

    // Test fields
    assert_eq!(config.ws_host, "127.0.0.1");
    assert_eq!(config.ws_port, 9000);
    assert_eq!(config.rcp_host, "localhost");
    assert_eq!(config.rcp_port, 8716);
}

#[test]
fn test_bridge_config_cloning() {
    // Create a bridge configuration
    let config = BridgeConfig {
        ws_host: "127.0.0.1".to_string(),
        ws_port: 9000,
        rcp_host: "localhost".to_string(),
        rcp_port: 8716,
    };

    // Clone the configuration
    let cloned_config = config.clone();

    // Test that the clone has the same values
    assert_eq!(cloned_config.ws_host, config.ws_host);
    assert_eq!(cloned_config.ws_port, config.ws_port);
    assert_eq!(cloned_config.rcp_host, config.rcp_host);
    assert_eq!(cloned_config.rcp_port, config.rcp_port);

    // Make sure they're actually different objects
    std::mem::forget(config);

    // We should still be able to access the cloned config
    assert_eq!(cloned_config.ws_host, "127.0.0.1");
}

#[test]
fn test_bridge_config_debug_representation() {
    // Create a bridge configuration
    let config = BridgeConfig {
        ws_host: "127.0.0.1".to_string(),
        ws_port: 9000,
        rcp_host: "localhost".to_string(),
        rcp_port: 8716,
    };

    // Test Debug trait implementation
    let debug_str = format!("{:?}", config);

    // Check that the debug string contains all fields
    assert!(debug_str.contains("ws_host"));
    assert!(debug_str.contains("127.0.0.1"));
    assert!(debug_str.contains("ws_port"));
    assert!(debug_str.contains("9000"));
    assert!(debug_str.contains("rcp_host"));
    assert!(debug_str.contains("localhost"));
    assert!(debug_str.contains("rcp_port"));
    assert!(debug_str.contains("8716"));
}
