// SPDX-License-Identifier: AGPL-3.0-or-later

//! `RustCrypto` Provider
//!
//! Implementation of `UniversalCryptoProvider` using the `RustCrypto` ecosystem.

use crate::tunnel::hsm::crypto::algorithms::{
    AesMode, AsymmetricAlgorithm, CryptoAlgorithm, DecryptionOptions, EncryptedData,
    EncryptionOptions, HashAlgorithm, KdfAlgorithm, Signature, SignatureAlgorithm, SigningOptions,
    SymmetricAlgorithm, VerificationOptions,
};
use crate::tunnel::hsm::crypto::capabilities::{
    CryptoCapabilities, HardwareFeature, PerformanceProfile, Platform, SideChannelResistance,
};
use crate::tunnel::hsm::crypto::provider::{NonceGenerator, UniversalCryptoProvider};
use async_trait::async_trait;
use beardog_errors::BearDogError;

/// `RustCrypto` provider implementation
#[derive(Debug, Clone)]
pub struct RustCryptoProvider {
    capabilities: CryptoCapabilities,
}

impl RustCryptoProvider {
    /// Create a new `RustCrypto` provider
    pub fn new() -> Self {
        Self {
            capabilities: Self::build_capabilities(),
        }
    }

    /// Build the capabilities for `RustCrypto`
    fn build_capabilities() -> CryptoCapabilities {
        CryptoCapabilities {
            provider_name: "RustCrypto".to_string(),
            provider_version: env!("CARGO_PKG_VERSION").to_string(),

            symmetric_algorithms: vec![
                SymmetricAlgorithm::Aes {
                    mode: AesMode::Gcm,
                    key_size: 256,
                },
                SymmetricAlgorithm::Aes {
                    mode: AesMode::Gcm,
                    key_size: 128,
                },
                SymmetricAlgorithm::ChaCha20Poly1305,
                SymmetricAlgorithm::Aes256Gcm,
                SymmetricAlgorithm::Aes128Gcm,
            ],

            asymmetric_algorithms: vec![
                // Future: Add RSA, ECIES support
            ],

            signature_algorithms: vec![
                SignatureAlgorithm::Ed25519,
                SignatureAlgorithm::EcdsaP256 {
                    hash: HashAlgorithm::Sha256,
                },
                SignatureAlgorithm::EcdsaP384 {
                    hash: HashAlgorithm::Sha384,
                },
            ],

            hash_algorithms: vec![
                HashAlgorithm::Sha256,
                HashAlgorithm::Sha384,
                HashAlgorithm::Sha512,
                HashAlgorithm::Blake3,
            ],

            kdf_algorithms: vec![
                KdfAlgorithm::HkdfSha256,
                KdfAlgorithm::HkdfSha384,
                KdfAlgorithm::HkdfSha512,
            ],

            performance_profile: PerformanceProfile {
                throughput_mbps: std::collections::HashMap::from([
                    ("AES-256-GCM".to_string(), 1500.0),
                    ("ChaCha20-Poly1305".to_string(), 2000.0),
                    ("Ed25519".to_string(), 5000.0),
                ]),
                latency_us: std::collections::HashMap::from([
                    ("AES-256-GCM".to_string(), 5.0),
                    ("ChaCha20-Poly1305".to_string(), 3.0),
                    ("Ed25519".to_string(), 50.0),
                ]),
                memory_overhead_bytes: 4096,
            },

            side_channel_resistance: SideChannelResistance::Partial,
            constant_time_ops: vec![
                "AES-256-GCM".to_string(),
                "AES-128-GCM".to_string(),
                "ChaCha20-Poly1305".to_string(),
                "Ed25519".to_string(),
            ],

            supported_platforms: vec![Platform::Linux, Platform::MacOs, Platform::Windows],

            hardware_acceleration: vec![HardwareFeature::AesNi],
        }
    }
}

