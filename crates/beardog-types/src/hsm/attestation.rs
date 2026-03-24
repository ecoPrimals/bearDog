// SPDX-License-Identifier: AGPL-3.0-only

//! HSM Attestation Types
//!
//! Provides types for hardware attestation and verification.

use serde::{Deserialize, Serialize};

/// Attestation data for hardware verification
///
/// Contains challenge-response data and certificate chains for
/// verifying hardware authenticity and security properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationData {
    /// Challenge sent to the hardware
    pub challenge: Vec<u8>,

    /// Response from the hardware
    pub response: Vec<u8>,

    /// Certificate chain proving hardware authenticity
    pub certificate_chain: Vec<Vec<u8>>,
}

impl AttestationData {
    /// Creates new attestation data
    #[must_use]
    pub const fn new(
        challenge: Vec<u8>,
        response: Vec<u8>,
        certificate_chain: Vec<Vec<u8>>,
    ) -> Self {
        Self {
            challenge,
            response,
            certificate_chain,
        }
    }

    /// Creates empty attestation data
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            challenge: Vec::new(),
            response: Vec::new(),
            certificate_chain: Vec::new(),
        }
    }

    /// Checks if attestation data is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.challenge.is_empty() && self.response.is_empty() && self.certificate_chain.is_empty()
    }

    /// Gets the number of certificates in the chain
    #[must_use]
    pub fn certificate_count(&self) -> usize {
        self.certificate_chain.len()
    }

    /// Adds a certificate to the chain
    pub fn add_certificate(&mut self, certificate: Vec<u8>) {
        self.certificate_chain.push(certificate);
    }
}

impl Default for AttestationData {
    fn default() -> Self {
        Self::empty()
    }
}

/// Audit statistics for HSM operations
///
/// Tracks operational metrics for compliance and monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStatistics {
    /// Total number of operations performed
    pub total_operations: u64,

    /// Number of failed operations
    pub failed_operations: u64,

    /// Timestamp of last audit
    pub last_audit: std::time::SystemTime,
}

impl AuditStatistics {
    /// Creates new audit statistics
    #[must_use]
    pub fn new() -> Self {
        Self {
            total_operations: 0,
            failed_operations: 0,
            last_audit: std::time::SystemTime::now(),
        }
    }

    /// Records a successful operation
    pub fn record_success(&mut self) {
        self.total_operations += 1;
        self.last_audit = std::time::SystemTime::now();
    }

    /// Records a failed operation
    pub fn record_failure(&mut self) {
        self.total_operations += 1;
        self.failed_operations += 1;
        self.last_audit = std::time::SystemTime::now();
    }

    /// Gets the success rate (0.0 to 1.0)
    #[must_use]
    pub fn success_rate(&self) -> f64 {
        if self.total_operations == 0 {
            return 1.0;
        }
        let successful = self.total_operations - self.failed_operations;
        // Note: Precision loss is acceptable for rate calculations (52-bit mantissa is sufficient)
        #[expect(
            clippy::cast_precision_loss,
            reason = "display/metric conversion, precision loss acceptable"
        )]
        let rate = successful as f64 / self.total_operations as f64;
        rate
    }

    /// Gets the failure rate (0.0 to 1.0)
    #[must_use]
    pub fn failure_rate(&self) -> f64 {
        1.0 - self.success_rate()
    }

    /// Resets all statistics
    pub fn reset(&mut self) {
        self.total_operations = 0;
        self.failed_operations = 0;
        self.last_audit = std::time::SystemTime::now();
    }
}

impl Default for AuditStatistics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attestation_data_creation() {
        let data = AttestationData::new(vec![1, 2, 3], vec![4, 5, 6], vec![vec![7, 8, 9]]);
        assert_eq!(data.challenge.len(), 3);
        assert_eq!(data.response.len(), 3);
        assert_eq!(data.certificate_count(), 1);
    }

    #[test]
    fn test_attestation_data_empty() {
        let data = AttestationData::empty();
        assert!(data.is_empty());
        assert_eq!(data.certificate_count(), 0);
    }

    #[test]
    fn test_attestation_data_add_certificate() {
        let mut data = AttestationData::empty();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        data.add_certificate(vec![1, 2, 3]);
        data.add_certificate(vec![4, 5, 6]);
        assert_eq!(data.certificate_count(), 2);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_audit_statistics_creation() {
        let stats = AuditStatistics::new();
        assert_eq!(stats.total_operations, 0);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(stats.failed_operations, 0);
        assert_eq!(stats.success_rate(), 1.0);
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_audit_statistics_record_success() {
        let mut stats = AuditStatistics::new();
        stats.record_success();
        stats.record_success();
        stats.record_success();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal

        assert_eq!(stats.total_operations, 3);
        assert_eq!(stats.failed_operations, 0);
        assert_eq!(stats.success_rate(), 1.0);
    }

    #[test]
    fn test_audit_statistics_record_failure() {
        let mut stats = AuditStatistics::new();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: important
        stats.record_success();
        stats.record_failure();
        stats.record_success();
        stats.record_failure();

        assert_eq!(stats.total_operations, 4);
        assert_eq!(stats.failed_operations, 2);
        assert_eq!(stats.success_rate(), 0.5);
        assert_eq!(stats.failure_rate(), 0.5);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_audit_statistics_reset() {
        let mut stats = AuditStatistics::new();
        stats.record_success();
        stats.record_failure();
        stats.reset();

        assert_eq!(stats.total_operations, 0);
        assert_eq!(stats.failed_operations, 0);
    }
}
