// SPDX-License-Identifier: AGPL-3.0-only

//! Adapter Unlock Certificates (Phase 2)
//!
//! Cryptographically enforced adapter usage based on genetic keys.
//!
//! ## Architecture
//!
//! ```text
//! BearDog Key (with constraints)
//!       ↓
//!   Issues Certificate
//!       ↓
//! Adapter Unlock Certificate (signed, time-limited)
//!       ↓
//!   Adapter validates certificate
//!       ↓
//!   Adapter allows/denies operation
//! ```
//!
//! ## Classification System
//!
//! - **Human**: Free usage, issued from Human-sourced entropy keys
//! - **Commercial**: Paid usage, requires billing/pricing enforcement
//!
//! ## Example
//!
//! ```rust,ignore
//! // Issue certificate from key
//! let certificate = key.issue_adapter_certificate(
//!     "beardog-adapters::{adapter}::network",
//!     AdapterClassification::Human,
//!     Duration::hours(24),
//! )?;
//!
//! // Adapter validates certificate
//! if !adapter.verify_certificate(&certificate)? {
//!     return Err("Certificate invalid or expired");
//! }
//!
//! // Proceed with operation
//! adapter.send_request(payload)?;
//! ```

use crate::genetics_constraints::KeyConstraints;
use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// ADAPTER CLASSIFICATION
// ============================================================================

/// Classification of adapter usage (determines pricing/billing)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdapterClassification {
    /// Human usage - Free (entropy sourced from human biometrics/input)
    ///
    /// Keys with human-sourced entropy can issue Human certificates.
    /// These are exempt from billing and usage fees.
    Human,

    /// Commercial usage - Paid (standard entropy or machine-only entropy)
    ///
    /// Keys without human entropy or explicitly marked commercial
    /// require payment/billing enforcement.
    Commercial,
}

impl AdapterClassification {
    /// Check if this classification requires payment
    #[must_use]
    pub const fn requires_payment(&self) -> bool {
        matches!(self, Self::Commercial)
    }

    /// Get human-readable description
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Human => "Human usage (free)",
            Self::Commercial => "Commercial usage (paid)",
        }
    }
}

// ============================================================================
// ADAPTER UNLOCK CERTIFICATE
// ============================================================================

/// Adapter Unlock Certificate
///
/// A cryptographically signed certificate that grants permission to use
/// a specific adapter. The certificate:
///
/// - Is issued by a BearDog genetic key
/// - Inherits constraints from the issuing key
/// - Has a time-limited validity period
/// - Is bound to a specific adapter ID
/// - Cannot be forged or tampered with (Ed25519 signature)
///
/// ## Security Properties
///
/// 1. **Tamper-Evident**: Ed25519 signature detects any modification
/// 2. **Time-Limited**: Automatic expiration prevents indefinite use
/// 3. **Constraint-Bound**: Inherits all constraints from issuing key
/// 4. **Adapter-Specific**: Can only unlock the specified adapter
/// 5. **Classification-Enforced**: Billing/pricing based on classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterUnlockCertificate {
    /// Unique certificate ID
    pub cert_id: String,

    /// Genetic key that issued this certificate
    pub issuer_key_id: String,

    /// Target adapter id (orchestrator-specific; from discovery / manifest, not hardcoded here)
    ///
    /// Format: `crate::module::submodule`
    pub adapter_id: String,

    /// Classification (Human = free, Commercial = paid)
    pub classification: AdapterClassification,

    /// Constraints inherited from issuing key
    ///
    /// The adapter must enforce these constraints on all operations
    /// performed using this certificate.
    pub constraints: Option<KeyConstraints>,

    /// Certificate issue time
    pub issued_at: DateTime<Utc>,

    /// Certificate expiration time
    pub expires_at: DateTime<Utc>,

    /// Cryptographic signature (Ed25519)
    ///
    /// Signs: cert_id + issuer_key_id + adapter_id + classification + constraints + timestamps
    pub signature: Vec<u8>,

    /// Public key of issuer (for verification)
    pub issuer_public_key: Vec<u8>,
}

impl AdapterUnlockCertificate {
    /// Check if this certificate has expired
    #[must_use]
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Check if this certificate is not yet valid
    #[must_use]
    pub fn not_yet_valid(&self) -> bool {
        Utc::now() < self.issued_at
    }

