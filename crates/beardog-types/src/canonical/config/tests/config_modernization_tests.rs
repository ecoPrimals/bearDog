//! Tests for Modernized Configuration Structs
//!
//! This module tests all configs that have been modernized to follow the pattern:
//! - `with_defaults()`: Deterministic, no env vars
//! - `from_env()`: Explicit env var reading
//! - `Default::default()`: Delegates to `with_defaults()`
//!
//! These tests ensure:
//! 1. Deterministic behavior in `with_defaults()`
//! 2. Environment variable reading in `from_env()`
//! 3. Correct default delegation
//! 4. Constant values match defaults

use crate::canonical::config::domains::{bootstrap, database, network, system, testing};
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
    ConnectionConfig, GcTuningConfig, NetworkResourceConfig, StorageResourceConfig,
};
use crate::canonical::config::security::authentication::CanonicalAuthenticationConfig;
use crate::canonical::config::security::RateLimitingConfig;
use std::time::Duration;

/// Helper to set env var for test scope
struct EnvGuard {
    key: String,
}

impl EnvGuard {
    fn set(key: &str, value: &str) -> Self {
        std::env::set_var(key, value);
        Self {
            key: key.to_string(),
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        std::env::remove_var(&self.key);
    }
}

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
    let _g1 = EnvGuard::set("BEARDOG_PATTERN_MAX_AGE_SECS", "7200");
    let _g2 = EnvGuard::set("BEARDOG_PATTERN_CONSOLIDATION_INTERVAL_SECS", "600");

    let config = bootstrap::InfantPatternConfig::from_env();

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
    let _g1 = EnvGuard::set("DATABASE_URL", "postgres://localhost/test");
    let _g2 = EnvGuard::set("DATABASE_TIMEOUT_SECONDS", "60");
    let _g3 = EnvGuard::set("DATABASE_SSL", "true");

    let config = database::DatabaseConnectionConfig::from_env();

    assert_eq!(config.url, "postgres://localhost/test");
    assert_eq!(config.timeout, Duration::from_secs(60));
    assert_eq!(config.ssl, true);
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
    let _g1 = EnvGuard::set("DATABASE_POOL_MIN_IDLE", "5");
    let _g2 = EnvGuard::set("DATABASE_POOL_IDLE_TIMEOUT_SECONDS", "1200");

    let config = database::DatabasePoolConfig::from_env();

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
    let _g1 = EnvGuard::set("DATABASE_AUTO_MIGRATE", "false");
    let _g2 = EnvGuard::set("DATABASE_MIGRATION_DIR", "db/migrations");

    let config = database::MigrationConfig::from_env();

    assert_eq!(config.auto_migrate, false);
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
    let _g1 = EnvGuard::set("BEARDOG_SYSTEM_LOG_MAX_SIZE_MB", "200");
    let _g2 = EnvGuard::set("BEARDOG_SYSTEM_LOG_MAX_FILES", "20");

    let config = system::LogRotationConfig::from_env();

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
    let _g1 = EnvGuard::set("BEARDOG_MAX_FILE_DESCRIPTORS", "32768");
    let _g2 = EnvGuard::set("BEARDOG_SYSTEM_MAX_CONNECTIONS", "5000");
    let _g3 = EnvGuard::set("BEARDOG_SYSTEM_MONITORING_INTERVAL_SECS", "30");

    let config = system::ResourceConfig::from_env();

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
    let _g = EnvGuard::set("BEARDOG_ENVIRONMENT", "production");

    let config = system::EnvironmentConfig::from_env();

    assert_eq!(config.environment_type, "production");
}

// ============================================================================
// Network Config Tests
// ============================================================================

