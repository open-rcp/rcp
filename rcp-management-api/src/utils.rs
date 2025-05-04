// Utility functions and helpers for the management API

/// Parse a Thing ID from SurrealDB to extract just the ID part
pub fn parse_thing_id(thing_id: &str) -> Option<String> {
    thing_id.split(':').nth(1).map(String::from)
}

/// Generate a timestamp for the current time
pub fn current_timestamp() -> String {
    use time::OffsetDateTime;
    OffsetDateTime::now_utc().to_string()
}

/// Hash a password using bcrypt
pub fn hash_password(password: &str) -> Result<String, String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| format!("Password hashing error: {}", e))
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    bcrypt::verify(password, hash).map_err(|e| format!("Password verification error: {}", e))
}
