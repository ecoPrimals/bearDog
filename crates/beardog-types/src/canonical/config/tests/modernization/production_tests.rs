// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for Production Configuration
//!
//! Tests production-related configs including application, storage, operations,
//! observability, deployment, and feature flags.

use crate::canonical::config::domains::system;
use crate::canonical::config::production::core::ProductionFeatureFlags;
use crate::canonical::config::production::deployment::{CanaryConfig, RolloutConfig};
use crate::canonical::config::production::environment::EnvironmentValidation;
use crate::canonical::config::production::observability::{
    DashboardConfig, ProductionLoggingConfig, ProductionMetricsConfig, ProductionTracingConfig,
};
use crate::canonical::config::production::operations::{
    BackupConfig, DisasterRecoveryConfig, MaintenanceConfig,
};
use crate::canonical::config::production::resources::{
    ConnectionConfig, GcTuningConfig, StorageResourceConfig,
};
use std::time::Duration;

// ============================================================================
// Application Configuration Tests
// ============================================================================

/// Test ApplicationConfig::with_defaults()
#[test]
fn test_application_config_with_defaults() {
    let config = system::ApplicationConfig::with_defaults();
    assert_eq!(config.name, system::ApplicationConfig::DEFAULT_NAME);
    assert_eq!(config.version, env!("CARGO_PKG_VERSION"));
    assert!(
        config.instance_id.starts_with("instance-"),
        "expected instance-{{pid}} default, got {:?}",
        config.instance_id
    );
    assert_eq!(
        config.description,
        system::ApplicationConfig::DEFAULT_DESCRIPTION
    );
    assert!(config.features.is_empty());
}

/// Test ApplicationConfig::from_env()
#[test]
fn test_application_config_from_env() {
    let config = system::ApplicationConfig::from_env_provider(|k| match k {
        "BEARDOG_APP_NAME" => Some("CustomApp".to_string()),
        "BEARDOG_APP_DESCRIPTION" => Some("Custom Description".to_string()),
        _ => None,
    });
    assert_eq!(config.name, "CustomApp");
    assert_eq!(config.description, "Custom Description");
    // Version comes from CARGO_PKG_VERSION (compile time)
    assert!(!config.version.is_empty());
    // Instance ID is UUID (non-deterministic but valid format)
    assert!(!config.instance_id.is_empty());
}

// ============================================================================
// Storage & Resource Configuration Tests
// ============================================================================

/// Test StorageResourceConfig::with_defaults()
#[test]
fn test_storage_resource_config_with_defaults() {
    let config = StorageResourceConfig::with_defaults();
    assert_eq!(
        config.max_disk_usage_percent,
        StorageResourceConfig::DEFAULT_MAX_DISK_USAGE_PERCENT
    );
    assert_eq!(
        config.temp_dir_cleanup_interval,
        Duration::from_secs(StorageResourceConfig::DEFAULT_TEMP_CLEANUP_INTERVAL_SECS)
    );
    assert_eq!(
        config.log_rotation_size_mb,
        StorageResourceConfig::DEFAULT_LOG_ROTATION_SIZE_MB
    );
    assert_eq!(
        config.log_retention_days,
        StorageResourceConfig::DEFAULT_LOG_RETENTION_DAYS
    );
}

/// Test StorageResourceConfig::from_env()
#[test]
fn test_storage_resource_config_from_env() {
    let config = StorageResourceConfig::from_env_provider(|k| match k {
        "BEARDOG_MAX_DISK_USAGE_PERCENT" => Some("90.5".to_string()),
        "BEARDOG_TEMP_CLEANUP_INTERVAL_SECS" => Some("7200".to_string()),
        "BEARDOG_LOG_ROTATION_SIZE_MB" => Some("200".to_string()),
        "BEARDOG_LOG_RETENTION_DAYS" => Some("60".to_string()),
        _ => None,
    });
    assert_eq!(config.max_disk_usage_percent, 90.5);
    assert_eq!(config.temp_dir_cleanup_interval, Duration::from_secs(7200));
    assert_eq!(config.log_rotation_size_mb, 200);
    assert_eq!(config.log_retention_days, 60);
}

/// Test ConnectionConfig::with_defaults()
#[test]
fn test_connection_config_with_defaults() {
    let config = ConnectionConfig::with_defaults();
    assert_eq!(config.pool_size, ConnectionConfig::DEFAULT_POOL_SIZE);
    assert_eq!(
        config.max_idle_connections,
        ConnectionConfig::DEFAULT_MAX_IDLE_CONNECTIONS
    );
    assert_eq!(
        config.connection_lifetime,
        Duration::from_secs(ConnectionConfig::DEFAULT_CONNECTION_LIFETIME_SECS)
    );
    assert_eq!(
        config.health_check_interval,
        Duration::from_secs(ConnectionConfig::DEFAULT_HEALTH_CHECK_INTERVAL_SECS)
    );
}

