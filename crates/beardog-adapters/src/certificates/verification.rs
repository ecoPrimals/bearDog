// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Certificate verification

use super::types::*;
use chrono::Utc;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use std::fmt;

/// Verifies adapter unlock certificates
pub struct CertificateVerifier {
    /// BearDog public key for verification
    verifying_key: VerifyingKey,
}

impl CertificateVerifier {
    /// Create new verifier with public key
    pub fn new(verifying_key: VerifyingKey) -> Self {
        Self { verifying_key }
    }

    /// Verify a certificate is valid
    pub fn verify(&self, cert: &AdapterUnlockCertificate) -> Result<(), VerificationError> {
        // 1. Check expiry
        if cert.is_expired() {
            return Err(VerificationError::Expired {
                expired_at: cert.expires_at,
            });
        }

        // 2. Verify cryptographic signature
        self.verify_signature(cert)?;

        // 3. Check license if required
        self.verify_license(cert)?;

        Ok(())
    }

    /// Verify certificate signature
    fn verify_signature(&self, cert: &AdapterUnlockCertificate) -> Result<(), VerificationError> {
        // Compute expected hash
        let hash = Self::compute_cert_hash(cert)?;

        // Parse signature
        let sig_bytes: [u8; 64] = cert
            .signature
            .get(..64)
            .ok_or_else(|| VerificationError::InvalidSignature {
                reason: "Signature must be 64 bytes".to_string(),
            })?
            .try_into()
            .map_err(|_| VerificationError::InvalidSignature {
                reason: "Failed to parse signature bytes".to_string(),
            })?;

        let signature = Signature::from_bytes(&sig_bytes);

        // Verify
        self.verifying_key.verify(&hash, &signature).map_err(|e| {
            VerificationError::InvalidSignature {
                reason: format!("Signature verification failed: {}", e),
            }
        })?;

        Ok(())
    }

    /// Verify license if present
    fn verify_license(&self, cert: &AdapterUnlockCertificate) -> Result<(), VerificationError> {
        // Check if commercial classification requires license
        if let CommercialClassification::Commercial { risk, .. } = &cert.classification {
            if matches!(risk, RiskLevel::High | RiskLevel::Critical) {
                // High risk must have valid license
                let license =
                    cert.license
                        .as_ref()
                        .ok_or_else(|| VerificationError::LicenseRequired {
                            classification: format!("{:?}", cert.classification),
                        })?;

                // Check license expiry
                if license.expires_at < Utc::now() {
                    return Err(VerificationError::LicenseExpired {
                        expired_at: license.expires_at,
                    });
                }
            }
        }

        Ok(())
    }

    /// Compute certificate hash (same as issuance)
    fn compute_cert_hash(cert: &AdapterUnlockCertificate) -> Result<Vec<u8>, VerificationError> {
        use blake3::Hasher;

        let mut hasher = Hasher::new();
        hasher.update(cert.id.as_bytes());
        hasher.update(cert.adapter_id.as_bytes());
        hasher.update(&bincode::serialize(&cert.classification).map_err(|e| {
            VerificationError::HashingFailed {
                reason: format!("Failed to serialize classification: {}", e),
            }
        })?);
        hasher.update(&cert.expires_at.timestamp().to_le_bytes());
        hasher.update(&bincode::serialize(&cert.scope).map_err(|e| {
            VerificationError::HashingFailed {
                reason: format!("Failed to serialize scope: {}", e),
            }
        })?);

        Ok(hasher.finalize().as_bytes().to_vec())
    }

    /// Verify certificate allows a specific operation
    pub fn verify_operation(
        &self,
        cert: &AdapterUnlockCertificate,
        operation: &AdapterOperation,
    ) -> Result<(), VerificationError> {
        // First verify certificate is valid
        self.verify(cert)?;

        // Check if operation is allowed
        if !cert.allows_operation(operation) {
            return Err(VerificationError::OperationNotAllowed {
                operation: format!("{:?}", operation),
                allowed: cert
                    .scope
                    .operations
                    .iter()
                    .map(|op| format!("{:?}", op))
                    .collect(),
            });
        }

        Ok(())
    }
}

/// Errors during certificate verification
#[derive(Debug, Clone)]
pub enum VerificationError {
    /// Certificate has expired
    Expired {
        expired_at: chrono::DateTime<chrono::Utc>,
    },

    /// Invalid signature
    InvalidSignature { reason: String },

    /// License required but not present
    LicenseRequired { classification: String },

    /// License has expired
    LicenseExpired {
        expired_at: chrono::DateTime<chrono::Utc>,
    },

