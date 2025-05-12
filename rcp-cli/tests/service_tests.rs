use rcp_cli::service::{ServerInfo, ServiceClient, ServiceStatus};
use tokio::test;

/// Test service status formatting
#[test]
async fn test_status_formatting() {
    // Create a service status
    let status = ServiceStatus {
        service_status: "running".to_string(),
        uptime: 3600,
        active_servers: vec![ServerInfo {
            id: "server1".to_string(),
            name: "Test Server 1".to_string(),
            status: "running".to_string(),
            port: 8080,
            active_connections: 2,
        }],
        active_connections: 2,
    };

    // Verify the structure
    assert_eq!(status.service_status, "running");
    assert_eq!(status.uptime, 3600);
    assert_eq!(status.active_connections, 2);
    assert_eq!(status.active_servers.len(), 1);
    assert_eq!(status.active_servers[0].name, "Test Server 1");
}

/// Test server info formatting
#[test]
async fn test_server_info() {
    // Create server info
    let server = ServerInfo {
        id: "test-server".to_string(),
        name: "Test Server".to_string(),
        status: "running".to_string(),
        port: 8080,
        active_connections: 5,
    };

    // Verify the structure
    assert_eq!(server.id, "test-server");
    assert_eq!(server.name, "Test Server");
    assert_eq!(server.status, "running");
    assert_eq!(server.port, 8080);
    assert_eq!(server.active_connections, 5);
}

/// Test connection timeout - assumes valid implementation in ServiceClient
#[test]
async fn test_connect_timeout() {
    // Try to connect to a non-existent socket with short timeout
    // This should error quickly (if actual implementation is present)
    let socket_path = "/tmp/non-existent-socket-path-for-testing";
    let result = ServiceClient::connect(socket_path, 1).await;

    // Should be an error of some kind - don't be too specific as implementation might change
    assert!(result.is_err());
}
