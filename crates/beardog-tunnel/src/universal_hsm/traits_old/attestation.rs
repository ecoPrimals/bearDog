//! Attestation types and functionality for HSM operations

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Attestation data for HSM operations
#[derive(Debug, Clone)]
pub struct AttestationData {
    /// Attestation level
    pub level: AttestationLevel,
    /// Certificate chain
    pub certificate_chain: Vec<Vec<u8>>,
    /// Attestation signature
    pub attestation_signature: Vec<u8>,
    /// Nonce used in attestation
    pub nonce: Vec<u8>,
    /// When the attestation was generated
    pub generated_at: DateTime<Utc>,
    /// Challenge data
    pub challenge: Vec<u8>,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl AttestationData {
    /// Create a new attestation data instance
    pub fn new(
        level: AttestationLevel,
        certificate_chain: Vec<Vec<u8>>,
        attestation_signature: Vec<u8>,
        nonce: Vec<u8>,
        challenge: Vec<u8>,
    ) -> Self {
        Self {
            level,
            certificate_chain,
            attestation_signature,
            nonce,
            generated_at: Utc::now(),
            challenge,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to attestation
    pub fn with_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }

    /// Check if attestation is expired
    pub fn is_expired(&self, validity_duration_seconds: i64) -> bool {
        let expiry_time = self.generated_at + chrono::Duration::seconds(validity_duration_seconds);
        Utc::now() > expiry_time
    }

    /// Get root certificate from chain
    pub fn get_root_certificate(&self) -> Option<&Vec<u8>> {
        self.certificate_chain.last()
    }

    /// Get device certificate from chain
    pub fn get_device_certificate(&self) -> Option<&Vec<u8>> {
        self.certificate_chain.first()
    }
}

/// Attestation level enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AttestationLevel {
    /// No attestation
    None = 0,
    /// Software-based attestation
    Software = 1,
    /// Trusted Execution Environment
    TEE = 2,
    /// Hardware Security Module
    HSM = 3,
    /// Secure Element
    SecureElement = 4,
}

impl std::fmt::Display for AttestationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttestationLevel::None => write!(f, "No Attestation"),
            AttestationLevel::Software => write!(f, "Software Attestation"),
            AttestationLevel::TEE => write!(f, "TEE Attestation"),
            AttestationLevel::HSM => write!(f, "HSM Attestation"),
            AttestationLevel::SecureElement => write!(f, "Secure Element Attestation"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attestation_level_ordering() {
        assert!(AttestationLevel::SecureElement > AttestationLevel::HSM);
        assert!(AttestationLevel::HSM > AttestationLevel::TEE);
        assert!(AttestationLevel::TEE > AttestationLevel::Software);
        assert!(AttestationLevel::Software > AttestationLevel::None);
    }

    #[test]
    fn test_attestation_data_creation() {
        let attestation = AttestationData::new(
            AttestationLevel::HSM,
            vec![vec![1, 2, 3, 4]],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        );
        
        assert_eq!(attestation.level, AttestationLevel::HSM);
        assert!(!attestation.is_expired(3600)); // 1 hour validity
        assert_eq!(attestation.get_device_certificate(), Some(&vec![1, 2, 3, 4]));
    }
}
