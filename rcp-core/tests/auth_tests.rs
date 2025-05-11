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
