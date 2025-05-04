pub mod app;
pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod utils;

// Re-export commonly used types
pub use app::AppState;
pub use db::init_database;
pub use error::{ApiError, ApiResult};