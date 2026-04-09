// SPDX-License-Identifier: AGPL-3.0-or-later

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

/// First segment of dash-separated license keys (`{prefix}-<type>-<expiry>-<trailing>`).
/// This is the license **wire format** discriminator, not primal identity.
const LICENSE_KEY_FORMAT_PREFIX: &str = "BEARDOG";

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
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when classification, license checks, or certificate construction fails.
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
        if parts.len() != 4 || parts[0] != LICENSE_KEY_FORMAT_PREFIX {
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
    #![expect(clippy::unwrap_used, reason = "test assertions")]

    use super::*;

    fn high_risk_context() -> RequestContext {
        RequestContext {
            user_agent: "curl/7.68.0".to_string(),
            automation_score: 0.95,
            pattern_consistency: 0.98,
            request_rate: 1000,
            ..Default::default()
        }
    }

    #[test]
    fn license_inputs_default_is_empty() {
        let li = LicenseInputs::default();
        assert!(li.license_key.is_none());
    }

    #[test]
    fn license_inputs_from_env_is_callable() {
        let li = LicenseInputs::from_env();
        let _ = li.license_key.as_ref().map(String::len);
    }

    #[test]
    fn license_inputs_clone_and_eq() {
        let a = LicenseInputs {
            license_key: Some("BEARDOG-ENT-20991231-X".into()),
        };
        assert_eq!(a, a.clone());
    }

    #[test]
    fn certificate_issuer_new_matches_explicit_default_license() {
        let sk = SigningKey::from_bytes(&[14u8; 32]);
        let a = CertificateIssuer::new(sk.clone(), CommercialExtractionDetector);
        let b = CertificateIssuer::with_license_inputs(
            sk,
            CommercialExtractionDetector,
            LicenseInputs::default(),
        );
        assert_eq!(a.public_key(), b.public_key());
    }

    #[test]
    fn certificate_issuer_from_env_builds_and_exposes_verifying_key() {
        let sk = SigningKey::from_bytes(&[15u8; 32]);
        let issuer = CertificateIssuer::from_env(sk.clone(), CommercialExtractionDetector);
        assert_eq!(issuer.public_key(), sk.verifying_key());
    }

    #[tokio::test]
    async fn high_risk_empty_license_string_skips_valid_license_path() {
        let signing_key = SigningKey::from_bytes(&[13u8; 32]);
        let issuer = CertificateIssuer::with_license_inputs(
            signing_key,
            CommercialExtractionDetector,
            LicenseInputs {
                license_key: Some(String::new()),
            },
        );
        let err = issuer
            .issue_certificate("blocked".into(), &high_risk_context())
            .await
            .unwrap_err();
        assert!(err.to_string().contains("license"));
    }

    #[test]
    fn certificate_issuer_public_key_matches_signing_key() {
        let signing_key = SigningKey::from_bytes(&[5u8; 32]);
        let issuer = CertificateIssuer::new(signing_key.clone(), CommercialExtractionDetector);
        assert_eq!(issuer.public_key(), signing_key.verifying_key());
    }

    #[tokio::test]
    async fn certificate_issuer_high_risk_succeeds_with_future_dated_license() {
        let signing_key = SigningKey::from_bytes(&[6u8; 32]);
        let issuer = CertificateIssuer::with_license_inputs(
            signing_key,
            CommercialExtractionDetector,
            LicenseInputs {
                license_key: Some("BEARDOG-ENT-20991231-VERIFYME".to_string()),
            },
        );
        let ctx = high_risk_context();
        let cert = issuer
            .issue_certificate("licensed-adapter".into(), &ctx)
            .await
            .unwrap();
        assert_eq!(cert.adapter_id, "licensed-adapter");
    }

    #[tokio::test]
    async fn has_valid_license_rejects_wrong_segment_count() {
        let signing_key = SigningKey::from_bytes(&[7u8; 32]);
        let issuer = CertificateIssuer::with_license_inputs(
            signing_key,
            CommercialExtractionDetector,
            LicenseInputs {
                license_key: Some("BEARDOG-a-b".to_string()),
            },
        );
        let err = issuer
            .issue_certificate("a".into(), &high_risk_context())
            .await
            .unwrap_err();
        assert!(err.to_string().contains("license"));
    }

    #[tokio::test]
    async fn has_valid_license_rejects_bad_prefix() {
        let signing_key = SigningKey::from_bytes(&[8u8; 32]);
        let issuer = CertificateIssuer::with_license_inputs(
            signing_key,
            CommercialExtractionDetector,
            LicenseInputs {
                license_key: Some("OTHER-T-20991231-X".to_string()),
            },
        );
        let err = issuer
            .issue_certificate("a".into(), &high_risk_context())
            .await
            .unwrap_err();
        assert!(err.to_string().contains("license"));
    }

    #[tokio::test]
    async fn has_valid_license_rejects_expired_key() {
        let signing_key = SigningKey::from_bytes(&[9u8; 32]);
        let issuer = CertificateIssuer::with_license_inputs(
            signing_key,
            CommercialExtractionDetector,
            LicenseInputs {
                license_key: Some("BEARDOG-T-20200101-X".to_string()),
            },
        );
        let err = issuer
            .issue_certificate("a".into(), &high_risk_context())
            .await
            .unwrap_err();
        assert!(err.to_string().contains("license"));
    }

    #[tokio::test]
    async fn has_valid_license_rejects_unparseable_expiry() {
        let signing_key = SigningKey::from_bytes(&[10u8; 32]);
        let issuer = CertificateIssuer::with_license_inputs(
            signing_key,
            CommercialExtractionDetector,
            LicenseInputs {
                license_key: Some("BEARDOG-T-notadate-X".to_string()),
            },
        );
        let err = issuer
            .issue_certificate("a".into(), &high_risk_context())
            .await
            .unwrap_err();
        assert!(err.to_string().contains("license"));
    }

    #[tokio::test]
    async fn commercial_medium_risk_issues_without_license() {
        let signing_key = SigningKey::from_bytes(&[11u8; 32]);
        let issuer = CertificateIssuer::new(signing_key, CommercialExtractionDetector);
        let ctx = RequestContext {
            automation_score: 0.6,
            pattern_consistency: 0.5,
            request_rate: 50,
            ..Default::default()
        };
        let cert = issuer.issue_certificate("mid".into(), &ctx).await.unwrap();
        assert_eq!(cert.adapter_id, "mid");
    }

    #[tokio::test]
    async fn test_certificate_issuance_human() {
        let signing_key = SigningKey::from_bytes(&[1u8; 32]);
        let detector = CommercialExtractionDetector;
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

        let cert = result.unwrap();
        assert_eq!(cert.adapter_id, "test-adapter");

        let lifetime = cert.expires_at - cert.issued_at;
        assert!(lifetime.num_hours() >= 23 && lifetime.num_hours() <= 25);
    }

    #[tokio::test]
    async fn test_certificate_issuance_commercial_high_risk_blocked() {
        let signing_key = SigningKey::from_bytes(&[2u8; 32]);
        let detector = CommercialExtractionDetector;
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

        let err = result.unwrap_err();
        assert!(err.to_string().contains("license"));
    }
}
