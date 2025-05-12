use rcp_client::ConnectionString;
use tokio::test;

/// Test parsing a complete RCP URL
#[test]
async fn test_parse_complete_url() {
    // Test a complete RCP URL with all parts
    let conn_str = ConnectionString::parse("rcp://user:pass@example.com:8080/path").unwrap();
    
    assert_eq!(conn_str.username, Some("user".to_string()));
    assert_eq!(conn_str.password, Some("pass".to_string()));
    assert_eq!(conn_str.host, "example.com");
    assert_eq!(conn_str.port, Some(8080));
    assert_eq!(conn_str.path, Some("/path".to_string()));
}

/// Test parsing a simple host-only URL
#[test]
async fn test_parse_host_only() {
    // Test a simple host-only connection string
    let conn_str = ConnectionString::parse("example.com").unwrap();
    
    assert_eq!(conn_str.username, None);
    assert_eq!(conn_str.password, None);
    assert_eq!(conn_str.host, "example.com");
    assert_eq!(conn_str.port, None);
    assert_eq!(conn_str.path, None);
}

/// Test parsing a host with port
#[test]
async fn test_parse_host_port() {
    // Test parsing host with port
    let conn_str = ConnectionString::parse("example.com:8080").unwrap();
    
    assert_eq!(conn_str.username, None);
    assert_eq!(conn_str.password, None);
    assert_eq!(conn_str.host, "example.com");
    assert_eq!(conn_str.port, Some(8080));
    assert_eq!(conn_str.path, None);
}

/// Test parsing a user and host
#[test]
async fn test_parse_user_host() {
    // Test parsing a user and host
    let conn_str = ConnectionString::parse("user@example.com").unwrap();
    
    assert_eq!(conn_str.username, Some("user".to_string()));
    assert_eq!(conn_str.password, None);
    assert_eq!(conn_str.host, "example.com");
    assert_eq!(conn_str.port, None);
    assert_eq!(conn_str.path, None);
}

/// Test parsing an IPv4 address
#[test]
async fn test_parse_ipv4() {
    // Test parsing an IPv4 address
    let conn_str = ConnectionString::parse("192.168.1.100").unwrap();
    
    assert_eq!(conn_str.username, None);
    assert_eq!(conn_str.password, None);
    assert_eq!(conn_str.host, "192.168.1.100");
    assert_eq!(conn_str.port, None);
    assert_eq!(conn_str.path, None);
}

/// Test parsing an invalid connection string
#[test]
async fn test_parse_invalid() {
    // Test parsing with invalid port
    let result = ConnectionString::parse("example.com:invalid");
    assert!(result.is_err());
}
