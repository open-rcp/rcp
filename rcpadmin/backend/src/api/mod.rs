use axum::{
    middleware,
    Router,
};

mod auth;
mod applications;
mod sessions;
mod server;
mod users;
mod system;

use crate::auth::{protect, require_admin};
use crate::AppState;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        // Auth routes (no authentication required)
        .nest("/auth", auth::create_routes())
        
        // Protected routes (require authentication)
        .nest(
            "/server",
            server::create_routes().layer(middleware::from_fn(protect))
        )
        .nest(
            "/applications",
            applications::create_routes().layer(middleware::from_fn(protect))
        )
        .nest(
            "/sessions",
            sessions::create_routes().layer(middleware::from_fn(protect))
        )
        .nest(
            "/system",
            system::create_routes().layer(middleware::from_fn(protect))
        )
        
        // Admin-only routes
        .nest(
            "/users",
            users::create_routes().layer(middleware::from_fn(require_admin))
        )
}