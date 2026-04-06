// SPDX-License-Identifier: AGPL-3.0-or-later

//! Configuration types for the crypto service
//!
//! Defines configuration and runtime state for cryptographic operations.

use std::sync::atomic::AtomicU64;
use std::time::SystemTime;

/// Configuration for crypto service
///
/// Controls which features are enabled and operational limits.
#[derive(Debug, Clone)]
pub struct CryptoServiceConfig {
    /// Service name (e.g., "beardog-northgate")
    pub service_name: String,
    /// Enable HSM operations
    pub hsm_enabled: bool,
    /// Enable genetic key mixing
    pub genetic_enabled: bool,
    /// Maximum data size for single operation (bytes)
    pub max_data_size: usize,
    /// Enable audit logging
    pub audit_enabled: bool,
}

impl Default for CryptoServiceConfig {
    fn default() -> Self {
        Self {
            service_name: "beardog".to_string(),
            hsm_enabled: false,
            genetic_enabled: true,
            max_data_size: 10 * 1024 * 1024, // 10MB
            audit_enabled: true,
        }
    }
}

/// Runtime state for crypto service
///
/// Tracks operational metrics and service lifecycle.
pub(super) struct CryptoServiceState {
    /// Operation counter (for audit trail)
    pub(super) operation_count: AtomicU64,
    /// Service start time
    pub(super) start_time: SystemTime,
}

impl CryptoServiceState {
    /// Create new service state
    pub(super) fn new() -> Self {
        Self {
            operation_count: AtomicU64::new(0),
            start_time: SystemTime::now(),
        }
    }
}