#[test]
fn test_connection_pool_config_with_defaults() {
    use network::connection::ConnectionPoolConfig;

    let config = ConnectionPoolConfig::with_defaults();

    assert_eq!(config.max_size, ConnectionPoolConfig::DEFAULT_MAX_SIZE);
    assert_eq!(
        config.idle_timeout,
        Duration::from_secs(ConnectionPoolConfig::DEFAULT_IDLE_TIMEOUT_SECS)
    );
    assert_eq!(
        config.acquire_timeout,
        Duration::from_secs(ConnectionPoolConfig::DEFAULT_ACQUIRE_TIMEOUT_SECS)
    );
    assert_eq!(
        config.maintenance_interval,
        Duration::from_secs(ConnectionPoolConfig::DEFAULT_MAINTENANCE_INTERVAL_SECS)
    );
}

#[test]
fn test_connection_pool_config_from_env() {
    use network::connection::ConnectionPoolConfig;

    let _g1 = EnvGuard::set("BEARDOG_CONNECTION_POOL_MAX_SIZE", "200");
    let _g2 = EnvGuard::set("BEARDOG_CONNECTION_IDLE_TIMEOUT_SECS", "1200");

    let config = ConnectionPoolConfig::from_env();

    assert_eq!(config.max_size, 200);
    assert_eq!(config.idle_timeout, Duration::from_secs(1200));
}

#[test]
fn test_timeout_configuration_with_defaults() {
    use network::connection::TimeoutConfiguration;

    let config = TimeoutConfiguration::with_defaults();

    assert_eq!(
        config.keepalive_timeout_seconds,
        TimeoutConfiguration::DEFAULT_KEEPALIVE_TIMEOUT_SECS
    );
    assert_eq!(
        config.tls_handshake_timeout_seconds,
        TimeoutConfiguration::DEFAULT_TLS_HANDSHAKE_TIMEOUT_SECS
    );
    assert_eq!(
        config.read_timeout_seconds,
        TimeoutConfiguration::DEFAULT_READ_TIMEOUT_SECS
    );
    assert_eq!(
        config.write_timeout_seconds,
        TimeoutConfiguration::DEFAULT_WRITE_TIMEOUT_SECS
    );
    assert_eq!(
        config.shutdown_timeout_seconds,
        TimeoutConfiguration::DEFAULT_SHUTDOWN_TIMEOUT_SECS
    );
}

#[test]
fn test_timeout_configuration_from_env() {
    use network::connection::TimeoutConfiguration;

    let _g1 = EnvGuard::set("BEARDOG_KEEPALIVE_TIMEOUT_SECS", "120");
    let _g2 = EnvGuard::set("BEARDOG_TLS_HANDSHAKE_TIMEOUT_SECS", "60");

    let config = TimeoutConfiguration::from_env();

    assert_eq!(config.keepalive_timeout_seconds, 120);
    assert_eq!(config.tls_handshake_timeout_seconds, 60);
}

#[test]
fn test_circuit_breaker_configuration_with_defaults() {
    use network::connection::CircuitBreakerConfiguration;

    let config = CircuitBreakerConfiguration::with_defaults();

    assert_eq!(
        config.recovery_timeout_seconds,
        CircuitBreakerConfiguration::DEFAULT_RECOVERY_TIMEOUT_SECS
    );
    assert_eq!(
        config.minimum_throughput,
        CircuitBreakerConfiguration::DEFAULT_MIN_THROUGHPUT
    );
    assert!(config.enabled);
}

#[test]
fn test_circuit_breaker_configuration_from_env() {
    use network::connection::CircuitBreakerConfiguration;

    let _g1 = EnvGuard::set("BEARDOG_CIRCUIT_BREAKER_RECOVERY_TIMEOUT_SECS", "120");
    let _g2 = EnvGuard::set("BEARDOG_CIRCUIT_BREAKER_MIN_THROUGHPUT", "50");

    let config = CircuitBreakerConfiguration::from_env();

    assert_eq!(config.recovery_timeout_seconds, 120);
    assert_eq!(config.minimum_throughput, 50);
}

