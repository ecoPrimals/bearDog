// SPDX-License-Identifier: AGPL-3.0-only

//! Meta Tests for Configuration Modernization
//!
//! These tests verify that the modernization pattern is correctly implemented:
//! - Deterministic `with_defaults()` behavior
//! - Constants match actual default values
//! - `from_env()` falls back to defaults
//! - `Default` trait delegates to `with_defaults()`

use crate::canonical::config::domains::{bootstrap, database, system};

/// Helper to set env var for test scope
#[allow(dead_code)] // Used in some test scenarios
struct EnvGuard {
    key: String,
}

#[allow(dead_code)] // Used in some test scenarios
impl EnvGuard {
    fn set(key: &str, value: &str) -> Self {
        beardog_errors::process_env::set_var(key, value);
        Self {
            key: key.to_string(),
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        beardog_errors::process_env::remove_var(&self.key);
    }
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
    beardog_errors::process_env::remove_var("BEARDOG_PATTERN_MAX_AGE_SECS");
    beardog_errors::process_env::remove_var("BEARDOG_PATTERN_CONSOLIDATION_INTERVAL_SECS");

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
