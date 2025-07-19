//! BearDog secret management
//!
//! This module handles secure secret management for BearDog,
//! including API keys, database credentials, and other sensitive data.
//!
//! **NOTE**: No JWT secrets as BearDog uses decentralized Ed25519 authentication.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use tokio::time::{Duration, Instant};

use crate::core::BearDogResult;

/// Secret source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretSourceConfig {
    /// Primary secret source
    pub primary_source: SecretSource,
    /// Fallback secret sources
    pub fallback_sources: Vec<SecretSource>,
    /// Whether to cache secrets in memory
    pub cache_secrets: bool,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
}

/// Secret source types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecretSource {
    /// Environment variables
    Environment,
    /// File-based secrets
    File(FileSecretConfig),
    /// HashiCorp Vault
    Vault(VaultSecretConfig),
    /// AWS Secrets Manager
    AwsSecretsManager(AwsSecretsConfig),
    /// Azure Key Vault
    AzureKeyVault(AzureKeyVaultConfig),
}

/// File-based secret configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSecretConfig {
    /// Base path for secret files
    pub base_path: String,
    /// File permissions (Unix only)
    pub file_permissions: Option<u32>,
    /// Enable file watching for changes
    pub watch_for_changes: bool,
}

/// HashiCorp Vault configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultSecretConfig {
    /// Vault server URL
    pub url: String,
    /// Authentication method
    pub auth_method: VaultAuthMethod,
    /// Secret mount path
    pub mount_path: String,
    /// Connection timeout
    pub timeout_seconds: u64,
}

/// Vault authentication methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VaultAuthMethod {
    /// Token-based authentication
    Token(String),
    /// AppRole authentication
    AppRole { role_id: String, secret_id: String },
    /// Kubernetes authentication
    Kubernetes { role: String, jwt_path: String },
}

/// AWS Secrets Manager configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsSecretsConfig {
    /// AWS region
    pub region: String,
    /// Secret name prefix
    pub secret_prefix: String,
    /// Access key ID (optional, can use IAM roles)
    pub access_key_id: Option<String>,
    /// Secret access key (optional, can use IAM roles)
    pub secret_access_key: Option<String>,
}

/// Azure Key Vault configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureKeyVaultConfig {
    /// Key vault URL
    pub vault_url: String,
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Tenant ID
    pub tenant_id: String,
}

/// Secret manager for handling various secret sources
pub struct SecretManager {
    /// Configuration
    pub config: SecretSourceConfig,
    /// In-memory cache
    cache: HashMap<String, CachedSecret>,
    /// Last cache update time
    last_cache_update: Instant,
}

/// Cached secret with TTL
#[derive(Debug, Clone)]
struct CachedSecret {
    /// Secret value
    value: String,
    /// When the secret was cached
    cached_at: Instant,
    /// TTL for the secret
    ttl: Duration,
}

impl SecretManager {
    /// Create a new secret manager
    pub fn new(config: SecretSourceConfig) -> Self {
        Self {
            config,
            cache: HashMap::new(),
            last_cache_update: Instant::now(),
        }
    }

    /// Get a secret by key
    pub async fn get_secret(&mut self, key: &str) -> BearDogResult<String> {
        // Check cache first if enabled
        if self.config.cache_secrets {
            if let Some(cached) = self.cache.get(key) {
                if cached.cached_at.elapsed() < cached.ttl {
                    return Ok(cached.value.clone());
                }
            }
        }

        // Try primary source
        if let Ok(secret) = self
            .get_secret_from_source(&self.config.primary_source, key)
            .await
        {
            self.cache_secret(key, &secret).await;
            return Ok(secret);
        }

        // Try fallback sources
        for source in &self.config.fallback_sources {
            if let Ok(secret) = self.get_secret_from_source(source, key).await {
                self.cache_secret(key, &secret).await;
                return Ok(secret);
            }
        }

        Err(anyhow::anyhow!("Secret not found: {}", key))
    }

    /// Get secret from a specific source
    async fn get_secret_from_source(
        &self,
        source: &SecretSource,
        key: &str,
    ) -> BearDogResult<String> {
        match source {
            SecretSource::Environment => self.get_secret_from_env(key).await,
            SecretSource::File(config) => self.get_secret_from_file(config, key).await,
            SecretSource::Vault(_config) => {
                // TODO: Implement Vault integration
                Err(anyhow::anyhow!("Vault integration not yet implemented"))
            }
            SecretSource::AwsSecretsManager(_config) => {
                // TODO: Implement AWS Secrets Manager integration
                Err(anyhow::anyhow!(
                    "AWS Secrets Manager integration not yet implemented"
                ))
            }
            SecretSource::AzureKeyVault(_config) => {
                // TODO: Implement Azure Key Vault integration
                Err(anyhow::anyhow!(
                    "Azure Key Vault integration not yet implemented"
                ))
            }
        }
    }