    /// Operation not allowed by certificate
    OperationNotAllowed {
        operation: String,
        allowed: Vec<String>,
    },

    /// Failed to compute hash
    HashingFailed { reason: String },
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expired { expired_at } => {
                write!(f, "🔒 Certificate expired at {}", expired_at)
            }
            Self::InvalidSignature { reason } => {
                write!(f, "🔒 Invalid certificate signature: {}", reason)
            }
            Self::LicenseRequired { classification } => {
                write!(
                    f,
                    "🔒 License required for {} classification. Visit https://beardog.dev/pricing",
                    classification
                )
            }
            Self::LicenseExpired { expired_at } => {
                write!(f, "🔒 License expired at {}", expired_at)
            }
            Self::OperationNotAllowed { operation, allowed } => {
                write!(
                    f,
                    "🔒 Operation '{}' not allowed. Allowed: {:?}",
                    operation, allowed
                )
            }
            Self::HashingFailed { reason } => {
                write!(f, "🔒 Certificate hashing failed: {}", reason)
            }
        }
    }
}

impl std::error::Error for VerificationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::certificates::issuance::CertificateIssuer;
    use chrono::Duration;
    use ed25519_dalek::SigningKey;

    fn create_test_pair() -> (CertificateIssuer, CertificateVerifier) {
        let secret_bytes: [u8; 32] = rand::random();
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let verifying_key = signing_key.verifying_key();

        let issuer = CertificateIssuer::new(signing_key);
        let verifier = CertificateVerifier::new(verifying_key);

        (issuer, verifier)
    }

    #[test]
    fn test_verify_valid_certificate() {
        let (issuer, verifier) = create_test_pair();

        let classification = CommercialClassification::Human {
            confidence: 95,
            reasons: vec!["Interactive session".to_string()],
        };

        let cert = issuer.issue(classification, "prometheus").unwrap();

        // Should verify successfully
        assert!(verifier.verify(&cert).is_ok());
    }

    #[test]
    fn test_verify_expired_certificate() {
        let (issuer, verifier) = create_test_pair();

        let classification = CommercialClassification::Human {
            confidence: 95,
            reasons: vec![],
        };

        let mut cert = issuer.issue(classification, "prometheus").unwrap();

        // Make it expired
        cert.expires_at = Utc::now() - Duration::hours(1);

        // Should fail verification
        let result = verifier.verify(&cert);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            VerificationError::Expired { .. }
        ));
    }

    #[test]
    fn test_verify_tampered_certificate() {
        let (issuer, verifier) = create_test_pair();

        let classification = CommercialClassification::Human {
            confidence: 95,
            reasons: vec![],
        };

        let mut cert = issuer.issue(classification, "prometheus").unwrap();

        // Tamper with adapter_id
        cert.adapter_id = "grafana".to_string();

        // Should fail verification (signature won't match)
        let result = verifier.verify(&cert);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            VerificationError::InvalidSignature { .. }
        ));
    }

    #[test]
    fn test_operation_verification() {
        let (issuer, verifier) = create_test_pair();

        let classification = CommercialClassification::Human {
            confidence: 95,
            reasons: vec![],
        };

        let cert = issuer.issue(classification, "prometheus").unwrap();

        // Read should be allowed
        assert!(verifier
            .verify_operation(&cert, &AdapterOperation::Read)
            .is_ok());

        // Write should be allowed for high confidence human
        assert!(verifier
            .verify_operation(&cert, &AdapterOperation::Write)
            .is_ok());

        // Admin may not be allowed
        let result = verifier.verify_operation(&cert, &AdapterOperation::Admin);
        // Depending on scope, this might fail
        if result.is_err() {
            assert!(matches!(
                result.unwrap_err(),
                VerificationError::OperationNotAllowed { .. }
            ));
        }
    }

    #[test]
    fn test_commercial_requires_license() {
        let (issuer, verifier) = create_test_pair();

        let classification = CommercialClassification::Commercial {
            risk: RiskLevel::High,
            indicators: vec![],
            confidence: 90,
        };

        // Issuing without license should fail
        let result = issuer.issue(classification.clone(), "prometheus");
        assert!(result.is_err());

        // With license should work
        let license = LicenseInfo {
            key: "test-key".to_string(),
            license_type: LicenseType::Professional,
            expires_at: Utc::now() + Duration::days(365),
            entity: "Test Corp".to_string(),
            rate_limit: Some(100_000),
        };

        let cert = issuer
            .issue_with_license(classification, "prometheus", license)
            .unwrap();

        assert!(verifier.verify(&cert).is_ok());
    }
}
