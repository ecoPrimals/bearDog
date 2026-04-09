// SPDX-License-Identifier: AGPL-3.0-or-later

//! Secrets manager: provider selection, retrieval, and in-memory caching.

use super::secrets_backend::{FileVaultBackend, SecretsBackend};
use super::{Environment, Result, SecretValue, SecretsManager, SecretsProvider};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use tracing::debug;

impl SecretsManager {
    /// Creates a new secrets manager
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] if the provider list for the environment cannot be built.
    pub fn new(environment: &Environment) -> Result<Self> {
        let providers = Self::determine_providers(environment)?;

        Ok(Self {
            providers,
            cache: HashMap::with_capacity(16),
            _cache_ttl: 300, // 5 minutes
        })
    }

    /// Gets a secret by key
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] if the secret is not found in any configured provider.
    pub fn get_secret(&mut self, key: &str) -> Result<SecretValue> {
        // Check cache first
        if let Some(cached_value) = self.cache.get(key)
            && let Some(expires_at) = cached_value.expires_at
            && std::time::SystemTime::now() < expires_at
        {
            return Ok(cached_value.clone());
        }

        // Try each provider
        for provider in &self.providers {
            if let Ok(value) = self.get_secret_from_provider(provider, key) {
                self.cache.insert(key.to_string(), value.clone());
                return Ok(value);
            }
        }

        Err(BearDogError::system(format!("Secret not found: {key}")))
    }

    /// Gets secret from specific provider
    fn get_secret_from_provider(
        &self,
        provider: &SecretsProvider,
        key: &str,
    ) -> Result<SecretValue> {
        match provider {
            SecretsProvider::EnvironmentVariables => {
                let env_key = format!("BEARDOG_SECRET_{}", key.replace('/', "_").to_uppercase());
                if let Ok(value) = beardog_errors::process_env::var(&env_key) {
                    Ok(SecretValue {
                        value,
                        expires_at: None,
                        metadata: HashMap::new(),
                    })
                } else {
                    Err(BearDogError::system(
                        "Secret not found in environment".to_string(),
                    ))
                }
            }
            SecretsProvider::Vault {
                endpoint,
                token,
                mount_path,
            } => {
                debug!(
                    target: "beardog_production",
                    endpoint = %endpoint,
                    mount_path = %mount_path,
                    "retrieving secret from BearDog local vault"
                );
                let backend =
                    FileVaultBackend::open_for_vault_provider(endpoint, token, mount_path)?;
                let value = backend.retrieve(key)?;
                Ok(SecretValue {
                    value,
                    expires_at: None,
                    metadata: HashMap::new(),
                })
            }
            SecretsProvider::UniversalSecretsManagement {
                endpoint,
                provider_type,
                auth_config,
            } => {
                debug!(
                    target: "beardog_production",
                    endpoint = %endpoint,
                    provider_type = %provider_type,
                    "retrieving secret from USM (file vault backend)"
                );
                let backend = FileVaultBackend::open_for_usm(endpoint, auth_config)?;
                let value = backend.retrieve(key)?;
                Ok(SecretValue {
                    value,
                    expires_at: None,
                    metadata: HashMap::new(),
                })
            }
        }
    }

    /// Determines secrets providers based on environment
    fn determine_providers(environment: &Environment) -> Result<Vec<SecretsProvider>> {
        let mut providers = vec![];

        // Always add environment variables as fallback
        providers.push(SecretsProvider::EnvironmentVariables);

        if matches!(environment, Environment::Production) {
            // Add production secrets providers
            if let Ok(vault_endpoint) = beardog_errors::process_env::var("VAULT_ENDPOINT")
                && let Ok(vault_token) = beardog_errors::process_env::var("VAULT_TOKEN")
            {
                providers.insert(
                    0,
                    SecretsProvider::Vault {
                        endpoint: vault_endpoint,
                        token: vault_token,
                        mount_path: beardog_errors::process_env::var("VAULT_MOUNT_PATH")
                            .unwrap_or_else(|_| "secret".to_string()),
                    },
                );
            }
        }

        Ok(providers)
    }
}

#[cfg(test)]
impl SecretsManager {
    /// Construct with explicit providers for unit tests (e.g. [`SecretsProvider::UniversalSecretsManagement`]).
    pub(crate) fn from_providers_for_test(providers: Vec<SecretsProvider>) -> Self {
        Self {
            providers,
            cache: HashMap::with_capacity(16),
            _cache_ttl: 300,
        }
    }

    /// Insert a cached secret (used to exercise TTL-based cache hits in [`SecretsManager::get_secret`]).
    pub(crate) fn insert_cache_entry_for_test(
        &mut self,
        key: impl Into<String>,
        value: SecretValue,
    ) {
        self.cache.insert(key.into(), value);
    }
}
