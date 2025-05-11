use axum::{
    response::{Response, IntoResponse},
    http::StatusCode,
    Json,
};
use serde_json::json;
use thiserror::Error;

/// API Error types
#[derive(Debug, Error)]
pub enum ApiError {
    /// Not found error
    #[error("Resource not found: {0}")]
    NotFoundError(String),
    
    /// Bad request error
    #[error("Bad request: {0}")]
    BadRequestError(String),
    
    /// Authentication error
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    /// Forbidden error
    #[error("Forbidden: {0}")]
    ForbiddenError(String),
    
    /// Database error
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    /// Service error
    #[error("Service error: {0}")]
    ServiceError(String),
    
    /// Server error
    #[error("Server error: {0}")]
    ServerError(String),
    
    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    /// Conflict error
    #[error("Conflict error: {0}")]
    ConflictError(String),
}

/// Convert SQLx errors to ApiError
impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => {
                ApiError::NotFoundError("Resource not found".to_string())
            }
            sqlx::Error::Database(e) => {
                ApiError::DatabaseError(format!("Database error: {}", e))
            }
            _ => ApiError::DatabaseError(format!("Database error: {}", error)),
        }
    }
}

/// Convert standard errors to ApiError
impl From<std::io::Error> for ApiError {
    fn from(error: std::io::Error) -> Self {
        ApiError::ServerError(format!("I/O error: {}", error))
    }
}

/// Convert Anyhow errors to ApiError
impl From<anyhow::Error> for ApiError {
    fn from(error: anyhow::Error) -> Self {
        ApiError::ServerError(format!("Server error: {}", error))
    }
}

/// Convert serde_json errors to ApiError
impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        ApiError::ServerError(format!("JSON serialization error: {}", error))
    }
}

/// Implement Axum's IntoResponse for ApiError
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::NotFoundError(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequestError(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::AuthError(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::ForbiddenError(msg) => (StatusCode::FORBIDDEN, msg),
            ApiError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::ServiceError(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
            ApiError::ServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::ConflictError(msg) => (StatusCode::CONFLICT, msg),
        };

        // Create a JSON response with error details
        let body = Json(json!({
            "success": false,
            "error": {
                "message": error_message,
                "code": status.as_u16()
            }
        }));

        // Combine status and JSON body
        (status, body).into_response()
    }
}

/// Result type alias for API handlers
pub type ApiResult<T> = Result<T, ApiError>;