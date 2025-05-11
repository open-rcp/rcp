use rcp_api::service::ServiceClient;
use serde_json::json;
use std::time::Duration;
use tokio::test;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Test connection to service with valid URL
#[test]
async fn test_connect_valid_url() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock service health check endpoint
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "ok",
            "version": "1.0.0"
        })))
        .mount(&mock_server)
        .await;

    // Connect to mock server
    let service_url = format!("http://{}", mock_server.address());
    let result = ServiceClient::connect(&service_url, None).await;

    // Should connect successfully
    assert!(result.is_ok());
}

/// Test connection to service with timeout
#[test]
async fn test_connect_timeout() {
    // Try to connect to a non-existent server with short timeout
    let service_url = "http://localhost:1234"; // Assuming this port is not in use
    let timeout = Duration::from_millis(100);

    let result = ServiceClient::connect(service_url, Some(timeout)).await;

    // Should fail to connect
    assert!(result.is_err());
}

/// Test sending command to service
#[test]
async fn test_send_command() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Test command data
    let command = "test-command";
    let args = serde_json::to_vec(&json!({ "key": "value" })).unwrap();
    let expected_response = json!({ "result": "success" });

    // Mock command endpoint
    Mock::given(method("POST"))
        .and(path("/command"))
        .and(header("Content-Type", "application/octet-stream"))
        .and(header("X-RCP-Command", command))
        .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Connect to mock server
    let service_url = format!("http://{}", mock_server.address());
    let client = ServiceClient::connect(&service_url, None).await.unwrap();

    // Send command
    let response = client.send_command(command, &args).await.unwrap();

    // Parse and verify response
    let response_json: serde_json::Value = serde_json::from_slice(&response).unwrap();
    assert_eq!(response_json, expected_response);
}

/// Test sending command with error response
#[test]
async fn test_send_command_error() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Test command data
    let command = "error-command";
    let args = serde_json::to_vec(&json!({ "key": "value" })).unwrap();

    // Mock command endpoint with error response
    Mock::given(method("POST"))
        .and(path("/command"))
        .and(header("X-RCP-Command", command))
        .respond_with(ResponseTemplate::new(500))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Connect to mock server
    let service_url = format!("http://{}", mock_server.address());
    let client = ServiceClient::connect(&service_url, None).await.unwrap();

    // Send command - should fail
    let response = client.send_command(command, &args).await;
    assert!(response.is_err());
}

/// Test health check with healthy service
#[test]
async fn test_health_check_healthy() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock health endpoint
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "ok"
        })))
        .mount(&mock_server)
        .await;

    // Connect to mock server
    let service_url = format!("http://{}", mock_server.address());
    let client = ServiceClient::connect(&service_url, None).await.unwrap();

    // Check health
    let is_healthy = client.health_check().await.unwrap();
    assert!(is_healthy);
}

/// Test health check with unhealthy service
#[test]
async fn test_health_check_unhealthy() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock health endpoint with error
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!({
            "status": "error"
        })))
        .mount(&mock_server)
        .await;

    // Connect to mock server
    let service_url = format!("http://{}", mock_server.address());
    let client = ServiceClient::connect(&service_url, None).await.unwrap();

    // Check health - should be unhealthy
    let is_healthy = client.health_check().await.unwrap();
    assert!(!is_healthy);
}
