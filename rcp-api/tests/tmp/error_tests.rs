use rcp_api::error::{ApiError, ApiResult};
use axum::response::Response;
use axum::http::StatusCode;
use axum::body::Body;
use axum::response::IntoResponse;
use axum_core::body::boxed;
use http_body_util::BodyExt;

/// Test not found error response
#[tokio::test]
async fn test_not_found_error() {
    let error = ApiError::NotFoundError("Resource not found".to_string());
    let response: Response = error.into_response();
    
    // Check status code
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    
    // Check response body
    let body = response.into_body();
    let bytes = BodyExt::collect(body)
        .await
        .unwrap()
        .to_bytes();
    let body_str = String::from_utf8(bytes.to_vec()).unwrap();
    
    // Body should contain error message
    assert!(body_str.contains("Resource not found"));
    assert!(body_str.contains("404"));
}

/// Test bad request error response
#[tokio::test]
async fn test_bad_request_error() {
    let error = ApiError::BadRequestError("Invalid input".to_string());
    let response: Response = error.into_response();
    
    // Check status code
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    // Check response body
    let body = response.into_body();
    let bytes = BodyExt::collect(body)
        .await
        .unwrap()
        .to_bytes();
    let body_str = String::from_utf8(bytes.to_vec()).unwrap();
    
    // Body should contain error message
    assert!(body_str.contains("Invalid input"));
    assert!(body_str.contains("400"));
}

/// Test validation error response
#[tokio::test]
async fn test_validation_error() {
    let error = ApiError::ValidationError("Field is required".to_string());
    let response: Response = error.into_response();
    
    // Check status code
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    // Check response body
    let body = response.into_body();
    let bytes = BodyExt::collect(body)
        .await
        .unwrap()
        .to_bytes();
    let body_str = String::from_utf8(bytes.to_vec()).unwrap();
    
    // Body should contain error message
    assert!(body_str.contains("Field is required"));
    assert!(body_str.contains("400"));
}

/// Test auth error response
#[tokio::test]
async fn test_auth_error() {
    let error = ApiError::AuthError("Invalid credentials".to_string());
    let response: Response = error.into_response();
    
    // Check status code
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    
    // Check response body
    let body = response.into_body();
    let bytes = BodyExt::collect(body)
        .await
        .unwrap()
        .to_bytes();
    let body_str = String::from_utf8(bytes.to_vec()).unwrap();
    
    // Body should contain error message
    assert!(body_str.contains("Invalid credentials"));
    assert!(body_str.contains("401"));
}

/// Test API result Ok conversion
#[test]
fn test_api_result_ok() {
    let result: ApiResult<String> = Ok("Test".to_string());
    assert_eq!(result.unwrap(), "Test");
}

/// Test API result Err conversion
#[test]
fn test_api_result_err() {
    let error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let result: ApiResult<()> = Err(error.into());
    
    match result {
        Ok(_) => panic!("Should be an error"),
        Err(e) => {
            match e {
                ApiError::IoError(_) => (), // Expected
                _ => panic!("Should be an IoError"),
            }
        }
    }
}
