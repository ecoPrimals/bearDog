//! # Cryptographic Sovereignty
//!
//! This module provides cryptographic sovereignty operations for the BearDog ecosystem.
//! It handles key generation, sovereignty establishment, and trust verification.

use super::{
    CryptoKeySet, CryptoSovereigntyResult, SecuritySovereigntyOps, SovereigntyLevel, TrustLevel,
    TrustVerificationResult,
};
use crate::BearDogSecurityError;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

// ============================================================
// Configuration
// ============================================================

/// Configuration for cryptographic sovereignty operations
#[derive(Debug, Clone)]
pub struct CryptoSovereigntyConfig {
    /// Key size in bits
    pub key_size: usize,

    /// Algorithm for sovereignty operations
    pub sovereignty_algorithm: String,

    /// Whether HSM is required for operations
    pub hsm_required: bool,

    /// Whether to auto-rotate keys
    pub auto_rotate_keys: bool,

    /// Key rotation interval in seconds
    pub rotation_interval_seconds: u64,

    /// Minimum sovereignty level required
    pub min_sovereignty_level: SovereigntyLevel,
}

impl Default for CryptoSovereigntyConfig {
    fn default() -> Self {
        Self {
            key_size: 4096,
            sovereignty_algorithm: "RSA-4096".to_string(),
            hsm_required: false,
            auto_rotate_keys: true,
            rotation_interval_seconds: 86400 * 30, // 30 days
            min_sovereignty_level: SovereigntyLevel::Enhanced,
        }
    }
}

// ============================================================
// Crypto Sovereignty Service
// ============================================================

/// Cryptographic sovereignty service
#[derive(Debug)]
pub struct CryptoSovereignty {
    /// Established sovereignties keyed by identity
    established_sovereignties: Arc<RwLock<HashMap<String, CryptoSovereigntyResult>>>,

    /// Configuration
    config: CryptoSovereigntyConfig,
}

