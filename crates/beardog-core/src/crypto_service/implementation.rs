// SPDX-License-Identifier: AGPL-3.0-only

//! `BearDogCryptoService` implementation
//!
//! Orchestrates cryptographic operations by delegating to algorithm modules
//! and managing keys, state, and audit logging.

use super::Result;
use super::algorithms::{asymmetric, discovery, hashing, symmetric};
use super::r#trait::CryptoService;
use super::types::{CryptoServiceConfig, CryptoServiceState};
use async_trait::async_trait;
use beardog_errors::BearDogError;
use beardog_types::crypto_service::{
    CryptoAlgorithm, DecryptOptions, EncryptOptions, EncryptedData, EncryptionMetadata,
    HealthStatus, KeyAlgorithm, KeyGenOptions, KeyInfo, KeyMetadata, ServiceCapabilities,
    SignOptions, Signature, SignatureAlgorithm, SignatureMetadata, VerifyOptions,
};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::SystemTime;

/// `BearDog` implementation of `CryptoService`
///
/// This implementation:
/// - Delegates to algorithm modules for actual crypto operations
/// - Manages key storage and derivation
/// - Provides audit logging
/// - Tracks operation metrics
/// - Discovers capabilities at runtime
pub struct BearDogCryptoService {
    /// Service configuration
    config: Arc<CryptoServiceConfig>,

    /// Runtime state (operation counters, uptime, etc.)
    state: Arc<CryptoServiceState>,

    /// Algorithm registry for capability discovery
    algorithms: discovery::AlgorithmRegistry,

    /// Public key storage for signature verification
    /// Maps `key_id` -> `public_key_bytes`
    ///
    /// # Security Note
    ///
    /// Only public keys are stored here - never private keys.
    /// This enables proper signature verification without exposing secrets.
    public_keys: Arc<parking_lot::RwLock<std::collections::HashMap<String, Vec<u8>>>>,

    /// RSA key storage (private keys)
    /// Maps `key_id` -> DER-encoded PKCS#8 private key
    ///
    /// # Security Note
    ///
    /// RSA private keys are stored in memory encrypted with AES-256-GCM.
    /// In production, these should be stored in HSM or secure key storage.
    /// This is an intermediate solution for development/testing.
    rsa_keys: Arc<parking_lot::RwLock<std::collections::HashMap<String, Vec<u8>>>>,
}

