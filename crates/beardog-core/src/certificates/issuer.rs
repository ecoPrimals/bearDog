// SPDX-License-Identifier: AGPL-3.0-only

//! Certificate issuance for adapter unlocking
//!
//! This module implements the core certificate issuance logic that determines
//! whether an adapter should be unlocked and for how long.

use super::CommercialExtractionDetector;
use super::context::RequestContext;
use beardog_errors::BearDogError;
use beardog_types::adapters::{
    AdapterUnlockCertificate, CertificateClassification, ExtractionRisk,
};
use ed25519_dalek::{SigningKey, VerifyingKey};
use std::sync::Arc;
use tracing::{debug, info, warn};

/// License material for [`CertificateIssuer`] (no environment reads in [`Default`]).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LicenseInputs {
    /// `BEARDOG_LICENSE_KEY` value when present and non-empty.
    pub license_key: Option<String>,
}

impl LicenseInputs {
    /// Read `BEARDOG_LICENSE_KEY` via `std::env::var`.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            license_key: std::env::var("BEARDOG_LICENSE_KEY")
                .ok()
                .filter(|s| !s.is_empty()),
        }
    }
}

/// Certificate issuer - determines adapter unlock eligibility
///
/// This is the core enforcement point for the "open for humans, locked for
/// extraction" model. It classifies incoming requests and issues appropriate
/// certificates.
///
/// # Philosophy
///
/// - **Individuals**: Get 24-hour certificates automatically
/// - **Small teams**: Get 8-hour certificates with monitoring
/// - **Commercial low/medium risk**: Get 15-minute certificates
/// - **Commercial high risk**: Require explicit license before certificate
///
/// # Example
///
/// ```rust,ignore
/// let issuer = CertificateIssuer::new(signing_key, detector);
///
/// let cert = issuer.issue_certificate(
///     "prometheus".to_string(),
///     &request_context,
/// ).await?;
///
/// // Certificate is now signed and time-limited
/// adapter.initialize(cert)?;
/// ```
#[derive(Debug, Clone)]
pub struct CertificateIssuer {
    /// Signing key for certificate creation
    signing_key: Arc<SigningKey>,

    /// Classification detector
    detector: Arc<CommercialExtractionDetector>,

    /// Injected license material (use [`LicenseInputs::from_env`] at process entry if needed)
    license: LicenseInputs,
}

impl CertificateIssuer {
    /// Create a new certificate issuer
    #[must_use]
    pub fn new(signing_key: SigningKey, detector: CommercialExtractionDetector) -> Self {
        Self::with_license_inputs(signing_key, detector, LicenseInputs::default())
    }

    /// Create issuer with explicit license inputs (tests and non-env configuration).
    #[must_use]
    pub fn with_license_inputs(
        signing_key: SigningKey,
        detector: CommercialExtractionDetector,
        license: LicenseInputs,
    ) -> Self {
        Self {
            signing_key: Arc::new(signing_key),
            detector: Arc::new(detector),
            license,
        }
    }

    /// Load license from the process environment ([`LicenseInputs::from_env`]).
    #[must_use]
    pub fn from_env(signing_key: SigningKey, detector: CommercialExtractionDetector) -> Self {
        Self::with_license_inputs(signing_key, detector, LicenseInputs::from_env())
    }

    /// Issue a certificate for an adapter
    pub async fn issue_certificate(
        &self,
        adapter_id: String,
        request_context: &RequestContext,
    ) -> Result<AdapterUnlockCertificate, BearDogError> {
        let classification = self.classify_request(request_context).await?;

        info!(
            "Issuing certificate for adapter '{}': {:?}",
            adapter_id, classification
        );

        if let CertificateClassification::Commercial {
            risk_level,
            automation_percent,
            ..
        } = &classification
            && matches!(risk_level, ExtractionRisk::High)
            && !self.has_valid_license(request_context).await?
        {
            warn!(
                "High-risk commercial request without license (automation: {}%)",
                automation_percent
            );
            return Err(BearDogError::unauthorized(format!(
                "Adapter '{adapter_id}' requires commercial license for automated extraction"
            )));
        }

        AdapterUnlockCertificate::issue(adapter_id, classification, &self.signing_key)
    }

