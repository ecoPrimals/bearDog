// **SIMPLIFIED UNIFIED CONFIGURATION** - Phase 3 Implementation
//
// ⚠️ DEPRECATED: This module has been merged into unified.rs
// Use `SimplifiedBearDogConfig` from `canonical::config::unified` instead
//
// This is a **working implementation** of the unified configuration system that
// eliminates fragmentation while maintaining compilation compatibility.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Duration imported via other modules

/// **WORKING UNIFIED CONFIGURATION** - Simplified but complete
///
/// This demonstrates the unification approach while ensuring compilation success.
/// It consolidates the most important configuration domains into a single structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingUnifiedConfig {
    /// **SYSTEM METADATA**
    /// The version value
    pub version: String,
    /// Environment
    /// The environment value
    pub environment: String,
    /// Instance Id
    pub instance_id: String,

    /// **CORE CONFIGURATIONS**
    /// The network value
    pub network: NetworkSettings,
    /// Security
    /// The security value
    pub security: SecuritySettings,
    /// Database
    /// The database value
    pub database: DatabaseSettings,
    /// Monitoring
    /// The monitoring value
    pub monitoring: MonitoringSettings,

    /// **FEATURE FLAGS**
    /// Mapping of features
    pub features: HashMap<String, bool>,

    /// **PERFORMANCE SETTINGS**
    pub performance: PerformanceSettings,
}

/// Network configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettings {
    /// Bind Address
    /// The bind address value
    pub bind_address: String,
    /// Port
    /// Number of port
    pub port: u16,
    /// Max Connections
    /// Number of `max_connections`
    pub max_connections: usize,
    /// Timeout Seconds
    pub timeout_seconds: u64,
    /// Enable Tls
    /// Whether `enable_tls` is enabled
    pub enable_tls: bool,
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1".to_string(),
            port: 8080,
            max_connections: 1000,
            timeout_seconds: 30,
            enable_tls: true,
        }
    }
}

/// Security configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    /// Session Timeout Seconds
    pub session_timeout_seconds: u64,
    /// Max Login Attempts
    /// Number of `max_login_attempts`
    pub max_login_attempts: u32,
    /// Enable Mfa
    /// Whether `enable_mfa` is enabled
    pub enable_mfa: bool,
    /// Hash Rounds
    /// Number of `hash_rounds`
    pub hash_rounds: u32,
    /// Audit Retention Days
    /// Number of `audit_retention_days`
    pub audit_retention_days: u32,
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            session_timeout_seconds: 3600, // 1 hour
            max_login_attempts: 5,
            enable_mfa: true,
            hash_rounds: 12,
            audit_retention_days: 365,
        }
    }
}

/// Database configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    /// Connection String
    /// The connection string value
    pub connection_string: String,
    /// Pool Size
    /// Number of `pool_size`
    pub pool_size: u32,
    /// Timeout Seconds
    pub timeout_seconds: u64,
    /// Enable Encryption
    /// Whether `enable_encryption` is enabled
    pub enable_encryption: bool,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            connection_string: "sqlite://beardog.db".to_string(),
            pool_size: 10,
            timeout_seconds: 30,
            enable_encryption: true,
        }
    }
}

/// Monitoring configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSettings {
    /// Enable Metrics
    /// Whether `enable_metrics` is enabled
    pub enable_metrics: bool,
    /// Metrics Interval Seconds
    /// Number of `metrics_interval_seconds`
    pub metrics_interval_seconds: u64,
    /// Log Level
    /// The log level value
    pub log_level: String,
    /// Health Check Interval Seconds
    /// Number of `health_check_interval_seconds`
    pub health_check_interval_seconds: u64,
}

impl Default for MonitoringSettings {
    fn default() -> Self {
        Self {
            enable_metrics: true,
            metrics_interval_seconds: 60,
            log_level: "info".to_string(),
            health_check_interval_seconds: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    /// Max Memory Mb
    /// Number of `max_memory_mb`
    pub max_memory_mb: usize,
    /// Worker Threads
    /// Number of `worker_threads`
    pub worker_threads: usize,
    /// Cache Size Mb
    /// Number of `cache_size_mb`
    pub cache_size_mb: usize,
    /// Enable Optimization
    /// Whether `enable_optimization` is enabled
    pub enable_optimization: bool,
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        Self {
            max_memory_mb: 512,
            worker_threads: 4,
            cache_size_mb: 64,
            enable_optimization: true,
        }
    }
}

impl WorkingUnifiedConfig {
    /// Create a new unified configuration with defaults
    #[must_use]
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Load configuration from environment variables
    /// Creates instance from env
    pub fn from_env() -> Result<Self, BearDogError> {
        let mut config = Self::default();

        // Load from environment variables with BEARDOG_ prefix
        if let Ok(port) = std::env::var("BEARDOG_PORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                config.network.port = port_num;
            }
        }

        if let Ok(log_level) = std::env::var("BEARDOG_LOG_LEVEL") {
            config.monitoring.log_level = log_level;
        }

        if let Ok(environment) = std::env::var("BEARDOG_ENVIRONMENT") {
            config.environment = environment;
        }

        Ok(config)
    }

    /// Validate the configuration
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        // Validate network settings
        if self.network.port == 0 {
            return Err(BearDogError::Configuration {
                message: "Port cannot be 0".to_string(),
                category: beardog_errors::categories::ConfigurationErrorCategory::Validation,
            });
        }

        if self.network.max_connections == 0 {
            return Err(BearDogError::Configuration {
                message: "Max connections must be greater than 0".to_string(),
                category: beardog_errors::categories::ConfigurationErrorCategory::Validation,
            });
        }

