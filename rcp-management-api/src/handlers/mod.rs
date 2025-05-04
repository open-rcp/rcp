pub mod auth;
pub mod config;
pub mod logs;
pub mod server;
pub mod sessions;
pub mod users;

use actix_web::{HttpResponse, Result};
use std::path::Path;
use std::fs;

/// Handler for the SPA frontend - serves index.html for any path that doesn't match an API endpoint
/// This enables client-side routing to work properly
pub async fn spa_index_handler() -> Result<HttpResponse> {
    // Path to the index.html file
    let index_path = Path::new("./static/index.html");
    
    // Read index.html content
    if let Ok(content) = fs::read_to_string(index_path) {
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(content))
    } else {
        // Fallback response if index.html can't be read
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body("<html><body><h1>RCP Management</h1><p>Welcome to the RCP Management interface.</p></body></html>"))
    }
}