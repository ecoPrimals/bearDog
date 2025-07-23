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

use anyhow::Result as BearDogResult;
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, Instant};
use tracing::{debug, info, warn};

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
            SecretSource::Vault(config) => self.get_secret_from_vault(config, key).await,
            SecretSource::AwsSecretsManager(config) => {
                self.get_secret_from_aws_secrets_manager(config, key).await
            }
            SecretSource::AzureKeyVault(config) => {
                self.get_secret_from_azure_key_vault(config, key).await
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

    /// Get secret from HashiCorp Vault
    async fn get_secret_from_vault(
        &self,
        config: &VaultSecretConfig,
        key: &str,
    ) -> BearDogResult<String> {
        info!("🔐 Retrieving secret from HashiCorp Vault: {}", key);

        // Build Vault API endpoint
        let endpoint = format!("{}/v1/{}/{}", config.url, config.mount_path, key);

        // Create HTTP client
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create HTTP client: {}", e))?;

        // Prepare request with authentication
        let mut request_builder = client.get(&endpoint);

        // Add authentication headers
        match &config.auth_method {
            VaultAuthMethod::Token(token) => {
                request_builder = request_builder.header("X-Vault-Token", token);
            }
            VaultAuthMethod::AppRole { role_id, secret_id } => {
                // For AppRole, we need to first authenticate to get a token
                let auth_endpoint = format!("{}/v1/auth/approle/login", config.url);
                let auth_payload = serde_json::json!({
                    "role_id": role_id,
                    "secret_id": secret_id
                });

                let auth_response = client
                    .post(&auth_endpoint)
                    .json(&auth_payload)
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("Vault AppRole authentication failed: {}", e))?;

                if !auth_response.status().is_success() {
                    return Err(anyhow::anyhow!(
                        "Vault AppRole authentication failed with status: {}",
                        auth_response.status()
                    ));
                }

                let auth_data: serde_json::Value = auth_response
                    .json()
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to parse Vault auth response: {}", e))?;

                let token = auth_data
                    .get("auth")
                    .and_then(|auth| auth.get("client_token"))
                    .and_then(|token| token.as_str())
                    .ok_or_else(|| anyhow::anyhow!("No client_token in Vault auth response"))?;

                request_builder = request_builder.header("X-Vault-Token", token);
            }
            VaultAuthMethod::Kubernetes {
                role: _,
                jwt_path: _,
            } => {
                return Err(anyhow::anyhow!(
                    "Kubernetes authentication not yet implemented"
                ));
            }
        }

        // Make the request
        let response = request_builder
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to fetch secret from Vault: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Vault API request failed with status: {}",
                response.status()
            ));
        }

        // Parse response
        let vault_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse Vault response: {}", e))?;

        // Extract secret value (supports both KV v1 and v2)
        let secret_value = vault_data
            .get("data")
            .and_then(|data| {
                // Try KV v2 format first (nested data)
                if let Some(nested_data) = data.get("data") {
                    nested_data.get("value").or_else(|| nested_data.get(key))
                } else {
                    // Fall back to KV v1 format (direct data)
                    data.get("value").or_else(|| data.get(key))
                }
            })
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Secret '{}' not found in Vault response", key))?;

        debug!("✅ Successfully retrieved secret from Vault: {}", key);
        Ok(secret_value.to_string())
    }

    /// Get secret from AWS Secrets Manager
    async fn get_secret_from_aws_secrets_manager(
        &self,
        config: &AwsSecretsConfig,
        key: &str,
    ) -> BearDogResult<String> {
        info!("🔐 Retrieving secret from AWS Secrets Manager: {}", key);

        // Build AWS Secrets Manager API endpoint
        let endpoint = format!(
            "https://secretsmanager.{}.amazonaws.com",
            config.region.as_str()
        );

        // Create HTTP client
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create HTTP client: {}", e))?;

        // Prepare AWS API request payload
        let payload = serde_json::json!({
            "SecretId": key
        });

        // Create AWS signature (simplified - in production use aws-sdk-rust or similar)
        let mut request_builder = client
            .post(&endpoint)
            .header("Content-Type", "application/x-amz-json-1.1")
            .header("X-Amz-Target", "secretsmanager.GetSecretValue");

        // Add authentication headers (simplified implementation)
        if let (Some(access_key), Some(_secret_key)) =
            (&config.access_key_id, &config.secret_access_key)
        {
            // In a real implementation, you would properly sign the request with AWS Signature V4
            // For now, we'll use basic authentication approach
            request_builder = request_builder
                .header(
                    "Authorization",
                    format!("AWS4-HMAC-SHA256 Credential={access_key}"),
                )
                .header(
                    "X-Amz-Date",
                    chrono::Utc::now().format("%Y%m%dT%H%M%SZ").to_string(),
                );

            warn!("⚠️ Using simplified AWS authentication - implement proper AWS Signature V4 for production");
        } else {
            warn!("⚠️ No AWS credentials provided - IAM role authentication not fully implemented");
            return Err(anyhow::anyhow!(
                "AWS authentication requires access keys or full AWS SDK integration"
            ));
        }

        // Make the request
        let response = request_builder.json(&payload).send().await.map_err(|e| {
            anyhow::anyhow!("Failed to fetch secret from AWS Secrets Manager: {}", e)
        })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "AWS Secrets Manager API request failed with status {}: {}",
                status,
                error_text
            ));
        }

        // Parse response
        let aws_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse AWS Secrets Manager response: {}", e))?;

        // Extract secret value
        let secret_value = aws_data
            .get("SecretString")
            .and_then(|v| v.as_str())
            .or_else(|| {
                // Handle binary secrets (base64 encoded)
                aws_data.get("SecretBinary").and_then(|v| v.as_str())
            })
            .ok_or_else(|| {
                anyhow::anyhow!("No SecretString or SecretBinary found in AWS response")
            })?;

        debug!(
            "✅ Successfully retrieved secret from AWS Secrets Manager: {}",
            key
        );
        Ok(secret_value.to_string())
    }

    /// Get secret from Azure Key Vault
    async fn get_secret_from_azure_key_vault(
        &self,
        config: &AzureKeyVaultConfig,
        key: &str,
    ) -> BearDogResult<String> {
        info!("🔐 Retrieving secret from Azure Key Vault: {}", key);

        // Build Azure Key Vault API endpoint
        let endpoint = format!(
            "https://{}.vault.azure.net/secrets/{}?api-version=7.3",
            config.vault_url, key
        );

        // Create HTTP client
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create HTTP client: {}", e))?;

        // Prepare request with authentication
        let mut request_builder = client.get(&endpoint);

        // Add authentication (simplified - in production use azure-identity crate)
        if !config.client_id.is_empty()
            && !config.client_secret.is_empty()
            && !config.tenant_id.is_empty()
        {
            let client_id = &config.client_id;
            let client_secret = &config.client_secret;
            let tenant_id = &config.tenant_id;
            // Get OAuth token for service principal authentication
            let token_endpoint =
                format!("https://login.microsoftonline.com/{tenant_id}/oauth2/v2.0/token");

            let token_payload = [
                ("client_id", client_id.as_str()),
                ("client_secret", client_secret.as_str()),
                ("scope", "https://vault.azure.net/.default"),
                ("grant_type", "client_credentials"),
            ];

            let token_response = client
                .post(&token_endpoint)
                .form(&token_payload)
                .send()
                .await
                .map_err(|e| anyhow::anyhow!("Azure OAuth token request failed: {}", e))?;

            if !token_response.status().is_success() {
                return Err(anyhow::anyhow!(
                    "Azure OAuth token request failed with status: {}",
                    token_response.status()
                ));
            }

            let token_data: serde_json::Value = token_response
                .json()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to parse Azure OAuth response: {}", e))?;

            let access_token = token_data
                .get("access_token")
                .and_then(|token| token.as_str())
                .ok_or_else(|| anyhow::anyhow!("No access_token in Azure OAuth response"))?;

            request_builder =
                request_builder.header("Authorization", format!("Bearer {access_token}"));
        } else {
            warn!("⚠️ Using managed identity for Azure Key Vault authentication");
            // For managed identity, we would need to get a token from the Azure Instance Metadata Service
            // This is a simplified implementation
            return Err(anyhow::anyhow!(
                "Managed identity authentication requires full Azure SDK integration"
            ));
        }

        // Make the request
        let response = request_builder
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to fetch secret from Azure Key Vault: {}", e))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Azure Key Vault API request failed with status {}: {}",
                status,
                error_text
            ));
        }

        // Parse response
        let azure_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse Azure Key Vault response: {}", e))?;

        // Extract secret value
        let secret_value = azure_data
            .get("value")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("No 'value' field found in Azure Key Vault response"))?;

        debug!(
            "✅ Successfully retrieved secret from Azure Key Vault: {}",
            key
        );
        Ok(secret_value.to_string())
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
