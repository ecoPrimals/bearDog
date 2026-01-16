//! Cryptographic Operations for BTSP
//!
//! This module handles all cryptographic operations for secure tunnels:
//! - Session key derivation using genetic lineage
//! - Encryption and decryption
//! - HSM integration for cryptographic operations

use std::sync::Arc;

use tracing::{debug, warn};
use zeroize::Zeroizing;

use crate::tunnel::hsm::manager::HsmManager;
use crate::tunnel::hsm::{KeyType, KeyMaterial};
use beardog_errors::BearDogError;

// =============================================================================
// Crypto Operations
// =============================================================================

/// Cryptographic operations helper for BTSP tunnels
pub(super) struct CryptoOperations {
    hsm: Arc<HsmManager>,
}

impl CryptoOperations {
    /// Create new crypto operations helper
    pub(super) fn new(hsm: Arc<HsmManager>) -> Self {
        Self { hsm }
    }

    /// Derive session key for tunnel using genetic lineage
    ///
    /// # Arguments
    /// * `peer_id` - Peer identifier
    /// * `tunnel_id` - Tunnel identifier
    ///
    /// # Returns
    /// Derived session key (32 bytes for ChaCha20-Poly1305)
    pub(super) async fn derive_session_key(
        &self,
        peer_id: &str,
        tunnel_id: &str,
    ) -> Result<Zeroizing<Vec<u8>>, BearDogError> {
        debug!(
            peer_id = %peer_id,
            tunnel_id = %tunnel_id,
            "Deriving session key using genetic lineage"
        );

        // Use key derivation context combining peer and tunnel IDs
        let context = format!("btsp-tunnel-{}-{}", peer_id, tunnel_id);
        
        // Derive key using HSM (leverages genetic lineage)
        let key = self.hsm
            .derive_key("tunnel_master", context.as_bytes(), 32)
            .await?;

        // Extract key material
        let session_key = match &key.key_material {
            KeyMaterial::Encrypted { encrypted_data, .. } => {
                // For software HSM, we get encrypted data
                encrypted_data.clone()
            }
            KeyMaterial::Reference { key_reference, .. } => {
                // For hardware HSM with reference, derive from reference
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(key_reference.as_bytes());
                hasher.finalize().to_vec()
            }
            KeyMaterial::HardwareReference { reference, .. } => {
                // Hardware-backed key reference
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(reference.as_bytes());
                hasher.finalize().to_vec()
            }
            KeyMaterial::Handle { key_handle, .. } => {
                // Key handle from HSM
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(key_handle.as_bytes());
                hasher.finalize().to_vec()
            }
        };

        Ok(Zeroizing::new(session_key))
    }

    /// Encrypt data for tunnel using session key
    ///
    /// Uses ChaCha20-Poly1305 AEAD for encryption
    pub(super) fn encrypt_tunnel_data(
        &self,
        data: &[u8],
        session_key: &[u8],
        nonce: &[u8; 12],
    ) -> Result<Vec<u8>, BearDogError> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            ChaCha20Poly1305, Nonce,
        };

        // Create cipher from session key
        let key = chacha20poly1305::Key::from_slice(session_key);
        let cipher = ChaCha20Poly1305::new(key);

        // Encrypt data
        let nonce = Nonce::from_slice(nonce);
        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| BearDogError::encryption(format!("Encryption failed: {}", e)))?;

        debug!(plaintext_len = data.len(), ciphertext_len = ciphertext.len(), "Data encrypted");
        Ok(ciphertext)
    }

    /// Decrypt data from tunnel using session key
    ///
    /// Uses ChaCha20-Poly1305 AEAD for decryption
    pub(super) fn decrypt_tunnel_data(
        &self,
        ciphertext: &[u8],
        session_key: &[u8],
        nonce: &[u8; 12],
    ) -> Result<Vec<u8>, BearDogError> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            ChaCha20Poly1305, Nonce,
        };

        // Create cipher from session key
        let key = chacha20poly1305::Key::from_slice(session_key);
        let cipher = ChaCha20Poly1305::new(key);

        // Decrypt data
        let nonce = Nonce::from_slice(nonce);
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| BearDogError::decryption(format!("Decryption failed: {}", e)))?;

        debug!(ciphertext_len = ciphertext.len(), plaintext_len = plaintext.len(), "Data decrypted");
        Ok(plaintext)
    }

    /// Generate random nonce for encryption
    pub(super) fn generate_nonce(&self) -> [u8; 12] {
        use rand::RngCore;
        let mut nonce = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce);
        nonce
    }

    /// Generate master key for BirdSong
    ///
    /// This creates a master secret key used for BirdSong encrypted discovery
    pub(super) async fn generate_birdsong_key(&self) -> Result<Vec<u8>, BearDogError> {
        debug!("Generating BirdSong master key");

        let key = self.hsm
            .generate_key("birdsong_master", &KeyType::ChaCha20)
            .await?;

        // Extract key material
        let master_secret = match &key.key_material {
            KeyMaterial::Encrypted { encrypted_data, .. } => encrypted_data.clone(),
            KeyMaterial::Reference { key_reference, .. } => {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(key_reference.as_bytes());
                hasher.finalize().to_vec()
            }
            KeyMaterial::HardwareReference { reference, .. } => {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(reference.as_bytes());
                hasher.finalize().to_vec()
            }
            KeyMaterial::Handle { key_handle, .. } => {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(key_handle.as_bytes());
                hasher.finalize().to_vec()
            }
        };

        Ok(master_secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_hsm() -> Arc<HsmManager> {
        Arc::new(HsmManager::new_for_testing())
    }

    #[tokio::test]
    async fn test_session_key_derivation() {
        let hsm = create_test_hsm();
        let crypto = CryptoOperations::new(hsm);

        let key = crypto.derive_session_key("peer-123", "tunnel-1").await.unwrap();
        assert_eq!(key.len(), 32); // ChaCha20 key size
    }

    #[test]
    fn test_encryption_decryption() {
        let hsm = create_test_hsm();
        let crypto = CryptoOperations::new(hsm);

        let session_key = [0u8; 32]; // Test key
        let nonce = crypto.generate_nonce();
        let plaintext = b"Hello, secure tunnel!";

        // Encrypt
        let ciphertext = crypto.encrypt_tunnel_data(plaintext, &session_key, &nonce).unwrap();
        assert_ne!(ciphertext, plaintext);

        // Decrypt
        let decrypted = crypto.decrypt_tunnel_data(&ciphertext, &session_key, &nonce).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_nonce_generation() {
        let hsm = create_test_hsm();
        let crypto = CryptoOperations::new(hsm);

        let nonce1 = crypto.generate_nonce();
        let nonce2 = crypto.generate_nonce();

        assert_eq!(nonce1.len(), 12);
        assert_eq!(nonce2.len(), 12);
        assert_ne!(nonce1, nonce2); // Should be different
    }

    #[tokio::test]
    async fn test_birdsong_key_generation() {
        let hsm = create_test_hsm();
        let crypto = CryptoOperations::new(hsm);

        let key = crypto.generate_birdsong_key().await.unwrap();
        assert!(!key.is_empty());
    }
}

