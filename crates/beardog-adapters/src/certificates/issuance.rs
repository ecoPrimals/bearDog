// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Certificate issuance by BearDog daemon

use super::types::*;
use beardog_errors::BearDogError;
use chrono::{Duration, Utc};
use ed25519_dalek::{Signer, SigningKey};
use uuid::Uuid;

/// Issues adapter unlock certificates
pub struct CertificateIssuer {
    /// BearDog signing key
    signing_key: SigningKey,
}

impl CertificateIssuer {
    /// Create new issuer with signing key
    pub fn new(signing_key: SigningKey) -> Self {
        Self { signing_key }
    }

    /// Issue a certificate based on classification
    pub fn issue(
        &self,
        classification: CommercialClassification,
        adapter_id: &str,
    ) -> Result<AdapterUnlockCertificate, BearDogError> {
        // Determine expiry based on classification
        let expiry = self.determine_expiry(&classification);

        // Determine scope based on classification
        let scope = self.determine_scope(&classification);

        // Check if license is required
        let license = match &classification {
            CommercialClassification::Commercial { risk, .. } => {
                if matches!(risk, RiskLevel::High | RiskLevel::Critical) {
                    // High risk commercial must have license
                    return Err(BearDogError::security(
                        "Commercial usage requires license. Visit https://beardog.dev/pricing"
                            .to_string(),
                    ));
                }
                None
            }
            _ => None,
        };

        let now = Utc::now();
        let expires_at = now + Duration::minutes(expiry.minutes() as i64);

        let mut cert = AdapterUnlockCertificate {
            id: Uuid::new_v4().to_string(),
            adapter_id: adapter_id.to_string(),
            classification,
            expires_at,
            scope,
            signature: vec![], // Will be filled below
            signed_by: self.get_key_id(),
            issued_at: now,
            license,
            constraints: None,
        };

        // Sign certificate
        cert.signature = self.sign_certificate(&cert)?;

        Ok(cert)
    }

    /// Issue with explicit license
    pub fn issue_with_license(
        &self,
        classification: CommercialClassification,
        adapter_id: &str,
        license: LicenseInfo,
    ) -> Result<AdapterUnlockCertificate, BearDogError> {
        // Verify license is valid
        if license.expires_at < Utc::now() {
            return Err(BearDogError::security("License has expired".to_string()));
        }

        let expiry = self.determine_expiry(&classification);
        let scope = self.determine_scope_with_license(&classification, &license);

        let now = Utc::now();
        let expires_at = now + Duration::minutes(expiry.minutes() as i64);

        let mut cert = AdapterUnlockCertificate {
            id: Uuid::new_v4().to_string(),
            adapter_id: adapter_id.to_string(),
            classification,
            expires_at,
            scope,
            signature: vec![],
            signed_by: self.get_key_id(),
            issued_at: now,
            license: Some(license),
            constraints: None,
        };

        cert.signature = self.sign_certificate(&cert)?;

        Ok(cert)
    }

    /// Determine appropriate expiry time
    fn determine_expiry(&self, classification: &CommercialClassification) -> CertificateExpiry {
        match classification {
            CommercialClassification::Human { .. } => CertificateExpiry::Human,
            CommercialClassification::SmallTeam { .. } => CertificateExpiry::SmallTeam,
            CommercialClassification::Commercial { .. } => CertificateExpiry::Commercial,
            CommercialClassification::Uncertain { .. } => CertificateExpiry::Commercial, // Conservative
        }
    }

    /// Determine scope based on classification
    fn determine_scope(&self, classification: &CommercialClassification) -> CertificateScope {
        match classification {
            CommercialClassification::Human { confidence, .. } => {
                if *confidence >= 80 {
                    // High confidence human - generous scope
                    CertificateScope {
                        operations: vec![
                            AdapterOperation::Read,
                            AdapterOperation::Query,
                            AdapterOperation::Write,
                        ],
                        read_only: false,
                        rate_limit: Some(10_000), // 10k req/hour
                    }
                } else {
                    CertificateScope::default()
                }
            }

            CommercialClassification::SmallTeam { .. } => CertificateScope {
                operations: vec![
                    AdapterOperation::Read,
                    AdapterOperation::Query,
                    AdapterOperation::Write,
                ],
                read_only: false,
                rate_limit: Some(5_000),
            },

            CommercialClassification::Commercial { risk, .. } => {
                // Commercial without license gets very limited access
                match risk {
                    RiskLevel::Low => CertificateScope {
                        operations: vec![AdapterOperation::Read],
                        read_only: true,
                        rate_limit: Some(100), // Very limited
                    },
                    _ => CertificateScope {
                        operations: vec![],
                        read_only: true,
                        rate_limit: Some(10),
                    },
                }
            }

            CommercialClassification::Uncertain { .. } => CertificateScope {
                operations: vec![AdapterOperation::Read],
                read_only: true,
                rate_limit: Some(500),
            },
        }
    }