/// Test ConnectionConfig::from_env()
#[test]
fn test_connection_config_from_env() {
    let config = ConnectionConfig::from_env_provider(|k| match k {
        "BEARDOG_CONNECTION_POOL_SIZE" => Some("20".to_string()),
        "BEARDOG_MAX_IDLE_CONNECTIONS" => Some("10".to_string()),
        "BEARDOG_CONNECTION_LIFETIME_SECS" => Some("3600".to_string()),
        "BEARDOG_CONNECTION_HEALTH_CHECK_INTERVAL_SECS" => Some("120".to_string()),
        _ => None,
    });
    assert_eq!(config.pool_size, 20);
    assert_eq!(config.max_idle_connections, 10);
    assert_eq!(config.connection_lifetime, Duration::from_secs(3600));
    assert_eq!(config.health_check_interval, Duration::from_secs(120));
}

// ============================================================================
// Operations Configuration Tests
// ============================================================================

/// Test MaintenanceConfig::with_defaults()
#[test]
fn test_maintenance_config_with_defaults() {
    let config = MaintenanceConfig::with_defaults();
    assert!(!config.enabled);
    assert_eq!(
        config.window_duration,
        Duration::from_secs(MaintenanceConfig::DEFAULT_WINDOW_DURATION_SECS)
    );
}

/// Test MaintenanceConfig::from_env()
#[test]
fn test_maintenance_config_from_env() {
    let config = MaintenanceConfig::from_env_provider(|k| match k {
        "BEARDOG_MAINTENANCE_WINDOW_SECS" => Some("7200".to_string()),
        _ => None,
    });
    assert!(!config.enabled);
    assert_eq!(config.window_duration, Duration::from_secs(7200));
}

/// Test BackupConfig::with_defaults()
#[test]
fn test_backup_config_with_defaults() {
    let config = BackupConfig::with_defaults();
    assert!(config.enabled);
    assert_eq!(
        config.interval,
        Duration::from_secs(BackupConfig::DEFAULT_INTERVAL_SECS)
    );
}

/// Test BackupConfig::from_env()
#[test]
fn test_backup_config_from_env() {
    let config = BackupConfig::from_env_provider(|k| match k {
        "BEARDOG_BACKUP_INTERVAL_SECS" => Some("43200".to_string()),
        _ => None,
    });
    assert!(config.enabled);
    assert_eq!(config.interval, Duration::from_secs(43200));
}

/// Test DisasterRecoveryConfig::with_defaults()
#[test]
fn test_disaster_recovery_config_with_defaults() {
    let config = DisasterRecoveryConfig::with_defaults();
    assert!(!config.enabled);
    assert_eq!(
        config.rto,
        Duration::from_secs(DisasterRecoveryConfig::DEFAULT_RTO_SECS)
    );
}

/// Test DisasterRecoveryConfig::from_env()
#[test]
fn test_disaster_recovery_config_from_env() {
    let config = DisasterRecoveryConfig::from_env_provider(|k| match k {
        "BEARDOG_DR_RTO_SECS" => Some("1800".to_string()),
        _ => None,
    });
    assert!(!config.enabled);
    assert_eq!(config.rto, Duration::from_secs(1800));
}

// ============================================================================
// Environment & Observability Configuration Tests
// ============================================================================

/// Test EnvironmentValidation::with_defaults()
#[test]
fn test_environment_validation_with_defaults() {
    let config = EnvironmentValidation::with_defaults();
    assert!(config.validate_on_startup);
    assert!(!config.validate_periodically);
    assert_eq!(
        config.validation_interval,
        Duration::from_secs(EnvironmentValidation::DEFAULT_VALIDATION_INTERVAL_SECS)
    );
    assert_eq!(
        config.min_disk_space_mb,
        Some(EnvironmentValidation::DEFAULT_MIN_DISK_SPACE_MB)
    );
    assert_eq!(
        config.min_memory_mb,
        Some(EnvironmentValidation::DEFAULT_MIN_MEMORY_MB)
    );
}

/// Test EnvironmentValidation::from_env()
#[test]
fn test_environment_validation_from_env() {
    let config = EnvironmentValidation::from_env_provider(|k| match k {
        "BEARDOG_ENV_VALIDATION_INTERVAL_SECS" => Some("600".to_string()),
        _ => None,
    });
    assert_eq!(config.validation_interval, Duration::from_secs(600));
}

/// Test ProductionMetricsConfig::with_defaults()
#[test]
fn test_production_metrics_config_with_defaults() {
    let config = ProductionMetricsConfig::with_defaults();
    assert!(config.enabled);
    assert_eq!(config.endpoint, ProductionMetricsConfig::DEFAULT_ENDPOINT);
}

/// Test ProductionMetricsConfig::from_env()
#[test]
fn test_production_metrics_config_from_env() {
    let config = ProductionMetricsConfig::from_env_provider(|k| match k {
        "BEARDOG_METRICS_ENDPOINT" => Some("/custom-metrics".to_string()),
        _ => None,
    });
    assert_eq!(config.endpoint, "/custom-metrics");
}

