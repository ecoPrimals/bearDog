//! Automatic certificate renewal
//!
//! Handles automatic renewal for certificates that are about to expire.

use super::{CertificateIssuer, CertificateStore};
use super::issuer::RequestContext;
use beardog_errors::BearDogError;
use chrono::{Duration, Utc};
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Certificate renewal manager
///
/// Monitors certificates and automatically renews them before expiry
/// for approved use cases.
#[derive(Debug, Clone)]
pub struct CertificateRenewal {
    /// Certificate issuer for creating new certificates
    issuer: Arc<CertificateIssuer>,
    
    /// Certificate store
    store: Arc<CertificateStore>,
    
    /// How far before expiry to renew (default: 5 minutes)
    renewal_buffer: Duration,
}

impl CertificateRenewal {
    /// Create a new renewal manager
    pub fn new(issuer: CertificateIssuer, store: CertificateStore) -> Self {
        Self {
            issuer: Arc::new(issuer),
            store: Arc::new(store),
            renewal_buffer: Duration::minutes(5),
        }
    }

    /// Check if an adapter's certificate needs renewal
    ///
    /// Returns true if:
    /// - Certificate exists
    /// - Certificate is valid
    /// - Certificate expires within renewal_buffer time
    pub async fn needs_renewal(&self, adapter_id: &str) -> bool {
        if let Some(cert) = self.store.get(adapter_id).await {
            let time_until_expiry = cert.expires_at - Utc::now();
            time_until_expiry < self.renewal_buffer
        } else {
            false
        }
    }

    /// Attempt to renew a certificate
    ///
    /// Issues a new certificate if the usage pattern remains valid.
    ///
    /// # Arguments
    ///
    /// * `adapter_id` - The adapter to renew
    /// * `context` - Current request context for re-classification
    ///
    /// # Returns
    ///
    /// Ok(true) if renewed successfully
    /// Ok(false) if renewal not needed or not allowed
    /// Err if renewal failed
    pub async fn renew(
        &self,
        adapter_id: &str,
        context: &RequestContext,
    ) -> Result<bool, BearDogError> {
        // Check if renewal is needed
        if !self.needs_renewal(adapter_id).await {
            debug!("Certificate for '{}' does not need renewal yet", adapter_id);
            return Ok(false);
        }

        info!("Renewing certificate for adapter: {}", adapter_id);

        // Issue new certificate (will re-classify and check eligibility)
        match self.issuer.issue_certificate(adapter_id.to_string(), context).await {
            Ok(new_cert) => {
                // Store the new certificate
                self.store.store(new_cert).await?;
                info!("Successfully renewed certificate for: {}", adapter_id);
                Ok(true)
            }
            Err(e) => {
                warn!("Failed to renew certificate for '{}': {}", adapter_id, e);
                Err(e)
            }
        }
    }

    /// Set the renewal buffer time
    ///
    /// Certificates will be renewed when they have less than this time remaining.
    pub fn set_renewal_buffer(&mut self, buffer: Duration) {
        self.renewal_buffer = buffer;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::adapter_certificates::AdapterClassification;
    use beardog_types::commercial_extraction::CommercialExtractionDetector;
    use ed25519_dalek::SigningKey;

    #[tokio::test]
    async fn test_needs_renewal() {
        let signing_key = SigningKey::from_bytes(&[1u8; 32]);
        let detector = CommercialExtractionDetector::default();
        let issuer = CertificateIssuer::new(signing_key, detector);
        let store = CertificateStore::new();
        let renewal = CertificateRenewal::new(issuer, store.clone());

        // Create certificate that expires soon
        let signing_key = SigningKey::from_bytes(&[1u8; 32]);
        let mut cert = beardog_types::adapter_certificates::AdapterUnlockCertificate::issue(
            "test-adapter".to_string(),
            AdapterClassification::Human { confidence: 0.9 },
            &signing_key,
        ).unwrap();
        
        // Set to expire in 3 minutes (within 5 minute buffer)
        cert.expires_at = Utc::now() + Duration::minutes(3);
        store.store(cert).await.unwrap();

        // Should need renewal
        assert!(renewal.needs_renewal("test-adapter").await);
    }

    #[tokio::test]
    async fn test_no_renewal_needed() {
        let signing_key = SigningKey::from_bytes(&[1u8; 32]);
        let detector = CommercialExtractionDetector::default();
        let issuer = CertificateIssuer::new(signing_key, detector);
        let store = CertificateStore::new();
        let renewal = CertificateRenewal::new(issuer, store.clone());

        // Create certificate with plenty of time
        let signing_key = SigningKey::from_bytes(&[1u8; 32]);
        let cert = beardog_types::adapter_certificates::AdapterUnlockCertificate::issue(
            "test-adapter".to_string(),
            AdapterClassification::Human { confidence: 0.9 },
            &signing_key,
        ).unwrap();
        
        store.store(cert).await.unwrap();

        // Should not need renewal
        assert!(!renewal.needs_renewal("test-adapter").await);
    }
}

