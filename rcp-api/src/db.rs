use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use anyhow::Result;
use uuid::Uuid;
use std::path::Path;
use std::fs;
use tracing::info;

/// User model
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub active: bool,
    pub created_at: String,
    pub last_login: Option<String>,
}

/// Initialize the database connection pool
pub async fn init_db_pool(database_url: &str) -> Result<SqlitePool> {
    // Ensure the database directory exists
    if let Some(path) = Path::new(database_url)
        .to_str()
        .and_then(|s| s.strip_prefix("sqlite://"))
    {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
    }

    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;
    
    info!("Connected to database: {}", database_url);
    
    // Initialize database schema
    init_db_schema(&pool).await?;
    
    Ok(pool)
}

/// Initialize database schema if needed
async fn init_db_schema(pool: &SqlitePool) -> Result<()> {
    // Enable foreign key constraints
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(pool)
        .await?;
    
    // Create tables
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL,
            active INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL,
            last_login TEXT
        );
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tokens (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            token TEXT NOT NULL,
            expires_at TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        );
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS server_configs (
            id TEXT PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            port INTEGER NOT NULL,
            max_connections INTEGER NOT NULL,
            tls_enabled INTEGER NOT NULL DEFAULT 0,
            cert_path TEXT,
            key_path TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS configurations (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            config_type TEXT NOT NULL,
            data TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            UNIQUE(name, config_type)
        );
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS audit_log (
            id TEXT PRIMARY KEY,
            user_id TEXT,
            action TEXT NOT NULL,
            entity_type TEXT,
            entity_id TEXT,
            details TEXT,
            timestamp TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE SET NULL
        );
        "#
    )
    .execute(pool)
    .await?;

    // Create initial admin user if not exists
    let admin_exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE role = 'admin'"
    )
    .fetch_one(pool)
    .await?;

    if admin_exists == 0 {
        let now = chrono::Utc::now().to_rfc3339();
        let id = Uuid::new_v4().to_string();
        
        // Default admin password is 'admin123' - this should be changed immediately in production
        // In a real app, use bcrypt or argon2 for password hashing
        let password_hash = "$2b$12$placeholder-hash-admin123";
        
        sqlx::query(
            r#"
            INSERT INTO users (id, username, password_hash, role, active, created_at)
            VALUES (?, ?, ?, 'admin', 1, ?)
            "#
        )
        .bind(&id)
        .bind("admin")
        .bind(password_hash)
        .bind(&now)
        .execute(pool)
        .await?;
        
        info!("Created initial admin user");
    }

    Ok(())
}

/// Check if a user with the given username exists
pub async fn user_exists(pool: &SqlitePool, username: &str) -> Result<bool> {
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE username = ?"
    )
    .bind(username)
    .fetch_one(pool)
    .await?;
    
    Ok(count > 0)
}

/// Get a user by username
pub async fn get_user_by_username(pool: &SqlitePool, username: &str) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, role, active, created_at, last_login
        FROM users WHERE username = ?
        "#,
        username
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(user)
}

/// Update user's last login time
pub async fn update_last_login(pool: &SqlitePool, user_id: &str) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        "UPDATE users SET last_login = ? WHERE id = ?"
    )
    .bind(&now)
    .bind(user_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Create a new token record
pub async fn create_token(
    pool: &SqlitePool, 
    user_id: &str,
    token: &str,
    expires_at: &str,
) -> Result<()> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"
        INSERT INTO tokens (id, user_id, token, expires_at, created_at)
        VALUES (?, ?, ?, ?, ?)
        "#
    )
    .bind(&id)
    .bind(user_id)
    .bind(token)
    .bind(expires_at)
    .bind(&now)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Add entry to audit log
pub async fn add_audit_log(
    pool: &SqlitePool,
    user_id: Option<&str>,
    action: &str,
    entity_type: Option<&str>,
    entity_id: Option<&str>,
    details: Option<&str>,
) -> Result<()> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"
        INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, details, timestamp)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&id)
    .bind(user_id)
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(details)
    .bind(&now)
    .execute(pool)
    .await?;
    
    Ok(())
}