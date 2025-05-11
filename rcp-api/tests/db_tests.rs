use chrono::{Duration, Utc};
use rcp_api::db;
use sqlx::{query, Row};
use tokio::test;

/// Test initializing the database
#[test]
async fn test_init_db() {
    // Initialize in-memory database
    let result = db::init_db("sqlite::memory:").await;

    // Should succeed
    assert!(result.is_ok());

    // Get the database pool
    let pool = result.unwrap();

    // Check that tables exist by querying them
    let api_tokens_query =
        "SELECT name FROM sqlite_master WHERE type='table' AND name='api_tokens';";
    let api_sessions_query =
        "SELECT name FROM sqlite_master WHERE type='table' AND name='api_sessions';";
    let audit_logs_query =
        "SELECT name FROM sqlite_master WHERE type='table' AND name='audit_logs';";

    // Execute queries
    let api_tokens_result: Option<(String,)> = query(api_tokens_query)
        .map(|row: sqlx::sqlite::SqliteRow| (row.get(0),))
        .fetch_optional(&pool)
        .await
        .unwrap();

    let api_sessions_result: Option<(String,)> = query(api_sessions_query)
        .map(|row: sqlx::sqlite::SqliteRow| (row.get(0),))
        .fetch_optional(&pool)
        .await
        .unwrap();

    let audit_logs_result: Option<(String,)> = query(audit_logs_query)
        .map(|row: sqlx::sqlite::SqliteRow| (row.get(0),))
        .fetch_optional(&pool)
        .await
        .unwrap();

    // All tables should exist
    assert!(api_tokens_result.is_some());
    assert!(api_sessions_result.is_some());
    assert!(audit_logs_result.is_some());
}

/// Test creating and validating API tokens
#[test]
async fn test_create_token() {
    // Initialize in-memory database
    let pool = db::init_db("sqlite::memory:").await.unwrap();

    // Create a token valid for 10 minutes
    let token = db::create_token(&pool, 10).await.unwrap();

    // Check token was created with the right properties
    assert!(!token.token.is_empty());
    assert!(token.token.len() >= 32); // Token should be long enough for security

    // Check expiration time is in the future
    let now = Utc::now();
    assert!(token.expires_at > now);

    // Should be approximately 10 minutes in the future
    let duration = token.expires_at - now;
    let minutes = duration.num_minutes();
    assert!(minutes >= 9 && minutes <= 11); // Allow for slight timing differences
}

/// Test adding and retrieving audit logs
#[test]
async fn test_audit_log() {
    // Initialize in-memory database
    let pool = db::init_db("sqlite::memory:").await.unwrap();

    // Create a test user ID
    let user_id = "test-user-123";

    // Add an audit log entry
    let result = db::add_audit_log(
        &pool,
        Some(user_id),
        "test_action",
        Some("test_resource_type"),
        Some("resource-456"),
        Some("Additional test details"),
    )
    .await;

    // Should succeed
    assert!(result.is_ok());

    // Query the audit log
    let query = "SELECT user_id, action FROM audit_logs WHERE user_id = ?";
    let log: (String, String) = sqlx::query_as(query)
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    // Verify the log entry
    assert_eq!(log.0, user_id);
    assert_eq!(log.1, "test_action");
}

/// Test accessing a non-existent database
#[test]
async fn test_db_connection_error() {
    // Try to connect to a non-existent database
    let result = db::init_db("sqlite:///nonexistent/path/db.sqlite").await;

    // Should fail
    assert!(result.is_err());
}