    async fn classify_request(
        &self,
        context: &RequestContext,
    ) -> Result<CertificateClassification, BearDogError> {
        self.detector.classify(context).await
    }

    /// Check if a valid license exists for this request
    ///
    /// Uses injected [`LicenseInputs::license_key`] when set:
    /// - Validates format and expiration date from the key payload
    ///
    /// The trailing segment is reserved for future cryptographic verification of the token.
    async fn has_valid_license(&self, context: &RequestContext) -> Result<bool, BearDogError> {
        let license_key = match &self.license.license_key {
            Some(key) if !key.is_empty() => key.clone(),
            _ => {
                debug!(
                    "No license key found for {}, assuming free tier",
                    context.requester_id
                );
                return Ok(false);
            }
        };

        let parts: Vec<&str> = license_key.split('-').collect();
        if parts.len() != 4 || parts[0] != "BEARDOG" {
            warn!("Invalid license key format for {}", context.requester_id);
            return Ok(false);
        }

        let license_type = parts[1];
        let expiry_str = parts[2];

        if let Ok(expiry_date) = chrono::NaiveDate::parse_from_str(expiry_str, "%Y%m%d") {
            let Some(expiry) = expiry_date.and_hms_opt(23, 59, 59) else {
                warn!(
                    "Failed to create expiry datetime for {} (date: {})",
                    context.requester_id, expiry_str
                );
                return Ok(false);
            };

            let now = chrono::Utc::now().naive_utc();

            if expiry < now {
                warn!(
                    "License expired for {} (expired: {})",
                    context.requester_id, expiry_str
                );
                return Ok(false);
            }

            info!(
                "Valid {} license for {} (expires: {})",
                license_type, context.requester_id, expiry_str
            );
            Ok(true)
        } else {
            warn!(
                "Invalid license expiry format for {}: {}",
                context.requester_id, expiry_str
            );
            Ok(false)
        }
    }

    /// Get the public key for certificate verification
    #[must_use]
    pub fn public_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_certificate_issuance_human() {
        let signing_key = SigningKey::from_bytes(&[1u8; 32]);
        let detector = CommercialExtractionDetector::default();
        let issuer = CertificateIssuer::new(signing_key, detector);

        let context = RequestContext {
            user_agent: "Mozilla/5.0".to_string(),
            automation_score: 0.1,
            pattern_consistency: 0.3,
            request_rate: 10,
            ..Default::default()
        };

        let result = issuer
            .issue_certificate("test-adapter".to_string(), &context)
            .await;

        assert!(result.is_ok());

        let cert = result.expect("human issuance should succeed");
        assert_eq!(cert.adapter_id, "test-adapter");

        let lifetime = cert.expires_at - cert.issued_at;
        assert!(lifetime.num_hours() >= 23 && lifetime.num_hours() <= 25);
    }

    #[tokio::test]
    async fn test_certificate_issuance_commercial_high_risk_blocked() {
        let signing_key = SigningKey::from_bytes(&[2u8; 32]);
        let detector = CommercialExtractionDetector::default();
        let issuer = CertificateIssuer::new(signing_key, detector);

        let context = RequestContext {
            user_agent: "curl/7.68.0".to_string(),
            automation_score: 0.95,
            pattern_consistency: 0.98,
            request_rate: 1000,
            ..Default::default()
        };

        let result = issuer
            .issue_certificate("test-adapter".to_string(), &context)
            .await;

        assert!(result.is_err());

        let err = result.expect_err("high-risk commercial without license must fail");
        assert!(err.to_string().contains("license"));
    }
}
