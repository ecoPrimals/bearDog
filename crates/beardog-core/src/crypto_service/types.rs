// SPDX-License-Identifier: AGPL-3.0-only

//! Types for crypto service configuration and state
//!
//! This module contains all internal types used by the crypto service,
//! separated from the public trait API for better organization.

use std::sync::atomic::AtomicU64;
use std::time::SystemTime;

/// Configuration for crypto service
///
/// Controls how the crypto service operates, what features are enabled,
/// and operational limits.
///
/// # Configuration Philosophy
///
/// All configuration should be:
/// - Discoverable at runtime (no hardcoded assumptions)
/// - Overridable via environment variables
/// - Documented with sensible defaults
///
/// # Example
///
/// ```rust
/// use beardog_core::crypto_service::CryptoServiceConfig;
///
/// let config = CryptoServiceConfig {
///     service_name: "my-primal".to_string(),
///     hsm_enabled: true,          // Use hardware HSM if available
///     genetic_enabled: true,      // Enable genetic key mixing
///     max_data_size: 50 * 1024 * 1024, // 50MB max
///     audit_enabled: true,        // Log all operations
/// };
/// ```
#[derive(Debug, Clone)]
pub struct CryptoServiceConfig {
    /// Service name for identification and audit logs
    ///
    /// This should be the primal's self-identified name, not hardcoded.
    /// Used for audit trails and capability announcements.
    pub service_name: String,

    /// Enable HSM operations
    ///
    /// If `true`, the service will attempt to use hardware HSMs for
    /// cryptographic operations when available. Falls back to software
    /// crypto if no HSM is discovered.
    ///
    /// Discovery is runtime-based, not configuration-based.
    pub hsm_enabled: bool,

    /// Enable genetic key mixing
    ///
    /// If `true`, keys can be derived through genetic algorithms
    /// (mixed lineage keys with primal + human components).
    pub genetic_enabled: bool,

    /// Maximum data size for single operation (bytes)
    ///
    /// Protects against `DoS` attacks and memory exhaustion.
    /// For larger data, use streaming or chunked operations.
    pub max_data_size: usize,

    /// Enable audit logging
    ///
    /// If `true`, all operations are logged for compliance and debugging.
    /// Logs metadata only, never key material or plaintext.
    pub audit_enabled: bool,
}

impl Default for CryptoServiceConfig {
    fn default() -> Self {
        Self {
            // Default name - should be overridden with actual primal identity
            service_name: "beardog".to_string(),

            // Conservative defaults - disable advanced features
            hsm_enabled: false,    // Require explicit opt-in
            genetic_enabled: true, // Enable by default (BearDog feature)

            // Reasonable limits
            max_data_size: 10 * 1024 * 1024, // 10MB default

            // Enable audit by default for security
            audit_enabled: true,
        }
    }
}

impl CryptoServiceConfig {
    /// Load configuration from environment variables
    ///
    /// Checks for environment variables in this order:
    /// 1. `BEARDOG_CRYPTO_*` variables
    /// 2. Falls back to defaults
    ///
    /// # Example
    ///
    /// ```bash
    /// export BEARDOG_CRYPTO_SERVICE_NAME=my-primal
    /// export BEARDOG_CRYPTO_HSM_ENABLED=true
    /// export BEARDOG_CRYPTO_MAX_DATA_SIZE=52428800  # 50MB
    /// ```
    #[must_use]
    pub fn from_env() -> Self {
        let mut config = Self::default();

        // Service name
        if let Ok(name) = beardog_errors::process_env::var("BEARDOG_CRYPTO_SERVICE_NAME") {
            config.service_name = name;
        }

        // HSM enabled
        if let Ok(enabled) = beardog_errors::process_env::var("BEARDOG_CRYPTO_HSM_ENABLED") {
            config.hsm_enabled = enabled.parse().unwrap_or(false);
        }

        // Genetic enabled
        if let Ok(enabled) = beardog_errors::process_env::var("BEARDOG_CRYPTO_GENETIC_ENABLED") {
            config.genetic_enabled = enabled.parse().unwrap_or(true);
        }

        // Max data size
        if let Ok(size) = beardog_errors::process_env::var("BEARDOG_CRYPTO_MAX_DATA_SIZE")
            && let Ok(parsed) = size.parse()
        {
            config.max_data_size = parsed;
        }

        // Audit enabled
        if let Ok(enabled) = beardog_errors::process_env::var("BEARDOG_CRYPTO_AUDIT_ENABLED") {
            config.audit_enabled = enabled.parse().unwrap_or(true);
        }

        config
    }
}

/// Runtime state for crypto service
///
/// Tracks operational metrics and service state.
/// Designed for concurrent access (uses atomics).
#[derive(Debug)]
pub struct CryptoServiceState {
    /// Operation counter for audit trail and metrics
    ///
    /// Atomically incremented for each operation.
    /// Used for generating unique operation IDs.
    pub(crate) operation_count: AtomicU64,

    /// Service start time
    ///
    /// Used to calculate uptime and for health checks.
    pub(crate) start_time: SystemTime,
}

impl CryptoServiceState {
    /// Create new service state
    #[must_use]
    pub fn new() -> Self {
        Self {
            operation_count: AtomicU64::new(0),
            start_time: SystemTime::now(),
        }
    }

    /// Get current operation count
    pub fn operation_count(&self) -> u64 {
        self.operation_count
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Get service uptime
    pub fn uptime(&self) -> std::time::Duration {
        self.start_time
            .elapsed()
            .unwrap_or(std::time::Duration::from_secs(0))
    }
}

impl Default for CryptoServiceState {
    fn default() -> Self {
        Self::new()
    }
}
