use surrealdb::engine::local::{Db, RocksDb, Mem};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use std::sync::Arc;
use tracing::info;

/// Database connection type that can be memory or persistent
pub enum DatabaseEngine {
    Memory,
    RocksDb,
}

/// Initialize the SurrealDB connection and set up namespaces/databases
pub async fn init_database() -> Result<Surreal<Db>, Box<dyn std::error::Error>> {
    // Load environment variables to determine which database engine to use
    let engine_type = std::env::var("DB_ENGINE").unwrap_or_else(|_| "memory".to_string());
    let db_path = std::env::var("DB_PATH").unwrap_or_else(|_| "rcp_data".to_string());
    
    // Connect to database based on configuration
    let db = match engine_type.to_lowercase().as_str() {
        "rocksdb" => {
            info!("Connecting to RocksDB database at {}", db_path);
            Surreal::new::<RocksDb>(db_path).await?
        },
        _ => {
            info!("Using in-memory database");
            Surreal::new::<Mem>(()).await?
        }
    };
    
    // Get credentials from environment variables
    let username = std::env::var("DB_USER").unwrap_or_else(|_| "root".to_string());
    let password = std::env::var("DB_PASS").unwrap_or_else(|_| "root".to_string());
    
    // Authenticate to the database
    db.signin(Root {
        username: &username,
        password: &password,
    }).await?;
    
    // Select namespace and database
    db.use_ns("rcp").use_db("management").await?;
    
    // Initialize database schema
    init_database_schema(&db).await?;
    
    info!("Database initialized successfully");
    Ok(db)
}

/// Set up the database schema and initial data if needed
async fn init_database_schema(db: &Surreal<Db>) -> Result<(), Box<dyn std::error::Error>> {
    // Define tables and their schemas
    let schema_queries = vec![
        // Users table
        "DEFINE TABLE user SCHEMAFULL;",
        "DEFINE FIELD username ON user TYPE string;",
        "DEFINE FIELD email ON user TYPE string;",
        "DEFINE FIELD password_hash ON user TYPE string;",
        "DEFINE FIELD role ON user TYPE string;",
        "DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now();",
        "DEFINE FIELD updated_at ON user TYPE datetime DEFAULT time::now();",
        "DEFINE INDEX user_username ON user COLUMNS username UNIQUE;",
        "DEFINE INDEX user_email ON user COLUMNS email UNIQUE;",
        
        // Servers table
        "DEFINE TABLE server SCHEMAFULL;",
        "DEFINE FIELD name ON server TYPE string;",
        "DEFINE FIELD host ON server TYPE string;",
        "DEFINE FIELD port ON server TYPE int;",
        "DEFINE FIELD status ON server TYPE string DEFAULT 'offline';",
        "DEFINE FIELD created_at ON server TYPE datetime DEFAULT time::now();",
        "DEFINE FIELD updated_at ON server TYPE datetime DEFAULT time::now();",
        
        // Sessions table
        "DEFINE TABLE session SCHEMAFULL;",
        "DEFINE FIELD server_id ON session TYPE record(server);",
        "DEFINE FIELD client_id ON session TYPE string;",
        "DEFINE FIELD client_ip ON session TYPE string;",
        "DEFINE FIELD started_at ON session TYPE datetime DEFAULT time::now();",
        "DEFINE FIELD last_activity ON session TYPE datetime DEFAULT time::now();",
        "DEFINE FIELD status ON session TYPE string DEFAULT 'active';",
        
        // Create a default admin user if none exists
        "CREATE user:admin SET username = 'admin', 
            email = 'admin@example.com', 
            password_hash = '$2b$10$3euPcmQFCiblsZeEu5s7p.9NDIG1darP.dJoe4Zj5zSqmZ9J9FKK2', 
            role = 'admin' 
            IF NOT EXISTS;",
    ];
    
    // Execute each query to set up the schema
    for query in schema_queries {
        db.query(query).await?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_db_connection() {
        let db = init_database().await.expect("Failed to connect to database");
        // Verify connection by running a simple query
        let result: Vec<surrealdb::sql::Thing> = db.select("user").await.expect("Failed to query");
        println!("Found {} users", result.len());
    }
}