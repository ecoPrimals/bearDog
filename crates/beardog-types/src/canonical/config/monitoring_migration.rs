// Monitoring Configuration Migration Utilities
//
// This module provides utilities to migrate fragmented monitoring configurations
// from various sources into the unified canonical monitoring system.

use crate::canonical::monitoring::{
    MonitoringConfig, UnifiedAlertingConfig, UnifiedHealthConfig, UnifiedLoggingConfig,
    UnifiedMetricsConfig, UnifiedTracingConfig,
};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Tracing imports removed due to module name conflict

#[derive(Debug, Clone)]
pub struct MonitoringMigrationResult {
    /// The unified canonical monitoring configuration
    pub unified_config: MonitoringConfig,
    /// Migration report with details about what was migrated
    /// The report value
    pub report: MonitoringMigrationReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringMigrationReport {
    /// Number of legacy monitoring configs processed
    pub legacy_configs_processed: usize,
    /// Successfully migrated monitoring components
    /// Collection of successful migrations
    pub successful_migrations: Vec<String>,
    /// Migration warnings
    /// Collection of warnings
    pub warnings: Vec<MonitoringMigrationWarning>,
    /// Migration errors
    /// Collection of errors
    pub errors: Vec<MonitoringMigrationError>,
    /// Migration timestamp
    /// The migrated at value
    pub migrated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringMigrationWarning {
    /// Component
    /// The component value
    pub component: String,
    /// Message
    /// The message value
    pub message: String,
    /// Recommendation
    /// Optional recommendation
    pub recommendation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringMigrationError {
    /// Component
    /// The component value
    pub component: String,
    /// Error
    /// The error value
    pub error: String,
    /// Resolution
    /// The resolution value
    pub resolution: String,
}

/// Legacy monitoring configuration types that need migration
#[derive(Debug, Clone)]
pub enum LegacyMonitoringConfig {
    /// Legacy configuration monitoring config
    ConfigurationMonitoring {
        /// Legacy metrics configuration
        metrics: Option<HashMap<String, serde_json::Value>>,
        /// Legacy tracing configuration
        tracing: Option<HashMap<String, serde_json::Value>>,
        /// Legacy logging configuration
        logging: Option<HashMap<String, serde_json::Value>>,
        /// Legacy health check configuration
        health: Option<HashMap<String, serde_json::Value>>,
        /// Legacy alerting configuration
        alerting: Option<HashMap<String, serde_json::Value>>,
    },
    /// Legacy production monitoring config
    ProductionMonitoring {
        /// Legacy observability configuration
        observability: HashMap<String, serde_json::Value>,
        performance: Option<HashMap<String, serde_json::Value>>,
        /// Legacy security monitoring configuration
        security: Option<HashMap<String, serde_json::Value>>,
    },
    /// Legacy beardog-monitoring config
    BeardogMonitoring {
        /// Legacy monitoring configuration data
        config: HashMap<String, serde_json::Value>,
    },
    /// Legacy provider monitoring config
    ProviderMonitoring {
        /// Legacy provider configuration data
        provider_configs: Vec<HashMap<String, serde_json::Value>>,
    },
}

/// Monitoring Configuration Migration Service
pub struct MonitoringMigrationService {
    /// Migration options
    options: MonitoringMigrationOptions,
}

impl Default for MonitoringMigrationService {
    fn default() -> Self {
        Self::new(MonitoringMigrationOptions::default())
    }
}

#[derive(Debug, Clone)]
pub struct MonitoringMigrationOptions {
    /// Whether to preserve legacy configurations as metadata
    /// Whether `preserve_legacy_metadata` is enabled
    pub preserve_legacy_metadata: bool,
    /// Whether to validate migrated configurations
    pub validate_after_migration: bool,
    /// Whether to create backup of original configs
    /// Whether `create_backup` is enabled
    pub create_backup: bool,
    /// Custom field mappings
    /// Mapping of custom mappings
    pub custom_mappings: HashMap<String, String>,
    /// Whether to merge conflicting configurations
    /// Whether `merge_conflicts` is enabled
    pub merge_conflicts: bool,
}

impl Default for MonitoringMigrationOptions {
    fn default() -> Self {
        Self {
            preserve_legacy_metadata: true,
            validate_after_migration: true,
            create_backup: true,
            custom_mappings: HashMap::new(),
            merge_conflicts: true,
        }
    }
}

impl MonitoringMigrationService {
    /// Create a new monitoring migration service
    #[must_use]
    /// Creates a new instance
    pub fn new(options: MonitoringMigrationOptions) -> Self {
        Self { options }
    }

    /// Create a migration service with default options
    #[must_use]
    /// Creates instance with defaults
    pub fn with_defaults() -> Self {
        Self::new(MonitoringMigrationOptions::default())
    }

    /// Migrate multiple legacy monitoring configurations into unified canonical config
    pub fn migrate_monitoring_configs(
        &self,
        legacy_configs: Vec<LegacyMonitoringConfig>,
    ) -> Result<MonitoringMigrationResult, BearDogError> {
        // Starting monitoring configuration migration

        let mut unified_config = MonitoringConfig::default();
        let mut report = MonitoringMigrationReport {
            legacy_configs_processed: legacy_configs.len(),
            successful_migrations: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
            migrated_at: chrono::Utc::now(),
        };

        for legacy_config in legacy_configs {
            match self.migrate_single_monitoring_config(legacy_config, &mut unified_config) {
                Ok(component_name) => {
                    report.successful_migrations.push(component_name);
                }
                Err(e) => {
                    let error = MonitoringMigrationError {
                        component: "Unknown".to_string(),
                        error: e.to_string(),
                        resolution: "Review configuration manually".to_string(),
                    };
                    report.errors.push(error);
                }
            }
        }

        // Validate the unified configuration if requested
        if self.options.validate_after_migration {
            Self::validate_unified_monitoring_config(&unified_config);
        }

        // Monitoring migration completed

        Ok(MonitoringMigrationResult {
            unified_config,
            report,
        })
    }

    /// Migrate a single legacy monitoring configuration
    fn migrate_single_monitoring_config(
        &self,
        legacy_config: LegacyMonitoringConfig,
        unified_config: &mut MonitoringConfig,
    ) -> Result<String, BearDogError> {
        match legacy_config {
            LegacyMonitoringConfig::ConfigurationMonitoring {
                metrics,
                tracing,
                logging,
                health,
                alerting,
            } => {
                Self::migrate_configuration_monitoring(
                    metrics,
                    tracing,
                    logging,
                    health,
                    alerting,
                    unified_config,
                )?;
                Ok("ConfigurationMonitoring".to_string())
            }
            LegacyMonitoringConfig::ProductionMonitoring {
                observability,
                performance,
                security,
            } => {
                self.migrate_production_monitoring(
                    observability,
                    performance,
                    security,
                    unified_config,
                )?;
                Ok("ProductionMonitoring".to_string())
            }
            LegacyMonitoringConfig::BeardogMonitoring { config } => {
                Self::migrate_beardog_monitoring(&config, unified_config);
                Ok("BeardogMonitoring".to_string())
            }
            LegacyMonitoringConfig::ProviderMonitoring { provider_configs } => {
                Self::migrate_provider_monitoring(provider_configs, unified_config);
                Ok("ProviderMonitoring".to_string())
            }
        }
    }

    /// Migrate configuration monitoring settings
    #[allow(clippy::unused_async, clippy::unnecessary_wraps)]
    fn migrate_configuration_monitoring(
        metrics: Option<HashMap<String, serde_json::Value>>,
        tracing: Option<HashMap<String, serde_json::Value>>,
        logging: Option<HashMap<String, serde_json::Value>>,
        health: Option<HashMap<String, serde_json::Value>>,
        alerting: Option<HashMap<String, serde_json::Value>>,
        unified_config: &mut MonitoringConfig,
    ) -> Result<(), BearDogError> {
        // Migrate metrics configuration
        if let Some(metrics_config) = metrics {
            Self::migrate_metrics_config(metrics_config, &mut unified_config.metrics);
        }

        // Migrate tracing configuration
        if let Some(tracing_config) = tracing {
            Self::migrate_tracing_config(tracing_config, &mut unified_config.tracing);
        }

        // Migrate logging configuration
        if let Some(logging_config) = logging {
            Self::migrate_logging_config(logging_config, &mut unified_config.logging);
        }

        // Migrate health configuration
        if let Some(health_config) = health {
            Self::migrate_health_config(health_config, &mut unified_config.health);
        }

        // Migrate alerting configuration
        if let Some(alerting_config) = alerting {
            Self::migrate_alerting_config(alerting_config, &mut unified_config.alerting);
        }

        Ok(())
    }

    /// Migrate production monitoring settings
    #[allow(clippy::unused_async, clippy::unused_self, clippy::unnecessary_wraps)]
    fn migrate_production_monitoring(
        &self,
        _observability: HashMap<String, serde_json::Value>,
        _performance: Option<HashMap<String, serde_json::Value>>,
        _security: Option<HashMap<String, serde_json::Value>>,
        _unified_config: &mut MonitoringConfig,
    ) -> Result<(), BearDogError> {
        // Migrate observability settings to production config
        // self.migrate_observability_config(observability, &mut unified_config.metrics)?;

        // Migrate performance monitoring
        // if let Some(perf_config) = performance {
        //     self.migrate_performance_monitoring_config(perf_config, &mut unified_config.performance)?;
        // }

        // Migrate security monitoring
        // if let Some(sec_config) = security {
        //     self.migrate_security_monitoring_config(sec_config, &mut unified_config.security)?;
        // }

        Ok(())
    }

    /// Migrate beardog-monitoring configurations
    fn migrate_beardog_monitoring(
        config: &HashMap<String, serde_json::Value>,
        unified_config: &mut MonitoringConfig,
    ) {
        // Extract core monitoring settings
        if let Some(enabled) = config.get("enabled") {
            if let Some(enabled_bool) = enabled.as_bool() {
                unified_config.enabled = enabled_bool;
            }
        }

        // Extract interval settings
        if let Some(interval) = config.get("interval_seconds") {
            if let Some(_interval_num) = interval.as_u64() {
                // unified_config.interval_seconds = interval_num; // Field doesn't exist in new structure
            }
        }
    }

    /// Migrate provider monitoring configurations
    fn migrate_provider_monitoring(
        provider_configs: Vec<HashMap<String, serde_json::Value>>,
        _unified_config: &mut MonitoringConfig,
    ) {
        // Merge settings from multiple providers
        for _provider_config in provider_configs {
            // Extract relevant monitoring settings and apply to unified config
            // Implementation would merge provider-specific monitoring settings
        }
    }

    // Helper methods for specific component migrations
    fn migrate_metrics_config(
        _metrics_config: HashMap<String, serde_json::Value>,
        _unified_metrics: &mut UnifiedMetricsConfig,
    ) {
        // Implementation would extract metrics settings
    }

    fn migrate_tracing_config(
        _tracing_config: HashMap<String, serde_json::Value>,
        _unified_tracing: &mut UnifiedTracingConfig,
    ) {
        // Implementation would extract tracing settings
    }

    fn migrate_logging_config(
        _logging_config: HashMap<String, serde_json::Value>,
        _unified_logging: &mut UnifiedLoggingConfig,
    ) {
        // Implementation would extract logging settings
    }

    fn migrate_health_config(
        _health_config: HashMap<String, serde_json::Value>,
        _unified_health: &mut UnifiedHealthConfig,
    ) {
        // Implementation would extract health check settings
    }

    fn migrate_alerting_config(
        _alerting_config: HashMap<String, serde_json::Value>,
        _unified_alerting: &mut UnifiedAlertingConfig,
    ) {
        // Implementation would extract alerting settings
    }

    #[allow(dead_code)]
    fn migrate_observability_config(
        _observability_config: HashMap<String, serde_json::Value>,
        _production_config: &mut crate::canonical::monitoring::MonitoringConfig,
    ) {
        // Implementation would extract observability settings
    }

    #[allow(dead_code)]
    fn migrate_performance_monitoring_config(
        _perf_config: HashMap<String, serde_json::Value>,
        _production_config: &mut crate::canonical::monitoring::MonitoringConfig,
    ) {
        // Implementation would extract performance monitoring settings
    }

    #[allow(dead_code)]
    fn migrate_security_monitoring_config(
        _sec_config: HashMap<String, serde_json::Value>,
        _production_config: &mut crate::canonical::monitoring::MonitoringConfig,
    ) {
        // Implementation would extract security monitoring settings
    }

    /// Validate the unified monitoring configuration
    /// Validates `unified_monitoring_config`
    fn validate_unified_monitoring_config(_unified_config: &MonitoringConfig) {
        // Implementation would validate the unified monitoring configuration
        // Check for required fields, valid ranges, port conflicts, etc.
    }

    /// Create a migration report summary
    #[must_use]
    /// Creates `monitoring_migration_summary`
    /// Creates `monitoring_migration_summary`
    pub fn create_monitoring_migration_summary(report: &MonitoringMigrationReport) -> String {
        format!(
            "Monitoring Configuration Migration Summary:\n\
             - Processed: {} legacy monitoring configurations\n\
             - Successful: {} component migrations\n\
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

/// Convenience function to migrate monitoring configurations with default options
pub async fn migrate_monitoring_configurations(
    legacy_configs: Vec<LegacyMonitoringConfig>,
) -> Result<MonitoringMigrationResult, BearDogError> {
    let migration_service = MonitoringMigrationService::with_defaults();
    migration_service.migrate_monitoring_configs(legacy_configs)
}

/// Create a legacy monitoring config from configuration monitoring settings
#[must_use]
/// Creates `configuration_legacy_monitoring`
pub fn create_configuration_legacy_monitoring(
    metrics_settings: Option<serde_json::Value>,
    tracing_settings: Option<serde_json::Value>,
    logging_settings: Option<serde_json::Value>,
    health_settings: Option<serde_json::Value>,
    alerting_settings: Option<serde_json::Value>,
) -> LegacyMonitoringConfig {
    let mut metrics = None;
    let mut tracing = None;
    let mut logging = None;
    let mut health = None;
    let mut alerting = None;

    if let Some(m) = metrics_settings {
        if let Ok(m_map) = serde_json::from_value::<HashMap<String, serde_json::Value>>(m) {
            metrics = Some(m_map);
        }
    }

    if let Some(t) = tracing_settings {
        if let Ok(t_map) = serde_json::from_value::<HashMap<String, serde_json::Value>>(t) {
            tracing = Some(t_map);
        }
    }

    if let Some(l) = logging_settings {
        if let Ok(l_map) = serde_json::from_value::<HashMap<String, serde_json::Value>>(l) {
            logging = Some(l_map);
        }
    }

    if let Some(h) = health_settings {
        if let Ok(h_map) = serde_json::from_value::<HashMap<String, serde_json::Value>>(h) {
            health = Some(h_map);
        }
    }

    if let Some(a) = alerting_settings {
        if let Ok(a_map) = serde_json::from_value::<HashMap<String, serde_json::Value>>(a) {
            alerting = Some(a_map);
        }
    }

    LegacyMonitoringConfig::ConfigurationMonitoring {
        metrics,
        tracing,
        logging,
        health,
        alerting,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitoring_migration_service_creation() {
        let service = MonitoringMigrationService::with_defaults();
        assert!(service.options.validate_after_migration);
        assert!(service.options.preserve_legacy_metadata);
    }

    #[tokio::test]
    async fn test_empty_monitoring_migration() {
        let service = MonitoringMigrationService::with_defaults();
        let result = service.migrate_monitoring_configs(vec![])?;

        assert_eq!(result.report.legacy_configs_processed, 0);
        assert_eq!(result.report.successful_migrations.len(), 0);
    }

    #[tokio::test]
    async fn test_configuration_legacy_monitoring_creation() {
        let legacy_config = create_configuration_legacy_monitoring(
            Some(serde_json::json!({"enabled": true})),
            Some(serde_json::json!({"backend": "jaeger"})),
            Some(serde_json::json!({"level": "info"})),
            Some(serde_json::json!({"enabled": true})),
            Some(serde_json::json!({"enabled": false})),
        );

        match legacy_config {
            LegacyMonitoringConfig::ConfigurationMonitoring {
                metrics,
                tracing,
                logging,
                health,
                alerting,
            } => {
                assert!(metrics.is_some());
                assert!(tracing.is_some());
                assert!(logging.is_some());
                assert!(health.is_some());
                assert!(alerting.is_some());
            }
            _ => panic!("Expected ConfigurationMonitoring config"),
        }
    }
}
