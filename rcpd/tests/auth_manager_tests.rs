use anyhow::Result;
use rcpd::auth::factory::{AuthConfig, AuthProviderType, NativeAuthConfig};
use rcpd::auth::manager::AuthManager;
use rcpd::auth::mock_provider::MockAuthProvider;
use rcpd::server::user::{User, UserRole};
use std::collections::HashMap;
use tokio::test;
use uuid::Uuid;

#[test]
async fn test_auth_manager_creation() -> Result<()> {
    // Create a test auth configuration with the mock provider
    let mut auth_config = create_test_auth_config();
    auth_config.provider = AuthProviderType::Mock;

    // Create the authentication manager
    let manager = AuthManager::new(auth_config).await?;

    // Verify it was created successfully
    assert!(!manager.initialized);

    Ok(())
}

#[test]
async fn test_auth_manager_initialization() -> Result<()> {
    // Create a test auth configuration with the mock provider
    let mut auth_config = create_test_auth_config();
    auth_config.provider = AuthProviderType::Mock;

    // Create and initialize the authentication manager
    let mut manager = AuthManager::new(auth_config).await?;
    manager.initialize().await?;

    // Verify it was initialized
    assert!(manager.initialized);

    Ok(())
}

#[test]
async fn test_auth_manager_validate_credentials() -> Result<()> {
    // Create a mock provider
    let provider = MockAuthProvider::new()
        .with_user(create_test_user())
        .with_credential("testuser", b"password123");

    // Create a test auth configuration with the mock provider
    let mut auth_config = create_test_auth_config();
    auth_config.provider = AuthProviderType::Mock;

    // Create the authentication manager with our mock provider
    let mut manager = AuthManager::new(auth_config).await?;
    // Replace the provider with our configured mock
    manager.provider = std::sync::Arc::new(tokio::sync::RwLock::new(Box::new(provider)));
    manager.initialize().await?;

    // Test valid credentials
    let result = manager
        .validate_credentials("testuser", b"password123", "password")
        .await?;
    assert!(result, "Valid credentials should be accepted");

    // Test invalid credentials
    let result = manager
        .validate_credentials("testuser", b"wrongpassword", "password")
        .await?;
    assert!(!result, "Invalid credentials should be rejected");

    // Test nonexistent user
    let result = manager
        .validate_credentials("nonexistent", b"password123", "password")
        .await?;
    assert!(!result, "Nonexistent user should be rejected");

    Ok(())
}

#[test]
async fn test_auth_manager_has_permission() -> Result<()> {
    // Create a mock provider with permissions
    let provider = MockAuthProvider::new()
        .with_user(create_test_user())
        .with_permission("testuser", "app:safari")
        .with_permission("testuser", "connect:*");

    // Create a test auth configuration with the mock provider
    let mut auth_config = create_test_auth_config();
    auth_config.provider = AuthProviderType::Mock;

    // Create the authentication manager with our mock provider
    let mut manager = AuthManager::new(auth_config).await?;
    // Replace the provider with our configured mock
    manager.provider = std::sync::Arc::new(tokio::sync::RwLock::new(Box::new(provider)));
    manager.initialize().await?;

    // Get the test user
    let user = create_test_user();

    // Test permissions the user has
    let result = manager.has_permission(&user, "app:safari").await?;
    assert!(result, "User should have the app:safari permission");

    let result = manager.has_permission(&user, "connect:basic").await?;
    assert!(
        result,
        "User should have the connect:basic permission via wildcard"
    );

    // Test permissions the user doesn't have
    let result = manager.has_permission(&user, "admin:users").await?;
    assert!(!result, "User should not have admin permissions");

    Ok(())
}

#[test]
async fn test_auth_manager_get_permissions() -> Result<()> {
    // Create a mock provider with permissions
    let provider = MockAuthProvider::new()
        .with_user(create_test_user())
        .with_permission("testuser", "app:safari")
        .with_permission("testuser", "connect:*");

    // Create a test auth configuration with the mock provider
    let mut auth_config = create_test_auth_config();
    auth_config.provider = AuthProviderType::Mock;

    // Create the authentication manager with our mock provider
    let mut manager = AuthManager::new(auth_config).await?;
    // Replace the provider with our configured mock
    manager.provider = std::sync::Arc::new(tokio::sync::RwLock::new(Box::new(provider)));
    manager.initialize().await?;

    // Get the test user
    let user = create_test_user();

    // Get all permissions
    let permissions = manager.get_permissions(&user).await?;

    // Verify the permissions
    assert!(
        permissions.contains(&"app:safari".to_string()),
        "Permissions should include app:safari"
    );
    assert!(
        permissions.contains(&"connect:*".to_string()),
        "Permissions should include connect:*"
    );
    assert_eq!(
        permissions.len(),
        2,
        "User should have exactly 2 permissions"
    );

    Ok(())
}

/// Create a test user
fn create_test_user() -> User {
    User {
        id: Uuid::new_v4(),
        username: "testuser".to_string(),
        full_name: Some("Test User".to_string()),
        email: Some("test@example.com".to_string()),
        role: UserRole::User,
        password_hash: "hash".to_string(),
        created_at: "2023-01-01T00:00:00Z".to_string(),
        updated_at: "2023-01-01T00:00:00Z".to_string(),
    }
}

/// Create test auth configuration
fn create_test_auth_config() -> AuthConfig {
    AuthConfig {
        provider: AuthProviderType::Native,
        required: true,
        psk: Some("testkey".to_string()),
        fallback_to_internal: true,
        native: NativeAuthConfig {
            allow_all_users: false,
            require_group: Some("staff".to_string()),
            permission_mapping: true,
            admin_groups: vec!["admin".to_string(), "wheel".to_string()],
            permission_mappings: HashMap::new(),
        },
        ldap: HashMap::new(),
        oauth: HashMap::new(),
    }
}
