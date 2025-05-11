use rcp_api::{AppState, is_healthy};
use rcp_api::config::ApiConfig;
use rcp_api::service::ServiceClient;
use rcp_api::db;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::test;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use serde_json::json;

// Helper function to create a test app state
async fn create_test_app_state() -> AppState {
    // Start mock server for service client
    let mock_server = MockServer::start().await;
    
    // Mock health check endpoint
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "status": "ok",
                "version": "1.0.0"
            }))
        )
        .mount(&mock_server)
        .await;
    
    // Create service client connected to mock
    let service_url = format!("http://{}", mock_server.address());
    let service_client = ServiceClient::connect(&service_url, None)
        .await
        .expect("Failed to create service client");
    
    // Create in-memory SQLite database
    let db_pool = db::init_db("sqlite::memory:")
        .await
        .expect("Failed to create database");
    
    // Create app state
    let config = ApiConfig::default();
    AppState {
        service_client: Arc::new(Mutex::new(service_client)),
        db_pool,
        config: Arc::new(config),
    }
}

/// Test health check with healthy state
#[test]
async fn test_health_check_healthy() {
    // Create test app state
    let app_state = create_test_app_state().await;
    
    // Check health status
    let is_healthy_result = is_healthy(&app_state).await;
    
    // Should be healthy
    assert!(is_healthy_result);
}

/// Test health check with unhealthy service
#[test]
async fn test_health_check_unhealthy_service() {
    // Start mock server for unhealthy service
    let mock_server = MockServer::start().await;
    
    // Mock health check endpoint with error
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(500)
            .set_body_json(json!({
                "status": "error",
                "message": "Service unavailable"
            }))
        )
        .mount(&mock_server)
        .await;
    
    // Create service client connected to unhealthy mock
    let service_url = format!("http://{}", mock_server.address());
    let service_client = ServiceClient::connect(&service_url, None)
        .await
        .expect("Failed to create service client");
    
    // Create in-memory SQLite database
    let db_pool = db::init_db("sqlite::memory:")
        .await
        .expect("Failed to create database");
    
    // Create app state
    let config = ApiConfig::default();
    let app_state = AppState {
        service_client: Arc::new(Mutex::new(service_client)),
        db_pool,
        config: Arc::new(config),
    };
    
    // Override the mock to respond with errors after creation
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;
    
    // Check health status
    let is_healthy_result = is_healthy(&app_state).await;
    
    // Should be unhealthy
    assert!(!is_healthy_result);
}