/// Test ProductionLoggingConfig::with_defaults()
#[test]
fn test_production_logging_config_with_defaults() {
    let config = ProductionLoggingConfig::with_defaults();
    assert_eq!(config.level, ProductionLoggingConfig::DEFAULT_LEVEL);
    assert_eq!(config.format, ProductionLoggingConfig::DEFAULT_FORMAT);
}

/// Test ProductionLoggingConfig::from_env()
#[test]
fn test_production_logging_config_from_env() {
    let config = ProductionLoggingConfig::from_env_provider(|k| match k {
        "BEARDOG_LOG_LEVEL" => Some("debug".to_string()),
        "BEARDOG_LOG_FORMAT" => Some("text".to_string()),
        _ => None,
    });
    assert_eq!(config.level, "debug");
    assert_eq!(config.format, "text");
}

/// Test ProductionTracingConfig::with_defaults()
#[test]
fn test_production_tracing_config_with_defaults() {
    let config = ProductionTracingConfig::with_defaults();
    assert!(!config.enabled);
    assert_eq!(config.endpoint, ProductionTracingConfig::DEFAULT_ENDPOINT);
}

/// Test ProductionTracingConfig::from_env()
#[test]
fn test_production_tracing_config_from_env() {
    let config = ProductionTracingConfig::from_env_provider(|k| match k {
        "BEARDOG_TRACING_ENDPOINT" => Some("/custom-traces".to_string()),
        _ => None,
    });
    assert_eq!(config.endpoint, "/custom-traces");
}

/// Test DashboardConfig::with_defaults()
#[test]
fn test_dashboard_config_with_defaults() {
    let config = DashboardConfig::with_defaults();
    assert!(!config.enabled);
    assert_eq!(config.endpoint, DashboardConfig::DEFAULT_ENDPOINT);
}

/// Test DashboardConfig::from_env()
#[test]
fn test_dashboard_config_from_env() {
    let config = DashboardConfig::from_env_provider(|k| match k {
        "BEARDOG_DASHBOARD_ENDPOINT" => Some("/admin/dashboard".to_string()),
        _ => None,
    });
    assert_eq!(config.endpoint, "/admin/dashboard");
}

// ============================================================================
// Deployment Configuration Tests
// ============================================================================

/// Test RolloutConfig::with_defaults()
#[test]
fn test_rollout_config_with_defaults() {
    let config = RolloutConfig::with_defaults();
    assert_eq!(config.percentage, RolloutConfig::DEFAULT_PERCENTAGE);
}

/// Test RolloutConfig::from_env()
#[test]
fn test_rollout_config_from_env() {
    let config = RolloutConfig::from_env_provider(|k| match k {
        "BEARDOG_ROLLOUT_PERCENTAGE" => Some("50.0".to_string()),
        _ => None,
    });
    assert_eq!(config.percentage, 50.0);
}

/// Test CanaryConfig::with_defaults()
#[test]
fn test_canary_config_with_defaults() {
    let config = CanaryConfig::with_defaults();
    assert_eq!(config.percentage, CanaryConfig::DEFAULT_PERCENTAGE);
}

/// Test CanaryConfig::from_env()
#[test]
fn test_canary_config_from_env() {
    let config = CanaryConfig::from_env_provider(|k| match k {
        "BEARDOG_CANARY_PERCENTAGE" => Some("5.0".to_string()),
        _ => None,
    });
    assert_eq!(config.percentage, 5.0);
}

// ============================================================================
// Core & Feature Flags Configuration Tests
// ============================================================================

/// Test ProductionFeatureFlags::with_defaults()
#[test]
fn test_production_feature_flags_with_defaults() {
    let config = ProductionFeatureFlags::with_defaults();
    assert!(config.enable_advanced_monitoring);
    assert!(config.enable_distributed_tracing);
    assert!(!config.enable_performance_profiling);
    assert!(config.enable_security_auditing);
}

/// Test ProductionFeatureFlags::from_env()
#[test]
fn test_production_feature_flags_from_env() {
    let config = ProductionFeatureFlags::from_env_provider(|k| match k {
        "BEARDOG_ENABLE_PERFORMANCE_PROFILING" => Some("true".to_string()),
        "BEARDOG_ENABLE_AUTO_SCALING" => Some("true".to_string()),
        _ => None,
    });
    assert!(config.enable_performance_profiling);
    assert!(config.enable_auto_scaling);
}

/// Test GcTuningConfig::with_defaults()
#[test]
fn test_gc_tuning_config_with_defaults() {
    let config = GcTuningConfig::with_defaults();
    assert!(config.target_pause_ms.is_none());
    assert!(config.throughput_target_percent.is_none());
}

/// Test GcTuningConfig::from_env()
#[test]
fn test_gc_tuning_config_from_env() {
    let config = GcTuningConfig::from_env_provider(|k| match k {
        "BEARDOG_GC_TARGET_PAUSE_MS" => Some("100".to_string()),
        "BEARDOG_GC_THROUGHPUT_TARGET_PERCENT" => Some("95".to_string()),
        _ => None,
    });
    assert_eq!(config.target_pause_ms, Some(100));
    assert_eq!(config.throughput_target_percent, Some(95));
}