    /// Check if this certificate is currently valid (not expired, not future-dated)
    #[must_use]
    pub fn is_valid_now(&self) -> bool {
        !self.is_expired() && !self.not_yet_valid()
    }

    /// Get time remaining until expiration
    #[must_use]
    pub fn time_remaining(&self) -> chrono::Duration {
        self.expires_at - Utc::now()
    }

    /// Get human-readable status
    #[must_use]
    pub fn status(&self) -> &'static str {
        if self.is_expired() {
            "Expired"
        } else if self.not_yet_valid() {
            "Not yet valid"
        } else {
            "Valid"
        }
    }

    /// Get certificate metadata for display
    #[must_use]
    pub fn metadata_string(&self) -> String {
        format!(
            "Certificate {} | Adapter: {} | Classification: {} | Status: {} | Expires: {}",
            &self.cert_id[..8],
            self.adapter_id,
            self.classification.description(),
            self.status(),
            self.expires_at.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }

    /// Serialize certificate data for signing/verification
    ///
    /// This is used to create the signature and verify it later.
    /// The format is deterministic to ensure consistent signatures.
    pub fn signable_data(&self) -> Vec<u8> {
        use sha3::{Digest, Sha3_256};

        let mut hasher = Sha3_256::new();

        // Add all certificate fields in deterministic order
        hasher.update(self.cert_id.as_bytes());
        hasher.update(self.issuer_key_id.as_bytes());
        hasher.update(self.adapter_id.as_bytes());

        // Classification (serialize to bytes)
        let classification_byte = match self.classification {
            AdapterClassification::Human => 0u8,
            AdapterClassification::Commercial => 1u8,
        };
        hasher.update([classification_byte]);

        // Constraints (if present)
        if let Some(ref constraints) = self.constraints
            && let Ok(constraint_hash) = constraints.hash()
        {
            hasher.update(constraint_hash);
        }

        // Timestamps
        hasher.update(self.issued_at.timestamp().to_le_bytes());
        hasher.update(self.expires_at.timestamp().to_le_bytes());

        hasher.finalize().to_vec()
    }

    /// Verify certificate signature and validity
    ///
    /// This checks:
    /// 1. Certificate is not expired
    /// 2. Certificate is not future-dated
    /// 3. Signature is valid (detects tampering)
    /// 4. Optionally: certificate is for correct adapter
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let result = certificate.verify(Some("beardog-adapters::peer-network::network"))?;
    /// match result {
    ///     CertificateVerificationResult::Valid => {
    ///         // Proceed with operation
    ///     }
    ///     CertificateVerificationResult::Expired => {
    ///         return Err("Certificate has expired");
    ///     }
    ///     _ => {
    ///         return Err("Certificate verification failed");
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns error if signature verification fails at the cryptographic level.
    /// Returns `Ok(result)` with verification result enum for all other cases.
    pub fn verify(
        &self,
        expected_adapter_id: Option<&str>,
    ) -> Result<CertificateVerificationResult, BearDogError> {
        use ed25519_dalek::{Signature, Verifier, VerifyingKey};

        // 1. Check expiration
        if self.is_expired() {
            return Ok(CertificateVerificationResult::Expired);
        }

        // 2. Check if not yet valid
        if self.not_yet_valid() {
            return Ok(CertificateVerificationResult::NotYetValid);
        }

        // 3. Check adapter ID if specified
        if let Some(expected) = expected_adapter_id
            && expected != self.adapter_id
        {
            return Ok(CertificateVerificationResult::WrongAdapter {
                expected: expected.to_string(),
                found: self.adapter_id.clone(),
            });
        }

        // 4. Verify cryptographic signature
        let public_key_array: [u8; 32] =
            self.issuer_public_key.as_slice().try_into().map_err(|_| {
                BearDogError::crypto_error("Invalid issuer public key length (expected 32 bytes)")
            })?;

        let verifying_key = VerifyingKey::from_bytes(&public_key_array).map_err(|e| {
            BearDogError::crypto_error(format!("Invalid Ed25519 verifying key: {e}"))
        })?;

        let signature_array: [u8; 64] = self.signature.as_slice().try_into().map_err(|_| {
            BearDogError::crypto_error("Invalid signature length (expected 64 bytes)")
        })?;

        let signature = Signature::from_bytes(&signature_array);

        let signable_data = self.signable_data();

        if verifying_key.verify(&signable_data, &signature).is_err() {
            return Ok(CertificateVerificationResult::InvalidSignature);
        }

        // 5. Verify constraints haven't been tampered with
        if let Some(ref constraints) = self.constraints {
            // The signature already covers the constraints hash,
            // so if signature is valid, constraints are valid
            // Additional check: ensure constraints can be hashed
            if constraints.hash().is_err() {
                return Ok(CertificateVerificationResult::ConstraintsTampered);
            }
        }

        Ok(CertificateVerificationResult::Valid)
    }
}

