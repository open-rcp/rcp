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
pub use config::Config;
pub use db::init_database;
pub use error::{ApiError, ApiResult};

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use log::info;
use std::net::SocketAddr;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

/// Run the management API server with the provided configuration
pub async fn run_server(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database connection
    let db_pool = init_database(&config.database_url).await?;

    // Initialize SurrealDB as a fallback
    let surreal_db = db::init_surrealdb().await?;

    // Create shared application state
    let app_state = web::Data::new(AppState {
        db: surreal_db,
        db_pool,
        server_handle: config.server_handle,
        jwt_secret: config.jwt_secret.clone(),
    });

    // Set up address
    let addr = SocketAddr::new(
        config.host.parse().unwrap_or("127.0.0.1".parse().unwrap()),
        config.port,
    );

    info!(
        "Starting RCP Management API server at http://{}:{}",
        config.host, config.port
    );

    // Build and start the server
    HttpServer::new(move || {
        // Configure CORS for frontend access
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            // Serve the Svelte frontend from the static directory
            // First, try API routes
            .service(web::scope("/api").configure(routes::configure_routes))
            // Then try static files
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            // SPA fallback - redirect all other routes to index.html
            .default_service(web::route().to(handlers::spa_index_handler))
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