#[test]
fn test_load_balancer_health_check_config_with_defaults() {
    use network::connection::LoadBalancerHealthCheckConfiguration;

    let config = LoadBalancerHealthCheckConfiguration::with_defaults();

    assert_eq!(
        config.interval_seconds,
        LoadBalancerHealthCheckConfiguration::DEFAULT_INTERVAL_SECS
    );
    assert_eq!(
        config.unhealthy_threshold,
        LoadBalancerHealthCheckConfiguration::DEFAULT_UNHEALTHY_THRESHOLD
    );
    assert_eq!(
        config.healthy_threshold,
        LoadBalancerHealthCheckConfiguration::DEFAULT_HEALTHY_THRESHOLD
    );
}

#[test]
fn test_load_balancer_health_check_config_from_env() {
    use network::connection::LoadBalancerHealthCheckConfiguration;

    let _g1 = EnvGuard::set("BEARDOG_LB_HEALTH_CHECK_INTERVAL_SECS", "60");
    let _g2 = EnvGuard::set("BEARDOG_LB_UNHEALTHY_THRESHOLD", "5");
    let _g3 = EnvGuard::set("BEARDOG_LB_HEALTHY_THRESHOLD", "3");

    let config = LoadBalancerHealthCheckConfiguration::from_env();

    assert_eq!(config.interval_seconds, 60);
    assert_eq!(config.unhealthy_threshold, 5);
    assert_eq!(config.healthy_threshold, 3);
}

// ============================================================================
// Testing Config Tests
// ============================================================================

#[test]
fn test_canonical_api_test_config_with_defaults() {
    use testing::CanonicalApiTestConfig;

    let config = CanonicalApiTestConfig::with_defaults();

    assert_eq!(
        config.timeout_seconds,
        CanonicalApiTestConfig::DEFAULT_TIMEOUT_SECS
    );
    assert_eq!(
        config.max_concurrent_requests,
        CanonicalApiTestConfig::DEFAULT_MAX_CONCURRENT
    );
    assert_eq!(
        config.max_retries,
        CanonicalApiTestConfig::DEFAULT_MAX_RETRIES
    );
    assert_eq!(
        config.retry_delay_ms,
        CanonicalApiTestConfig::DEFAULT_RETRY_DELAY_MS
    );
}

#[test]
fn test_canonical_api_test_config_from_env() {
    use testing::CanonicalApiTestConfig;

    let _g1 = EnvGuard::set("BEARDOG_API_TEST_TIMEOUT_SECS", "60");
    let _g2 = EnvGuard::set("BEARDOG_API_TEST_MAX_CONCURRENT", "20");

    let config = CanonicalApiTestConfig::from_env();

    assert_eq!(config.timeout_seconds, 60);
    assert_eq!(config.max_concurrent_requests, 20);
}

#[test]
fn test_canonical_production_test_config_with_defaults() {
    use testing::CanonicalProductionTestConfig;

    let config = CanonicalProductionTestConfig::with_defaults();

    assert_eq!(
        config.health_check_timeout_seconds,
        CanonicalProductionTestConfig::DEFAULT_HEALTH_TIMEOUT_SECS
    );
    assert_eq!(
        config.canary_percentage,
        CanonicalProductionTestConfig::DEFAULT_CANARY_PERCENTAGE
    );
}

#[test]
fn test_canonical_production_test_config_from_env() {
    use testing::CanonicalProductionTestConfig;

    let _g1 = EnvGuard::set("BEARDOG_PROD_TEST_HEALTH_TIMEOUT_SECS", "60");
    let _g2 = EnvGuard::set("BEARDOG_CANARY_PERCENTAGE", "10.0");

    let config = CanonicalProductionTestConfig::from_env();

    assert_eq!(config.health_check_timeout_seconds, 60);
    assert_eq!(config.canary_percentage, 10.0);
}

// ============================================================================
// Determinism Tests (Critical for Test Reliability)
// ============================================================================

