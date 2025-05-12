use rcp_api::config::ApiConfig;
use rcp_api::db;
use rcp_api::service::ServiceClient;
use rcp_api::{is_healthy, AppState};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::test;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Helper function to create a test app state
async fn create_test_app_state() -> AppState {
    // Start mock server for service client
    let mock_server = MockServer::start().await;

    // Mock health check endpoint
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "ok",
            "version": "1.0.0"
        })))
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

    // Mock health check for initial connection (only responds once)
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "ok",
            "version": "1.0.0"
        })))
        .expect(1) // Only respond successfully once
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
    let app_state = AppState {
        service_client: Arc::new(Mutex::new(service_client)),
        db_pool,
        config: Arc::new(config),
    };

    // Add a failing mock for subsequent health checks with higher priority (0)
    // This ensures it will be matched before any existing mocks
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Service unavailable"))
        .with_priority(0) // Higher priority (lower number) means it matches first
        .mount(&mock_server)
        .await;

    // Now the health check will fail because our mock returns a 500 error
    let is_healthy_result = is_healthy(&app_state).await;

    // Should be unhealthy since the mock server returns an error
    assert!(!is_healthy_result);
}
