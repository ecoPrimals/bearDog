// SPDX-License-Identifier: AGPL-3.0-only

// ! Tests for Core Configuration (Bootstrap, Database, System)
//!
//! Tests configs that have been modernized to follow the pattern:
//! - `with_defaults()`: Deterministic, no env vars
//! - `from_env()`: Explicit env var reading
//! - `Default::default()`: Delegates to `with_defaults()`

use crate::canonical::config::domains::{bootstrap, database, system};
use std::time::Duration;

// ============================================================================
// Bootstrap Config Tests
// ============================================================================

#[test]
fn test_infant_pattern_config_with_defaults() {
    let config = bootstrap::InfantPatternConfig::with_defaults();

    assert_eq!(
        config.min_observations,
        bootstrap::InfantPatternConfig::DEFAULT_MIN_OBSERVATIONS
    );
    assert_eq!(
        config.confidence_threshold,
        bootstrap::InfantPatternConfig::DEFAULT_CONFIDENCE_THRESHOLD
    );
    assert_eq!(
        config.pattern_max_age,
        Duration::from_secs(bootstrap::InfantPatternConfig::DEFAULT_PATTERN_MAX_AGE_SECS)
    );
    assert_eq!(
        config.learning_rate,
        bootstrap::InfantPatternConfig::DEFAULT_LEARNING_RATE
    );
    assert!(config.enable_continuous_learning);
    assert_eq!(
        config.consolidation_interval,
        Duration::from_secs(bootstrap::InfantPatternConfig::DEFAULT_CONSOLIDATION_INTERVAL_SECS)
    );
}

#[test]
fn test_infant_pattern_config_from_env() {
    let config = bootstrap::InfantPatternConfig::from_env_provider(|k| match k {
        "BEARDOG_PATTERN_MAX_AGE_SECS" => Some("7200".to_string()),
        "BEARDOG_PATTERN_CONSOLIDATION_INTERVAL_SECS" => Some("600".to_string()),
        _ => None,
    });

    assert_eq!(config.pattern_max_age, Duration::from_secs(7200));
    assert_eq!(config.consolidation_interval, Duration::from_secs(600));
}

#[test]
fn test_infant_pattern_config_default_uses_with_defaults() {
    let default_config = bootstrap::InfantPatternConfig::default();
    let with_defaults_config = bootstrap::InfantPatternConfig::with_defaults();

    assert_eq!(default_config, with_defaults_config);
}

// ============================================================================
// Database Config Tests
// ============================================================================

#[test]
fn test_database_connection_config_with_defaults() {
    let config = database::DatabaseConnectionConfig::with_defaults();

    assert_eq!(config.url, database::DatabaseConnectionConfig::DEFAULT_URL);
    assert_eq!(
        config.timeout,
        Duration::from_secs(database::DatabaseConnectionConfig::DEFAULT_TIMEOUT_SECS)
    );
    assert_eq!(config.ssl, database::DatabaseConnectionConfig::DEFAULT_SSL);
}

#[test]
fn test_database_connection_config_from_env() {
    let config = database::DatabaseConnectionConfig::from_env_provider(|k| match k {
        "DATABASE_URL" => Some("postgres://localhost/test".to_string()),
        "DATABASE_TIMEOUT_SECONDS" => Some("60".to_string()),
        "DATABASE_SSL" => Some("true".to_string()),
        _ => None,
    });

    assert_eq!(config.url, "postgres://localhost/test");
    assert_eq!(config.timeout, Duration::from_secs(60));
    assert!(config.ssl);
}

#[test]
fn test_database_pool_config_with_defaults() {
    let config = database::DatabasePoolConfig::with_defaults();

    assert_eq!(
        config.min_idle,
        database::DatabasePoolConfig::DEFAULT_MIN_IDLE
    );
    assert_eq!(
        config.idle_timeout,
        Duration::from_secs(database::DatabasePoolConfig::DEFAULT_IDLE_TIMEOUT_SECS)
    );
}

