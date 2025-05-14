// filepath: /Volumes/EXT/repos/open-rcp/rcp/rcpp/tests/auth_test.rs
use rcpp::*;

#[test]
fn test_auth_method_equality() {
    // Test that authentication methods compare correctly
    let method1 = AuthMethod::PreSharedKey;
    let method2 = AuthMethod::PreSharedKey;
    let method3 = AuthMethod::PublicKey;

    assert_eq!(method1, method2);
    assert_ne!(method1, method3);
}
