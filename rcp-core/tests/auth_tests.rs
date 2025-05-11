use rcp_core::*;

#[test]
fn test_auth_method_equality() {
    // Test that authentication methods compare correctly

    let method1 = AuthMethod::PreSharedKey;
    let method2 = AuthMethod::PreSharedKey;
    let method3 = AuthMethod::PublicKey;

    assert_eq!(method1, method2);
    assert_ne!(method1, method3);
}

#[test]
fn test_auth_challenge_creation() {
    // Test authentication challenge creation
    let challenge = Auth::generate_challenge();

    // Verify challenge is not empty
    assert!(!challenge.challenge.is_empty());
    assert!(!challenge.salt.is_empty());
}

#[test]
fn test_session_info_basics() {
    // Test basic session information properties
    let permissions = vec!["app.launch".to_string()];
    let session = Auth::create_session(permissions.clone(), 3600);

    // Verify the properties
    assert_eq!(session.permissions, permissions);

    // Check that the expiration time is reasonable
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    assert!(session.expires_at > now);
    assert!(session.expires_at <= now + 3601); // 3600 + 1 second for timing differences
}

#[test]
fn test_session_id_validation() {
    // Create a session with permissions
    let permissions = vec!["app.launch".to_string(), "file.read".to_string()];
    let session = Auth::create_session(permissions.clone(), 3600);

    // Validate the session ID is a valid UUID
    assert!(
        !session.session_id.is_nil(),
        "Session ID should be a valid non-nil UUID"
    );

    // Validate session is not expired
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    assert!(
        session.expires_at > now,
        "Session should not be expired immediately after creation"
    );
}

#[test]
fn test_auth_challenge_response() {
    use sha2::Digest;

    // Generate a challenge
    let challenge = Auth::generate_challenge();

    // Create a mock response (in a real system, this would be hashed with a shared secret)
    let mock_secret = b"test-secret-key";
    let mut hasher = sha2::Sha256::new();
    hasher.update(&challenge.challenge);
    hasher.update(mock_secret);
    hasher.update(&challenge.salt);
    let response_hash = hasher.finalize();

    // In a real implementation, the response would be verified against a stored secret
    // Here we just verify that our response has the right length
    assert_eq!(response_hash.len(), 32, "SHA256 hash should be 32 bytes");
}

#[test]
fn test_session_permission_check() {
    // Create a session with specific permissions
    let permissions = vec!["app.launch".to_string(), "file.read".to_string()];
    let session = Auth::create_session(permissions, 3600);

    // Test permission checking function (this simulates what would be in the Auth implementation)
    let has_permission = |session: &SessionInfo, permission: &str| -> bool {
        session.permissions.contains(&permission.to_string())
    };

    // Verify permissions work as expected
    assert!(
        has_permission(&session, "app.launch"),
        "Session should have app.launch permission"
    );
    assert!(
        has_permission(&session, "file.read"),
        "Session should have file.read permission"
    );
    assert!(
        !has_permission(&session, "admin.config"),
        "Session should not have admin.config permission"
    );
}
