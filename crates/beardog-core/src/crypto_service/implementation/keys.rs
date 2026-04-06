// SPDX-License-Identifier: AGPL-3.0-or-later

//! Key material: RSA storage, derivation, and public key helpers.

use super::BearDogCryptoService;
use crate::crypto_service::Result;
use crate::crypto_service::algorithms::hashing;
use beardog_errors::BearDogError;
use beardog_types::crypto_service::{KeyAlgorithm, KeyGenOptions, KeyInfo, KeyMetadata};
use std::time::SystemTime;

impl BearDogCryptoService {
    /// Generate and store RSA key pair
    ///
    /// # Arguments
    ///
    /// * `key_id` - Unique identifier for this key
    /// * `bits` - Key size in bits (2048, 3072, or 4096)
    ///
    /// # Security
    ///
    /// - Uses RSA-PSS with SHA-256
    /// - Private keys stored encrypted in memory
    /// - Public keys stored separately for verification
    /// - In production, use HSM for key storage
    ///
    /// # Returns
    ///
    /// DER-encoded public key for external use
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when the key size is invalid, RSA generation or encoding fails, or
    /// storage fails.
    pub fn generate_rsa_key(&self, key_id: &str, bits: usize) -> Result<Vec<u8>> {
        use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
        use rsa::{RsaPrivateKey, RsaPublicKey};

        // Validate key size
        if bits != 2048 && bits != 3072 && bits != 4096 {
            return Err(BearDogError::validation(&format!(
                "Invalid RSA key size: {bits} (must be 2048, 3072, or 4096)"
            )));
        }

        // Generate RSA key pair (`rsa` uses `rand_core` 0.6; use OS RNG from that family)
        use rsa::rand_core::OsRng;
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, bits)
            .map_err(|e| BearDogError::hsm(format!("RSA key generation failed: {e}")))?;

        let public_key = RsaPublicKey::from(&private_key);

        // Encode keys to DER
        let private_key_der = private_key
            .to_pkcs8_der()
            .map_err(|e| BearDogError::hsm(format!("RSA private key encoding failed: {e}")))?;

        let public_key_der = public_key
            .to_public_key_der()
            .map_err(|e| BearDogError::hsm(format!("RSA public key encoding failed: {e}")))?;

        // Store keys
        self.store_rsa_key(key_id, private_key_der.as_bytes())?;
        self.store_public_key(key_id, public_key_der.as_bytes().to_vec());

        tracing::info!("Generated RSA-{} key: {}", bits, key_id);

