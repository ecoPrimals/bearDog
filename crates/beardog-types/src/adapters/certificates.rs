// SPDX-License-Identifier: AGPL-3.0-only

//! # Adapter Unlock Certificates
//!
//! Cryptographically signed certificates that unlock external adapters.
//!
//! ## Philosophy
//!
//! "Open gates for humans, locked tight for commercial extraction."
//!
//! ## Architecture
//!
//! External adapters (Prometheus, Grafana, Consul, etc.) require unlock certificates.
//! These certificates are:
//! - Cryptographically signed by `BearDog` daemon
//! - Time-limited (15 min for commercial, 24hr for individual)
//! - Classification-based (Human gets automatic access)
//! - Renewable
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_types::adapters::AdapterUnlockCertificate;
//!
//! let cert = AdapterUnlockCertificate::issue(
//!     "prometheus".to_string(),
//!     CommercialClassification::Human { confidence: 0.95 },
//!     &daemon_signing_key,
//! )?;
//! cert.verify(&daemon_public_key)?;
//! ```

use beardog_errors::BearDogError;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// ADAPTER UNLOCK CERTIFICATE
// ============================================================================

/// Cryptographically signed certificate that unlocks an external adapter
///
/// This certificate is issued by the `BearDog` daemon after classification and
/// license checking. It grants time-limited access to an external adapter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterUnlockCertificate {
    /// Unique certificate ID
    pub certificate_id: String,

    /// `BearDog` key that issued this certificate
    pub issuing_key_id: String,

    /// What adapter this unlocks ("prometheus", "grafana", "consul", etc.)
    pub adapter_id: String,

    /// Classification that earned this unlock
    pub classification: CertificateClassification,

    /// When this certificate was issued
    pub issued_at: DateTime<Utc>,

    /// When this certificate expires
    pub expires_at: DateTime<Utc>,

    /// Cryptographic proof (Ed25519 signature over certificate data)
    pub signature: Vec<u8>,

    /// Optional license information (for commercial usage)
    pub license_info: Option<LicenseInfo>,

    /// Self-enforcing constraints (inherited from key constraints)
    pub constraints: Option<super::super::genetics_constraints::KeyConstraints>,

    /// Usage metering (for billing)
    pub metering: MeteringConfig,
}

/// Classification used for certificate issuance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CertificateClassification {
    /// Individual human developer (automatic unlock)
    Human {
        /// Confidence score (0.0 - 1.0)
        confidence: f64,
    },

    /// Small team (automatic unlock with monitoring)
    SmallTeam {
        /// Team size estimate
        team_size: u32,
        /// Confidence score
        confidence: f64,
    },

    /// Commercial extraction (requires license)
    Commercial {
        /// Risk level
        risk_level: ExtractionRisk,
        /// Confidence score
        confidence: f64,
        /// Automation percentage (0-100)
        automation_percent: u8,
    },

    /// Uncertain classification (cautious unlock)
    Uncertain {
        /// Best guess
        likely_classification: Box<Self>,
        /// Confidence in guess
        confidence: f64,
    },
}

/// Extraction risk level for commercial classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExtractionRisk {
    /// Low risk - interactive use
    Low,
    /// Medium risk - partial automation
    Medium,
    /// High risk - full automation pipeline
    High,
}

/// License information for commercial usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    /// License key
    pub license_key: String,

    /// License type
    pub license_type: LicenseType,

    /// When license expires
    pub expires_at: DateTime<Utc>,

    /// What adapters are licensed
    pub licensed_adapters: Vec<String>,

    /// Licensee information
    pub licensee: String,
}

/// Type of license
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LicenseType {
    /// Per-adapter license
    PerAdapter,
    /// Unlimited adapters
    Enterprise,
    /// Evaluation license
    Evaluation,
    /// Academic/research license
    Academic,
}

/// Metering configuration for usage tracking
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MeteringConfig {
    /// Track usage?
    pub enabled: bool,

    /// Meter reads, writes, or both
    pub meter_operations: Vec<String>,

    /// Aggregate usage per period
    pub aggregation_period_seconds: u64,

    /// Current usage stats
    pub usage: UsageStats,
}

