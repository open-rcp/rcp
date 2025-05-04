use rcp_management_api::app;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load environment variables from .env file if present
    dotenv::dotenv().ok();
    
    // Get port from environment or use default
    let port = std::env::var("RCP_MANAGEMENT_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Starting RCP Management API server with SurrealDB on {}", addr);
    
    // Build and start the application
    let app = app::create_app().await?;
    
    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}