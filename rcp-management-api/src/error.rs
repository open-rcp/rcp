use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Invalid input: {0}")]
    BadRequest(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("RCP server error: {0}")]
    RcpServer(String),
}

// Implement conversions from SurrealDB errors to our ApiError
impl From<surrealdb::Error> for ApiError {
    fn from(err: surrealdb::Error) -> Self {
        match err {
            surrealdb::Error::Db(msg) => ApiError::Database(format!("Database error: {}", msg)),
            surrealdb::Error::Api(msg) => ApiError::Internal(format!("API error: {}", msg)),
            surrealdb::Error::Auth(msg) => ApiError::Authentication(format!("Auth error: {}", msg)),
            _ => ApiError::Internal(format!("Unexpected SurrealDB error: {}", err))
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::Authentication(msg) => (StatusCode::UNAUTHORIZED, msg),
            Self::Authorization(msg) => (StatusCode::FORBIDDEN, msg),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            Self::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            Self::RcpServer(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "code": status.as_u16()
            }
        }));

        (status, body).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;