    /// Determine scope with valid license
    fn determine_scope_with_license(
        &self,
        _classification: &CommercialClassification,
        license: &LicenseInfo,
    ) -> CertificateScope {
        let operations = match license.license_type {
            LicenseType::Free => vec![AdapterOperation::Read, AdapterOperation::Query],
            LicenseType::Startup => vec![
                AdapterOperation::Read,
                AdapterOperation::Query,
                AdapterOperation::Write,
            ],
            LicenseType::Professional | LicenseType::Enterprise | LicenseType::Custom => {
                vec![
                    AdapterOperation::Read,
                    AdapterOperation::Query,
                    AdapterOperation::Write,
                    AdapterOperation::Configure,
                    AdapterOperation::Admin,
                ]
            }
        };

        CertificateScope {
            operations,
            read_only: matches!(license.license_type, LicenseType::Free),
            rate_limit: license.rate_limit,
        }
    }

    /// Sign certificate with BearDog key
    fn sign_certificate(&self, cert: &AdapterUnlockCertificate) -> Result<Vec<u8>, BearDogError> {
        // Compute hash of certificate fields (excluding signature)
        let hash = Self::compute_cert_hash(cert)?;

        // Sign with Ed25519
        let signature = self.signing_key.sign(&hash);

        Ok(signature.to_bytes().to_vec())
    }

    /// Compute certificate hash for signing
    fn compute_cert_hash(cert: &AdapterUnlockCertificate) -> Result<Vec<u8>, BearDogError> {
        use blake3::Hasher;

        // Serialize certificate fields (excluding signature)
        let mut hasher = Hasher::new();
        hasher.update(cert.id.as_bytes());
        hasher.update(cert.adapter_id.as_bytes());
        hasher.update(&bincode::serialize(&cert.classification).map_err(|e| {
            BearDogError::system(format!("Failed to serialize classification: {}", e))
        })?);
        hasher.update(&cert.expires_at.timestamp().to_le_bytes());
        hasher.update(
            &bincode::serialize(&cert.scope)
                .map_err(|e| BearDogError::system(format!("Failed to serialize scope: {}", e)))?,
        );

        Ok(hasher.finalize().as_bytes().to_vec())
    }

    /// Get key ID for this issuer
    fn get_key_id(&self) -> String {
        let verifying_key = self.signing_key.verifying_key();
        let key_bytes = verifying_key.to_bytes();
        hex::encode(&key_bytes[..8]) // First 8 bytes as hex
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_issuer() -> CertificateIssuer {
        let secret_bytes: [u8; 32] = rand::random();
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        CertificateIssuer::new(signing_key)
    }

    #[test]
    fn test_issue_human_certificate() {
        let issuer = create_test_issuer();
        let classification = CommercialClassification::Human {
            confidence: 95,
            reasons: vec!["Interactive session".to_string()],
        };

        let cert = issuer.issue(classification, "prometheus").unwrap();

        assert_eq!(cert.adapter_id, "prometheus");
        assert!(!cert.is_expired());
        assert!(cert.allows_operation(&AdapterOperation::Read));
        assert!(!cert.signature.is_empty());
    }

    #[test]
    fn test_commercial_without_license_denied() {
        let issuer = create_test_issuer();
        let classification = CommercialClassification::Commercial {
            risk: RiskLevel::High,
            indicators: vec![CommercialIndicator::HighVolume {
                requests_per_hour: 10000,
            }],
            confidence: 90,
        };

        let result = issuer.issue(classification, "prometheus");
        assert!(result.is_err());
    }

    #[test]
    fn test_commercial_with_license_allowed() {
        let issuer = create_test_issuer();
        let classification = CommercialClassification::Commercial {
            risk: RiskLevel::Medium,
            indicators: vec![],
            confidence: 80,
        };

        let license = LicenseInfo {
            key: "test-license-key".to_string(),
            license_type: LicenseType::Professional,
            expires_at: Utc::now() + Duration::days(365),
            entity: "Acme Corp".to_string(),
            rate_limit: Some(100_000),
        };

        let cert = issuer
            .issue_with_license(classification, "prometheus", license)
            .unwrap();

        assert!(cert.allows_operation(&AdapterOperation::Write));
        assert!(cert.allows_operation(&AdapterOperation::Configure));
    }
}
