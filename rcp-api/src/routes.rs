use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use std::time::Duration;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::handlers;
use crate::AppState;

/// Create API router with all routes and middleware
pub fn create_router(app_state: AppState) -> Router {
    // Base router with middleware
    

    Router::new()
        .nest("/api/v1", api_routes())
        .with_state(app_state.clone())
        .layer(TraceLayer::new_for_http())
        .layer(compression_layer(&app_state))
        .layer(cors_layer(&app_state))
}

/// API routes
fn api_routes() -> Router<AppState> {
    // Public routes that don't require authentication
    let public_routes = Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/refresh", post(handlers::auth::refresh_token));

    // Protected routes that require authentication
    let protected_routes = Router::new()
        .route("/servers", get(handlers::servers::list_servers))
        .route("/servers/:name", get(handlers::servers::get_server))
        .route("/servers", post(handlers::servers::create_server))
        .route(
            "/servers/:name/start",
            post(handlers::servers::start_server),
        )
        .route("/servers/:name/stop", post(handlers::servers::stop_server))
        .route(
            "/servers/:name/restart",
            post(handlers::servers::restart_server),
        )
        .route("/servers/:name", delete(handlers::servers::delete_server))
        .route("/sessions", get(handlers::sessions::list_sessions))
        .route("/sessions/:id", get(handlers::sessions::get_session))
        .route(
            "/sessions/:id",
            delete(handlers::sessions::terminate_session),
        )
        .route(
            "/sessions/:id/message",
            post(handlers::sessions::send_message),
        )
        .route("/users", get(handlers::users::list_users))
        .route("/users/:id", get(handlers::users::get_user))
        .route("/users", post(handlers::users::create_user))
        .route("/users/:id", put(handlers::users::update_user))
        .route("/users/:id", delete(handlers::users::delete_user))
        .route("/users/:id/password", put(handlers::users::change_password))
        .route("/profile", get(handlers::users::get_profile))
        .route("/auth/logout", post(handlers::auth::logout))
        .route("/apps", get(handlers::apps::list_apps))
        .route("/apps", post(handlers::apps::create_app))
        .route("/apps/:id", get(handlers::apps::get_app))
        .route("/apps/:id", put(handlers::apps::update_app))
        .route("/apps/:id", delete(handlers::apps::delete_app))
        .route("/apps/:id/enable", post(handlers::apps::enable_app))
        .route("/apps/:id/disable", post(handlers::apps::disable_app))
        .route(
            "/apps/:id/launch/:user_id",
            post(handlers::apps::launch_app),
        )
        .route("/app-instances", get(handlers::apps::list_app_instances))
        .route(
            "/app-instances/:instance_id",
            delete(handlers::apps::terminate_app_instance),
        )
        .route("/system/status", get(handlers::system::system_status))
        .route("/system/logs", get(handlers::system::get_logs))
        .layer(middleware::from_fn(handlers::auth::auth_middleware));

    // Admin-only routes
    let admin_routes = Router::new()
        .route(
            "/admin/system/config",
            get(handlers::admin::get_system_config),
        )
        .route(
            "/admin/system/config",
            put(handlers::admin::update_system_config),
        )
        .route("/admin/audit", get(handlers::admin::get_audit_logs))
        .layer(middleware::from_fn(handlers::auth::admin_middleware));

    // Combine all routes
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(admin_routes)
}

/// Configure CORS middleware based on application config
fn cors_layer(app_state: &AppState) -> CorsLayer {
    if app_state.config.enable_cors {
        

        if app_state.config.cors_origins.contains(&"*".to_string()) {
            // Allow any origin
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
                .max_age(Duration::from_secs(3600))
        } else {
            // Allow specific origins
            let mut layer = CorsLayer::new();
            for origin in &app_state.config.cors_origins {
                // This is a simplification - in production you'd want to parse and validate these origins
                layer = layer.allow_origin(origin.parse::<axum::http::HeaderValue>().unwrap());
            }
            layer
                .allow_methods(Any)
                .allow_headers(Any)
                .max_age(Duration::from_secs(3600))
        }
    } else {
        // CORS disabled
        CorsLayer::new()
    }
}

/// Configure compression middleware
fn compression_layer(app_state: &AppState) -> CompressionLayer {
    if app_state.config.enable_compression {
        CompressionLayer::new()
    } else {
        CompressionLayer::new()
            .no_br()
            .no_gzip()
            .no_deflate()
            .no_zstd()
    }
}
