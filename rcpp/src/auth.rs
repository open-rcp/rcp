//! # Authentication Module for RCP
//!
//! This module provides authentication mechanisms for the Rust/Remote Control Protocol,
//! supporting various authentication methods suitable for both individual users
//! and enterprise deployments. The authentication system is designed to be:
//!
//! - Secure by default
//! - Flexible for different deployment scenarios
//! - Extensible for future authentication methods
//! - Compatible with clustering and multi-server deployments

use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Length of challenge bytes used in authentication
pub const CHALLENGE_LENGTH: usize = 32;

/// Length of salt bytes for password hashing
pub const SALT_LENGTH: usize = 16;

/// Authentication method types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthMethod {
    /// Pre-shared key authentication
    PreSharedKey,

    /// Public key authentication
    PublicKey,

    /// Two-factor authentication
    TwoFactor,

    /// Username and password authentication
    Password(String, String),
}

/// Authentication payload for client-server handshake
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthPayload {
    /// Client ID (UUID)
    pub client_id: Uuid,

    /// Client name
    pub client_name: String,

    /// Authentication method
    pub auth_method: AuthMethod,

    /// Authentication data (depends on method)
    #[serde(with = "serde_bytes")]
    pub auth_data: Vec<u8>,
}

/// Authentication challenge from server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthChallenge {
    /// Random challenge bytes
    #[serde(with = "serde_bytes")]
    pub challenge: Vec<u8>,

    /// Salt for password hashing
    #[serde(with = "serde_bytes")]
    pub salt: Vec<u8>,
}

/// Authentication response from client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    /// Client ID (UUID)
    pub client_id: Uuid,

    /// Response to challenge
    #[serde(with = "serde_bytes")]
    pub response: Vec<u8>,
}

/// Session information returned after successful authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    /// Session ID
    pub session_id: Uuid,

    /// Session expiration time (in seconds since UNIX epoch)
    pub expires_at: u64,

    /// Available permissions
    pub permissions: Vec<String>,
}

/// Authentication manager
pub struct Auth;

impl Auth {
    /// Generate a new random challenge
    pub fn generate_challenge() -> AuthChallenge {
        let mut challenge = vec![0u8; CHALLENGE_LENGTH];
        let mut salt = vec![0u8; SALT_LENGTH];

        let mut rng = rand::rng();
        rng.fill_bytes(&mut challenge);
        rng.fill_bytes(&mut salt);

        AuthChallenge { challenge, salt }
    }

    /// Verify a password against a challenge using the pre-shared key method
    pub fn verify_psk(password: &str, challenge: &[u8], salt: &[u8], response: &[u8]) -> bool {
        let expected = Self::compute_psk_response(password, challenge, salt);
        crypto_compare(&expected, response)
    }

    /// Compute the response to a challenge using the pre-shared key method
    pub fn compute_psk_response(password: &str, challenge: &[u8], salt: &[u8]) -> Vec<u8> {
        // First round: hash(password + salt)
        let mut hasher1 = Sha256::new();
        hasher1.update(password.as_bytes());
        hasher1.update(salt);
        let password_hash = hasher1.finalize();

        // Second round: hash(password_hash + challenge)
        let mut hasher2 = Sha256::new();
        hasher2.update(password_hash);
        hasher2.update(challenge);

        hasher2.finalize().to_vec()
    }

    /// Create a new session
    pub fn create_session(permissions: Vec<String>, expires_in_secs: u64) -> SessionInfo {
        let session_id = Uuid::new_v4();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        SessionInfo {
            session_id,
            expires_at: now + expires_in_secs,
            permissions,
        }
    }
}

/// Compare two byte slices in constant time to prevent timing attacks
fn crypto_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }

    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_generation() {
        let challenge = Auth::generate_challenge();
        assert_eq!(challenge.challenge.len(), CHALLENGE_LENGTH);
        assert_eq!(challenge.salt.len(), SALT_LENGTH);
    }

    #[test]
    fn test_psk_authentication() {
        let password = "secure_password";
        let challenge = Auth::generate_challenge();

        let response = Auth::compute_psk_response(password, &challenge.challenge, &challenge.salt);
        assert!(Auth::verify_psk(
            password,
            &challenge.challenge,
            &challenge.salt,
            &response
        ));

        // Wrong password should fail
        assert!(!Auth::verify_psk(
            "wrong_password",
            &challenge.challenge,
            &challenge.salt,
            &response
        ));

        // Tampered response should fail
        let mut tampered = response.clone();
        tampered[0] ^= 1;
        assert!(!Auth::verify_psk(
            password,
            &challenge.challenge,
            &challenge.salt,
            &tampered
        ));
    }

    #[test]
    fn test_session_creation() {
        let permissions = vec![
            "display".to_string(),
            "input".to_string(),
            "audio".to_string(),
        ];
        let session = Auth::create_session(permissions.clone(), 3600);

        assert_eq!(session.permissions, permissions);

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        assert!(session.expires_at > now);
        assert!(session.expires_at <= now + 3600 + 1); // Add 1 for potential timing differences
    }
}
