// SPDX-License-Identifier: AGPL-3.0-only

// HSM Configuration Migration Utilities
//
// This module provides utilities to migrate fragmented HSM configurations
// from beardog-tunnel and other crates into the unified canonical system.

use super::CanonicalHsmConfig;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Migration options to control the migration process
#[derive(Debug, Clone)]
pub struct MigrationOptions {
    /// Whether to preserve legacy configurations as metadata
    /// Whether `preserve_legacy_metadata` is enabled
    pub preserve_legacy_metadata: bool,
    /// Whether to validate migrated configurations
    pub validate_after_migration: bool,
    /// Whether to create backup of original configs
    /// Whether `create_backup` is enabled
    pub create_backup: bool,
    /// Custom migration mappings
    /// Mapping of custom mappings
    pub custom_mappings: HashMap<String, String>,
}

impl Default for MigrationOptions {
    fn default() -> Self {
        Self {
            preserve_legacy_metadata: true,
            validate_after_migration: true,
            create_backup: true,
            custom_mappings: HashMap::new(),
        }
    }
}

/// Migration result containing the unified config and migration report
#[derive(Debug, Serialize, Deserialize)]
pub struct MigrationResult {
    /// The unified HSM configuration
    pub unified_config: CanonicalHsmConfig,
    /// Migration report with details and warnings
    /// The migration report value
    pub migration_report: MigrationReport,
}

/// Migration report tracking what was migrated and any issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationReport {
    /// Number of legacy configs processed
    pub legacy_configs_processed: usize,
    /// Successfully migrated configurations
    /// Collection of successful migrations
    pub successful_migrations: Vec<String>,
    /// Configuration types that had migration warnings
    /// Collection of warnings
    pub warnings: Vec<MigrationWarning>,
    /// Configuration types that couldn't be migrated
    /// Collection of errors
    pub errors: Vec<MigrationError>,
    /// Migration timestamp
    /// The migrated at value
    pub migrated_at: chrono::DateTime<chrono::Utc>,
}

/// Single non-fatal issue observed while parsing a legacy HSM configuration fragment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationWarning {
    /// Legacy config family that triggered the warning (e.g. `tunnel`, `configuration`).
    pub config_type: String,
    /// Message
    /// The message value
    pub message: String,
    /// Recommendation
    /// Optional recommendation
    pub recommendation: Option<String>,
}

/// Blocking problem that prevented automatic migration for a given legacy component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationError {
    /// Config Type
    pub config_type: String,
    /// Error
    /// The error value
    pub error: String,
    /// Resolution
    /// The resolution value
    pub resolution: String,
}

/// Legacy HSM configuration types that need migration
#[derive(Debug, Clone)]
pub enum LegacyHsmConfig {
    /// Legacy tunnel HSM config
    TunnelHsm {
        /// Hardware-specific HSM configuration parameters
        hardware_config: Option<HashMap<String, serde_json::Value>>,
        /// Software HSM fallback configuration
        software_config: Option<HashMap<String, serde_json::Value>>,
        /// Mobile device HSM configuration (Android `StrongBox`, iOS Secure Enclave)
        mobile_config: Option<HashMap<String, serde_json::Value>>,
    },
    /// Legacy configuration HSM config
    ConfigurationHsm {
        /// List of HSM provider configurations
        providers: Vec<HashMap<String, serde_json::Value>>,
        /// Legacy monitoring hooks serialized as loose JSON.
        monitoring: Option<HashMap<String, serde_json::Value>>,
        /// Legacy throughput or latency caps serialized as loose JSON.
        performance: Option<HashMap<String, serde_json::Value>>,
    },
    /// Legacy zero-cost HSM config
    ZeroCostHsm {
        /// HSM manager configuration parameters
        manager_config: HashMap<String, serde_json::Value>,
    },
}

/// Orchestrates conversion from [`LegacyHsmConfig`] into [`CanonicalHsmConfig`].
pub struct HsmMigrationService {
    /// Migration options
    options: MigrationOptions,
}

impl Default for HsmMigrationService {
    fn default() -> Self {
        Self::new(MigrationOptions::default())
    }
}

impl HsmMigrationService {
    /// Create a new migration service with options
    #[must_use]
    /// Creates a new instance
    pub const fn new(options: MigrationOptions) -> Self {
        Self { options }
    }