        // Validate security settings
        if self.security.hash_rounds < 4 || self.security.hash_rounds > 31 {
            return Err(BearDogError::Configuration {
                message: "Hash rounds must be between 4 and 31".to_string(),
                category: beardog_errors::categories::ConfigurationErrorCategory::Validation,
            });
        }

        // Validate database settings
        if self.database.pool_size == 0 {
            return Err(BearDogError::Configuration {
                message: "Database pool size must be greater than 0".to_string(),
                category: beardog_errors::categories::ConfigurationErrorCategory::Validation,
            });
        }

        Ok(())
    }

    #[must_use]
    pub fn summary(&self) -> HashMap<String, String> {
        let mut summary = HashMap::new();

        summary.insert("version".to_string(), self.version.clone());
        summary.insert("environment ".to_string(), self.environment.clone());
        summary.insert("network_port".to_string(), self.network.port.to_string());
        summary.insert(
            "security_mfa".to_string(),
            self.security.enable_mfa.to_string(),
        );
        summary.insert(
            "database_pool_size".to_string(),
            self.database.pool_size.to_string(),
        );
        summary.insert(
            "monitoring_enabled".to_string(),
            self.monitoring.enable_metrics.to_string(),
        );

        summary
    }

    /// Apply configuration overrides
    #[must_use]
    /// Creates instance with overrides
    pub fn with_overrides(mut self, overrides: HashMap<String, String>) -> Self {
        for (key, value) in overrides {
            match key.as_str() {
                "network.port" => {
                    if let Ok(port) = value.parse::<u16>() {
                        self.network.port = port;
                    }
                }
                "network.max_connections" => {
                    if let Ok(connections) = value.parse::<usize>() {
                        self.network.max_connections = connections;
                    }
                }
                "security.enable_mfa" => {
                    if let Ok(enable) = value.parse::<bool>() {
                        self.security.enable_mfa = enable;
                    }
                }
                "monitoring.log_level" => {
                    self.monitoring.log_level = value;
                }
                _ => {
                    // Store unknown overrides in features
                    if let Ok(bool_value) = value.parse::<bool>() {
                        self.features.insert(key, bool_value);
                    }
                }
            }
        }

        self
    }
}

/// **CONFIGURATION MIGRATION HELPER** - Migrate from legacy configurations
pub struct ConfigurationMigrator;

impl ConfigurationMigrator {
    /// Migrate from legacy `BearDogConfig` to unified config
    pub fn migrate_from_legacy() -> Result<WorkingUnifiedConfig, BearDogError> {
        let mut config = WorkingUnifiedConfig {
            version: crate::constants::domains::system::versions::BEARDOG_VERSION.to_string(),
            environment: "production".to_string(),
            instance_id: format!("beardog-{}", std::process::id()),
            ..Default::default()
        };

        // Enable key features by default
        config
            .features
            .insert("security_enhanced".to_string(), true);
        config
            .features
            .insert("monitoring_enabled".to_string(), true);
        config.features.insert("hsm_support".to_string(), true);
        config.features.insert("genetics_enabled".to_string(), true);

        Ok(config)
    }

    /// Check if migration is needed
    #[must_use]
    pub fn needs_migration() -> bool {
        // Check if legacy config files exist
        std::path::Path::new("/etc/beardog/legacy-config.toml").exists()
            || std::path::Path::new("~/.config/beardog/legacy-config.toml").exists()
    }

    /// Validate unified configuration
    /// Validates config
    /// Validates config
    pub fn validate_config(config: &WorkingUnifiedConfig) -> Result<(), BearDogError> {
        // Validate network settings
        if config.network.port == 0 {
            return Err(BearDogError::Business {
                message: "Network port cannot be zero".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        // Validate security settings
        if config.security.hash_rounds < 4 {
            return Err(BearDogError::Security {
                message: "Hash rounds must be at least 4".to_string(),
                category: beardog_errors::SecurityErrorCategory::Configuration,
            });
        }

        // Validate database settings
        if config.database.connection_string.is_empty() {
            return Err(BearDogError::Business {
                message: "Database connection string cannot be empty".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        Ok(())
    }
}

impl Default for WorkingUnifiedConfig {
    fn default() -> Self {
        let mut features = HashMap::new();
        features.insert("security_enhanced".to_string(), true);
        features.insert("performance_optimized".to_string(), true);
        features.insert("monitoring_enabled".to_string(), true);

        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: "development".to_string(),
            instance_id: format!("beardog-{}", std::process::id()),
            network: NetworkSettings::default(),
            security: SecuritySettings::default(),
            database: DatabaseSettings::default(),
            monitoring: MonitoringSettings::default(),
            features,
            performance: PerformanceSettings::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_config_creation() {
        let config = WorkingUnifiedConfig::new();
        assert_eq!(config.network.port, 8080);
        assert_eq!(config.security.max_login_attempts, 5);
        assert!(config.security.enable_mfa);
    }

    #[test]
    fn test_config_validation() {
        let config = WorkingUnifiedConfig::default();
        assert!(config.validate().is_ok());

        let mut invalid_config = config.clone();
        invalid_config.network.port = 0;
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_config_overrides() {
        let mut overrides = HashMap::new();
        overrides.insert("network.port".to_string(), "9090".to_string());
        overrides.insert("security.enable_mfa".to_string(), "false".to_string());

        let config = WorkingUnifiedConfig::default().with_overrides(overrides);
        assert_eq!(config.network.port, 9090);
        assert!(!config.security.enable_mfa);
    }

    #[test]
    fn test_configuration_migration() {
        let migrated = ConfigurationMigrator::migrate_from_legacy().unwrap();
        assert!(!migrated.version.is_empty());
        assert!(!migrated.instance_id.is_empty());
        assert!(migrated.features.get("security_enhanced").unwrap_or(&false));
    }
}
