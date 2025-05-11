use rcp_api::service::{ServiceClient};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use serde_json::json;
use std::time::Duration;

/// Test connecting to the service
#[tokio::test]
async fn test_service_connect() {
    // Start mock server
    let mock_server = MockServer::start().await;
    
    // Mock health check endpoint for connect
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "status": "ok",
                "version": "1.0.0"
            }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;
    
    // Connect to service
    let service_url = format!("http://{}", mock_server.address());
    let client = ServiceClient::connect(&service_url, Some(Duration::from_secs(1)))
        .await
        .expect("Failed to connect to service");
    
    // Verify connection was successful by checking ping result
    let version = client.ping().await.expect("Ping failed");
    assert_eq!(version, "1.0.0");
}

/// Test sending commands to the service
#[tokio::test]
async fn test_send_command() {
    // Start mock server
    let mock_server = MockServer::start().await;
    
    // Mock command endpoint
    let command_name = "test-command";
    let command_args = json!({"key": "value"}).to_string().into_bytes();
    let command_result = json!({"result": "success", "data": {"id": "123"}});
    
    Mock::given(method("POST"))
        .and(path(format!("/command/{}", command_name)))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(command_result.clone())
        )
        .expect(1)
        .mount(&mock_server)
        .await;
    
    // Initialize client
    let service_url = format!("http://{}", mock_server.address());
    let client = ServiceClient::connect(&service_url, Some(Duration::from_secs(1)))
        .await
        .expect("Failed to connect to service");
    
    // Send command
    let result = client.send_command(command_name, &command_args)
        .await
        .expect("Command failed");
    
    // Verify result - should be the raw bytes of the JSON response
    let result_json: serde_json::Value = serde_json::from_slice(&result)
        .expect("Failed to parse result");
    
    assert_eq!(result_json["result"], "success");
    assert_eq!(result_json["data"]["id"], "123");
}