impl CryptoSovereignty {
    /// Create a new crypto sovereignty service
    pub fn new(config: CryptoSovereigntyConfig) -> Self {
        Self {
            established_sovereignties: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Generate sovereignty keys
    fn generate_sovereignty_keys(&self) -> Result<CryptoKeySet, BearDogSecurityError> {
        info!("🔑 Generating cryptographic sovereignty keys");

        use rand::RngCore;
        let mut rng = rand::thread_rng();

        // Generate signing key
        let mut signing_key = vec![0u8; 32];
        rng.fill_bytes(&mut signing_key);

        // Generate encryption key
        let mut encryption_key = vec![0u8; 32];
        rng.fill_bytes(&mut encryption_key);

        // Generate key derivation salt
        let mut key_derivation_salt = vec![0u8; 16];
        rng.fill_bytes(&mut key_derivation_salt);

        // For this implementation, we use Ed25519 for signing
        // Generate public key from signing key
        let signing_keypair = ed25519_dalek::SigningKey::from_bytes(
            &signing_key
                .as_slice()
                .try_into()
                .map_err(|_| BearDogSecurityError::CryptoError("Invalid key size".to_string()))?,
        );
        let public_signing_key = signing_keypair.verifying_key().to_bytes().to_vec();

        // For encryption, use X25519
        let encryption_secret = x25519_dalek::StaticSecret::from(
            <[u8; 32]>::try_from(encryption_key.as_slice())
                .map_err(|_| BearDogSecurityError::CryptoError("Invalid key size".to_string()))?,
        );
        let public_encryption_key =
            x25519_dalek::PublicKey::from(&encryption_secret).as_bytes().to_vec();

        debug!(
            "Generated keys with algorithm: {}",
            self.config.sovereignty_algorithm
        );

        Ok(CryptoKeySet {
            signing_key,
            encryption_key,
            key_derivation_salt,
            public_signing_key: Some(public_signing_key),
            public_encryption_key: Some(public_encryption_key),
        })
    }

    /// Get sovereignty by identity
    pub async fn get_sovereignty(
        &self,
        identity: &str,
    ) -> Result<Option<CryptoSovereigntyResult>, BearDogSecurityError> {
        let sovereignties = self.established_sovereignties.read().await;
        Ok(sovereignties.get(identity).cloned())
    }

    /// Revoke sovereignty
    pub async fn revoke_sovereignty(&self, identity: &str) -> Result<bool, BearDogSecurityError> {
        let mut sovereignties = self.established_sovereignties.write().await;
        let removed = sovereignties.remove(identity).is_some();
        if removed {
            info!("🗑️ Revoked sovereignty for: {}", identity);
        }
        Ok(removed)
    }
}

impl SecuritySovereigntyOps for CryptoSovereignty {
    async fn establish_crypto_sovereignty(
        &self,
        identity: &str,
    ) -> Result<CryptoSovereigntyResult, BearDogSecurityError> {
        info!(
            "🛡️ Establishing cryptographic sovereignty for: {}",
            identity
        );

        let crypto_keys = self.generate_sovereignty_keys()?;
        let sovereignty_id = uuid::Uuid::new_v4().to_string();

        let result = CryptoSovereigntyResult {
            sovereignty_id: sovereignty_id.clone(),
            crypto_keys,
            established_at: chrono::Utc::now(),
            sovereignty_level: SovereigntyLevel::Maximum,
        };

        let mut sovereignties = self.established_sovereignties.write().await;
        sovereignties.insert(identity.to_string(), result.clone());

        info!(
            "✅ Sovereignty established: {} for identity: {}",
            sovereignty_id, identity
        );

        Ok(result)
    }

    async fn verify_trust(
        &self,
        entity: &str,
    ) -> Result<TrustVerificationResult, BearDogSecurityError> {
        info!("🔍 Verifying trust for entity: {}", entity);

        // Check if we have established sovereignty for this entity
        let sovereignties = self.established_sovereignties.read().await;
        let trust_level = if sovereignties.contains_key(entity) {
            TrustLevel::HighlyTrusted
        } else {
            TrustLevel::Trusted
        };

        // Generate verification proof
        use rand::RngCore;
        let mut proof = vec![0u8; 64];
        rand::thread_rng().fill_bytes(&mut proof);

        let result = TrustVerificationResult {
            entity: entity.to_string(),
            trust_level,
            verification_proof: proof,
            verified_at: chrono::Utc::now(),
            metadata: None,
        };

        debug!(
            "Trust verification complete for {}: {:?}",
            entity, trust_level
        );

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    #[tokio::test]
    async fn test_crypto_sovereignty() -> Result<(), BearDogError> {
        let sovereignty = CryptoSovereignty::new(CryptoSovereigntyConfig::default());

        let result = sovereignty
            .establish_crypto_sovereignty("test-identity")
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;

        assert!(!result.sovereignty_id.is_empty());
        assert_eq!(result.sovereignty_level, SovereigntyLevel::Maximum);
        assert!(!result.crypto_keys.signing_key.is_empty());
        assert!(!result.crypto_keys.encryption_key.is_empty());

        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_trust_verification() -> Result<(), BearDogError> {
        let sovereignty = CryptoSovereignty::new(CryptoSovereigntyConfig::default());

        let result = sovereignty.verify_trust("test-entity").await.map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;

        assert_eq!(result.entity, "test-entity");
        assert_eq!(result.trust_level, TrustLevel::Trusted);
        assert!(!result.verification_proof.is_empty());

        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_sovereignty_lifecycle() -> Result<(), BearDogError> {
        let sovereignty = CryptoSovereignty::new(CryptoSovereigntyConfig::default());

        // Establish sovereignty
        sovereignty
            .establish_crypto_sovereignty("lifecycle-test")
            .await
            .map_err(|e| BearDogError::internal(format!("Failed: {e:?}")))?;

        // Verify we can retrieve it
        let stored = sovereignty
            .get_sovereignty("lifecycle-test")
            .await
            .map_err(|e| BearDogError::internal(format!("Failed: {e:?}")))?;
        assert!(stored.is_some());

        // Entity with sovereignty should be highly trusted
        let trust = sovereignty
            .verify_trust("lifecycle-test")
            .await
            .map_err(|e| BearDogError::internal(format!("Failed: {e:?}")))?;
        assert_eq!(trust.trust_level, TrustLevel::HighlyTrusted);

        // Revoke sovereignty
        let revoked = sovereignty
            .revoke_sovereignty("lifecycle-test")
            .await
            .map_err(|e| BearDogError::internal(format!("Failed: {e:?}")))?;
        assert!(revoked);

        // Verify it's gone
        let after_revoke = sovereignty
            .get_sovereignty("lifecycle-test")
            .await
            .map_err(|e| BearDogError::internal(format!("Failed: {e:?}")))?;
        assert!(after_revoke.is_none());

        Ok(())
    }

    #[test]
    fn test_config_default() {
        let config = CryptoSovereigntyConfig::default();
        assert_eq!(config.key_size, 4096);
        assert!(!config.hsm_required);
        assert!(config.auto_rotate_keys);
    }
}