impl Default for RustCryptoProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UniversalCryptoProvider for RustCryptoProvider {
    fn provider_name(&self) -> &'static str {
        "RustCrypto"
    }

    fn provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    async fn discover_capabilities(&self) -> Result<CryptoCapabilities, BearDogError> {
        Ok(self.capabilities.clone())
    }

    async fn supports_algorithm(&self, algorithm: &CryptoAlgorithm) -> bool {
        match algorithm {
            CryptoAlgorithm::Symmetric(sym) => self.capabilities.supports_symmetric(sym),
            CryptoAlgorithm::Signature(sig) => self.capabilities.supports_signature(sig),
            CryptoAlgorithm::Hash(hash) => self.capabilities.supports_hash(hash),
            _ => false,
        }
    }

    async fn encrypt_symmetric(
        &self,
        algorithm: SymmetricAlgorithm,
        key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
    ) -> Result<EncryptedData, BearDogError> {
        match algorithm {
            SymmetricAlgorithm::Aes {
                mode: AesMode::Gcm,
                key_size: 256,
            }
            | SymmetricAlgorithm::Aes256Gcm => {
                self.encrypt_aes_256_gcm(key, plaintext, options).await
            }
            SymmetricAlgorithm::Aes {
                mode: AesMode::Gcm,
                key_size: 128,
            }
            | SymmetricAlgorithm::Aes128Gcm => {
                self.encrypt_aes_128_gcm(key, plaintext, options).await
            }
            SymmetricAlgorithm::ChaCha20Poly1305 => {
                self.encrypt_chacha20_poly1305(key, plaintext, options)
                    .await
            }
            _ => Err(BearDogError::unsupported_operation(format!(
                "RustCrypto doesn't support: {algorithm}"
            ))),
        }
    }

    async fn decrypt_symmetric(
        &self,
        algorithm: SymmetricAlgorithm,
        key: &[u8],
        ciphertext: &EncryptedData,
        options: &DecryptionOptions,
    ) -> Result<Vec<u8>, BearDogError> {
        match algorithm {
            SymmetricAlgorithm::Aes {
                mode: AesMode::Gcm,
                key_size: 256,
            }
            | SymmetricAlgorithm::Aes256Gcm => {
                self.decrypt_aes_256_gcm(key, ciphertext, options).await
            }
            SymmetricAlgorithm::Aes {
                mode: AesMode::Gcm,
                key_size: 128,
            }
            | SymmetricAlgorithm::Aes128Gcm => {
                self.decrypt_aes_128_gcm(key, ciphertext, options).await
            }
            SymmetricAlgorithm::ChaCha20Poly1305 => {
                self.decrypt_chacha20_poly1305(key, ciphertext, options)
                    .await
            }
            _ => Err(BearDogError::unsupported_operation(format!(
                "RustCrypto doesn't support: {algorithm}"
            ))),
        }
    }

    async fn encrypt_asymmetric(
        &self,
        algorithm: AsymmetricAlgorithm,
        _public_key: &[u8],
        _plaintext: &[u8],
        _options: &EncryptionOptions,
    ) -> Result<EncryptedData, BearDogError> {
        Err(BearDogError::unsupported_operation(format!(
            "RustCrypto asymmetric encryption not yet implemented: {algorithm}"
        )))
    }

    async fn decrypt_asymmetric(
        &self,
        algorithm: AsymmetricAlgorithm,
        _private_key: &[u8],
        _ciphertext: &EncryptedData,
        _options: &DecryptionOptions,
    ) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::unsupported_operation(format!(
            "RustCrypto asymmetric decryption not yet implemented: {algorithm}"
        )))
    }

    async fn sign(
        &self,
        algorithm: SignatureAlgorithm,
        private_key: &[u8],
        message: &[u8],
        _options: &SigningOptions,
    ) -> Result<Signature, BearDogError> {
        match algorithm {
            SignatureAlgorithm::Ed25519 => self.sign_ed25519(private_key, message).await,
            SignatureAlgorithm::EcdsaP256 { .. } => {
                self.sign_ecdsa_p256(private_key, message).await
            }
            _ => Err(BearDogError::unsupported_operation(format!(
                "RustCrypto doesn't support signing with: {algorithm}"
            ))),
        }
    }

    async fn verify(
        &self,
        algorithm: SignatureAlgorithm,
        public_key: &[u8],
        message: &[u8],
        signature: &Signature,
        _options: &VerificationOptions,
    ) -> Result<bool, BearDogError> {
        match algorithm {
            SignatureAlgorithm::Ed25519 => {
                self.verify_ed25519(public_key, message, signature).await
            }
            SignatureAlgorithm::EcdsaP256 { .. } => {
                self.verify_ecdsa_p256(public_key, message, signature).await
            }
            _ => Err(BearDogError::unsupported_operation(format!(
                "RustCrypto doesn't support verification with: {algorithm}"
            ))),
        }
    }

    async fn hash(&self, algorithm: HashAlgorithm, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        match algorithm {
            HashAlgorithm::Sha256 => Ok(self.hash_sha256(data)),
            HashAlgorithm::Sha384 => Ok(self.hash_sha384(data)),
            HashAlgorithm::Sha512 => Ok(self.hash_sha512(data)),
            HashAlgorithm::Blake3 => Ok(self.hash_blake3(data)),
            _ => Err(BearDogError::unsupported_operation(format!(
                "RustCrypto doesn't support hashing with: {algorithm}"
            ))),
        }
    }

    async fn derive_key(
        &self,
        algorithm: KdfAlgorithm,
        input_key: &[u8],
        salt: &[u8],
        info: &[u8],
        output_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        match algorithm {
            KdfAlgorithm::HkdfSha256 => {
                self.derive_hkdf_sha256(input_key, salt, info, output_length)
            }
            KdfAlgorithm::HkdfSha384 => {
                self.derive_hkdf_sha384(input_key, salt, info, output_length)
            }
            KdfAlgorithm::HkdfSha512 => {
                self.derive_hkdf_sha512(input_key, salt, info, output_length)
            }
            _ => Err(BearDogError::unsupported_operation(format!(
                "RustCrypto doesn't support KDF with: {algorithm}"
            ))),
        }
    }
}