#[test]
fn test_all_with_defaults_are_deterministic() {
    // Call with_defaults() multiple times and ensure same results
    let c1 = bootstrap::InfantPatternConfig::with_defaults();
    let c2 = bootstrap::InfantPatternConfig::with_defaults();
    assert_eq!(
        c1, c2,
        "InfantPatternConfig::with_defaults() is not deterministic"
    );

    let c1 = database::DatabaseConnectionConfig::with_defaults();
    let c2 = database::DatabaseConnectionConfig::with_defaults();
    assert_eq!(c1.url, c2.url);
    assert_eq!(c1.timeout, c2.timeout);
    assert_eq!(c1.ssl, c2.ssl);

    let c1 = system::LogRotationConfig::with_defaults();
    let c2 = system::LogRotationConfig::with_defaults();
    assert_eq!(c1.max_size_mb, c2.max_size_mb);
    assert_eq!(c1.max_files, c2.max_files);
}

#[test]
fn test_constants_match_defaults() {
    use bootstrap::InfantPatternConfig;
    use database::DatabaseConnectionConfig;
    use system::ResourceConfig;

    // Ensure constants match what with_defaults() actually returns
    let config = InfantPatternConfig::with_defaults();
    assert_eq!(
        config.min_observations,
        InfantPatternConfig::DEFAULT_MIN_OBSERVATIONS
    );
    assert_eq!(
        config.confidence_threshold,
        InfantPatternConfig::DEFAULT_CONFIDENCE_THRESHOLD
    );

    let config = DatabaseConnectionConfig::with_defaults();
    assert_eq!(config.url, DatabaseConnectionConfig::DEFAULT_URL);

    let config = ResourceConfig::with_defaults();
    assert_eq!(
        config.max_connections,
        ResourceConfig::DEFAULT_MAX_CONNECTIONS
    );
}

#[test]
fn test_from_env_fallbacks_to_defaults() {
    // Ensure from_env() uses defaults when env vars not set
    // Clear any existing env vars first
    std::env::remove_var("BEARDOG_PATTERN_MAX_AGE_SECS");
    std::env::remove_var("BEARDOG_PATTERN_CONSOLIDATION_INTERVAL_SECS");

    let config = bootstrap::InfantPatternConfig::from_env();
    let defaults = bootstrap::InfantPatternConfig::with_defaults();

    assert_eq!(
        config, defaults,
        "from_env() should fallback to defaults when env vars not set"
    );
}

#[test]
fn test_default_delegates_to_with_defaults() {
    // Verify Default trait delegates to with_defaults()
    let default_config = bootstrap::InfantPatternConfig::default();
    let with_defaults_config = bootstrap::InfantPatternConfig::with_defaults();

    assert_eq!(default_config, with_defaults_config);
}

// ============================================================================
// Config Batch 5 Tests (Modernized: Nov 20, 2025)
// ============================================================================

/// Test FailoverConfiguration::with_defaults()
#[test]
fn test_failover_configuration_with_defaults() {
    let config = network::connection::FailoverConfiguration::with_defaults();
    assert_eq!(config.enabled, true);
    assert_eq!(
        config.detection_timeout_seconds,
        network::connection::FailoverConfiguration::DEFAULT_DETECTION_TIMEOUT_SECS
    );
    assert_eq!(
        config.max_attempts,
        network::connection::FailoverConfiguration::DEFAULT_MAX_ATTEMPTS
    );
}

/// Test FailoverConfiguration::from_env()
#[test]
fn test_failover_configuration_from_env() {
    let _g1 = EnvGuard::set("BEARDOG_FAILOVER_DETECTION_TIMEOUT_SECS", "60");
    let _g2 = EnvGuard::set("BEARDOG_FAILOVER_MAX_ATTEMPTS", "5");

    let config = network::connection::FailoverConfiguration::from_env();
    assert_eq!(config.detection_timeout_seconds, 60);
    assert_eq!(config.max_attempts, 5);
}

