use rcp_api::db;
use tokio::test;

/// Test database initialization
#[test]
async fn test_db_init() {
    // Create an in-memory SQLite database for testing
    let db_url = "sqlite::memory:";
    
    // Initialize the database
    let result = db::init_db(db_url).await;
    assert!(result.is_ok(), "Database initialization should succeed");
}

/// Test token creation and retrieval
#[test]
async fn test_token_operations() {
    // Create an in-memory SQLite database for testing
    let db_url = "sqlite::memory:";
    let pool = db::init_db(db_url).await.expect("Failed to initialize database");
    
    // Create a token with 60-minute expiry
    let expires_in_minutes = 60;
    let token = db::create_token(&pool, expires_in_minutes)
        .await
        .expect("Failed to create token");
    
    // Verify the token
    assert!(!token.id.is_empty());
    assert!(!token.token_value.is_empty());
    assert!(!token.expires_at.is_empty());
    
    // Get the token by ID
    let retrieved_token = db::get_token_by_id(&pool, &token.id)
        .await
        .expect("Failed to get token")
        .expect("Token not found");
    
    // Verify retrieved token matches
    assert_eq!(retrieved_token.id, token.id);
    assert_eq!(retrieved_token.token_value, token.token_value);
    assert_eq!(retrieved_token.expires_at, token.expires_at);
    
    // Get token by value
    let retrieved_by_value = db::get_token_by_value(&pool, &token.token_value)
        .await
        .expect("Failed to get token by value")
        .expect("Token not found by value");
    
    assert_eq!(retrieved_by_value.id, token.id);
}

/// Test session creation and retrieval
#[test]
async fn test_session_operations() {
    // Create an in-memory SQLite database for testing
    let db_url = "sqlite::memory:";
    let pool = db::init_db(db_url).await.expect("Failed to initialize database");
    
    // Create a token first
    let expires_in_minutes = 60;
    let token = db::create_token(&pool, expires_in_minutes)
        .await
        .expect("Failed to create token");
    
    // Create a session
    let session = db::create_session(&pool, &token.id)
        .await
        .expect("Failed to create session");
    
    // Verify the session
    assert!(!session.id.is_empty());
    assert_eq!(session.token_id, token.id);
    assert!(!session.created_at.is_empty());
    assert!(!session.last_active.is_empty());
    
    // Get session by ID
    let retrieved_session = db::get_session(&pool, &session.id)
        .await
        .expect("Failed to get session")
        .expect("Session not found");
    
    assert_eq!(retrieved_session.id, session.id);
    assert_eq!(retrieved_session.token_id, session.token_id);
}

/// Test audit log functionality
#[test]
async fn test_audit_log() {
    // Create an in-memory SQLite database for testing
    let db_url = "sqlite::memory:";
    let pool = db::init_db(db_url).await.expect("Failed to initialize database");
    
    // Add an audit log entry
    let user_id = "test-user-123";
    let action = "login";
    let resource_type = Some("user");
    let resource_id = Some(user_id);
    let details = Some("Test login from 127.0.0.1");
    
    let result = db::add_audit_log(
        &pool,
        Some(user_id),
        action,
        resource_type,
        resource_id,
        details,
    )
    .await;
    
    assert!(result.is_ok(), "Adding audit log should succeed");
    
    // Get recent audit logs
    let logs = db::get_audit_logs(&pool, None, None, Some(10), Some(0))
        .await
        .expect("Failed to get audit logs");
    
    assert!(!logs.is_empty(), "Should have retrieved at least one audit log");
    assert_eq!(logs[0].action, action);
    assert_eq!(logs[0].user_id, Some(user_id.to_string()));
    assert_eq!(logs[0].resource_type, resource_type.map(|s| s.to_string()));
    assert_eq!(logs[0].resource_id, resource_id.map(|s| s.to_string()));
    assert_eq!(logs[0].details, details.map(|s| s.to_string()));
}