/// Usage statistics for billing
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageStats {
    /// Operations performed
    pub operation_count: u64,

    /// Data volume (bytes)
    pub data_volume_bytes: u64,

    /// Compute time (milliseconds)
    pub compute_time_ms: u64,

    /// Last metering update
    pub last_updated: Option<DateTime<Utc>>,
}

/// Certificate status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CertificateStatus {
    /// Valid and active
    Valid,
    /// Expired
    Expired,
    /// Revoked
    Revoked,
    /// Invalid signature
    Invalid,
}

// ============================================================================
// IMPLEMENTATION
// ============================================================================

impl AdapterUnlockCertificate {
    /// Issue a new certificate
    ///
    /// # Arguments
    ///
    /// * `adapter_id` - Which adapter to unlock
    /// * `classification` - User classification result
    /// * `signing_key` - `BearDog` daemon's signing key
    ///
    /// # Errors
    ///
    /// Returns error if signing fails
    pub fn issue(
        adapter_id: String,
        classification: CertificateClassification,
        signing_key: &ed25519_dalek::SigningKey,
    ) -> Result<Self, BearDogError> {
        use ed25519_dalek::Signer;
        use uuid::Uuid;

        let issued_at = Utc::now();

        // Determine expiry based on classification
        let expires_at = match &classification {
            CertificateClassification::Human { .. } => {
                issued_at + Duration::hours(24) // 24 hours for individuals
            }
            CertificateClassification::SmallTeam { .. } => {
                issued_at + Duration::hours(8) // 8 hours for teams
            }
            CertificateClassification::Commercial { risk_level, .. } => {
                let ttl = match risk_level {
                    ExtractionRisk::Low => Duration::hours(1),
                    ExtractionRisk::Medium => Duration::minutes(30),
                    ExtractionRisk::High => Duration::minutes(15),
                };
                issued_at + ttl
            }
            CertificateClassification::Uncertain { .. } => {
                issued_at + Duration::hours(1) // 1 hour for uncertain
            }
        };

        let certificate_id = Uuid::new_v4().to_string();
        let issuing_key_id = format!("beardog-daemon-{}", Uuid::new_v4());

        // Create certificate data for signing
        let cert_data = Self::create_signing_data(
            &certificate_id,
            &adapter_id,
            &classification,
            issued_at,
            expires_at,
        )?;

        // Sign the certificate
        let signature = signing_key.sign(&cert_data);

        Ok(Self {
            certificate_id,
            issuing_key_id,
            adapter_id,
            classification,
            issued_at,
            expires_at,
            signature: signature.to_bytes().to_vec(),
            license_info: None,
            constraints: None,
            metering: MeteringConfig::default(),
        })
    }

    /// Verify certificate signature
    ///
    /// # Errors
    ///
    /// Returns error if signature invalid or certificate expired
    pub fn verify(&self, verifying_key: &ed25519_dalek::VerifyingKey) -> Result<(), BearDogError> {
        use ed25519_dalek::Verifier;

        // Check expiry first
        if Utc::now() > self.expires_at {
            return Err(BearDogError::unauthorized(
                "Certificate expired".to_string(),
            ));
        }

        // Create the same data that was signed
        let cert_data = Self::create_signing_data(
            &self.certificate_id,
            &self.adapter_id,
            &self.classification,
            self.issued_at,
            self.expires_at,
        )?;

        // Convert signature bytes to array
        if self.signature.len() != 64 {
            return Err(BearDogError::security(format!(
                "Invalid signature length: {} (expected 64)",
                self.signature.len()
            )));
        }
        let mut sig_array = [0u8; 64];
        sig_array.copy_from_slice(&self.signature);

        let signature = ed25519_dalek::Signature::from_bytes(&sig_array);

        // Verify signature
        verifying_key.verify(&cert_data, &signature).map_err(|_| {
            BearDogError::security("Certificate signature verification failed".to_string())
        })?;

        Ok(())
    }

    /// Get current certificate status
    pub fn status(&self) -> CertificateStatus {
        if Utc::now() > self.expires_at {
            CertificateStatus::Expired
        } else {
            CertificateStatus::Valid
        }
    }

