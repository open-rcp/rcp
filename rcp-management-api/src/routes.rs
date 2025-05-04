use actix_web::web;
use actix_web::web::ServiceConfig;

// Import server handlers from the handlers module
use crate::handlers::auth as auth_handlers;
use crate::handlers::config as config_handlers;
use crate::handlers::logs as log_handlers;
use crate::handlers::server as server_handlers;
use crate::handlers::sessions as session_handlers;
use crate::handlers::users as user_handlers;

/// Configure all routes for the API
pub fn configure_routes(config: &mut ServiceConfig) {
    config
        // Server management routes
        .service(
            web::scope("/server")
                .route("/status", web::get().to(server_handlers::get_status))
                .route("/start", web::post().to(server_handlers::start_server))
                .route("/stop", web::post().to(server_handlers::stop_server))
                .route("/restart", web::post().to(server_handlers::restart_server)),
        )
        // Authentication routes
        .service(
            web::scope("/auth")
                .route("/login", web::post().to(auth_handlers::login))
                .route("/logout", web::post().to(auth_handlers::logout))
                .route("/me", web::get().to(auth_handlers::get_current_user)),
        )
        // User management routes
        .service(
            web::scope("/users")
                .route("", web::get().to(user_handlers::get_users))
                .route("", web::post().to(user_handlers::create_user))
                .route("/{id}", web::get().to(user_handlers::get_user))
                .route("/{id}", web::put().to(user_handlers::update_user))
                .route("/{id}", web::delete().to(user_handlers::delete_user)),
        )
        // Session management routes
        .service(
            web::scope("/sessions")
                .route("", web::get().to(session_handlers::get_sessions))
                .route("/{id}", web::get().to(session_handlers::get_session))
                .route(
                    "/{id}",
                    web::delete().to(session_handlers::terminate_session),
                ),
        )
        // Configuration management
        .service(
            web::scope("/config")
                .route("", web::get().to(config_handlers::get_config))
                .route("", web::put().to(config_handlers::update_config)),
        )
        // Log management
        .service(web::scope("/logs").route("", web::get().to(log_handlers::get_logs)));
}