/// Test RateLimitingConfig::with_defaults()
#[test]
fn test_rate_limiting_config_with_defaults() {
    let config = RateLimitingConfig::with_defaults();
    assert_eq!(config.enabled, true);
    assert_eq!(
        config.max_requests_per_minute,
        RateLimitingConfig::DEFAULT_MAX_REQUESTS_PER_MINUTE
    );
    assert_eq!(
        config.burst_capacity,
        RateLimitingConfig::DEFAULT_BURST_CAPACITY
    );
    assert_eq!(
        config.window_seconds,
        RateLimitingConfig::DEFAULT_WINDOW_SECS
    );
}

/// Test RateLimitingConfig::from_env()
#[test]
fn test_rate_limiting_config_from_env() {
    let _g1 = EnvGuard::set("BEARDOG_RATE_LIMIT_MAX_REQUESTS_PER_MIN", "200");
    let _g2 = EnvGuard::set("BEARDOG_RATE_LIMIT_BURST_CAPACITY", "20");
    let _g3 = EnvGuard::set("BEARDOG_RATE_LIMIT_WINDOW_SECS", "120");

    let config = RateLimitingConfig::from_env();
    assert_eq!(config.max_requests_per_minute, 200);
    assert_eq!(config.burst_capacity, 20);
    assert_eq!(config.window_seconds, 120);
}

/// Test ThreadingConfig::with_defaults()
#[test]
fn test_threading_config_with_defaults() {
    let config = system::ThreadingConfig::with_defaults();
    assert_eq!(
        config.worker_threads,
        system::ThreadingConfig::DEFAULT_WORKER_THREADS
    );
    assert_eq!(
        config.blocking_threads,
        system::ThreadingConfig::DEFAULT_BLOCKING_THREADS
    );
    assert_eq!(config.stack_size, None);
    assert_eq!(config.enable_tls_optimization, true);
}

/// Test ThreadingConfig::from_env()
#[test]
fn test_threading_config_from_env() {
    let _g1 = EnvGuard::set("BEARDOG_WORKER_THREADS", "8");
    let _g2 = EnvGuard::set("BEARDOG_BLOCKING_THREADS", "16");

    let config = system::ThreadingConfig::from_env();
    assert_eq!(config.worker_threads, 8);
    assert_eq!(config.blocking_threads, 16);
}

/// Test NetworkResourceConfig::with_defaults()
#[test]
fn test_network_resource_config_with_defaults() {
    let config = NetworkResourceConfig::with_defaults();
    assert_eq!(
        config.max_connections,
        NetworkResourceConfig::DEFAULT_MAX_CONNECTIONS
    );
    assert_eq!(
        config.connection_timeout,
        Duration::from_secs(NetworkResourceConfig::DEFAULT_CONNECTION_TIMEOUT_SECS)
    );
    assert_eq!(
        config.read_timeout,
        Duration::from_secs(NetworkResourceConfig::DEFAULT_READ_TIMEOUT_SECS)
    );
    assert_eq!(
        config.write_timeout,
        Duration::from_secs(NetworkResourceConfig::DEFAULT_WRITE_TIMEOUT_SECS)
    );
    assert_eq!(config.keep_alive, true);
    assert_eq!(config.tcp_nodelay, true);
}

/// Test NetworkResourceConfig::from_env()
#[test]
fn test_network_resource_config_from_env() {
    let _g1 = EnvGuard::set("BEARDOG_PROD_MAX_CONNECTIONS", "2000");
    let _g2 = EnvGuard::set("BEARDOG_PROD_CONNECTION_TIMEOUT_SECS", "60");
    let _g3 = EnvGuard::set("BEARDOG_PROD_READ_TIMEOUT_SECS", "45");
    let _g4 = EnvGuard::set("BEARDOG_PROD_WRITE_TIMEOUT_SECS", "45");

    let config = NetworkResourceConfig::from_env();
    assert_eq!(config.max_connections, 2000);
    assert_eq!(config.connection_timeout, Duration::from_secs(60));
    assert_eq!(config.read_timeout, Duration::from_secs(45));
    assert_eq!(config.write_timeout, Duration::from_secs(45));
}