#[test]
fn test_database_pool_config_from_env() {
    let config = database::DatabasePoolConfig::from_env_provider(|k| match k {
        "DATABASE_POOL_MIN_IDLE" => Some("5".to_string()),
        "DATABASE_POOL_IDLE_TIMEOUT_SECONDS" => Some("1200".to_string()),
        _ => None,
    });

    assert_eq!(config.min_idle, 5);
    assert_eq!(config.idle_timeout, Duration::from_secs(1200));
}

#[test]
fn test_migration_config_with_defaults() {
    let config = database::MigrationConfig::with_defaults();

    assert_eq!(
        config.auto_migrate,
        database::MigrationConfig::DEFAULT_AUTO_MIGRATE
    );
    assert_eq!(
        config.directory,
        database::MigrationConfig::DEFAULT_DIRECTORY
    );
}

#[test]
fn test_migration_config_from_env() {
    let config = database::MigrationConfig::from_env_provider(|k| match k {
        "DATABASE_AUTO_MIGRATE" => Some("false".to_string()),
        "DATABASE_MIGRATION_DIR" => Some("db/migrations".to_string()),
        _ => None,
    });

    assert!(!config.auto_migrate);
    assert_eq!(config.directory, "db/migrations");
}

// ============================================================================
// System Config Tests
// ============================================================================

#[test]
fn test_log_rotation_config_with_defaults() {
    let config = system::LogRotationConfig::with_defaults();

    assert_eq!(
        config.max_size_mb,
        system::LogRotationConfig::DEFAULT_MAX_SIZE_MB
    );
    assert_eq!(
        config.max_files,
        system::LogRotationConfig::DEFAULT_MAX_FILES
    );
}

#[test]
fn test_log_rotation_config_from_env() {
    let config = system::LogRotationConfig::from_env_provider(|k| match k {
        "BEARDOG_SYSTEM_LOG_MAX_SIZE_MB" => Some("200".to_string()),
        "BEARDOG_SYSTEM_LOG_MAX_FILES" => Some("20".to_string()),
        _ => None,
    });

    assert_eq!(config.max_size_mb, 200);
    assert_eq!(config.max_files, 20);
}

#[test]
fn test_resource_config_with_defaults() {
    let config = system::ResourceConfig::with_defaults();

    assert_eq!(
        config.max_file_descriptors,
        Some(system::ResourceConfig::DEFAULT_MAX_FILE_DESCRIPTORS)
    );
    assert_eq!(
        config.max_connections,
        system::ResourceConfig::DEFAULT_MAX_CONNECTIONS
    );
    assert_eq!(
        config.monitoring_interval,
        Duration::from_secs(system::ResourceConfig::DEFAULT_MONITORING_INTERVAL_SECS)
    );
}

#[test]
fn test_resource_config_from_env() {
    let config = system::ResourceConfig::from_env_provider(|k| match k {
        "BEARDOG_MAX_FILE_DESCRIPTORS" => Some("32768".to_string()),
        "BEARDOG_SYSTEM_MAX_CONNECTIONS" => Some("5000".to_string()),
        "BEARDOG_SYSTEM_MONITORING_INTERVAL_SECS" => Some("30".to_string()),
        _ => None,
    });

    assert_eq!(config.max_file_descriptors, Some(32768));
    assert_eq!(config.max_connections, 5000);
    assert_eq!(config.monitoring_interval, Duration::from_secs(30));
}

#[test]
fn test_environment_config_with_defaults() {
    let config = system::EnvironmentConfig::with_defaults();

    assert_eq!(
        config.environment_type,
        system::EnvironmentConfig::DEFAULT_ENVIRONMENT_TYPE
    );
    assert!(config.variables.is_empty());
    assert!(config.overrides.is_empty());
}

#[test]
fn test_environment_config_from_env() {
    let config = system::EnvironmentConfig::from_env_provider(|k| match k {
        "BEARDOG_ENVIRONMENT" => Some("production".to_string()),
        _ => None,
    });

    assert_eq!(config.environment_type, "production");
}
