use crate::error::{ApiError, ApiResult};
use crate::models::User;
use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // Subject (user ID)
    pub username: String, // Username
    pub role: String,     // User role
    pub exp: i64,         // Expiration time
    pub iat: i64,         // Issued at
}

impl Claims {
    pub fn new(user: &User, expiration_hours: i64) -> ApiResult<Self> {
        let user_id = user
            .get_id()
            .ok_or_else(|| ApiError::Internal("User has no ID".to_string()))?;

        let now = Utc::now();
        let expiration = now + Duration::hours(expiration_hours);

        Ok(Self {
            sub: user_id,
            username: user.username.clone(),
            role: user.role.clone(),
            iat: now.timestamp(),
            exp: expiration.timestamp(),
        })
    }
}

// JWT authentication service
pub struct JwtAuth {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

// Global JWT instance
static JWT_AUTH: OnceLock<JwtAuth> = OnceLock::new();

impl JwtAuth {
    pub fn global() -> &'static Self {
        JWT_AUTH.get_or_init(|| {
            // In a real application, this would be loaded from environment variables
            let secret = std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "default_jwt_secret_for_development".to_string());

            Self::new(secret.as_bytes())
        })
    }

    pub fn new(secret: &[u8]) -> Self {
        let encoding_key = EncodingKey::from_secret(secret);
        let decoding_key = DecodingKey::from_secret(secret);

        Self {
            encoding_key,
            decoding_key,
        }
    }

    pub fn create_token(&self, claims: &Claims) -> ApiResult<String> {
        encode(&Header::default(), claims, &self.encoding_key)
            .map_err(|e| ApiError::Internal(format!("Failed to create JWT token: {}", e)))
    }

    pub fn validate_token(&self, token: &str) -> ApiResult<Claims> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| ApiError::Authentication(format!("Invalid token: {}", e)))
    }
}

// Extractor for authenticated users
pub struct AuthUser {
    pub user_id: String,
    pub username: String,
    pub role: String,
}

impl AuthUser {
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the Authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
                .await
                .map_err(|_| {
                    ApiError::Authentication("Missing or invalid authorization header".to_string())
                })?;

        // Validate the token and extract claims
        let claims = JwtAuth::global().validate_token(bearer.token())?;

        // Return the authenticated user
        Ok(AuthUser {
            user_id: claims.sub,
            username: claims.username,
            role: claims.role,
        })
    }
}

// Helper trait for User model
pub trait WithId {
    fn get_id(&self) -> Option<String>;
}