    /// Check if commercial usage requires license
    pub fn requires_license(&self) -> bool {
        matches!(
            &self.classification,
            CertificateClassification::Commercial {
                risk_level: ExtractionRisk::High,
                ..
            }
        )
    }

    /// Create signing data (deterministic serialization)
    fn create_signing_data(
        certificate_id: &str,
        adapter_id: &str,
        classification: &CertificateClassification,
        issued_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<Vec<u8>, BearDogError> {
        use sha3::{Digest, Sha3_256};

        let mut hasher = Sha3_256::new();
        hasher.update(certificate_id.as_bytes());
        hasher.update(adapter_id.as_bytes());
        hasher.update(
            &postcard::to_allocvec(classification)
                .map_err(|e| BearDogError::serialization(&format!("Failed to serialize: {e}")))?,
        );
        hasher.update(issued_at.timestamp().to_le_bytes());
        hasher.update(expires_at.timestamp().to_le_bytes());
        hasher.update(b"BearDog-AdapterCertificate-v1");

        Ok(hasher.finalize().to_vec())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;

    #[test]
    fn test_issue_and_verify_certificate() {
        let signing_key = SigningKey::from_bytes(&[0u8; 32]);
        let verifying_key = signing_key.verifying_key();

        let cert = AdapterUnlockCertificate::issue(
            "prometheus".to_string(),
            CertificateClassification::Human { confidence: 0.95 },
            &signing_key,
        )
        .expect("certificate issuance should work");

        // Should verify successfully
        assert!(cert.verify(&verifying_key).is_ok());
    }

    #[test]
    fn test_expired_certificate_rejected() {
        let signing_key = SigningKey::from_bytes(&[0u8; 32]);
        let verifying_key = signing_key.verifying_key();

        let mut cert = AdapterUnlockCertificate::issue(
            "grafana".to_string(),
            CertificateClassification::Human { confidence: 0.90 },
            &signing_key,
        )
        .expect("certificate issuance should work");

        // Manually expire it
        cert.expires_at = Utc::now() - Duration::seconds(1);

        // Should reject as expired
        assert!(cert.verify(&verifying_key).is_err());
    }

    #[test]
    fn test_commercial_high_risk_requires_license() {
        let signing_key = SigningKey::from_bytes(&[0u8; 32]);

        let cert = AdapterUnlockCertificate::issue(
            "consul".to_string(),
            CertificateClassification::Commercial {
                risk_level: ExtractionRisk::High,
                confidence: 0.85,
                automation_percent: 95,
            },
            &signing_key,
        )
        .expect("certificate issuance should work");

        assert!(cert.requires_license());
    }

    #[test]
    fn test_individual_gets_long_expiry() {
        let signing_key = SigningKey::from_bytes(&[0u8; 32]);

        let cert = AdapterUnlockCertificate::issue(
            "prometheus".to_string(),
            CertificateClassification::Human { confidence: 0.95 },
            &signing_key,
        )
        .expect("certificate issuance should work");

        let duration = cert.expires_at - cert.issued_at;
        assert!(duration.num_hours() >= 23); // Should be ~24 hours
    }

    #[test]
    fn test_commercial_gets_short_expiry() {
        let signing_key = SigningKey::from_bytes(&[0u8; 32]);

        let cert = AdapterUnlockCertificate::issue(
            "grafana".to_string(),
            CertificateClassification::Commercial {
                risk_level: ExtractionRisk::High,
                confidence: 0.80,
                automation_percent: 100,
            },
            &signing_key,
        )
        .expect("certificate issuance should work");

        let duration = cert.expires_at - cert.issued_at;
        assert!(duration.num_minutes() <= 16); // Should be ~15 minutes
    }

    #[test]
    fn test_certificate_status() {
        let signing_key = SigningKey::from_bytes(&[0u8; 32]);

        let cert = AdapterUnlockCertificate::issue(
            "consul".to_string(),
            CertificateClassification::Human { confidence: 0.90 },
            &signing_key,
        )
        .expect("certificate issuance should work");

        assert_eq!(cert.status(), CertificateStatus::Valid);
    }
}
