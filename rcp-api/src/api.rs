// RCP API definitions and utilities
use serde::{Deserialize, Serialize};

/// API response wrapper for consistent response format
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            error: None,
        }
    }

    pub fn message(msg: &str) -> Self {
        Self {
            success: true,
            data: None,
            message: Some(msg.to_string()),
            error: None,
        }
    }

    pub fn error(err: &str) -> Self {
        Self {
            success: false,
            data: None,
            message: None,
            error: Some(err.to_string()),
        }
    }
}

/// Pagination parameters for API endpoints
#[derive(Deserialize, Debug)]
pub struct PaginationParams {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(20),
        }
    }
}

/// Common response metadata
#[derive(Serialize, Debug)]
pub struct ResponseMeta {
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub pages: usize,
}

/// Paginated response wrapper
#[derive(Serialize, Debug)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: ResponseMeta,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: usize, page: usize, per_page: usize) -> Self {
        let pages = (total as f64 / per_page as f64).ceil() as usize;
        Self {
            data,
            meta: ResponseMeta {
                total,
                page,
                per_page,
                pages,
            },
        }
    }
}
