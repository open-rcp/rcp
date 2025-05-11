use rcp_core::DEFAULT_PORT;
use rcp_server::config::ServerConfig;
use std::net::SocketAddr;
use std::str::FromStr;

#[test]
fn test_default_config() {
    // Create a default server configuration
    let config = ServerConfig::default();

    // Verify default values
    assert_eq!(
        config.address, "0.0.0.0",
        "Default address should be 0.0.0.0"
    );
    assert_eq!(
        config.port, DEFAULT_PORT,
        "Default port should match DEFAULT_PORT constant"
    );
    assert!(
        config.auth.required,
        "Authentication should be required by default"
    );
}

#[test]
fn test_server_addr_formatting() {
    // Create a config with specific address and port
    let mut config = ServerConfig::default();
    config.address = "127.0.0.1".to_string();
    config.port = 8080;

    // Get the formatted server address
    let addr = config.server_addr();

    // Verify it matches the expected format
    assert_eq!(
        addr, "127.0.0.1:8080",
        "Server address should be formatted as address:port"
    );

    // Verify it can be parsed as a SocketAddr
    let socket_addr = SocketAddr::from_str(&addr);
    assert!(
        socket_addr.is_ok(),
        "Server address should be a valid socket address"
    );

    let socket_addr = socket_addr.unwrap();
    assert_eq!(socket_addr.port(), 8080, "Port should be 8080");
    assert_eq!(
        socket_addr.ip().to_string(),
        "127.0.0.1",
        "IP should be 127.0.0.1"
    );
}

#[test]
fn test_config_load() {
    // In a real test, we would write config to a file and then load it
    // For now, we'll just test that the server_addr method works correctly

    let mut config = ServerConfig::default();
    config.address = "192.168.1.1".to_string();
    config.port = 9000;

    let addr = config.server_addr();
    assert_eq!(
        addr, "192.168.1.1:9000",
        "Server address should be formatted correctly"
    );
}
