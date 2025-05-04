use actix_web::{HttpResponse, ResponseError};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;
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

// Implement conversions from SurrealDB errors to our ApiError (updated for latest version)
impl From<surrealdb::Error> for ApiError {
    fn from(err: surrealdb::Error) -> Self {
        match err {
            // Check for specific error types by matching on the string representation
            // This is more resilient to API changes in SurrealDB
            err if err.to_string().contains("Database error") => {
                ApiError::Database(format!("Database error: {}", err))
            }
            err if err.to_string().contains("Query error") => {
                ApiError::Database(format!("Query error: {}", err))
            }
            err if err.to_string().contains("Api error") => {
                ApiError::Internal(format!("API error: {}", err))
            }
            _ => ApiError::Internal(format!("Unexpected SurrealDB error: {}", err)),
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

// Add ResponseError implementation for Actix Web
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_message) = match self {
            Self::Authentication(msg) => (StatusCode::UNAUTHORIZED, msg),
            Self::Authorization(msg) => (StatusCode::FORBIDDEN, msg),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            Self::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            Self::RcpServer(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
        };

        HttpResponse::build(actix_web::http::StatusCode::from_u16(status.as_u16()).unwrap()).json(
            json!({
                "error": {
                    "message": error_message,
                    "code": status.as_u16()
                }
            }),
        )
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::Authentication(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            Self::Authorization(_) => actix_web::http::StatusCode::FORBIDDEN,
            Self::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            Self::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
            Self::Database(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::Internal(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::RcpServer(_) => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
