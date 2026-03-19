// SPDX-License-Identifier: AGPL-3.0-only

//! BTSP Metrics
//!
//! Lock-free atomic metrics collection for BTSP operations.
//! All counters use atomic operations for thread-safe, high-performance tracking.

use serde::{Deserialize, Serialize};

/// Metrics for BTSP operations
///
/// All metrics are collected atomically for lock-free performance.
/// These metrics provide observability into BTSP tunnel operations,
/// encryption/decryption activities, and trust evaluations.
///
/// # Thread Safety
///
/// All fields are serialized snapshots. The actual implementation uses
/// `AtomicU64` for lock-free concurrent updates.
///
/// # Usage
///
/// ```rust,ignore
/// let metrics = btsp_provider.get_metrics();
/// println!("Tunnels established: {}", metrics.tunnels_established);
/// println!("Active tunnels: {}", metrics.tunnels_active);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BtspMetrics {
    /// Total tunnels established since start
    pub tunnels_established: u64,

    /// Currently active tunnels
    pub tunnels_active: u64,

    /// Total encryption operations
    pub encryption_operations: u64,

    /// Total decryption operations
    pub decryption_operations: u64,

    /// Total trust evaluations
    pub trust_evaluations: u64,
}

impl BtspMetrics {
    /// Create new metrics with all counters at zero
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any operations have been performed
    pub fn has_activity(&self) -> bool {
        self.tunnels_established > 0
            || self.encryption_operations > 0
            || self.decryption_operations > 0
            || self.trust_evaluations > 0
    }

    /// Get total cryptographic operations (encryption + decryption)
    pub fn total_crypto_ops(&self) -> u64 {
        self.encryption_operations
            .saturating_add(self.decryption_operations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_default() {
        let metrics = BtspMetrics::new();
        assert_eq!(metrics.tunnels_established, 0);
        assert_eq!(metrics.tunnels_active, 0);
        assert!(!metrics.has_activity());
    }

    #[test]
    fn test_has_activity() {
        let mut metrics = BtspMetrics::new();
        assert!(!metrics.has_activity());

        metrics.encryption_operations = 1;
        assert!(metrics.has_activity());
    }

    #[test]
    fn test_total_crypto_ops() {
        let mut metrics = BtspMetrics::new();
        metrics.encryption_operations = 10;
        metrics.decryption_operations = 5;
        assert_eq!(metrics.total_crypto_ops(), 15);
    }
}