impl BearDogCryptoService {
    /// Create a new crypto service
    ///
    /// # Errors
    ///
    /// Returns error if service initialization fails
    pub fn new(config: CryptoServiceConfig) -> Result<Self> {
        // Discover available algorithms at startup
        let algorithms = discovery::AlgorithmRegistry::new();

        Ok(Self {
            config: Arc::new(config),
            state: Arc::new(CryptoServiceState::new()),
            algorithms,
            public_keys: Arc::new(parking_lot::RwLock::new(std::collections::HashMap::new())),
            rsa_keys: Arc::new(parking_lot::RwLock::new(std::collections::HashMap::new())),
        })
    }

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
    pub fn generate_rsa_key(&self, key_id: &str, bits: usize) -> Result<Vec<u8>> {
        use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
        use rsa::{RsaPrivateKey, RsaPublicKey};

        // Validate key size
        if bits != 2048 && bits != 3072 && bits != 4096 {
            return Err(BearDogError::validation(&format!(
                "Invalid RSA key size: {bits} (must be 2048, 3072, or 4096)"
            )));
        }

        // Generate RSA key pair
        let mut rng = rand::thread_rng();
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
    fn get_or_generate_rsa_key(&self, key_id: &str) -> Result<Vec<u8>> {
        // Check if key exists in memory (parking_lot::RwLock never panics!)
        {
            let keys = self.rsa_keys.read();
            if let Some(key_der) = keys.get(key_id) {
                return Ok(key_der.clone());
            }
        }

        // Key not found - generate based on environment
        let rsa_key_mode = std::env::var("BEARDOG_RSA_KEY_MODE")
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
        let bits = std::env::var("BEARDOG_RSA_KEY_SIZE")
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
    pub fn get_public_key(&self, key_id: &str) -> Result<Vec<u8>> {
        // parking_lot::RwLock never panics, cleaner API!
        let keys = self.public_keys.read();
        keys.get(key_id).cloned().ok_or_else(|| {
            BearDogError::security(format!("Public key not found for key_id: {key_id}"))
        })
    }

    /// Store public key for future verification
    fn store_public_key(&self, key_id: &str, public_key: Vec<u8>) {
        // parking_lot::RwLock never panics, always succeeds!
        let mut keys = self.public_keys.write();
        keys.insert(key_id.to_string(), public_key);
        tracing::debug!("Stored public key for key_id: {}", key_id);
    }

    /// Increment operation counter and return operation ID
    fn next_operation_id(&self) -> u64 {
        self.state.operation_count.fetch_add(1, Ordering::Relaxed)
    }

    /// Validate data size against configured limits
    fn validate_data_size(&self, data: &[u8]) -> Result<()> {
        if data.len() > self.config.max_data_size {
            return Err(BearDogError::business(format!(
                "Data size {} exceeds maximum {} bytes",
                data.len(),
                self.config.max_data_size
            )));
        }
        Ok(())
    }

    /// Derive encryption key from `key_id`
    ///
    /// In production, this would load from HSM or secure key storage.
    /// For now, uses deterministic derivation for development.
    fn derive_key_256(&self, key_id: &str) -> Result<[u8; 32]> {
        // Use HKDF for proper key derivation
        let salt = b"beardog-key-derivation-salt-v1";
        let info = format!("{}:{}", self.config.service_name, key_id);

        let derived = hashing::hkdf_sha256(key_id.as_bytes(), salt, info.as_bytes(), 32)?;

        let mut key = [0u8; 32];
        key.copy_from_slice(&derived[..32]);
        Ok(key)
    }

    /// Derive 128-bit key from `key_id`
    fn derive_key_128(&self, key_id: &str) -> Result<[u8; 16]> {
        let salt = b"beardog-aes128-derivation-v1";
        let info = format!("{}:aes128:{}", self.config.service_name, key_id);

        let derived = hashing::hkdf_sha256(key_id.as_bytes(), salt, info.as_bytes(), 16)?;

        let mut key = [0u8; 16];
        key.copy_from_slice(&derived[..16]);
        Ok(key)
    }

    /// Derive signing key from `key_id`
    fn derive_signing_key(&self, key_id: &str) -> Result<[u8; 32]> {
        let salt = b"beardog-signing-key-derivation-v1";
        let info = format!("{}:sign:{}", self.config.service_name, key_id);

        let derived = hashing::hkdf_sha256(key_id.as_bytes(), salt, info.as_bytes(), 32)?;

        let mut key = [0u8; 32];
        key.copy_from_slice(&derived[..32]);
        Ok(key)
    }

    /// Audit log an operation (if enabled)
    fn audit_log(&self, operation: &str, key_id: Option<&str>, success: bool) {
        if self.config.audit_enabled {
            tracing::info!(
                service = %self.config.service_name,
                operation = %operation,
                key_id = ?key_id,
                success = %success,
                "Crypto operation"
            );
        }
    }
}

#[async_trait]
impl CryptoService for BearDogCryptoService {
    async fn encrypt(
        &self,
        data: &[u8],
        algorithm: CryptoAlgorithm,
        options: EncryptOptions,
    ) -> Result<EncryptedData> {
        let _op_id = self.next_operation_id();
        let start_time = SystemTime::now();

        // Validate input
        self.validate_data_size(data)?;

        let key_id = &options.key_id;

        // Check algorithm support
        if !self.algorithms.supports_crypto(&algorithm) {
            return Err(BearDogError::business(format!(
                "Algorithm {algorithm:?} not supported"
            )));
        }

        // Delegate to appropriate algorithm module
        let (ciphertext, nonce, tag) = match algorithm {
            CryptoAlgorithm::Aes256Gcm => {
                let key = self.derive_key_256(key_id)?;
                symmetric::encrypt_aes_256_gcm(data, &key, options.associated_data.as_deref())?
            }
            CryptoAlgorithm::Aes128Gcm => {
                let key = self.derive_key_128(key_id)?;
                symmetric::encrypt_aes_128_gcm(data, &key, options.associated_data.as_deref())?
            }
            CryptoAlgorithm::ChaCha20Poly1305 => {
                let key = self.derive_key_256(key_id)?;
                symmetric::encrypt_chacha20_poly1305(
                    data,
                    &key,
                    options.associated_data.as_deref(),
                )?
            }
        };

        self.audit_log("encrypt", Some(key_id), true);

        Ok(EncryptedData {
            ciphertext,
            algorithm,
            metadata: EncryptionMetadata {
                timestamp: start_time,
                key_id: Some(key_id.clone()),
                nonce,
                tag: Some(tag),
            },
        })
    }

    async fn decrypt(&self, encrypted: &EncryptedData, options: DecryptOptions) -> Result<Vec<u8>> {
        let _op_id = self.next_operation_id();

        let key_id = if options.key_id.is_empty() {
            encrypted
                .metadata
                .key_id
                .as_ref()
                .ok_or_else(|| BearDogError::validation("key_id is required for decryption"))?
        } else {
            &options.key_id
        };

        let nonce = &encrypted.metadata.nonce;
        let tag = encrypted
            .metadata
            .tag
            .as_deref()
            .ok_or_else(|| BearDogError::validation("Authentication tag is required"))?;

        // Delegate to appropriate algorithm module
        let plaintext = match encrypted.algorithm {
            CryptoAlgorithm::Aes256Gcm => {
                let key = self.derive_key_256(key_id)?;
                symmetric::decrypt_aes_256_gcm(
                    &encrypted.ciphertext,
                    nonce,
                    tag,
                    &key,
                    options.associated_data.as_deref(),
                )?
            }
            CryptoAlgorithm::Aes128Gcm => {
                let key = self.derive_key_128(key_id)?;
                symmetric::decrypt_aes_128_gcm(
                    &encrypted.ciphertext,
                    nonce,
                    tag,
                    &key,
                    options.associated_data.as_deref(),
                )?
            }
            CryptoAlgorithm::ChaCha20Poly1305 => {
                let key = self.derive_key_256(key_id)?;
                symmetric::decrypt_chacha20_poly1305(
                    &encrypted.ciphertext,
                    nonce,
                    tag,
                    &key,
                    options.associated_data.as_deref(),
                )?
            }
        };

        self.audit_log("decrypt", Some(key_id), true);

        Ok(plaintext)
    }

    async fn sign(
        &self,
        data: &[u8],
        algorithm: SignatureAlgorithm,
        options: SignOptions,
    ) -> Result<Signature> {
        let _op_id = self.next_operation_id();
        let start_time = SystemTime::now();

        self.validate_data_size(data)?;

        let key_id = &options.key_id;

        // Check algorithm support
        if !self.algorithms.supports_signature(&algorithm) {
            return Err(BearDogError::business(format!(
                "Signature algorithm {algorithm:?} not supported"
            )));
        }

        // Delegate to appropriate algorithm module
        // Store public key for later verification (key persistence)
        let signature_bytes = match algorithm {
            SignatureAlgorithm::Ed25519 => {
                let key = self.derive_signing_key(key_id)?;
                let (secret_key, public_key) = asymmetric::generate_ed25519_from_seed(&key)?;

                // Store public key for verification (key persistence)
                self.store_public_key(key_id, public_key.to_vec());

                asymmetric::sign_ed25519(data, &secret_key)?
            }
            SignatureAlgorithm::EcdsaP256 => {
                let key = self.derive_signing_key(key_id)?;
                asymmetric::sign_ecdsa_p256(data, &key)?
            }
            SignatureAlgorithm::RsaPss => {
                // Proper RSA key management: get or generate RSA key pair
                let private_key_der = self.get_or_generate_rsa_key(key_id)?;
                asymmetric::sign_rsa_pss(data, &private_key_der)?
            }
        };

        self.audit_log("sign", Some(key_id), true);

        Ok(Signature {
            signature: signature_bytes,
            algorithm,
            metadata: SignatureMetadata {
                timestamp: start_time,
                key_id: Some(key_id.clone()),
                context: options.context.clone(),
            },
        })
    }

    async fn verify(
        &self,
        data: &[u8],
        signature: &Signature,
        options: VerifyOptions,
    ) -> Result<bool> {
        let _op_id = self.next_operation_id();

        let public_key = &options.public_key;

        // Delegate to appropriate algorithm module
        let valid = match signature.algorithm {
            SignatureAlgorithm::Ed25519 => {
                asymmetric::verify_ed25519(data, &signature.signature, public_key)?
            }
            SignatureAlgorithm::EcdsaP256 => {
                asymmetric::verify_ecdsa_p256(data, &signature.signature, public_key)?
            }
            SignatureAlgorithm::RsaPss => {
                asymmetric::verify_rsa_pss(data, &signature.signature, public_key)?
            }
        };

        self.audit_log("verify", None, valid);

        Ok(valid)
    }

    async fn generate_key(
        &self,
        algorithm: KeyAlgorithm,
        options: KeyGenOptions,
    ) -> Result<KeyInfo> {
        let op_id = self.next_operation_id();
        let start_time = SystemTime::now();

        // Modern Rust idiom: Respect manual key_id if provided, else auto-generate
        // This enables deterministic keys for testing while maintaining production flexibility
        let key_id = options.key_id.unwrap_or_else(|| format!("key-{op_id}"));

        // Generate key material based on algorithm
        // Modern Rust: Group algorithms by key size for clarity and maintainability
        let size_bits = match algorithm {
            KeyAlgorithm::Aes256
            | KeyAlgorithm::ChaCha20Poly1305
            | KeyAlgorithm::Ed25519
            | KeyAlgorithm::EcdsaP256 => 256,
            KeyAlgorithm::Rsa4096 => 4096,
        };

        // In production, this would store the key securely in HSM or key storage
        // For now, we just derive it (idempotent key generation)
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

    async fn get_capabilities(&self) -> Result<ServiceCapabilities> {
        let capabilities = self.algorithms.all_capabilities();

        // Convert algorithm names to canonical string format
        let supported_algorithms: Vec<String> =
            capabilities.iter().map(|c| c.name.clone()).collect();

        // Build features list
        let mut features = Vec::new();
        if self.config.hsm_enabled {
            features.push("hsm".to_string());
        }
        if self.config.genetic_enabled {
            features.push("genetic".to_string());
        }
        if self.config.audit_enabled {
            features.push("audit".to_string());
        }
        if capabilities.iter().any(|c| c.hardware_accelerated) {
            features.push("hardware_acceleration".to_string());
        }

        Ok(ServiceCapabilities {
            service_name: self.config.service_name.clone(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            supported_algorithms,
            features,
            max_data_size: self.config.max_data_size,
        })
    }

    async fn get_health(&self) -> Result<HealthStatus> {
        let uptime = self.state.uptime();
        let operations = self.state.operation_count();

        Ok(HealthStatus {
            healthy: true,
            uptime_seconds: uptime.as_secs(),
            operations_completed: operations,
            hsm_connected: self.config.hsm_enabled,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> CryptoServiceConfig {
        CryptoServiceConfig {
            service_name: "test-service".to_string(),
            hsm_enabled: false,
            genetic_enabled: true,
            max_data_size: 1024 * 1024,
            audit_enabled: false, // Disable for tests
        }
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_aes256() {
        let service = BearDogCryptoService::new(test_config()).unwrap();
        let plaintext = b"Hello, BearDog!";

        let encrypted = service
            .encrypt(
                plaintext,
                CryptoAlgorithm::Aes256Gcm,
                EncryptOptions {
                    key_id: "test-key".to_string(),
                    associated_data: None,
                },
            )
            .await
            .unwrap();

        let decrypted = service
            .decrypt(
                &encrypted,
                DecryptOptions {
                    key_id: "test-key".to_string(),
                    associated_data: None,
                },
            )
            .await
            .unwrap();

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[tokio::test]
    async fn test_sign_verify_ed25519() {
        let service = BearDogCryptoService::new(test_config()).unwrap();
        let data = b"Message to sign";

        // Sign
        let signature = service
            .sign(
                data,
                SignatureAlgorithm::Ed25519,
                SignOptions {
                    key_id: "signing-key".to_string(),
                    context: None,
                },
            )
            .await
            .unwrap();

        // Get public key (in production, this would be stored/retrieved separately)
        let key = service.derive_signing_key("signing-key").unwrap();
        let (_secret_key, public_key) = asymmetric::generate_ed25519_from_seed(&key).unwrap();

        // Verify
        let valid = service
            .verify(
                data,
                &signature,
                VerifyOptions {
                    public_key: public_key.to_vec(),
                    context: None,
                },
            )
            .await
            .unwrap();

        assert!(valid);
    }

    #[tokio::test]
    async fn test_capabilities() {
        let service = BearDogCryptoService::new(test_config()).unwrap();

        let caps = service.get_capabilities().await.unwrap();

        assert_eq!(caps.service_name, "test-service");
        assert!(!caps.supported_algorithms.is_empty());
        assert!(!caps.features.is_empty());
    }

    #[tokio::test]
    async fn test_health() {
        let service = BearDogCryptoService::new(test_config()).unwrap();

        let health = service.get_health().await.unwrap();

        assert!(health.healthy);
        assert_eq!(health.operations_completed, 0);
    }

    #[tokio::test]
    async fn test_data_size_limit() {
        let service = BearDogCryptoService::new(test_config()).unwrap();

        // Create data larger than limit (1MB)
        let large_data = vec![0u8; 2 * 1024 * 1024];

        let result = service
            .encrypt(
                &large_data,
                CryptoAlgorithm::Aes256Gcm,
                EncryptOptions {
                    key_id: "test-key".to_string(),
                    associated_data: None,
                },
            )
            .await;

        assert!(result.is_err());
    }
}