/// Test CanonicalAuthenticationConfig::with_defaults()
#[test]
fn test_authentication_config_with_defaults() {
    let config = CanonicalAuthenticationConfig::with_defaults();
    assert_eq!(config.jwt_secret.as_ref(), "CHANGE_ME_IN_PRODUCTION");
    assert_eq!(
        config.jwt_expiration_seconds,
        CanonicalAuthenticationConfig::DEFAULT_JWT_EXPIRATION_SECS
    );
    assert_eq!(
        config.jwt_refresh_expiration_seconds,
        CanonicalAuthenticationConfig::DEFAULT_JWT_REFRESH_EXPIRATION_SECS
    );
    assert_eq!(
        config.api_key_min_length,
        CanonicalAuthenticationConfig::DEFAULT_API_KEY_MIN_LENGTH
    );
    assert_eq!(
        config.password_min_length,
        CanonicalAuthenticationConfig::DEFAULT_PASSWORD_MIN_LENGTH
    );
    assert_eq!(
        config.max_auth_attempts,
        CanonicalAuthenticationConfig::DEFAULT_MAX_AUTH_ATTEMPTS
    );
    assert_eq!(
        config.lockout_duration_seconds,
        CanonicalAuthenticationConfig::DEFAULT_LOCKOUT_DURATION_SECS
    );
}

/// Test CanonicalAuthenticationConfig::from_env()
#[test]
fn test_authentication_config_from_env() {
    let _g1 = EnvGuard::set("BEARDOG_JWT_EXPIRATION_SECS", "7200");
    let _g2 = EnvGuard::set("BEARDOG_JWT_REFRESH_EXPIRATION_SECS", "172800");
    let _g3 = EnvGuard::set("BEARDOG_API_KEY_MIN_LENGTH", "64");
    let _g4 = EnvGuard::set("BEARDOG_PASSWORD_MIN_LENGTH", "16");
    let _g5 = EnvGuard::set("BEARDOG_MAX_AUTH_ATTEMPTS", "5");
    let _g6 = EnvGuard::set("BEARDOG_LOCKOUT_DURATION_SECS", "1800");

    let config = CanonicalAuthenticationConfig::from_env();
    assert_eq!(config.jwt_expiration_seconds, 7200);
    assert_eq!(config.jwt_refresh_expiration_seconds, 172_800);
    assert_eq!(config.api_key_min_length, 64);
    assert_eq!(config.password_min_length, 16);
    assert_eq!(config.max_auth_attempts, 5);
    assert_eq!(config.lockout_duration_seconds, 1800);
}

// ============================================================================
// 50% Milestone Config (Nov 20, 2025)
// ============================================================================

/// Test ApplicationConfig::with_defaults()
#[test]
fn test_application_config_with_defaults() {
    let config = system::ApplicationConfig::with_defaults();
    assert_eq!(config.name, system::ApplicationConfig::DEFAULT_NAME);
    assert_eq!(config.version, "0.0.0-dev"); // Deterministic placeholder
    assert_eq!(config.instance_id, "default-instance"); // Deterministic placeholder
    assert_eq!(
        config.description,
        system::ApplicationConfig::DEFAULT_DESCRIPTION
    );
    assert!(config.features.is_empty());
}

/// Test ApplicationConfig::from_env()
#[test]
fn test_application_config_from_env() {
    let _g1 = EnvGuard::set("BEARDOG_APP_NAME", "CustomApp");
    let _g2 = EnvGuard::set("BEARDOG_APP_DESCRIPTION", "Custom Description");

    let config = system::ApplicationConfig::from_env();
    assert_eq!(config.name, "CustomApp");
    assert_eq!(config.description, "Custom Description");
    // Version comes from CARGO_PKG_VERSION (compile time)
    assert!(!config.version.is_empty());
    // Instance ID is UUID (non-deterministic but valid format)
    assert!(!config.instance_id.is_empty());
}

