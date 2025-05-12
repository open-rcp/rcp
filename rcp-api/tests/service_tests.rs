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

    // Mock health check endpoint first (needed for client connection)
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "ok",
            "version": "1.0.0"
        })))
        .mount(&mock_server)
        .await;

    // Test command data
    let command = "test-command";
    let args = serde_json::to_vec(&json!({ "key": "value" })).unwrap();
    let expected_response_value = json!({ "result": "success" });

    // Mock command endpoint - using the correct path format: /command/{command_name}
    Mock::given(method("POST"))
        .and(path(format!("/command/{}", command).as_str()))
        .and(header("Content-Type", "application/octet-stream"))
        .respond_with(ResponseTemplate::new(200).set_body_json(expected_response_value.clone()))
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
    assert_eq!(response_json, expected_response_value);
}

/// Test sending command with error response
#[test]
async fn test_send_command_error() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock health check endpoint first (needed for client connection)
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "ok",
            "version": "1.0.0"
        })))
        .mount(&mock_server)
        .await;

    // Test command data
    let command = "error-command";
    let args = serde_json::to_vec(&json!({ "key": "value" })).unwrap();

    // Mock command endpoint with error response - using the correct path format: /command/{command_name}
    Mock::given(method("POST"))
        .and(path(format!("/command/{}", command).as_str()))
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

/// Test ping with healthy service
#[test]
async fn test_health_check_healthy() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock health endpoint
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
    let client = ServiceClient::connect(&service_url, None).await.unwrap();

    // Check health using ping (not health_check which doesn't exist)
    let version = client.ping().await.unwrap();
    assert_eq!(version, "1.0.0");
}

/// Test health check with unhealthy service
#[test]
async fn test_health_check_unhealthy() {
    // Start a new test server for this specific test
    let mock_server = MockServer::start().await;

    // First set up a success response that will be used for initial connection and first ping
    // This needs to have a low priority so it's matched after more specific matchers
    let _success_mock = Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "ok",
            "version": "1.0.0"
        })))
        .with_priority(10) // Lower priority (higher number)
        .mount(&mock_server)
        .await;

    // Connect to mock server - this will use the success mock
    let service_url = format!("http://{}", mock_server.address());
    let client = ServiceClient::connect(&service_url, None).await.unwrap();

    // Now explicitly delete all existing mocks to ensure a clean state
    mock_server.reset().await;

    // Add an error response that will be used for the next ping call
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Service unavailable"))
        .expect(1) // Expect it to be called exactly once
        .mount(&mock_server)
        .await;

    // Try to ping - this should now fail
    let result = client.ping().await;

    // The ping should fail due to the 500 status response
    assert!(result.is_err());

    // The error message should contain "Service unavailable"
    match result {
        Err(e) => {
            let error_message = e.to_string();
            assert!(
                error_message.contains("Service unavailable"),
                "Error message should mention 'Service unavailable'"
            );
        }
        _ => panic!("Expected an error, but got Ok"),
    }
}
