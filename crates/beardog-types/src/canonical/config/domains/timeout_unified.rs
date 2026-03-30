// SPDX-License-Identifier: AGPL-3.0-only

//! Unified Timeout Configuration (DEPRECATED)
//!
//! **⚠️ DEPRECATION NOTICE**: This module has been unified into `timeout.rs`
//!
//! All functionality from `UnifiedTimeoutConfig` has been merged into
//! `CanonicalTimeoutConfig` to eliminate duplication.
//!
//! ## Migration Guide
//!
//! ### Old Code (before unification):
//! ```rust,ignore
//! use beardog_types::canonical::config::domains::timeout_unified::UnifiedTimeoutConfig;
//! let config = UnifiedTimeoutConfig::default();
//! let connect_timeout_secs = config.connect_timeout_secs;
//! ```
//!
//! ### New Code (after unification):
//! ```rust,ignore
//! use beardog_types::canonical::config::domains::timeout::CanonicalTimeoutConfig;
//! let config = CanonicalTimeoutConfig::default();
//! let connect_timeout = config.connect_timeout; // Now Duration, not u64
//! ```
//!
//! ## Key Changes
//!
//! 1. **Type Safety**: Changed from `u64` (seconds) to `Duration` for better type safety
//! 2. **Comprehensive**: Includes both network and domain-specific timeouts
//! 3. **Single Source**: One canonical timeout config, not two separate ones
//! 4. **Backward Compatible**: Type aliases provided for compatibility
//!
//! ## Benefits of Unification
//!
//! - ✅ **No Duplication**: One config instead of two
//! - ✅ **Type Safety**: Duration instead of raw u64
//! - ✅ **Better Validation**: Comprehensive validation in one place
//! - ✅ **Clearer Intent**: Duration makes code more readable
//! - ✅ **Single Source of Truth**: No confusion about which config to use

// Re-export the canonical type
pub use super::timeout::CanonicalTimeoutConfig;

/// `UnifiedTimeoutConfig` is now an alias to `CanonicalTimeoutConfig`
///
/// **⚠️ DEPRECATED**: Use `CanonicalTimeoutConfig` directly instead.
///
/// This type alias is provided for backward compatibility only and will be
/// removed in a future version.
#[deprecated(
    since = "3.1.0",
    note = "Use CanonicalTimeoutConfig from timeout module instead. UnifiedTimeoutConfig has been merged into CanonicalTimeoutConfig for better type safety and to eliminate duplication."
)]
pub type UnifiedTimeoutConfig = CanonicalTimeoutConfig;

/// `TimeoutConfig` is now an alias to `CanonicalTimeoutConfig`
///
/// **⚠️ DEPRECATED**: Use `CanonicalTimeoutConfig` directly instead.
///
/// This type alias is provided for backward compatibility only.
#[deprecated(
    since = "3.1.0",
    note = "Use CanonicalTimeoutConfig from timeout module instead"
)]
pub type TimeoutConfig = CanonicalTimeoutConfig;

// Note: UnifiedTimeoutConfigBuilder from the old implementation is no longer needed
// as CanonicalTimeoutConfig provides better preset methods (default, aggressive, conservative, etc.)

#[cfg(test)]
mod unification_migration_tests {
    use super::*;
    use std::time::Duration;

    #[test]
    #[allow(deprecated)]
    fn test_unified_timeout_config_alias_works() {
        // Old code using UnifiedTimeoutConfig should still compile
        let _config: UnifiedTimeoutConfig = UnifiedTimeoutConfig::default();

        // Verify it's actually CanonicalTimeoutConfig
        let config = CanonicalTimeoutConfig::default();
        assert_eq!(config.connect_timeout, Duration::from_secs(5));
        assert_eq!(config.health_check_timeout, Duration::from_secs(5));
    }

    #[test]
    fn test_canonical_timeout_has_all_features() {
        let config = CanonicalTimeoutConfig::default();

        // Network timeouts (from old CanonicalTimeoutConfig)
        assert_eq!(config.connect_timeout, Duration::from_secs(5));
        assert_eq!(config.read_timeout, Duration::from_secs(30));
        assert_eq!(config.write_timeout, Duration::from_secs(30));
        assert_eq!(config.operation_timeout, Duration::from_secs(60));

        // Domain timeouts (from old UnifiedTimeoutConfig)
        assert_eq!(config.health_check_timeout, Duration::from_secs(5));
        assert_eq!(config.hsm_operation_timeout, Duration::from_secs(2));
        assert_eq!(config.hsm_probe_timeout, Duration::from_millis(500));
        assert_eq!(config.discovery_timeout, Duration::from_secs(10));
        assert_eq!(config.ai_decision_timeout, Duration::from_secs(30));
        assert_eq!(config.ai_request_timeout, Duration::from_secs(30));
        assert_eq!(config.ai_batch_timeout, Duration::from_millis(10));
    }

    #[test]
    fn test_presets_include_domain_timeouts() {
        // Verify aggressive preset has all fields
        let aggressive = CanonicalTimeoutConfig::aggressive();
        assert_eq!(aggressive.connect_timeout, Duration::from_secs(1));
        assert_eq!(aggressive.health_check_timeout, Duration::from_secs(2));
        assert_eq!(aggressive.hsm_operation_timeout, Duration::from_secs(1));

        // Verify conservative preset has all fields
        let conservative = CanonicalTimeoutConfig::conservative();
        assert_eq!(conservative.connect_timeout, Duration::from_secs(30));
        assert_eq!(conservative.health_check_timeout, Duration::from_secs(15));
        assert_eq!(conservative.hsm_operation_timeout, Duration::from_secs(5));
    }
}
