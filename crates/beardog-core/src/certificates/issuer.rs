//! Certificate issuance for adapter unlocking
//!
//! This module implements the core certificate issuance logic that determines
//! whether an adapter should be unlocked and for how long.

use beardog_errors::BearDogError;
use beardog_types::adapter_certificates::{AdapterClassification, AdapterUnlockCertificate};
use beardog_types::commercial_extraction::CommercialExtractionDetector;
use chrono::{Duration, Utc};
use ed25519_dalek::{SigningKey, VerifyingKey};
use std::sync::Arc;
use tracing::{debug, info, warn};

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
}

impl CertificateIssuer {
    /// Create a new certificate issuer
    ///
    /// # Arguments
    ///
    /// * `signing_key` - Ed25519 key for signing certificates
    /// * `detector` - Commercial extraction detector for classification
    pub fn new(
        signing_key: SigningKey,
        detector: CommercialExtractionDetector,
    ) -> Self {
        Self {
            signing_key: Arc::new(signing_key),
            detector: Arc::new(detector),
        }
    }

    /// Issue a certificate for an adapter
    ///
    /// This is the main entry point for certificate issuance. It:
    /// 1. Classifies the request (human vs commercial)
    /// 2. Checks license requirements
    /// 3. Determines certificate lifetime
    /// 4. Creates and signs certificate
    ///
    /// # Arguments
    ///
    /// * `adapter_id` - Identifier for the adapter (e.g., "prometheus", "grafana")
    /// * `request_context` - Context about the requesting entity
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Classification fails
    /// - Commercial high-risk without license
    /// - Certificate signing fails
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let cert = issuer.issue_certificate(
    ///     "prometheus".to_string(),
    ///     &RequestContext {
    ///         user_agent: "curl/7.68.0".to_string(),
    ///         automation_score: 0.95,
    ///         pattern_consistency: 0.98,
    ///         // ...
    ///     },
    /// ).await?;
    /// ```
    pub async fn issue_certificate(
        &self,
        adapter_id: String,
        request_context: &RequestContext,
    ) -> Result<AdapterUnlockCertificate, BearDogError> {
        // 1. Classify the request
        let classification = self.classify_request(request_context).await?;
        
        info!(
            "Issuing certificate for adapter '{}': {:?}",
            adapter_id, classification
        );

        // 2. Check license requirements for high-risk commercial
        if let AdapterClassification::Commercial {
            risk_level,
            automation_score,
            ..
        } = &classification
        {
            use beardog_types::adapter_certificates::ExtractionRisk;
            
            if matches!(risk_level, ExtractionRisk::High) {
                // Check if license exists
                if !self.has_valid_license(request_context).await? {
                    warn!(
                        "High-risk commercial request without license (automation: {:.2})",
                        automation_score
                    );
                    return Err(BearDogError::license_required(format!(
                        "Adapter '{}' requires commercial license for automated extraction",
                        adapter_id
                    )));
                }
            }
        }

        // 3. Determine certificate lifetime based on classification
        let expires_at = self.determine_expiry(&classification);
        
        debug!(
            "Certificate for '{}' will expire at: {}",
            adapter_id, expires_at
        );

        // 4. Create and sign certificate
        AdapterUnlockCertificate::issue(
            adapter_id,
            classification,
            &self.signing_key,
        )
    }

    /// Classify an incoming request
    ///
    /// Uses the commercial extraction detector to determine whether this is
    /// human use or commercial extraction.
    async fn classify_request(
        &self,
        context: &RequestContext,
    ) -> Result<AdapterClassification, BearDogError> {
        // Use the detector's classification logic
        self.detector.classify(context).await
    }

    /// Determine certificate expiry based on classification
    ///
    /// # Lifetimes
    ///
    /// - **Human**: 24 hours (long-lived, minimal friction)
    /// - **Small Team**: 8 hours (moderate monitoring)
    /// - **Commercial Low/Medium**: 15 minutes (regular re-validation)
    /// - **Commercial High**: 15 minutes (strict monitoring)
    fn determine_expiry(&self, classification: &AdapterClassification) -> chrono::DateTime<Utc> {
        use beardog_types::adapter_certificates::ExtractionRisk;
        
        let duration = match classification {
            AdapterClassification::Human { .. } => {
                // Individual developers get long-lived certificates
                Duration::hours(24)
            }
            AdapterClassification::Commercial { risk_level, .. } => {
                match risk_level {
                    ExtractionRisk::Low => Duration::hours(1),
                    ExtractionRisk::Medium => Duration::minutes(30),
                    ExtractionRisk::High => Duration::minutes(15),
                }
            }
        };

        Utc::now() + duration
    }

    /// Check if a valid license exists for this request
    ///
    /// Environment-driven license validation:
    /// - Checks BEARDOG_LICENSE_KEY environment variable
    /// - Validates license format and expiration
    /// - Graceful fallback: No license = free tier (human use)
    ///
    /// Phase 5 will add: Usage metering, commercial tiers, HSM-backed validation
    async fn has_valid_license(&self, context: &RequestContext) -> Result<bool, BearDogError> {
        // Check for license key in environment
        let license_key = match std::env::var("BEARDOG_LICENSE_KEY") {
            Ok(key) if !key.is_empty() => key,
            _ => {
                // No license key = free tier (human use)
                debug!(
                    "No license key found for {}, assuming free tier",
                    context.requester_id
                );
                return Ok(false);
            }
        };

        // Basic license validation (Phase 1)
        // Format: BEARDOG-{TYPE}-{EXPIRY}-{SIGNATURE}
        // Example: BEARDOG-PRO-20261231-abc123def456
        let parts: Vec<&str> = license_key.split('-').collect();
        if parts.len() != 4 || parts[0] != "BEARDOG" {
            warn!("Invalid license key format for {}", context.requester_id);
            return Ok(false);
        }

        let license_type = parts[1];
        let expiry_str = parts[2];
        let _signature = parts[3]; // Phase 5: Verify signature with HSM

        // Check expiration
        if let Ok(expiry_date) = chrono::NaiveDate::parse_from_str(expiry_str, "%Y%m%d") {
            // Set expiration to end of day (23:59:59)
            // and_hms_opt returns None only for invalid times, which 23:59:59 is not
            let expiry = match expiry_date.and_hms_opt(23, 59, 59) {
                Some(dt) => dt,
                None => {
                    warn!(
                        "Failed to create expiry datetime for {} (date: {})",
                        context.requester_id, expiry_str
                    );
                    return Ok(false);
                }
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
                "✅ Valid {} license for {} (expires: {})",
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

        // Phase 5 TODO:
        // - Verify signature using HSM
        // - Check usage limits from metering system
        // - Validate license tier matches request type
        // - Support license revocation lists
    }

    /// Get the public key for certificate verification
    ///
    /// Adapters need this to verify certificates are authentic
    pub fn public_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }
}

/// Request context for classification
///
/// Contains information about the requesting entity that helps determine
/// whether this is human use or commercial extraction.
#[derive(Debug, Clone)]
pub struct RequestContext {
    /// User agent string
    pub user_agent: String,
    
    /// Automation score (0.0 = human, 1.0 = fully automated)
    pub automation_score: f64,
    
    /// Pattern consistency (0.0 = random, 1.0 = perfectly consistent)
    pub pattern_consistency: f64,
    
    /// Number of requests in the last hour
    pub request_rate: u32,
    
    /// Whether TLS was used
    pub tls_enabled: bool,
    
    /// Source IP address
    pub source_ip: String,
    
    /// Optional: License key if present
    pub license_key: Option<String>,
}

impl Default for RequestContext {
    fn default() -> Self {
        Self {
            user_agent: "unknown".to_string(),
            automation_score: 0.0,
            pattern_consistency: 0.0,
            request_rate: 0,
            tls_enabled: false,
            source_ip: "0.0.0.0".to_string(),
            license_key: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::commercial_extraction::CommercialExtractionDetector;

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

        let result = issuer.issue_certificate("test-adapter".to_string(), &context).await;
        
        // Should succeed for human use
        assert!(result.is_ok());
        
        let cert = result.unwrap();
        assert_eq!(cert.adapter_id, "test-adapter");
        
        // Human classification should get ~24 hour expiry
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

        let result = issuer.issue_certificate("test-adapter".to_string(), &context).await;
        
        // Should fail for high-risk commercial without license
        assert!(result.is_err());
        
        let err = result.unwrap_err();
        assert!(err.to_string().contains("license"));
    }
}

