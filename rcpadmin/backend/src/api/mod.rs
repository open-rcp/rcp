use axum::Router;

pub mod applications;
pub mod auth;
pub mod server;
pub mod sessions;
pub mod system;
pub mod users;

use crate::AppState;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        // Auth routes (no authentication required)
        .nest("/auth", auth::create_routes())
        // Protected routes (require authentication)
        .nest("/server", server::create_routes())
        .nest("/applications", applications::create_routes())
        .nest("/sessions", sessions::create_routes())
        .nest("/system", system::create_routes())
        // Admin-only routes
        .nest("/users", users::create_routes())
}