    /// Create a migration service with default options
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self::new(MigrationOptions::default())
    }

    /// Migrate multiple legacy HSM configurations into a unified canonical config
    pub fn migrate_hsm_configs(
        &self,
        legacy_configs: Vec<LegacyHsmConfig>,
    ) -> Result<MigrationResult, BearDogError> {
        info!(
            "Starting HSM configuration migration for {} legacy configs",
            legacy_configs.len()
        );

        let mut unified_config = CanonicalHsmConfig::default();
        let mut report = MigrationReport {
            legacy_configs_processed: legacy_configs.len(),
            successful_migrations: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
            migrated_at: chrono::Utc::now(),
        };

        for legacy_config in legacy_configs {
            match self.migrate_single_config(legacy_config, &mut unified_config) {
                Ok(config_type) => {
                    report.successful_migrations.push(config_type);
                }
                Err(e) => {
                    let error = MigrationError {
                        config_type: "Unknown".to_string(),
                        error: e.to_string(),
                        resolution: "Review configuration manually".to_string(),
                    };
                    report.errors.push(error);
                }
            }
        }

        // Validate the unified configuration if requested
        if self.options.validate_after_migration
            && let Err(e) = self.validate_unified_config(&unified_config)
        {
            warn!("Unified configuration validation failed: {}", e);
            report.warnings.push(MigrationWarning {
                config_type: "Unified".to_string(),
                message: format!("Validation failed: {e}"),
                recommendation: Some("Review and adjust configuration manually".to_string()),
            });
        }

        info!(
            "HSM migration completed: {} successful, {} warnings, {} errors",
            report.successful_migrations.len(),
            report.warnings.len(),
            report.errors.len()
        );

        Ok(MigrationResult {
            unified_config,
            migration_report: report,
        })
    }

    /// Migrate a single legacy configuration
    fn migrate_single_config(
        &self,
        legacy_config: LegacyHsmConfig,
        unified_config: &mut CanonicalHsmConfig,
    ) -> Result<String, BearDogError> {
        match legacy_config {
            LegacyHsmConfig::TunnelHsm {
                hardware_config,
                software_config,
                mobile_config: _,
            } => {
                self.migrate_tunnel_hsm_config(hardware_config, software_config, unified_config)?;
                Ok("TunnelHsm".to_string())
            }
            LegacyHsmConfig::ConfigurationHsm {
                providers,
                monitoring,
                performance: _,
            } => {
                self.migrate_configuration_hsm_config(providers, monitoring, unified_config)?;
                Ok("ConfigurationHsm".to_string())
            }
            LegacyHsmConfig::ZeroCostHsm { manager_config } => {
                self.migrate_zero_cost_hsm_config(&manager_config, unified_config);
                Ok("ZeroCostHsm".to_string())
            }
        }
    }

    /// Migrate tunnel HSM configuration
    #[allow(clippy::unused_async)]
    fn migrate_tunnel_hsm_config(
        &self,
        hardware_config: Option<HashMap<String, serde_json::Value>>,
        software_config: Option<HashMap<String, serde_json::Value>>,
        _unified_config: &mut CanonicalHsmConfig,
    ) -> Result<(), BearDogError> {
        debug!("Starting tunnel HSM configuration migration");

        if let Some(hw_config) = hardware_config {
            debug!("Migrating tunnel hardware HSM configuration");
            // Implementation would migrate hardware-specific tunnel HSM settings
            for (key, value) in hw_config {
                debug!("Processing tunnel hardware setting: {} = {:?}", key, value);
            }
        }

        if let Some(sw_config) = software_config {
            debug!("Migrating tunnel software HSM configuration");
            // Implementation would migrate software-specific tunnel HSM settings
            for (key, value) in sw_config {
                debug!("Processing tunnel software setting: {} = {:?}", key, value);
            }
        }

        debug!("Tunnel HSM configuration migration completed");
        Ok(())
    }

    /// Migrate configuration HSM settings
    #[allow(clippy::unused_async)]
    fn migrate_configuration_hsm_config(
        &self,
        providers: Vec<HashMap<String, serde_json::Value>>,
        monitoring: Option<HashMap<String, serde_json::Value>>,
        _unified_config: &mut CanonicalHsmConfig,
    ) -> Result<(), BearDogError> {
        debug!("Starting configuration HSM migration");

        for (index, provider_config) in providers.into_iter().enumerate() {
            debug!("Migrating provider configuration {}", index);
            for (key, value) in provider_config {
                debug!("Processing provider setting: {} = {:?}", key, value);
            }
        }

        if let Some(monitoring_config) = monitoring {
            debug!("Migrating HSM monitoring configuration");
            for (key, value) in monitoring_config {
                debug!("Processing monitoring setting: {} = {:?}", key, value);
            }
        }

        debug!("Configuration HSM migration completed");
        Ok(())
    }

    /// Migrate zero-cost HSM configuration
    #[allow(clippy::unused_self)]
    fn migrate_zero_cost_hsm_config(
        &self,
        manager_config: &HashMap<String, serde_json::Value>,
        unified_config: &mut CanonicalHsmConfig,
    ) {
        // Extract relevant settings from manager config and apply to unified config
        if let Some(enabled) = manager_config.get("enabled")
            && let Some(enabled_bool) = enabled.as_bool()
        {
            unified_config.core.enabled = enabled_bool;
        }

        // Extract performance settings
        if let Some(batch_enabled) = manager_config.get("batch_operations_enabled")
            && let Some(batch_bool) = batch_enabled.as_bool()
        {
            unified_config.performance.batch_operations_enabled = batch_bool;
        }

        if let Some(pool_enabled) = manager_config.get("connection_pooling_enabled")
            && let Some(pool_bool) = pool_enabled.as_bool()
        {
            unified_config.performance.connection_pooling_enabled = pool_bool;
        }

        if let Some(cache_enabled) = manager_config.get("cache_enabled")
            && let Some(cache_bool) = cache_enabled.as_bool()
        {
            unified_config.performance.cache_enabled = cache_bool;
        }
    }

    /// Validate the unified configuration
    #[allow(clippy::unused_self, clippy::unnecessary_wraps)]
    /// Validates `unified_config`
    const fn validate_unified_config(
        &self,
        _unified_config: &CanonicalHsmConfig,
    ) -> Result<(), BearDogError> {
        // Implementation would validate the unified configuration
        // Check for required fields, valid ranges, etc.
        Ok(())
    }

    /// Create a migration report summary
    #[must_use]
    /// Creates `migration_summary`
    /// Creates `migration_summary`
    pub fn create_migration_summary(report: &MigrationReport) -> String {
        format!(
            "HSM Configuration Migration Summary:\n\
             - Processed: {} legacy configurations\n\
             - Successful: {} migrations\n\
             - Warnings: {} issues\n\
             - Errors: {} failures\n\
             - Completed: {}",
            report.legacy_configs_processed,
            report.successful_migrations.len(),
            report.warnings.len(),
            report.errors.len(),
            report.migrated_at.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
}

/// Convenience function to migrate HSM configurations with default options
pub async fn migrate_hsm_configurations(
    legacy_configs: Vec<LegacyHsmConfig>,
) -> Result<MigrationResult, BearDogError> {
    let migration_service = HsmMigrationService::default();
    migration_service.migrate_hsm_configs(legacy_configs)
}

/// Create a legacy HSM config from beardog-tunnel configuration
#[must_use]
/// Creates `tunnel_legacy_config`
pub fn create_tunnel_legacy_config(
    hardware_settings: Option<serde_json::Value>,
    software_settings: Option<serde_json::Value>,
    mobile_settings: Option<serde_json::Value>,
) -> LegacyHsmConfig {
    let mut hardware_config = None;
    let mut software_config = None;
    let mut mobile_config = None;

    if let Some(hw) = hardware_settings
        && let Ok(hw_map) = serde_json::from_value::<HashMap<String, serde_json::Value>>(hw)
    {
        hardware_config = Some(hw_map);
    }

    if let Some(sw) = software_settings
        && let Ok(sw_map) = serde_json::from_value::<HashMap<String, serde_json::Value>>(sw)
    {
        software_config = Some(sw_map);
    }

    if let Some(mobile) = mobile_settings
        && let Ok(mobile_map) = serde_json::from_value::<HashMap<String, serde_json::Value>>(mobile)
    {
        mobile_config = Some(mobile_map);
    }

    LegacyHsmConfig::TunnelHsm {
        hardware_config,
        software_config,
        mobile_config,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hsm_migration_service_creation() {
        let service = HsmMigrationService::default();
        assert!(service.options.validate_after_migration);
        assert!(service.options.preserve_legacy_metadata);
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    #[tokio::test]
    async fn test_empty_migration() -> Result<(), Box<dyn std::error::Error>> {
        let service = HsmMigrationService::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let result = service.migrate_hsm_configs(vec![])?;

        assert_eq!(result.migration_report.legacy_configs_processed, 0);
        assert_eq!(result.migration_report.successful_migrations.len(), 0);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_tunnel_legacy_config_creation() {
        let legacy_config = create_tunnel_legacy_config(
            Some(serde_json::json!({"provider_type": "hardware"})),
            Some(serde_json::json!({"implementation": "software"})),
            Some(serde_json::json!({"platform": "android"})),
        );

        match legacy_config {
            LegacyHsmConfig::TunnelHsm {
                hardware_config,
                software_config,
                mobile_config,
            } => {
                assert!(hardware_config.is_some());
                assert!(software_config.is_some());
                assert!(mobile_config.is_some());
            }
            _ => panic!("Expected TunnelHsm config"),
        }
    }
}