// ============================================================================
// CERTIFICATE VERIFICATION RESULT
// ============================================================================

/// Result of certificate verification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificateVerificationResult {
    /// Certificate is valid and can be used
    Valid,

    /// Certificate has expired
    Expired,

    /// Certificate is not yet valid (future-dated)
    NotYetValid,

    /// Certificate signature is invalid (tampered or forged)
    InvalidSignature,

    /// Certificate is for a different adapter than the one performing verification.
    WrongAdapter {
        /// Adapter identity expected in this verification context (e.g. local adapter id).
        expected: String,
        /// Adapter identity present in the certificate payload or subject.
        found: String,
    },

    /// Certificate constraints have been tampered with
    ConstraintsTampered,
}

impl CertificateVerificationResult {
    /// Check if verification was successful
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        matches!(self, Self::Valid)
    }

    /// Get human-readable error message
    #[must_use]
    pub fn error_message(&self) -> Option<String> {
        match self {
            Self::Valid => None,
            Self::Expired => Some("Certificate has expired".to_string()),
            Self::NotYetValid => Some("Certificate is not yet valid".to_string()),
            Self::InvalidSignature => Some("Certificate signature is invalid".to_string()),
            Self::WrongAdapter { expected, found } => Some(format!(
                "Certificate is for adapter '{found}', but this is '{expected}'"
            )),
            Self::ConstraintsTampered => {
                Some("Certificate constraints have been tampered with".to_string())
            }
        }
    }
}

// ============================================================================
// CERTIFICATE USAGE RECORD (for billing/auditing)
// ============================================================================

/// Record of certificate usage (for billing and auditing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateUsageRecord {
    /// Certificate ID
    pub cert_id: String,

    /// Adapter ID
    pub adapter_id: String,

    /// Classification at time of use
    pub classification: AdapterClassification,

    /// Operation timestamp
    pub timestamp: DateTime<Utc>,

    /// Operation type (e.g., "send_request", "discover_primals")
    pub operation_type: String,

    /// Resource consumed (for billing)
    pub resource_usage: ResourceUsage,
}

/// Resource usage tracking (for billing)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceUsage {
    /// CPU seconds used
    pub cpu_seconds: f64,

    /// Memory bytes used
    pub memory_bytes: u64,

    /// Network bytes transferred
    pub network_bytes: u64,

    /// Storage bytes accessed
    pub storage_bytes: u64,

    /// Custom billing units (adapter-specific)
    pub custom_units: f64,
}

