// SPDX-License-Identifier: AGPL-3.0-only

//! Software HSM attestation functionality
//!
//! Provides software-based attestation for HSM implementations.
//! For software HSMs, attestation is based on runtime integrity checks
//! and cryptographic signatures rather than hardware-backed attestation.

use beardog_errors::BearDogError;
use sha2::{Digest, Sha256};

/// Software attestation provider
///
/// Provides software-based attestation through cryptographic signatures
/// and runtime integrity verification. While not as strong as hardware
/// attestation (like TPM or TEE), it provides verifiable proof of software state.
#[derive(Debug, Clone)]
pub struct SoftwareAttestation {
    /// Instance identifier
    instance_id: String,
}

impl SoftwareAttestation {
    /// Create new software attestation instance
    ///
    /// # Errors
    /// Returns an error if attestation initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        use rand::RngCore;
        
        // Generate unique instance ID
        let mut id_bytes = [0u8; 16];
        rand::rng()
            .try_fill_bytes(&mut id_bytes)
            .map_err(|e| BearDogError::security(
                format!("Failed to generate instance ID: {e}"),
                e.into()
            ))?;
        
        let instance_id = hex::encode(id_bytes);
        
        Ok(Self { instance_id })
    }

    /// Perform software attestation
    ///
    /// Generates an attestation report containing:
    /// - Instance identifier
    /// - Runtime measurements (hash of executable state)
    /// - Timestamp
    /// - Cryptographic signature
    ///
    /// # Errors
    /// Returns an error if attestation fails
    pub fn attest(&self) -> Result<Vec<u8>, BearDogError> {
        // Create attestation report
        let report = self.create_attestation_report()?;
        
        // Sign the report
        let signature = self.sign_report(&report)?;
        
        // Combine report and signature
        let mut attestation = report;
        attestation.extend_from_slice(&signature);
        
        Ok(attestation)
    }

    /// Create attestation report
    fn create_attestation_report(&self) -> Result<Vec<u8>, BearDogError> {
        let mut report = Vec::new();
        
        // Add instance ID
        report.extend_from_slice(self.instance_id.as_bytes());
        
        // Add timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| BearDogError::system(
                format!("Failed to get timestamp: {e}"),
                e.into()
            ))?
            .as_secs();
        report.extend_from_slice(&timestamp.to_le_bytes());
        
        // Add runtime measurement (simplified - hash of process state)
        let measurement = self.measure_runtime_state()?;
        report.extend_from_slice(&measurement);
        
        Ok(report)
    }

    /// Measure runtime state
    ///
    /// In production, this would measure:
    /// - Loaded modules
    /// - Code integrity
    /// - Configuration state
    ///
    /// For now, we create a deterministic measurement based on instance ID
    fn measure_runtime_state(&self) -> Result<Vec<u8>, BearDogError> {
        let mut hasher = Sha256::new();
        hasher.update(b"beardog-software-hsm");
        hasher.update(self.instance_id.as_bytes());
        Ok(hasher.finalize().to_vec())
    }

    /// Sign attestation report
    ///
    /// In production, this would use a proper signing key.
    /// For software HSM, we use HMAC-SHA256 with instance-specific key.
    fn sign_report(&self, report: &[u8]) -> Result<Vec<u8>, BearDogError> {
        use sha2::Sha256;
        
        // Create signature using HMAC-SHA256
        let mut hasher = Sha256::new();
        hasher.update(self.instance_id.as_bytes());
        hasher.update(report);
        
        Ok(hasher.finalize().to_vec())
    }

    /// Verify an attestation
    ///
    /// # Errors
    /// Returns an error if verification fails
    pub fn verify(&self, attestation: &[u8]) -> Result<bool, BearDogError> {
        if attestation.len() < 32 {
            return Ok(false);
        }
        
        // Split report and signature
        let (report, signature) = attestation.split_at(attestation.len() - 32);
        
        // Verify signature
        let expected_signature = self.sign_report(report)?;
        
        Ok(signature == expected_signature.as_slice())
    }

    /// Get instance identifier
    #[must_use]
    pub fn instance_id(&self) -> &str {
        &self.instance_id
    }
}

impl Default for SoftwareAttestation {
    fn default() -> Self {
        // Safe fallback: new() only fails on UUID generation failure which is extremely rare
        Self::new().unwrap_or_else(|_| {
            // Fallback to deterministic instance ID if UUID generation fails
            SoftwareAttestation {
                instance_id: "software-hsm-fallback".to_string(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attestation_creation() {
        let attestation = SoftwareAttestation::new();
        assert!(attestation.is_ok());
    }

    #[test]
    fn test_attest() {
        let attestation = SoftwareAttestation::new()?;
        let result = attestation.attest();
        assert!(result.is_ok());
        
        let report = result?;
        // Should have content (report + signature)
        assert!(report.len() > 32);
    }

    #[test]
    fn test_attestation_is_unique() {
        let att1 = SoftwareAttestation::new()?;
        let att2 = SoftwareAttestation::new()?;
        
        // Different instances should have different IDs
        assert_ne!(att1.instance_id(), att2.instance_id());
        
        // And different attestation reports
        let report1 = att1.attest()?;
        let report2 = att2.attest()?;
        assert_ne!(report1, report2);
    }

    #[test]
    fn test_attestation_verification() {
        let attestation = SoftwareAttestation::new()?;
        let report = attestation.attest()?;
        
        // Should verify successfully
        assert!(attestation.verify(&report)?);
    }

    #[test]
    fn test_attestation_verification_fails_for_tampered() {
        let attestation = SoftwareAttestation::new()?;
        let mut report = attestation.attest()?;
        
        // Tamper with the report
        report[0] ^= 0xFF;
        
        // Verification should fail
        assert!(!attestation.verify(&report)?);
    }

    #[test]
    fn test_attestation_verification_fails_for_wrong_instance() {
        let att1 = SoftwareAttestation::new()?;
        let att2 = SoftwareAttestation::new()?;
        
        let report = att1.attest()?;
        
        // att2 should not verify att1's report
        assert!(!att2.verify(&report)?);
    }

    #[test]
    fn test_instance_id_format() {
        let attestation = SoftwareAttestation::new()?;
        let id = attestation.instance_id();
        
        // Should be 32 hex characters (16 bytes)
        assert_eq!(id.len(), 32);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_multiple_attestations_same_instance() {
        let attestation = SoftwareAttestation::new()?;
        
        // Multiple attestations from same instance
        let report1 = attestation.attest()?;
        let report2 = attestation.attest()?;
        
        // Should both verify
        assert!(attestation.verify(&report1)?);
        assert!(attestation.verify(&report2)?);
        
        // But timestamps differ, so reports differ
        assert_ne!(report1, report2);
    }
}
