//! # `BearDog` Configuration
//!
//! Zero-hardcoding configuration system for `BearDog` with hierarchical loading:
//! 1. Command-line arguments (highest priority)
//! 2. Environment variables
//! 3. Configuration file
//! 4. Platform-specific defaults
//! 5. Secure fallback defaults
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use beardog_config::BearDogConfig;
//!
//! // Load with full hierarchy
//! let config = BearDogConfig::load().expect("Failed to load configuration");
//!
//! // Access configuration
//! println!("API Port: {}", config.network.api.port);
//! ```
//!
//! ## Features
//!
//! - **Zero Hardcoding**: All values configurable
//! - **Secure Defaults**: Works out-of-box with secure settings
//! - **Platform Aware**: Auto-detects platform-specific paths
//! - **Multiple Formats**: TOML, JSON, YAML support
//! - **Validation**: Compile-time and runtime validation
//! - **Hierarchical**: Clear priority system

pub mod defaults;
pub mod discovery;
pub mod domains;
pub mod error;
pub mod global;
pub mod hierarchy;
pub mod loader;
pub mod runtime_network_discovery;
pub mod validation;
pub mod zero_hardcoding;

pub use error::{ConfigError, ConfigResult};
pub use global::{config, BEARDOG_CONFIG};
pub use hierarchy::ConfigHierarchy;
pub use loader::ConfigLoader;

// Re-export commonly used types
pub use domains::network_addresses::NetworkAddressesConfig;
pub use domains::network_ports::{
    NetworkPortsConfig, DEFAULT_API_PORT, DEFAULT_DISCOVERY_PORT, DEFAULT_HEALTH_PORT,
    DEFAULT_HTTPS_PORT, DEFAULT_METRICS_PORT, DEFAULT_TCP_IPC_PORT, DEFAULT_VAULT_PORT,
};

// Re-export zero hardcoding for easy access
pub use zero_hardcoding::{
    EndpointConfig, RetryConfig, ZeroHardcodingConfig, ZeroHardcodingTimeouts as ZeroTimeouts,
};

use domains::{
    capacity::CapacityConfig, crypto::CryptoConfig, hsm::HsmConfig, limits::LimitsConfig,
    monitoring::MonitoringConfig, network::NetworkConfig, paths::PathConfig,
    security::SecurityConfig, timeouts_new::TimeoutConfig,
};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Primary configuration for BearDog
///
/// This is the root configuration structure that contains all configurable
/// aspects of BearDog. It can be loaded from files, environment variables,
/// or created programmatically.
///
/// # Examples
///
/// ```rust,no_run
/// use beardog_config::BearDogConfig;
///
/// // Load with defaults and environment variables
/// let config = BearDogConfig::load()?;
///
/// // Load from specific file
/// let config = BearDogConfig::from_file("config.toml")?;
///
/// // Create with custom values
/// let mut config = BearDogConfig::default();
/// config.network.api.port = 9000;
/// config.validate()?;
/// # Ok::<(), beardog_config::ConfigError>(())
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BearDogConfig {
    /// Network configuration (API, discovery, admin)
    #[serde(default)]
    pub network: NetworkConfig,

    /// Path configuration (config dir, data dir, library paths)
    #[serde(default)]
    pub paths: PathConfig,

    /// HSM configuration (hardware, software, TPM)
    #[serde(default)]
    pub hsm: HsmConfig,

    /// Limits and timeouts
    #[serde(default)]
    pub limits: LimitsConfig,

    /// Cryptographic parameters
    #[serde(default)]
    pub crypto: CryptoConfig,

    /// Security settings
    #[serde(default)]
    pub security: SecurityConfig,

    /// Monitoring and logging
    #[serde(default)]
    pub monitoring: MonitoringConfig,

    /// Operation timeouts
    #[serde(default)]
    pub timeouts: TimeoutConfig,

    /// Capacity limits (buffers, pools, caches)
    #[serde(default)]
    pub capacity: CapacityConfig,
}