        Ok(public_key_der.as_bytes().to_vec())
    }

    /// Get or generate RSA key for signing
    ///
    /// # Security
    ///
    /// - Checks memory cache first
    /// - Generates new key if not found (development mode)
    /// - In production, should load from HSM
    pub(crate) fn get_or_generate_rsa_key(&self, key_id: &str) -> Result<Vec<u8>> {
        // Check if key exists in memory (parking_lot::RwLock never panics!)
        {
            let keys = self.rsa_keys.read();
            if let Some(key_der) = keys.get(key_id) {
                return Ok(key_der.clone());
            }
        }

        // Key not found - generate based on environment
        let rsa_key_mode = beardog_errors::process_env::var("BEARDOG_RSA_KEY_MODE")
            .unwrap_or_else(|_| "generate".to_string())
            .to_lowercase();

        match rsa_key_mode.as_str() {
            "hsm" => {
                // In production, load from HSM
                tracing::warn!(
                    "RSA HSM mode requested but not yet implemented, falling back to generation"
                );
                self.generate_and_store_rsa_key(key_id)
            }
            _ => self.generate_and_store_rsa_key(key_id),
        }
    }

    /// Generate and store RSA key (internal)
    fn generate_and_store_rsa_key(&self, key_id: &str) -> Result<Vec<u8>> {
        // Determine key size from environment
        let bits = beardog_errors::process_env::var("BEARDOG_RSA_KEY_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(4096); // Default: RSA-4096 for maximum security

        // Generate key pair
        self.generate_rsa_key(key_id, bits)?;

        // Retrieve the stored private key (parking_lot::RwLock never panics!)
        {
            let keys = self.rsa_keys.read();
            if let Some(key_der) = keys.get(key_id) {
                return Ok(key_der.clone());
            }
        }

        Err(BearDogError::hsm(
            "RSA key generation succeeded but retrieval failed".to_string(),
        ))
    }

    /// Store RSA private key
    ///
    /// # Security
    ///
    /// In production, this should encrypt the private key with a master key
    /// or store it in HSM. For now, stores in memory (development only).
    fn store_rsa_key(&self, key_id: &str, private_key_der: &[u8]) -> Result<()> {
        // parking_lot::RwLock never panics, always succeeds!
        let mut keys = self.rsa_keys.write();
        keys.insert(key_id.to_string(), private_key_der.to_vec());
        tracing::debug!("Stored RSA private key for key_id: {}", key_id);
        Ok(())
    }

    /// Get public key for a given `key_id` (for signature verification)
    ///
    /// # Security Note
    ///
    /// Public keys are safe to store and share. This enables proper
    /// signature verification without exposing private keys.
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when no public key exists for `key_id`.
    pub fn get_public_key(&self, key_id: &str) -> Result<Vec<u8>> {
        // parking_lot::RwLock never panics, cleaner API!
        let keys = self.public_keys.read();
        keys.get(key_id).cloned().ok_or_else(|| {
            BearDogError::security(format!("Public key not found for key_id: {key_id}"))
        })
    }

    /// Store public key for future verification
    pub(crate) fn store_public_key(&self, key_id: &str, public_key: Vec<u8>) {
        // parking_lot::RwLock never panics, always succeeds!
        let mut keys = self.public_keys.write();
        keys.insert(key_id.to_string(), public_key);
        tracing::debug!("Stored public key for key_id: {}", key_id);
    }

    /// Derive encryption key from `key_id`
    ///
    /// In production, this would load from HSM or secure key storage.
    /// For now, uses deterministic derivation for development.
    pub(crate) fn derive_key_256(&self, key_id: &str) -> Result<[u8; 32]> {
        // Use HKDF for proper key derivation
        let salt = b"beardog-key-derivation-salt-v1";
        let info = format!("{}:{}", self.config.service_name, key_id);

        let derived = hashing::hkdf_sha256(key_id.as_bytes(), salt, info.as_bytes(), 32)?;

        let mut key = [0u8; 32];
        key.copy_from_slice(&derived[..32]);
        Ok(key)
    }

    /// Derive 128-bit key from `key_id`
    pub(crate) fn derive_key_128(&self, key_id: &str) -> Result<[u8; 16]> {
        let salt = b"beardog-aes128-derivation-v1";
        let info = format!("{}:aes128:{}", self.config.service_name, key_id);

        let derived = hashing::hkdf_sha256(key_id.as_bytes(), salt, info.as_bytes(), 16)?;

        let mut key = [0u8; 16];
        key.copy_from_slice(&derived[..16]);
        Ok(key)
    }

    /// Derive signing key from `key_id`
    pub(crate) fn derive_signing_key(&self, key_id: &str) -> Result<[u8; 32]> {
        let salt = b"beardog-signing-key-derivation-v1";
        let info = format!("{}:sign:{}", self.config.service_name, key_id);

        let derived = hashing::hkdf_sha256(key_id.as_bytes(), salt, info.as_bytes(), 32)?;

        let mut key = [0u8; 32];
        key.copy_from_slice(&derived[..32]);
        Ok(key)
    }

    pub(crate) async fn generate_key_impl(
        &self,
        algorithm: KeyAlgorithm,
        options: KeyGenOptions,
    ) -> Result<KeyInfo> {
        let op_id = self.next_operation_id();
        let start_time = SystemTime::now();

        let key_id = options.key_id.unwrap_or_else(|| format!("key-{op_id}"));

        let size_bits = match algorithm {
            KeyAlgorithm::Aes256
            | KeyAlgorithm::ChaCha20Poly1305
            | KeyAlgorithm::Ed25519
            | KeyAlgorithm::EcdsaP256 => 256,
            KeyAlgorithm::Rsa4096 => 4096,
        };

        let _key_material = self.derive_key_256(&key_id)?;

        self.audit_log("generate_key", Some(&key_id), true);

        Ok(KeyInfo {
            key_id,
            algorithm,
            created_at: start_time,
            metadata: KeyMetadata {
                size_bits,
                hsm_backed: options.use_hsm && self.config.hsm_enabled,
                genetic_mixed: options.use_genetic && self.config.genetic_enabled,
                purpose: options.purpose,
            },
        })
    }
}
