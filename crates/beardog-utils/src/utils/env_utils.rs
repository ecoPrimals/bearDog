// SPDX-License-Identifier: AGPL-3.0-only

// Environment variable utilities for BearDog
//
// Provides safe, type-checked access to environment variables with validation.

use beardog_errors::BearDogError;
use std::env;
use std::time::Duration;

pub struct EnvUtils;

impl EnvUtils {
    /// Get a required environment variable, returning an error if not set
    /// Gets required
    /// Gets required
    pub fn get_required(key: &str) -> Result<String, BearDogError> {
        beardog_errors::process_env::var(key).map_err(|_| {
            BearDogError::configuration(format!("Required environment variable {} not set", key))
        })
    }

    /// Get an optional environment variable with a default value
    /// Gets optional
    /// Gets optional
    pub fn get_optional(key: &str, default: &str) -> String {
        beardog_errors::process_env::var(key).unwrap_or_else(|_| default.to_string())
    }

    /// Get a boolean environment variable with a default value
    /// Gets bool
    /// Gets bool
    pub fn get_bool(key: &str, default: bool) -> bool {
        beardog_errors::process_env::var(key)
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(default)
    }

    /// Get a u16 environment variable with a default value
    /// Gets u16
    /// Gets u16
    pub fn get_u16(key: &str, default: u16) -> u16 {
        beardog_errors::process_env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// Get a u32 environment variable with a default value
    /// Gets u32
    /// Gets u32
    pub fn get_u32(key: &str, default: u32) -> u32 {
        beardog_errors::process_env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// Get a u64 environment variable with a default value
    /// Gets u64
    /// Gets u64
    pub fn get_u64(key: &str, default: u64) -> u64 {
        beardog_errors::process_env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// Get a duration from environment variable (in seconds)
    /// Gets duration_secs
    /// Gets duration_secs
    pub fn get_duration_secs(key: &str, default_secs: u64) -> Duration {
        let secs = Self::get_u64(key, default_secs);
        Duration::from_secs(secs)
    }

    /// Get a CSV list from environment variable
    /// Gets csv_list
    /// Gets csv_list
    pub fn get_csv_list(key: &str, default: Vec<&str>) -> Vec<String> {
        beardog_errors::process_env::var(key)
            .map(|v| v.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_else(|_| default.into_iter().map(|s| s.to_string()).collect())
    }

    /// Validate that all required production environment variables are set
    /// Validates production_env
    /// Validates production_env
    pub fn validate_production_env() -> Result<(), BearDogError> {
        let required_vars = [
            "BEARDOG_DATABASE_URL",
            "BEARDOG_SECRET_KEY",
            "BEARDOG_ENCRYPTION_KEY",
            "BEARDOG_API_BIND_ADDRESS",
        ];

        for var in &required_vars {
            Self::get_required(var)?;
        }

        // Universal capability-based environment variables (replaces hardcoded primal endpoints)
        use beardog_types::canonical::config::network::NetworkConfig;
        let network_config = NetworkConfig::default();
        
        let warnings = [
            ("BEARDOG_LOG_LEVEL", "INFO"),
            (
                "BEARDOG_DISCOVERY_ENDPOINT",
                &format!("https://discovery.ecosystem.internal:{}", network_config.service_ports.api_port),
            ),
            (
                "BEARDOG_CAPABILITY_REGISTRY",
                &format!("https://capabilities.ecosystem.internal:{}", network_config.service_ports.admin_port),
            ),
            (
                "BEARDOG_SERVICE_MESH_ENDPOINT",
                &format!("https://mesh.ecosystem.internal:{}", network_config.service_ports.admin_port),
            ),
            ("BEARDOG_SMTP_SERVER", "smtp.example.com"),
        ];

        for (var, example) in &warnings {
            if beardog_errors::process_env::var(var).is_err() {
                tracing::warn!(
                    "Environment variable {} not set, using default. Example: {}",
                    var,
                    example
                );
            }
        }

        Ok(())
    }

    /// Get database configuration from environment
    /// Gets database_config
    /// Gets database_config
    pub fn get_database_config() -> Result<DatabaseEnvConfig, BearDogError> {
        Ok(DatabaseEnvConfig {
            url: Self::get_required("BEARDOG_DATABASE_URL")?,
            pool_size: Self::get_u32("BEARDOG_DB_POOL_SIZE", 10),
            timeout_secs: Self::get_u64("BEARDOG_DB_TIMEOUT_SECS", 30),
            enable_ssl: Self::get_bool("BEARDOG_DB_SSL", true),
        })
    }

    /// Get network configuration from environment
    /// Gets network_config
    /// Gets network_config
    pub fn get_network_config() -> NetworkEnvConfig {
        // Use network config for consistent defaults
        let network_config = beardog_types::canonical::config::network::NetworkConfig::default();
        
        NetworkEnvConfig {
            host: Self::get_optional("BEARDOG_HOST", &network_config.default_host),
            port: Self::get_u16("BEARDOG_PORT", network_config.service_ports.api_port),
            max_connections: Self::get_u32("BEARDOG_MAX_CONNECTIONS", 1000) as usize,
            enable_tls: Self::get_bool("BEARDOG_TLS", false),
        }
    }

    /// Get security configuration from environment
    /// Gets security_config
    /// Gets security_config
    pub fn get_security_config() -> Result<SecurityEnvConfig, BearDogError> {
        Ok(SecurityEnvConfig {
            secret_key: Self::get_required("BEARDOG_SECRET_KEY")?,
            encryption_key: Self::get_required("BEARDOG_ENCRYPTION_KEY")?,
            session_timeout_secs: Self::get_u64("BEARDOG_SESSION_TIMEOUT", 3600),
            require_mfa: Self::get_bool("BEARDOG_REQUIRE_MFA", false),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DatabaseEnvConfig {
    /// The url value
    pub url: String,
    /// Number of pool_size
    pub pool_size: u32,
    pub timeout_secs: u64,
    /// Whether enable_ssl is enabled
    pub enable_ssl: bool,
}

#[derive(Debug, Clone)]
pub struct NetworkEnvConfig {
    /// The host value
    pub host: String,
    /// Number of port
    pub port: u16,
    /// Number of max_connections
    pub max_connections: usize,
    /// Whether enable_tls is enabled
    pub enable_tls: bool,
}

#[derive(Debug, Clone)]
pub struct SecurityEnvConfig {
    /// The secret key value
    pub secret_key: String,
    /// The encryption key value
    pub encryption_key: String,
    pub session_timeout_secs: u64,
    /// Whether require_mfa is enabled
    pub require_mfa: bool,
}

/// Environment validation utilities
pub struct EnvValidator;

impl EnvValidator {
    /// Validates development
    /// Validates development
    pub fn validate_development() -> Result<(), BearDogError> {
        let optional_vars = ["BEARDOG_LOG_LEVEL", "BEARDOG_HOST", "BEARDOG_PORT"];

        for var in &optional_vars {
            if beardog_errors::process_env::var(var).is_err() {
                tracing::debug!("Optional development variable {} not set", var);
            }
        }

        Ok(())
    }

    /// Validates production
    /// Validates production
    pub fn validate_production() -> Result<(), BearDogError> {
        EnvUtils::validate_production_env()
    }

    /// Check if running in production environment
    /// Checks if production
    /// Checks if production
    pub fn is_production() -> bool {
        EnvUtils::get_optional("BEARDOG_ENV", "development").to_lowercase() == "production"
    }

    /// Check if debug mode is enabled
    /// Checks if debug
    /// Checks if debug
    pub fn is_debug() -> bool {
        EnvUtils::get_bool("BEARDOG_DEBUG", false)
    }
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_get_optional() {
        beardog_errors::process_env::set_var("TEST_VAR", "test_value");
        assert_eq!(EnvUtils::get_optional("TEST_VAR", "default"), "test_value");
        assert_eq!(EnvUtils::get_optional("NON_EXISTENT", "default"), "default");
        beardog_errors::process_env::remove_var("TEST_VAR");
    }


    fn test_get_bool() {
        beardog_errors::process_env::set_var("TEST_BOOL_TRUE", "true");
        beardog_errors::process_env::set_var("TEST_BOOL_FALSE", "false");
        beardog_errors::process_env::set_var("TEST_BOOL_1", "1");
        assert!(EnvUtils::get_bool("TEST_BOOL_TRUE", false));
        assert!(!EnvUtils::get_bool("TEST_BOOL_FALSE", true));
        assert!(EnvUtils::get_bool("TEST_BOOL_1", false));
        assert!(EnvUtils::get_bool("NON_EXISTENT", true));
        beardog_errors::process_env::remove_var("TEST_BOOL_TRUE");
        beardog_errors::process_env::remove_var("TEST_BOOL_FALSE");
        beardog_errors::process_env::remove_var("TEST_BOOL_1");
    }


    fn test_get_csv_list() {
        beardog_errors::process_env::set_var("TEST_CSV", "item1,item2,item3");
        let result = EnvUtils::get_csv_list("TEST_CSV", vec!["default"]);
        assert_eq!(result, vec!["item1", "item2", "item3"]);
        let default_result = EnvUtils::get_csv_list("NON_EXISTENT", vec!["default1", "default2"]);
        assert_eq!(default_result, vec!["default1", "default2"]);
        beardog_errors::process_env::remove_var("TEST_CSV");
    }
}
