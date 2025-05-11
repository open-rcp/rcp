use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite, Row};
use anyhow::Result;
use uuid::Uuid;
use std::path::Path;
use std::fs;
use tracing::info;

/// Database pool type alias
pub type DbPool = Pool<Sqlite>;

/// API Token for authentication
pub struct ApiToken {
    pub id: String,
    pub token_value: String,
    pub expires_at: String,
}

/// API Session for tracking
pub struct ApiSession {
    pub id: String,
    pub token_id: String,
    pub created_at: String,
    pub last_active: String,
}

/// Initialize the database
pub async fn init_db(db_url: &str) -> Result<DbPool> {
    // Check if we need to create the file
    if db_url.starts_with("sqlite:") {
        let path = db_url.strip_prefix("sqlite:").unwrap_or(db_url);
        
        // Create parent directories if they don't exist
        if let Some(parent) = Path::new(path).parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
    }

    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    info!("Database initialized successfully");
    Ok(pool)
}

/// Ping the database to check connection
pub async fn ping(pool: &DbPool) -> Result<()> {
    let _: (i64,) = sqlx::query_as("SELECT 1")
        .fetch_one(pool)
        .await?;
    
    Ok(())
}

/// Create an API token
pub async fn create_token(pool: &DbPool, expires_in_minutes: i64) -> Result<ApiToken> {
    let id = Uuid::new_v4().to_string();
    let token_value = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    let expires_at = now + chrono::Duration::minutes(expires_in_minutes);
    let expires_at_str = expires_at.to_rfc3339();
    
    sqlx::query(
        r#"
        INSERT INTO api_tokens (id, token_value, expires_at)
        VALUES (?, ?, ?)
        "#
    )
    .bind(&id)
    .bind(&token_value)
    .bind(&expires_at_str)
    .execute(pool)
    .await?;
    
    Ok(ApiToken {
        id,
        token_value,
        expires_at: expires_at_str,
    })
}

/// Get an API token by ID
pub async fn get_token_by_id(pool: &DbPool, token_id: &str) -> Result<Option<ApiToken>> {
    let row = sqlx::query(
        r#"
        SELECT id, token_value, expires_at
        FROM api_tokens
        WHERE id = ?
        "#
    )
    .bind(token_id)
    .fetch_optional(pool)
    .await?;
    
    match row {
        Some(row) => Ok(Some(ApiToken {
            id: row.get("id"),
            token_value: row.get("token_value"),
            expires_at: row.get("expires_at"),
        })),
        None => Ok(None),
    }
}

/// Get an API token by value
pub async fn get_token_by_value(pool: &DbPool, value: &str) -> Result<Option<ApiToken>> {
    let row = sqlx::query(
        r#"
        SELECT id, token_value, expires_at
        FROM api_tokens
        WHERE token_value = ?
        "#
    )
    .bind(value)
    .fetch_optional(pool)
    .await?;
    
    match row {
        Some(row) => Ok(Some(ApiToken {
            id: row.get("id"),
            token_value: row.get("token_value"),
            expires_at: row.get("expires_at"),
        })),
        None => Ok(None),
    }
}

/// Create a new session with an API token
pub async fn create_session(pool: &DbPool, token_id: &str) -> Result<ApiSession> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"
        INSERT INTO api_sessions (id, token_id, created_at, last_active)
        VALUES (?, ?, ?, ?)
        "#
    )
    .bind(&id)
    .bind(token_id)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;
    
    Ok(ApiSession {
        id,
        token_id: token_id.to_string(),
        created_at: now.clone(),
        last_active: now,
    })
}

/// Get a session by ID
pub async fn get_session(pool: &DbPool, session_id: &str) -> Result<Option<ApiSession>> {
    let row = sqlx::query(
        r#"
        SELECT id, token_id, created_at, last_active
        FROM api_sessions
        WHERE id = ?
        "#
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await?;
    
    match row {
        Some(row) => Ok(Some(ApiSession {
            id: row.get("id"),
            token_id: row.get("token_id"),
            created_at: row.get("created_at"),
            last_active: row.get("last_active"),
        })),
        None => Ok(None),
    }
}

/// Clean up expired sessions
pub async fn cleanup_expired_sessions(pool: &DbPool) -> Result<u64> {
    let result = sqlx::query(
        r#"
        DELETE FROM api_sessions 
        WHERE token_id IN (
            SELECT id FROM api_tokens 
            WHERE expires_at < datetime('now')
        )
        "#
    )
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected())
}

/// Add an entry to the audit log
pub async fn add_audit_log(
    pool: &DbPool,
    user_id: Option<&str>,
    action: &str,
    resource_type: Option<&str>,
    resource_id: Option<&str>,
    details: Option<&str>,
) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"
        INSERT INTO audit_logs (id, timestamp, user_id, action, resource_type, resource_id, details)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(Uuid::new_v4().to_string())
    .bind(now)
    .bind(user_id)
    .bind(action)
    .bind(resource_type)
    .bind(resource_id)
    .bind(details)
    .execute(pool)
    .await?;
    
    Ok(())
}