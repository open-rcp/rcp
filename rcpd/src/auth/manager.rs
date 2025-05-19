use crate::auth::factory::{AuthConfig, AuthProviderFactory, AuthProviderType};
use crate::auth::provider::AuthProvider;
use crate::server::user::{User, UserRole};

use anyhow::{anyhow, Result};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Authentication manager that uses the configured provider
pub struct AuthManager {
    /// The authentication configuration
    pub config: AuthConfig,

    /// The active authentication provider
    pub provider: Arc<RwLock<Box<dyn AuthProvider>>>,

    /// Whether the provider has been initialized
    pub initialized: bool,
}

impl AuthManager {
    /// Create a new authentication manager with the specified configuration
    pub async fn new(config: AuthConfig) -> Result<Self> {
        let provider = AuthProviderFactory::create_provider(&config)?;

        Ok(Self {
            config,
            provider: Arc::new(RwLock::new(provider)),
            initialized: false,
        })
    }

    /// Initialize the authentication manager
    pub async fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }

        let mut provider = self.provider.write().await;
        provider.initialize().await?;
        self.initialized = true;

        info!(
            "Authentication manager initialized with provider: {}",
            provider.name()
        );
        Ok(())
    }

    /// Validate credentials for a user
    pub async fn validate_credentials(
        &self,
        username: &str,
        credentials: &[u8],
        method: &str,
    ) -> Result<bool> {
        let provider = self.provider.read().await;

        match provider
            .validate_credentials(username, credentials, method)
            .await
        {
            Ok(valid) => Ok(valid),
            Err(e) => {
                error!("Error validating credentials: {}", e);

                // If fallback is enabled and we're using native auth, try internal auth
                if self.config.fallback_to_internal && provider.name().contains("native") {
                    warn!("Native authentication failed, falling back to internal authentication");

                    // Create a temporary internal provider for fallback
                    // In a real implementation, this would be optimized to avoid creating a new provider each time
                    let mut fallback_config = self.config.clone();
                    fallback_config.provider = AuthProviderType::Internal;

                    match AuthProviderFactory::create_provider(&fallback_config) {
                        Ok(fallback_provider) => {
                            // Try validating with the fallback provider
                            match fallback_provider
                                .validate_credentials(username, credentials, method)
                                .await
                            {
                                Ok(valid) => Ok(valid),
                                Err(fallback_err) => {
                                    warn!("Fallback authentication also failed: {}", fallback_err);
                                    Ok(false)
                                }
                            }
                        }
                        Err(provider_err) => {
                            warn!("Failed to create fallback provider: {}", provider_err);
                            Ok(false)
                        }
                    }
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Get a user by their username
    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let provider = self.provider.read().await;
        provider.get_user_by_username(username).await
    }

    /// Check if a user has the specified permission
    pub async fn has_permission(&self, user: &User, permission: &str) -> Result<bool> {
        let provider = self.provider.read().await;
        provider.has_permission(user, permission).await
    }

    /// Get all permissions for a user
    pub async fn get_permissions(&self, user: &User) -> Result<Vec<String>> {
        let provider = self.provider.read().await;
        provider.get_permissions(user).await
    }
}