// Implementation helpers
impl RustCryptoProvider {
    /// Encrypt using AES-256-GCM
    async fn encrypt_aes_256_gcm(
        &self,
        key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
    ) -> Result<EncryptedData, BearDogError> {
        use aes_gcm::aead::Aead;
        use aes_gcm::{Aes256Gcm, KeyInit, Nonce};

        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| BearDogError::crypto_error(format!("Invalid AES-256 key: {e}")))?;

        let nonce_vec = options
            .nonce
            .clone()
            .unwrap_or_else(|| self.generate_nonce(12));
        let nonce = Nonce::from_slice(&nonce_vec);

        let ciphertext = cipher.encrypt(nonce, plaintext).map_err(|e| {
            BearDogError::crypto_error(format!("AES-256-GCM encryption failed: {e}"))
        })?;

        Ok(EncryptedData {
            algorithm: "AES-256-GCM".to_string(),
            ciphertext,
            nonce: Some(nonce_vec),
            tag: None, // GCM includes auth tag in ciphertext
        })
    }

    /// Decrypt using AES-256-GCM
    async fn decrypt_aes_256_gcm(
        &self,
        key: &[u8],
        encrypted: &EncryptedData,
        _options: &DecryptionOptions,
    ) -> Result<Vec<u8>, BearDogError> {
        use aes_gcm::aead::Aead;
        use aes_gcm::{Aes256Gcm, KeyInit, Nonce};

        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| BearDogError::crypto_error(format!("Invalid AES-256 key: {e}")))?;

        let nonce_vec = encrypted
            .nonce
            .as_ref()
            .ok_or_else(|| BearDogError::crypto_error("Missing nonce for AES-256-GCM"))?;
        let nonce = Nonce::from_slice(nonce_vec);

        let plaintext = cipher
            .decrypt(nonce, encrypted.ciphertext.as_ref())
            .map_err(|e| {
                BearDogError::crypto_error(format!("AES-256-GCM decryption failed: {e}"))
            })?;

        Ok(plaintext)
    }

    /// Encrypt using AES-128-GCM
    async fn encrypt_aes_128_gcm(
        &self,
        key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
    ) -> Result<EncryptedData, BearDogError> {
        use aes_gcm::aead::Aead;
        use aes_gcm::{Aes128Gcm, KeyInit, Nonce};

        let cipher = Aes128Gcm::new_from_slice(key)
            .map_err(|e| BearDogError::crypto_error(format!("Invalid AES-128 key: {e}")))?;

        let nonce_vec = options
            .nonce
            .clone()
            .unwrap_or_else(|| self.generate_nonce(12));
        let nonce = Nonce::from_slice(&nonce_vec);

        let ciphertext = cipher.encrypt(nonce, plaintext).map_err(|e| {
            BearDogError::crypto_error(format!("AES-128-GCM encryption failed: {e}"))
        })?;

        Ok(EncryptedData {
            algorithm: "AES-128-GCM".to_string(),
            ciphertext,
            nonce: Some(nonce_vec),
            tag: None,
        })
    }

    /// Decrypt using AES-128-GCM
    async fn decrypt_aes_128_gcm(
        &self,
        key: &[u8],
        encrypted: &EncryptedData,
        _options: &DecryptionOptions,
    ) -> Result<Vec<u8>, BearDogError> {
        use aes_gcm::aead::Aead;
        use aes_gcm::{Aes128Gcm, KeyInit, Nonce};

        let cipher = Aes128Gcm::new_from_slice(key)
            .map_err(|e| BearDogError::crypto_error(format!("Invalid AES-128 key: {e}")))?;

        let nonce_vec = encrypted
            .nonce
            .as_ref()
            .ok_or_else(|| BearDogError::crypto_error("Missing nonce for AES-128-GCM"))?;
        let nonce = Nonce::from_slice(nonce_vec);

        let plaintext = cipher
            .decrypt(nonce, encrypted.ciphertext.as_ref())
            .map_err(|e| {
                BearDogError::crypto_error(format!("AES-128-GCM decryption failed: {e}"))
            })?;

        Ok(plaintext)
    }

    /// Encrypt using ChaCha20-Poly1305
    async fn encrypt_chacha20_poly1305(
        &self,
        key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
    ) -> Result<EncryptedData, BearDogError> {
        use chacha20poly1305::aead::Aead;
        use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Nonce};

        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| BearDogError::crypto_error(format!("Invalid ChaCha20 key: {e}")))?;

        let nonce_vec = options
            .nonce
            .clone()
            .unwrap_or_else(|| self.generate_nonce(12));
        let nonce = Nonce::from_slice(&nonce_vec);

        let ciphertext = cipher.encrypt(nonce, plaintext).map_err(|e| {
            BearDogError::crypto_error(format!("ChaCha20-Poly1305 encryption failed: {e}"))
        })?;

        Ok(EncryptedData {
            algorithm: "ChaCha20-Poly1305".to_string(),
            ciphertext,
            nonce: Some(nonce_vec),
            tag: None,
        })
    }

    /// Decrypt using ChaCha20-Poly1305
    async fn decrypt_chacha20_poly1305(
        &self,
        key: &[u8],
        encrypted: &EncryptedData,
        _options: &DecryptionOptions,
    ) -> Result<Vec<u8>, BearDogError> {
        use chacha20poly1305::aead::Aead;
        use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Nonce};

        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| BearDogError::crypto_error(format!("Invalid ChaCha20 key: {e}")))?;

        let nonce_vec = encrypted
            .nonce
            .as_ref()
            .ok_or_else(|| BearDogError::crypto_error("Missing nonce for ChaCha20-Poly1305"))?;
        let nonce = Nonce::from_slice(nonce_vec);

        let plaintext = cipher
            .decrypt(nonce, encrypted.ciphertext.as_ref())
            .map_err(|e| {
                BearDogError::crypto_error(format!("ChaCha20-Poly1305 decryption failed: {e}"))
            })?;

        Ok(plaintext)
    }

    /// Sign using Ed25519
    async fn sign_ed25519(
        &self,
        private_key: &[u8],
        message: &[u8],
    ) -> Result<Signature, BearDogError> {
        use ed25519_dalek::{Signer, SigningKey};

        let signing_key = SigningKey::from_bytes(
            private_key
                .try_into()
                .map_err(|_| BearDogError::crypto_error("Invalid Ed25519 private key length"))?,
        );

        let signature = signing_key.sign(message);

        Ok(Signature {
            algorithm: "Ed25519".to_string(),
            signature: signature.to_bytes().to_vec(),
        })
    }

    /// Verify using Ed25519
    async fn verify_ed25519(
        &self,
        key_material: &[u8],
        message: &[u8],
        signature: &Signature,
    ) -> Result<bool, BearDogError> {
        use ed25519_dalek::{Signature as Ed25519Signature, SigningKey, Verifier, VerifyingKey};

        // If we have 32 bytes, it's a private key - derive the public key
        // If we have 64 bytes, it's a public key directly
        let verifying_key = if key_material.len() == 32 {
            // Derive public key from private key
            let signing_key =
                SigningKey::from_bytes(key_material.try_into().map_err(|_| {
                    BearDogError::crypto_error("Invalid Ed25519 private key length")
                })?);
            signing_key.verifying_key()
        } else {
            // Use public key directly
            VerifyingKey::from_bytes(
                key_material
                    .try_into()
                    .map_err(|_| BearDogError::crypto_error("Invalid Ed25519 public key length"))?,
            )
            .map_err(|e| BearDogError::crypto_error(format!("Invalid Ed25519 public key: {e}")))?
        };

        let sig = Ed25519Signature::from_bytes(
            signature
                .signature
                .as_slice()
                .try_into()
                .map_err(|_| BearDogError::crypto_error("Invalid Ed25519 signature length"))?,
        );

        Ok(verifying_key.verify(message, &sig).is_ok())
    }

    /// Sign using ECDSA P-256
    async fn sign_ecdsa_p256(
        &self,
        private_key: &[u8],
        message: &[u8],
    ) -> Result<Signature, BearDogError> {
        use p256::ecdsa::{SigningKey, signature::Signer};

        // Parse the private key
        let signing_key = SigningKey::from_bytes(private_key.into()).map_err(|e| {
            beardog_errors::crypto_error(
                "sign_ecdsa_p256",
                &format!("Invalid P-256 private key format: {e}"),
            )
        })?;

        // Sign the message
        let signature: p256::ecdsa::Signature = signing_key.sign(message);

        Ok(Signature {
            algorithm: "ECDSA-P256-SHA256".to_string(),
            signature: signature.to_bytes().to_vec(),
        })
    }

    /// Verify using ECDSA P-256
    async fn verify_ecdsa_p256(
        &self,
        public_key: &[u8],
        message: &[u8],
        signature: &Signature,
    ) -> Result<bool, BearDogError> {
        use p256::ecdsa::{VerifyingKey, signature::Verifier};

        // Parse the public key (SEC1 encoded point)
        let verifying_key = VerifyingKey::from_sec1_bytes(public_key).map_err(|e| {
            beardog_errors::crypto_error(
                "verify_ecdsa_p256",
                &format!("Invalid P-256 public key format: {e}"),
            )
        })?;

        // Parse the signature
        let sig = p256::ecdsa::Signature::from_bytes(signature.signature.as_slice().into())
            .map_err(|e| {
                beardog_errors::crypto_error(
                    "verify_ecdsa_p256",
                    &format!("Invalid signature format: {e}"),
                )
            })?;

        // Verify the signature
        Ok(verifying_key.verify(message, &sig).is_ok())
    }

    /// Hash using SHA-256
    fn hash_sha256(&self, data: &[u8]) -> Vec<u8> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Hash using SHA-384
    fn hash_sha384(&self, data: &[u8]) -> Vec<u8> {
        use sha2::{Digest, Sha384};
        let mut hasher = Sha384::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Hash using SHA-512
    fn hash_sha512(&self, data: &[u8]) -> Vec<u8> {
        use sha2::{Digest, Sha512};
        let mut hasher = Sha512::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Hash using BLAKE3
    fn hash_blake3(&self, data: &[u8]) -> Vec<u8> {
        blake3::hash(data).as_bytes().to_vec()
    }

    /// Derive key using HKDF-SHA256
    fn derive_hkdf_sha256(
        &self,
        input_key: &[u8],
        salt: &[u8],
        info: &[u8],
        output_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        use hkdf::Hkdf;
        use sha2::Sha256;

        let hkdf = Hkdf::<Sha256>::new(Some(salt), input_key);
        let mut output = vec![0u8; output_length];
        hkdf.expand(info, &mut output)
            .map_err(|e| BearDogError::crypto_error(format!("HKDF expansion failed: {e}")))?;

        Ok(output)
    }

    /// Derive key using HKDF-SHA384
    fn derive_hkdf_sha384(
        &self,
        input_key: &[u8],
        salt: &[u8],
        info: &[u8],
        output_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        use hkdf::Hkdf;
        use sha2::Sha384;

        let hkdf = Hkdf::<Sha384>::new(Some(salt), input_key);
        let mut output = vec![0u8; output_length];
        hkdf.expand(info, &mut output)
            .map_err(|e| BearDogError::crypto_error(format!("HKDF expansion failed: {e}")))?;

        Ok(output)
    }

    /// Derive key using HKDF-SHA512
    fn derive_hkdf_sha512(
        &self,
        input_key: &[u8],
        salt: &[u8],
        info: &[u8],
        output_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        use hkdf::Hkdf;
        use sha2::Sha512;

        let hkdf = Hkdf::<Sha512>::new(Some(salt), input_key);
        let mut output = vec![0u8; output_length];
        hkdf.expand(info, &mut output)
            .map_err(|e| BearDogError::crypto_error(format!("HKDF expansion failed: {e}")))?;

        Ok(output)
    }
}
