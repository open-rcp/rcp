use axum::{
    routing::{get, post},
    Router,
};

// Import server handlers from the handlers module
use crate::handlers::servers as server_handlers;
use crate::handlers::auth as auth_handlers;

// Define the main router that collects all API routes
pub fn api_routes() -> Router {
    Router::new()
        .nest("/api/v1", api_v1_routes())
        .route("/health", get(health_check))
}

// Version 1 of the API
fn api_v1_routes() -> Router {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/servers", servers::routes())
        .nest("/sessions", sessions::routes())
        .nest("/stats", stats::routes())
}

// Basic health check endpoint
async fn health_check() -> &'static str {
    "RCP Management API is running"
}

// Auth routes module
mod auth {
    use axum::{
        routing::{get, post},
        Router,
    };
    use crate::handlers::auth as handler;

    pub fn routes() -> Router {
        Router::new()
            .route("/login", post(handler::login))
            .route("/logout", post(handler::logout))
            .route("/me", get(handler::get_current_user))
    }
}

// Server management routes
mod servers {
    use axum::{
        routing::{get, post, patch, delete},
        Router,
    };
    use crate::handlers::servers as handler;

    pub fn routes() -> Router {
        Router::new()
            .route("/", get(handler::list_servers).post(handler::create_server))
            .route("/:id", get(handler::get_server).patch(handler::update_server).delete(handler::delete_server))
            .route("/:id/start", post(handler::start_server))
            .route("/:id/stop", post(handler::stop_server))
            .route("/:id/restart", post(handler::restart_server))
    }
}

// Session management routes
mod sessions {
    use axum::{
        routing::{get, delete},
        Router,
    };

    pub fn routes() -> Router {
        Router::new()
            .route("/", get(list_sessions))
            .route("/:id", get(get_session).delete(terminate_session))
    }

    async fn list_sessions() -> &'static str {
        "List sessions endpoint"
    }

    async fn get_session() -> &'static str {
        "Get session endpoint"
    }

    async fn terminate_session() -> &'static str {
        "Terminate session endpoint"
    }
}

// Stats and monitoring routes
mod stats {
    use axum::{
        routing::get,
        Router,
    };

    pub fn routes() -> Router {
        Router::new()
            .route("/overview", get(get_overview))
            .route("/servers", get(get_server_stats))
            .route("/connections", get(get_connection_stats))
            .route("/usage", get(get_usage_stats))
    }

    async fn get_overview() -> &'static str {
        "Get overview endpoint"
    }

    async fn get_server_stats() -> &'static str {
        "Get server stats endpoint"
    }

    async fn get_connection_stats() -> &'static str {
        "Get connection stats endpoint"
    }

    async fn get_usage_stats() -> &'static str {
        "Get usage stats endpoint"
    }
}