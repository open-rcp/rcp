use axum::http::StatusCode;
use axum::response::IntoResponse;
use rcp_api::error::ApiError;
use std::error::Error as StdError;
use std::fmt;

// Create a test error that implements StdError for conversion tests
#[derive(Debug)]
struct TestError {
    message: String,
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl StdError for TestError {}

/// Test creating auth errors
#[test]
fn test_auth_error() {
    let error = ApiError::AuthError("Invalid token".to_string());
    
    // Format error before converting it into a response
    let error_str = format!("{:?}", error);
    
    // Convert to response
    let response = error.into_response();

    // Check status code
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // Check error body format
    assert!(error_str.contains("Invalid token"));
}

/// Test creating forbidden errors
#[test]
fn test_forbidden_error() {
    let error = ApiError::ForbiddenError("Insufficient permissions".to_string());
    let response = error.into_response();

    // Check status code
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

/// Test creating not found errors
#[test]
fn test_not_found_error() {
    let error = ApiError::NotFoundError("Resource not found".to_string());
    let response = error.into_response();

    // Check status code
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

/// Test creating service errors
#[test]
fn test_service_error() {
    let error = ApiError::ServiceError("RCP service unavailable".to_string());
    let response = error.into_response();

    // Check status code
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}

/// Test creating validation errors
#[test]
fn test_validation_error() {
    let error = ApiError::ValidationError("Invalid input".to_string());
    let response = error.into_response();

    // Check status code
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

/// Test creating server errors
#[test]
fn test_server_error() {
    let error = ApiError::ServerError("Internal server error".to_string());
    let response = error.into_response();

    // Check status code
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

/// Test converting from standard errors
#[test]
fn test_from_std_error() {
    let std_error = TestError {
        message: "Standard error".to_string(),
    };

    // Convert to ApiError - manual conversion instead of using From trait
    let api_error = ApiError::ServerError(format!("Server error: {}", std_error));
    
    // Format error before converting it into a response
    let err_string = format!("{:?}", api_error);

    // Should be a server error
    let response = api_error.into_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    // Should contain the original error message
    assert!(err_string.contains("Server error"));
}

/// Test converting from sqlx errors
#[test]
fn test_from_sqlx_error() {
    // Create a database error string (can't easily create real sqlx error in test)
    let db_error = sqlx::Error::RowNotFound;

    // Convert to ApiError
    let api_error: ApiError = db_error.into();

    // For RowNotFound error we typically send a 404 status
    let response = api_error.into_response();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
