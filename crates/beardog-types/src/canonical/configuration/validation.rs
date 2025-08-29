//! Configuration Validation Module
//!
//! Provides validation logic for BearDog configurations.

use super::core::BearDogCanonicalConfig;

/// Configuration validator
#[derive(Debug, Clone)]
pub struct ConfigValidator;

impl ConfigValidator {
    /// Validate a BearDog configuration
    pub fn validate(config: &BearDogCanonicalConfig) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate app configuration
        if config.app.name.is_empty() {
            errors.push("App name cannot be empty".to_string());
        }

        if config.app.version.is_empty() {
            errors.push("App version cannot be empty".to_string());
        }

        // Validate network configuration
        if config.network.port == 0 {
            errors.push("Network port must be specified".to_string());
        }

        if config.network.host.is_empty() {
            errors.push("Network host cannot be empty".to_string());
        }

        if config.network.max_connections == 0 {
            errors.push("Max connections must be greater than 0".to_string());
        }

        // Validate database configuration
        if config.database.connection_string.is_empty() {
            errors.push("Database connection string cannot be empty".to_string());
        }

        if config.database.pool_size == 0 {
            errors.push("Database pool size must be greater than 0".to_string());
        }

        // Validate HSM configuration (simplified validation)
        // Note: HSM configuration validation depends on the specific provider implementation

        // Return validation result
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validate specific configuration sections
    pub fn validate_network(config: &super::network::NetworkConfig) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if config.port == 0 {
            errors.push("Port must be specified".to_string());
        }

        if config.host.is_empty() {
            errors.push("Host cannot be empty".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validate app configuration
    pub fn validate_app(config: &super::app::AppConfig) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if config.name.is_empty() {
            errors.push("App name cannot be empty".to_string());
        }

        if config.version.is_empty() {
            errors.push("App version cannot be empty".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Configuration migration utilities
#[derive(Debug, Clone)]
pub struct ConfigMigrator;

impl ConfigMigrator {
    /// Migrate from legacy configuration
    pub fn migrate_from_legacy() -> BearDogCanonicalConfig {
        // Migration logic would be implemented here
        BearDogCanonicalConfig::default()
    }

    /// Migrate specific configuration sections
    pub fn migrate_network_config() -> super::network::NetworkConfig {
        super::network::NetworkConfig::default()
    }

    /// Migrate app configuration
    pub fn migrate_app_config() -> super::app::AppConfig {
        super::app::AppConfig::default()
    }
}