impl ResourceUsage {
    /// Calculate billing amount (in arbitrary currency units)
    ///
    /// Pricing model (example):
    /// - CPU: $0.10 per second
    /// - Memory: $0.01 per GB
    /// - Network: $0.05 per GB
    /// - Storage: $0.02 per GB
    #[must_use]
    pub fn calculate_billing_amount(&self) -> f64 {
        const CPU_PRICE: f64 = 0.10;
        const MEMORY_PRICE: f64 = 0.01 / (1024.0 * 1024.0 * 1024.0);
        const NETWORK_PRICE: f64 = 0.05 / (1024.0 * 1024.0 * 1024.0);
        const STORAGE_PRICE: f64 = 0.02 / (1024.0 * 1024.0 * 1024.0);

        let cpu_cost = self.cpu_seconds * CPU_PRICE;
        // Precision loss acceptable for cost calculations (sub-cent precision not needed)
        #[expect(
            clippy::cast_precision_loss,
            reason = "display/metric conversion, precision loss acceptable"
        )]
        let memory_cost = (self.memory_bytes as f64) * MEMORY_PRICE;
        #[expect(
            clippy::cast_precision_loss,
            reason = "display/metric conversion, precision loss acceptable"
        )]
        let network_cost = (self.network_bytes as f64) * NETWORK_PRICE;
        #[expect(
            clippy::cast_precision_loss,
            reason = "display/metric conversion, precision loss acceptable"
        )]
        let storage_cost = (self.storage_bytes as f64) * STORAGE_PRICE;

        cpu_cost + memory_cost + network_cost + storage_cost + self.custom_units
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classification_requires_payment() {
        assert!(!AdapterClassification::Human.requires_payment());
        assert!(AdapterClassification::Commercial.requires_payment());
    }

    #[test]
    fn test_certificate_expiration() {
        use chrono::Duration;

        let cert = AdapterUnlockCertificate {
            cert_id: "test-cert".to_string(),
            issuer_key_id: "test-key".to_string(),
            adapter_id: "test-adapter".to_string(),
            classification: AdapterClassification::Human,
            constraints: None,
            issued_at: Utc::now() - Duration::hours(1),
            expires_at: Utc::now() - Duration::seconds(1), // Expired
            signature: vec![],
            issuer_public_key: vec![],
        };

        assert!(cert.is_expired());
        assert!(!cert.is_valid_now());
        assert_eq!(cert.status(), "Expired");
    }

    #[test]
    fn test_certificate_not_yet_valid() {
        use chrono::Duration;

        let cert = AdapterUnlockCertificate {
            cert_id: "test-cert".to_string(),
            issuer_key_id: "test-key".to_string(),
            adapter_id: "test-adapter".to_string(),
            classification: AdapterClassification::Human,
            constraints: None,
            issued_at: Utc::now() + Duration::hours(1), // Future
            expires_at: Utc::now() + Duration::hours(2),
            signature: vec![],
            issuer_public_key: vec![],
        };

        assert!(cert.not_yet_valid());
        assert!(!cert.is_valid_now());
        assert_eq!(cert.status(), "Not yet valid");
    }

    #[test]
    fn test_certificate_valid_now() {
        use chrono::Duration;

        let cert = AdapterUnlockCertificate {
            cert_id: "test-cert".to_string(),
            issuer_key_id: "test-key".to_string(),
            adapter_id: "test-adapter".to_string(),
            classification: AdapterClassification::Human,
            constraints: None,
            issued_at: Utc::now() - Duration::seconds(1),
            expires_at: Utc::now() + Duration::hours(1),
            signature: vec![],
            issuer_public_key: vec![],
        };

        assert!(!cert.is_expired());
        assert!(!cert.not_yet_valid());
        assert!(cert.is_valid_now());
        assert_eq!(cert.status(), "Valid");
    }

    #[test]
    fn test_verification_result_messages() {
        assert!(CertificateVerificationResult::Valid.is_valid());
        assert!(
            CertificateVerificationResult::Valid
                .error_message()
                .is_none()
        );

        let expired = CertificateVerificationResult::Expired;
        assert!(!expired.is_valid());
        assert!(
            expired
                .error_message()
                .expect("Expired variant has error message")
                .contains("expired")
        );

        let wrong_adapter = CertificateVerificationResult::WrongAdapter {
            expected: "adapter-a".to_string(),
            found: "adapter-b".to_string(),
        };
        assert!(!wrong_adapter.is_valid());
        assert!(
            wrong_adapter
                .error_message()
                .expect("WrongAdapter variant has error message")
                .contains("adapter-a")
        );
    }

    #[test]
    fn test_resource_usage_billing() {
        let usage = ResourceUsage {
            cpu_seconds: 10.0,                 // $1.00
            memory_bytes: 1024 * 1024 * 1024,  // 1 GB = $0.01
            network_bytes: 1024 * 1024 * 1024, // 1 GB = $0.05
            storage_bytes: 1024 * 1024 * 1024, // 1 GB = $0.02
            custom_units: 0.5,                 // $0.50
        };

        let billing_amount = usage.calculate_billing_amount();
        // Should be approximately $1.58 ($1.00 + $0.01 + $0.05 + $0.02 + $0.50)
        assert!((billing_amount - 1.58).abs() < 0.01);
    }
}