    /// Get secret from environment variables
    async fn get_secret_from_env(&self, key: &str) -> BearDogResult<String> {
        let env_key = match key {
            secret_keys::DATABASE_PASSWORD => "DATABASE_PASSWORD",
            secret_keys::ENCRYPTION_KEY => "ENCRYPTION_KEY",
            secret_keys::API_KEY => "API_KEY",
            _ => return Err(anyhow::anyhow!("Unknown secret key: {}", key)),
        };

        env::var(format!("BEARDOG_{env_key}"))
            .or_else(|_| env::var(env_key))
            .map_err(|_| anyhow::anyhow!("Environment variable not found: {}", env_key))
    }

    /// Get secret from file
    async fn get_secret_from_file(
        &self,
        config: &FileSecretConfig,
        key: &str,
    ) -> BearDogResult<String> {
        let file_path = format!("{}/{}.secret", config.base_path, key);

        if !Path::new(&file_path).exists() {
            return Err(anyhow::anyhow!("Secret file not found: {}", file_path));
        }

        let content = fs::read_to_string(&file_path)
            .map_err(|e| anyhow::anyhow!("Failed to read secret file: {}", e))?;

        Ok(content.trim().to_string())
    }

    /// Cache a secret
    async fn cache_secret(&mut self, key: &str, value: &str) {
        if self.config.cache_secrets {
            let cached_secret = CachedSecret {
                value: value.to_string(),
                cached_at: Instant::now(),
                ttl: Duration::from_secs(self.config.cache_ttl_seconds),
            };
            self.cache.insert(key.to_string(), cached_secret);
        }
    }

    /// Clear expired cache entries
    pub async fn cleanup_cache(&mut self) {
        let now = Instant::now();
        self.cache
            .retain(|_, cached| now.duration_since(cached.cached_at) < cached.ttl);
        self.last_cache_update = now;
    }
}

/// Default implementation
impl Default for SecretSourceConfig {
    fn default() -> Self {
        Self {
            primary_source: SecretSource::Environment,
            fallback_sources: vec![],
            cache_secrets: true,
            cache_ttl_seconds: 300, // 5 minutes
        }
    }
}

/// Secret key constants - NO JWT KEYS (decentralized auth only)
pub mod secret_keys {
    /// Database password
    pub const DATABASE_PASSWORD: &str = "database_password";
    /// Encryption key for data at rest
    pub const ENCRYPTION_KEY: &str = "encryption_key";
    /// API key for external services
    pub const API_KEY: &str = "api_key";
}

/// Environment variable constants - NO JWT VARS (decentralized auth only)
pub mod env_vars {
    /// Database password environment variable
    pub const BEARDOG_DATABASE_PASSWORD: &str = "BEARDOG_DATABASE_PASSWORD";
    /// Encryption key environment variable
    pub const BEARDOG_ENCRYPTION_KEY: &str = "BEARDOG_ENCRYPTION_KEY";
    /// API key environment variable
    pub const BEARDOG_API_KEY: &str = "BEARDOG_API_KEY";
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use tempfile::tempdir;

    #[tokio::test]
    #[serial_test::serial]
    async fn test_secret_manager_environment() {
        // Set the environment variable for this test
        env::set_var("BEARDOG_DATABASE_PASSWORD", "test-db-password");

        let config = SecretSourceConfig {
            primary_source: SecretSource::Environment,
            fallback_sources: vec![],
            cache_secrets: false,
            cache_ttl_seconds: 0,
        };

        let mut manager = SecretManager::new(config);
        let secret = manager
            .get_secret(secret_keys::DATABASE_PASSWORD)
            .await
            .unwrap();
        assert_eq!(secret, "test-db-password");

        // Clean up environment variable for other tests
        env::remove_var("BEARDOG_DATABASE_PASSWORD");
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_secret_manager_file() {
        // Ensure environment variable is not set
        env::remove_var("BEARDOG_DATABASE_PASSWORD");

        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path().to_str().unwrap();

        // Create a secret file
        let secret_file = format!("{base_path}/database_password.secret");
        fs::write(&secret_file, "file-db-password").unwrap();

        let config = SecretSourceConfig {
            primary_source: SecretSource::File(FileSecretConfig {
                base_path: base_path.to_string(),
                file_permissions: None,
                watch_for_changes: false,
            }),
            fallback_sources: vec![],
            cache_secrets: false,
            cache_ttl_seconds: 0,
        };

        let mut manager = SecretManager::new(config);
        let secret = manager
            .get_secret(secret_keys::DATABASE_PASSWORD)
            .await
            .unwrap();
        assert_eq!(secret, "file-db-password");
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_secret_manager_fallback() {
        // Ensure environment variable is not set
        env::remove_var("BEARDOG_DATABASE_PASSWORD");

        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path().to_str().unwrap();

        // Create a secret file
        let secret_file = format!("{base_path}/database_password.secret");
        fs::write(&secret_file, "fallback-db-password").unwrap();

        let config = SecretSourceConfig {
            primary_source: SecretSource::Environment,
            fallback_sources: vec![SecretSource::File(FileSecretConfig {
                base_path: base_path.to_string(),
                file_permissions: None,
                watch_for_changes: false,
            })],
            cache_secrets: false,
            cache_ttl_seconds: 0,
        };

        let mut manager = SecretManager::new(config);
        let secret = manager
            .get_secret(secret_keys::DATABASE_PASSWORD)
            .await
            .unwrap();
        assert_eq!(secret, "fallback-db-password");
    }
}
