// SPDX-License-Identifier: AGPL-3.0-only

//! Certificate storage and management
//!
//! Manages active certificates and handles expiry checking.

use beardog_errors::BearDogError;
use beardog_types::adapters::AdapterUnlockCertificate;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, warn};

/// Certificate store for managing active adapter certificates
///
/// Stores certificates keyed by adapter ID and provides
/// expiry checking and cleanup.
#[derive(Debug, Clone)]
pub struct CertificateStore {
    /// Active certificates by adapter ID
    certificates: Arc<RwLock<HashMap<String, AdapterUnlockCertificate>>>,
}

impl CertificateStore {
    /// Create a new certificate store
    #[must_use]
    pub fn new() -> Self {
        Self {
            certificates: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Store a certificate
    ///
    /// Replaces any existing certificate for the same adapter.
    pub async fn store(&self, cert: AdapterUnlockCertificate) -> Result<(), BearDogError> {
        let adapter_id = cert.adapter_id.clone();

        let mut certs = self.certificates.write().await;
        certs.insert(adapter_id.clone(), cert);

        debug!("Stored certificate for adapter: {}", adapter_id);
        Ok(())
    }

    /// Get a certificate by adapter ID
    ///
    /// Returns None if:
    /// - No certificate exists
    /// - Certificate has expired
    pub async fn get(&self, adapter_id: &str) -> Option<AdapterUnlockCertificate> {
        let certs = self.certificates.read().await;

        if let Some(cert) = certs.get(adapter_id) {
            // Check expiry
            if cert.expires_at > Utc::now() {
                return Some(cert.clone());
            }
            warn!("Certificate for '{}' has expired", adapter_id);
        }

        None
    }

    /// Remove a certificate
    pub async fn remove(&self, adapter_id: &str) -> Result<(), BearDogError> {
        let mut certs = self.certificates.write().await;
        certs.remove(adapter_id);

        debug!("Removed certificate for adapter: {}", adapter_id);
        Ok(())
    }

    /// Clean up expired certificates
    ///
    /// Should be called periodically to prevent memory buildup.
    pub async fn cleanup_expired(&self) -> Result<usize, BearDogError> {
        let mut certs = self.certificates.write().await;
        let now = Utc::now();

        let before_count = certs.len();
        certs.retain(|_, cert| cert.expires_at > now);
        let after_count = certs.len();

        let removed = before_count - after_count;
        if removed > 0 {
            debug!("Cleaned up {} expired certificates", removed);
        }

        Ok(removed)
    }

    /// Get count of active certificates
    pub async fn count(&self) -> usize {
        self.certificates.read().await.len()
    }
}

impl Default for CertificateStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::adapters::CertificateClassification;
    use chrono::Duration;
    use ed25519_dalek::SigningKey;

    #[tokio::test]
    async fn test_store_and_retrieve() {
        let store = CertificateStore::new();
        let signing_key = SigningKey::from_bytes(&[1u8; 32]);

        let cert = AdapterUnlockCertificate::issue(
            "test-adapter".to_string(),
            CertificateClassification::Human { confidence: 0.9 },
            &signing_key,
        )
        .unwrap();

        // Store certificate
        store.store(cert.clone()).await.unwrap();

        // Retrieve certificate
        let retrieved = store.get("test-adapter").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().adapter_id, "test-adapter");
    }

    #[tokio::test]
    async fn test_expired_certificate_not_returned() {
        let store = CertificateStore::new();
        let signing_key = SigningKey::from_bytes(&[1u8; 32]);

        // Create expired certificate
        let mut cert = AdapterUnlockCertificate::issue(
            "test-adapter".to_string(),
            CertificateClassification::Human { confidence: 0.9 },
            &signing_key,
        )
        .unwrap();

        // Set expiry to past
        cert.expires_at = Utc::now() - Duration::hours(1);

        store.store(cert).await.unwrap();

        // Should not retrieve expired certificate
        let retrieved = store.get("test-adapter").await;
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_cleanup_expired() {
        let store = CertificateStore::new();
        let signing_key = SigningKey::from_bytes(&[1u8; 32]);

        // Store valid certificate
        let valid_cert = AdapterUnlockCertificate::issue(
            "valid-adapter".to_string(),
            CertificateClassification::Human { confidence: 0.9 },
            &signing_key,
        )
        .unwrap();
        store.store(valid_cert).await.unwrap();

        // Store expired certificate
        let mut expired_cert = AdapterUnlockCertificate::issue(
            "expired-adapter".to_string(),
            CertificateClassification::Human { confidence: 0.9 },
            &signing_key,
        )
        .unwrap();
        expired_cert.expires_at = Utc::now() - Duration::hours(1);
        store.store(expired_cert).await.unwrap();

        assert_eq!(store.count().await, 2);

        // Cleanup
        let removed = store.cleanup_expired().await.unwrap();
        assert_eq!(removed, 1);
        assert_eq!(store.count().await, 1);

        // Valid certificate should still be there
        assert!(store.get("valid-adapter").await.is_some());
        assert!(store.get("expired-adapter").await.is_none());
    }
}