impl BearDogConfig {
    /// Load configuration from environment variables
    ///
    /// This method loads configuration from environment variables with fallback to defaults.
    /// For most use cases, prefer using the global `BEARDOG_CONFIG` singleton instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use beardog_config::BearDogConfig;
    ///
    /// let config = BearDogConfig::from_env();
    /// ```
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            network: domains::network::NetworkConfig::from_env(),
            paths: domains::paths::PathConfig::from_env(),
            hsm: domains::hsm::HsmConfig::from_env(),
            limits: domains::limits::LimitsConfig::from_env(),
            crypto: domains::crypto::CryptoConfig::from_env(),
            security: domains::security::SecurityConfig::from_env(),
            monitoring: domains::monitoring::MonitoringConfig::from_env(),
            timeouts: domains::timeouts_new::TimeoutConfig::from_env(),
            capacity: domains::capacity::CapacityConfig::from_env(),
        }
    }

    /// Load configuration with full hierarchy
    ///
    /// Loads configuration in order of priority:
    /// 1. Environment variables
    /// 2. Configuration file (if found)
    /// 3. Platform-specific defaults
    /// 4. Secure fallback defaults
    ///
    /// # Errors
    ///
    /// Returns error if configuration is invalid or cannot be loaded
    pub fn load() -> ConfigResult<Self> {
        ConfigLoader::new()
            .with_defaults()
            .with_platform_defaults()
            .with_config_file()?
            .with_env_vars()
            .build()
    }

    /// Load from specific configuration file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to configuration file (TOML, JSON, or YAML)
    ///
    /// # Errors
    ///
    /// Returns error if file cannot be read or parsed
    pub fn from_file<P: AsRef<Path>>(path: P) -> ConfigResult<Self> {
        ConfigLoader::new()
            .with_defaults()
            .with_file(path)?
            .with_env_vars()
            .build()
    }

    /// Validate configuration
    ///
    /// Checks that all configuration values are valid and consistent.
    ///
    /// # Errors
    ///
    /// Returns error if any configuration value is invalid
    pub fn validate(&self) -> ConfigResult<()> {
        self.network.validate()?;
        self.paths.validate()?;
        self.hsm.validate()?;
        self.limits.validate()?;
        self.crypto.validate()?;
        self.security.validate()?;
        self.monitoring.validate()?;
        self.timeouts.validate().map_err(ConfigError::Validation)?;
        Ok(())
    }

    /// Generate example configuration file
    ///
    /// Creates a configuration file with all options documented
    /// and set to their default values.
    ///
    /// # Arguments
    ///
    /// * `path` - Where to write the example configuration
    ///
    /// # Errors
    ///
    /// Returns error if file cannot be written
    pub fn generate_example<P: AsRef<Path>>(path: P) -> ConfigResult<()> {
        let config = Self::default();
        let content = toml::to_string_pretty(&config)
            .map_err(|e| ConfigError::Serialization(e.to_string()))?;

        std::fs::write(path, content).map_err(ConfigError::Io)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = BearDogConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_default_values() {
        let config = BearDogConfig::default();

        // Network defaults
        assert_eq!(
            config.network.api.port,
            crate::domains::network_ports::DEFAULT_API_PORT
        );
        assert_eq!(
            config.network.api.bind_address,
            std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)
        );

        // Limits defaults
        assert_eq!(config.limits.operation_timeout_secs, 30);
        assert_eq!(config.limits.max_retries, 3);

        // HSM defaults
        assert!(config.hsm.auto_detect);
        assert!(config.hsm.prefer_hardware);
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let config = BearDogConfig::default();

        // Serialize to TOML
        let toml_str = toml::to_string(&config).expect("Failed to serialize");

        // Deserialize back
        let deserialized: BearDogConfig = toml::from_str(&toml_str).expect("Failed to deserialize");

        // Should match
        assert_eq!(config.network.api.port, deserialized.network.api.port);
        assert_eq!(config.limits.max_retries, deserialized.limits.max_retries);
    }
}
