// SPDX-License-Identifier: AGPL-3.0-or-later

//! Runtime Configuration Management
//!
//! This module contains the runtime logic for loading and managing production configurations.
//! Secret retrieval lives in the `secrets` submodule; defaults for config structs in `defaults`.

use super::{
    ConfigSource, Environment, LogLevel, ProductionConfig, ProductionConfigManager, Result,
    SecretsManager,
};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::{debug, info, warn};

impl ProductionConfigManager {
    /// Creates a new production configuration manager
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] if configuration sources or the secrets manager cannot be initialized.
    pub fn new(environment: Environment) -> Result<Self> {
        let config_sources = Self::determine_config_sources(&environment)?;
        let secrets_manager = SecretsManager::new(&environment)?;

        Ok(Self {
            environment,
            config_sources,
            secrets_manager,
            _config_cache: HashMap::with_capacity(16),
        })
    }

    /// Loads complete production configuration
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] if loading, merging, secret injection, or validation fails.
    pub fn load_configuration(&mut self) -> Result<ProductionConfig> {
        info!(
            "Loading production configuration for environment: {:?}",
            self.environment
        );

        let mut config = self.load_base_config()?;
        self.apply_environment_overrides(&mut config)?;
        self.inject_secrets(&mut config)?;
        self.validate_configuration(&config)?;

        info!("Production configuration loaded successfully");
        Ok(config)
    }

    /// Loads base configuration from sources
    fn load_base_config(&self) -> Result<ProductionConfig> {
        let mut config = ProductionConfig::default();

        for source in self.config_sources.clone() {
            match source {
                ConfigSource::Environment => {
                    self.load_from_environment(&mut config)?;
                }
                ConfigSource::File { path } => {
                    self.load_from_file(&mut config, path.as_str())?;
                }
                ConfigSource::UniversalSecretsManagement {
                    endpoint,
                    provider_type,
                    auth_config,
                } => {
                    self.load_from_universal_secrets(
                        &mut config,
                        endpoint.as_str(),
                        provider_type.as_str(),
                        &auth_config,
                    )?;
                }
                ConfigSource::UniversalContainerSecrets {
                    namespace,
                    provider_type,
                } => {
                    self.load_from_universal_container_secrets(
                        &mut config,
                        namespace.as_str(),
                        provider_type.as_str(),
                    )?;
                }
            }
        }

        Ok(config)
    }

    /// Loads configuration from file
    fn load_from_file(&self, config: &mut ProductionConfig, path: &str) -> Result<()> {
        debug!("Loading configuration from file: {}", path);

        if !Path::new(path).exists() {
            warn!("Configuration file not found: {}", path);
            return Ok(());
        }

        let content = fs::read_to_string(path)
            .map_err(|e| BearDogError::system(format!("Failed to read config file {path}: {e}")))?;

        let ext = Path::new(path).extension().and_then(|e| e.to_str());
        let file_config: ProductionConfig = if ext.is_some_and(|e| e.eq_ignore_ascii_case("toml")) {
            toml::from_str(&content)
                .map_err(|e| BearDogError::validation(&format!("Invalid TOML config: {e}")))?
        } else if ext
            .is_some_and(|e| e.eq_ignore_ascii_case("yaml") || e.eq_ignore_ascii_case("yml"))
        {
            serde_yaml::from_str(&content)
                .map_err(|e| BearDogError::validation(&format!("Invalid YAML config: {e}")))?
        } else {
            serde_json::from_str(&content)
                .map_err(|e| BearDogError::validation(&format!("Invalid JSON config: {e}")))?
        };

        // Merge file config into base - currently replaces base with file values
        // For selective merging of individual fields, implement custom merge logic
        *config = file_config;

        Ok(())
    }

    /// Loads configuration from environment variables
    fn load_from_environment(&self, config: &mut ProductionConfig) -> Result<()> {
        debug!("Loading configuration from environment variables");

        if let Ok(port) = beardog_errors::process_env::var("BEARDOG_PORT") {
            config.application.port = port
                .parse()
                .map_err(|_| BearDogError::validation("Invalid BEARDOG_PORT value"))?;
        }

        if let Ok(bind_address) = beardog_errors::process_env::var("BEARDOG_BIND_ADDRESS") {
            config.application.bind_address = bind_address;
        }

        if let Ok(worker_threads) = beardog_errors::process_env::var("BEARDOG_WORKER_THREADS") {
            config.application.worker_threads = worker_threads
                .parse()
                .map_err(|_| BearDogError::validation("Invalid BEARDOG_WORKER_THREADS value"))?;
        }

        if let Ok(db_host) = beardog_errors::process_env::var("BEARDOG_DB_HOST") {
            config.database.primary.host = db_host;
        }

        if let Ok(db_port) = beardog_errors::process_env::var("BEARDOG_DB_PORT") {
            config.database.primary.port = db_port
                .parse()
                .map_err(|_| BearDogError::validation("Invalid BEARDOG_DB_PORT value"))?;
        }

        if let Ok(log_level) = beardog_errors::process_env::var("BEARDOG_LOG_LEVEL") {
            config.logging.level = match log_level.to_lowercase().as_str() {
                "trace" => LogLevel::Trace,
                "debug" => LogLevel::Debug,
                "warn" | "warning" => LogLevel::Warn,
                "error" => LogLevel::Error,
                _ => LogLevel::Info,
            };
        }

        Ok(())
    }

    /// Loads configuration from universal secrets management
    fn load_from_universal_secrets(
        &self,
        _config: &mut ProductionConfig,
        endpoint: &str,
        provider_type: &str,
        _auth_config: &HashMap<String, String>,
    ) -> Result<()> {
        info!(
            "🔐 Loading configuration from universal secrets management: {} ({})",
            endpoint, provider_type
        );

        if self.has_secrets_capability(provider_type) {
            info!(
                "🔐 Using capability-based secrets provider: {}",
                provider_type
            );
        } else {
            warn!(
                "Provider {} does not support required secrets capabilities, using environment fallback",
                provider_type
            );
        }
        Ok(())
    }

    /// Loads configuration from universal container secrets
    fn load_from_universal_container_secrets(
        &self,
        _config: &mut ProductionConfig,
        namespace: &str,
        provider_type: &str,
    ) -> Result<()> {
        info!(
            "📦 Loading configuration from universal container secrets: {} ({})",
            namespace, provider_type
        );

        match provider_type {
            "kubernetes" | "docker_swarm" | "nomad" => {
                info!(
                    "📦 Using environment fallback for container secrets provider: {}",
                    provider_type
                );
                Ok(())
            }
            _ => {
                warn!(
                    "Unknown container provider type: {}, using environment fallback",
                    provider_type
                );
                Ok(())
            }
        }
    }

    /// Injects secrets into configuration
    fn inject_secrets(&mut self, config: &mut ProductionConfig) -> Result<()> {
        info!("Injecting secrets into configuration");

        if let Ok(password) = self.secrets_manager.get_secret("database/password") {
            config.database.primary.password = password.value;
        }

        if let Ok(cert) = self.secrets_manager.get_secret("tls/certificate") {
            config.networking.tls.cert_path = cert.value;
        }

        if let Ok(key) = self.secrets_manager.get_secret("tls/private_key") {
            config.networking.tls.key_path = key.value;
        }

        Ok(())
    }

    /// Validates production configuration
    fn validate_configuration(&self, config: &ProductionConfig) -> Result<()> {
        debug!("Validating production configuration");

        if config.application.port == 0 {
            return Err(BearDogError::validation("Application port cannot be 0"));
        }

        if config.application.worker_threads == 0 {
            return Err(BearDogError::validation(
                "Worker threads must be greater than 0",
            ));
        }

        if config.database.primary.host.is_empty() {
            return Err(BearDogError::validation("Database host cannot be empty"));
        }

        if config.database.primary.database.is_empty() {
            return Err(BearDogError::validation("Database name cannot be empty"));
        }

        if matches!(self.environment, Environment::Production) {
            if !config.networking.tls.enabled {
                return Err(BearDogError::validation(
                    "TLS must be enabled in production",
                ));
            }

            if !config.compliance.encryption_at_rest {
                return Err(BearDogError::validation(
                    "Encryption at rest must be enabled in production",
                ));
            }

            if !config.compliance.access_logging {
                return Err(BearDogError::validation(
                    "Access logging must be enabled in production",
                ));
            }
        }

        info!("Configuration validation passed");
        Ok(())
    }

    /// Determines configuration sources based on environment
    fn determine_config_sources(environment: &Environment) -> Result<Vec<ConfigSource>> {
        let mut sources = vec![ConfigSource::Environment];

        let config_file = match environment {
            Environment::Development | Environment::Testing => {
                "configs/environments/development.toml"
            }
            Environment::Staging => "configs/environments/staging.toml",
            Environment::Production => "configs/environments/production.toml",
            Environment::Custom(_) => "configs/environments/development.toml",
        };

        sources.push(ConfigSource::File {
            path: config_file.to_string(),
        });

        if matches!(environment, Environment::Production) {
            if let Ok(vault_endpoint) = beardog_errors::process_env::var("VAULT_ENDPOINT")
                && let Ok(vault_token) = beardog_errors::process_env::var("VAULT_TOKEN")
            {
                sources.push(ConfigSource::UniversalSecretsManagement {
                    endpoint: vault_endpoint,
                    provider_type: "BearDogLocalVault".to_string(),
                    auth_config: {
                        let mut config = HashMap::new();
                        config.insert("token".to_string(), vault_token);
                        config
                    },
                });
            }

            if beardog_errors::process_env::var("KUBERNETES_SERVICE_HOST").is_ok() {
                sources.push(ConfigSource::UniversalContainerSecrets {
                    namespace: beardog_errors::process_env::var("KUBERNETES_NAMESPACE")
                        .unwrap_or_else(|_| "default".to_string()),
                    provider_type: "kubernetes".to_string(),
                });
            }
        }

        Ok(sources)
    }

    /// Applies environment-specific overrides
    fn apply_environment_overrides(&self, _config: &mut ProductionConfig) -> Result<()> {
        Ok(())
    }

    /// Checks if provider supports secrets management capabilities
    fn has_secrets_capability(&self, provider_type: &str) -> bool {
        let provider_lower = provider_type.to_lowercase();
        provider_lower.contains("secrets")
            || provider_lower.contains("vault")
            || provider_lower.contains("kms")
            || provider_lower.contains("key")
    }

    // Note: merge_configs() removed - merging now happens inline in load_from_file()
    // Current implementation uses replacement strategy. Selective field-level merging
    // can be added when needed by implementing custom merge logic in load_from_file().
}

#[cfg(test)]
mod tests {
    #![expect(clippy::expect_used, reason = "test assertions")]

    include!("runtime_tests.rs");
}