// ============================================================================
// Batch 6: Production Resources & Operations Configs
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
    let _g1 = EnvGuard::set("BEARDOG_MAX_DISK_USAGE_PERCENT", "90.5");
    let _g2 = EnvGuard::set("BEARDOG_TEMP_CLEANUP_INTERVAL_SECS", "7200");
    let _g3 = EnvGuard::set("BEARDOG_LOG_ROTATION_SIZE_MB", "200");
    let _g4 = EnvGuard::set("BEARDOG_LOG_RETENTION_DAYS", "60");

    let config = StorageResourceConfig::from_env();
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
    let _g1 = EnvGuard::set("BEARDOG_CONNECTION_POOL_SIZE", "20");
    let _g2 = EnvGuard::set("BEARDOG_MAX_IDLE_CONNECTIONS", "10");
    let _g3 = EnvGuard::set("BEARDOG_CONNECTION_LIFETIME_SECS", "3600");
    let _g4 = EnvGuard::set("BEARDOG_CONNECTION_HEALTH_CHECK_INTERVAL_SECS", "120");

    let config = ConnectionConfig::from_env();
    assert_eq!(config.pool_size, 20);
    assert_eq!(config.max_idle_connections, 10);
    assert_eq!(config.connection_lifetime, Duration::from_secs(3600));
    assert_eq!(config.health_check_interval, Duration::from_secs(120));
}

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
    let _g1 = EnvGuard::set("BEARDOG_MAINTENANCE_WINDOW_SECS", "7200");

    let config = MaintenanceConfig::from_env();
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
    let _g1 = EnvGuard::set("BEARDOG_BACKUP_INTERVAL_SECS", "43200");

    let config = BackupConfig::from_env();
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
    let _g1 = EnvGuard::set("BEARDOG_DR_RTO_SECS", "1800");

    let config = DisasterRecoveryConfig::from_env();
    assert!(!config.enabled);
    assert_eq!(config.rto, Duration::from_secs(1800));
}

// ============================================================================
// Batch 7: Environment & Observability Configs
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
    let _g1 = EnvGuard::set("BEARDOG_ENV_VALIDATION_INTERVAL_SECS", "600");

    let config = EnvironmentValidation::from_env();
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
    let _g1 = EnvGuard::set("BEARDOG_METRICS_ENDPOINT", "/custom-metrics");

    let config = ProductionMetricsConfig::from_env();
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
    let _g1 = EnvGuard::set("BEARDOG_LOG_LEVEL", "debug");
    let _g2 = EnvGuard::set("BEARDOG_LOG_FORMAT", "text");

    let config = ProductionLoggingConfig::from_env();
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
    let _g1 = EnvGuard::set("BEARDOG_TRACING_ENDPOINT", "/custom-traces");

    let config = ProductionTracingConfig::from_env();
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
    let _g1 = EnvGuard::set("BEARDOG_DASHBOARD_ENDPOINT", "/admin/dashboard");

    let config = DashboardConfig::from_env();
    assert_eq!(config.endpoint, "/admin/dashboard");
}

// ============================================================================
// Batch 8: Deployment, Core & Load Balancing Configs
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
    let _g1 = EnvGuard::set("BEARDOG_ROLLOUT_PERCENTAGE", "50.0");

    let config = RolloutConfig::from_env();
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
    let _g1 = EnvGuard::set("BEARDOG_CANARY_PERCENTAGE", "5.0");

    let config = CanaryConfig::from_env();
    assert_eq!(config.percentage, 5.0);
}

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
    let _g1 = EnvGuard::set("BEARDOG_ENABLE_PERFORMANCE_PROFILING", "true");
    let _g2 = EnvGuard::set("BEARDOG_ENABLE_AUTO_SCALING", "true");

    let config = ProductionFeatureFlags::from_env();
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
    let _g1 = EnvGuard::set("BEARDOG_GC_TARGET_PAUSE_MS", "100");
    let _g2 = EnvGuard::set("BEARDOG_GC_THROUGHPUT_TARGET_PERCENT", "95");

    let config = GcTuningConfig::from_env();
    assert_eq!(config.target_pause_ms, Some(100));
    assert_eq!(config.throughput_target_percent, Some(95));
}
