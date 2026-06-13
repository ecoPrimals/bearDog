// SPDX-License-Identifier: AGPL-3.0-or-later

//! `RustCrypto` Provider
//!
//! Implementation of `UniversalCryptoProvider` using the `RustCrypto` ecosystem.
//! Split by domain:
//! - `symmetric` — AES-GCM, ChaCha20-Poly1305
//! - `signatures` — Ed25519, ECDSA P-256/P-384
//! - `kdf` — hashing (SHA-2, BLAKE3) and key derivation (HKDF, PBKDF2, Argon2, scrypt)

mod kdf;
mod signatures;
mod symmetric;

use crate::tunnel::hsm::crypto::algorithms::{
    AesMode, Argon2Variant, AsymmetricAlgorithm, CryptoAlgorithm, DecryptionOptions, EncryptedData,
    EncryptionOptions, HashAlgorithm, KdfAlgorithm, Signature, SignatureAlgorithm, SigningOptions,
    SymmetricAlgorithm, VerificationOptions,
};
use crate::tunnel::hsm::crypto::capabilities::{
    CryptoCapabilities, HardwareFeature, PerformanceProfile, Platform, SideChannelResistance,
};
use crate::tunnel::hsm::crypto::provider::UniversalCryptoProvider;
use beardog_errors::BearDogError;
use std::future::Future;

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

            asymmetric_algorithms: vec![],

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
                KdfAlgorithm::Pbkdf2 {
                    hash: HashAlgorithm::Sha256,
                    iterations: 600_000,
                },
                KdfAlgorithm::Argon2 {
                    variant: Argon2Variant::Argon2id,
                },
                KdfAlgorithm::Scrypt {
                    n: 32768,
                    r: 8,
                    p: 1,
                },
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

impl UniversalCryptoProvider for RustCryptoProvider {
    fn provider_name(&self) -> &'static str {
        "RustCrypto"
    }

    fn provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn discover_capabilities(
        &self,
    ) -> impl Future<Output = Result<CryptoCapabilities, BearDogError>> + Send {
        let cap = self.capabilities.clone();
        async move { Ok(cap) }
    }

    fn supports_algorithm(&self, algorithm: &CryptoAlgorithm) -> impl Future<Output = bool> + Send {
        let algorithm = algorithm.clone();
        let caps = self.capabilities.clone();
        async move {
            match &algorithm {
                CryptoAlgorithm::Symmetric(sym) => caps.supports_symmetric(sym),
                CryptoAlgorithm::Signature(sig) => caps.supports_signature(sig),
                CryptoAlgorithm::Hash(hash) => caps.supports_hash(hash),
                _ => false,
            }
        }
    }

    fn encrypt_symmetric(
        &self,
        algorithm: SymmetricAlgorithm,
        key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
    ) -> impl Future<Output = Result<EncryptedData, BearDogError>> + Send {
        let key = key.to_vec();
        let plaintext = plaintext.to_vec();
        let options = options.clone();
        let this = self;
        async move {
            match algorithm {
                SymmetricAlgorithm::Aes {
                    mode: AesMode::Gcm,
                    key_size: 256,
                }
                | SymmetricAlgorithm::Aes256Gcm => {
                    this.encrypt_aes_256_gcm(&key, &plaintext, &options).await
                }
                SymmetricAlgorithm::Aes {
                    mode: AesMode::Gcm,
                    key_size: 128,
                }
                | SymmetricAlgorithm::Aes128Gcm => {
                    this.encrypt_aes_128_gcm(&key, &plaintext, &options).await
                }
                SymmetricAlgorithm::ChaCha20Poly1305 => {
                    this.encrypt_chacha20_poly1305(&key, &plaintext, &options)
                        .await
                }
                _ => Err(BearDogError::unsupported_operation(format!(
                    "RustCrypto doesn't support: {algorithm}"
                ))),
            }
        }
    }

    fn decrypt_symmetric(
        &self,
        algorithm: SymmetricAlgorithm,
        key: &[u8],
        ciphertext: &EncryptedData,
        options: &DecryptionOptions,
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let key = key.to_vec();
        let ciphertext = ciphertext.clone();
        let options = options.clone();
        let this = self;
        async move {
            match algorithm {
                SymmetricAlgorithm::Aes {
                    mode: AesMode::Gcm,
                    key_size: 256,
                }
                | SymmetricAlgorithm::Aes256Gcm => {
                    this.decrypt_aes_256_gcm(&key, &ciphertext, &options).await
                }
                SymmetricAlgorithm::Aes {
                    mode: AesMode::Gcm,
                    key_size: 128,
                }
                | SymmetricAlgorithm::Aes128Gcm => {
                    this.decrypt_aes_128_gcm(&key, &ciphertext, &options).await
                }
                SymmetricAlgorithm::ChaCha20Poly1305 => {
                    this.decrypt_chacha20_poly1305(&key, &ciphertext, &options)
                        .await
                }
                _ => Err(BearDogError::unsupported_operation(format!(
                    "RustCrypto doesn't support: {algorithm}"
                ))),
            }
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

    fn sign(
        &self,
        algorithm: SignatureAlgorithm,
        private_key: &[u8],
        message: &[u8],
        _options: &SigningOptions,
    ) -> impl Future<Output = Result<Signature, BearDogError>> + Send {
        let private_key = private_key.to_vec();
        let message = message.to_vec();
        let this = self;
        async move {
            match algorithm {
                SignatureAlgorithm::Ed25519 => this.sign_ed25519(&private_key, &message).await,
                SignatureAlgorithm::EcdsaP256 { .. } => {
                    this.sign_ecdsa_p256(&private_key, &message).await
                }
                SignatureAlgorithm::EcdsaP384 { .. } => {
                    this.sign_ecdsa_p384(&private_key, &message).await
                }
                _ => Err(BearDogError::unsupported_operation(format!(
                    "RustCrypto doesn't support signing with: {algorithm}"
                ))),
            }
        }
    }

    fn verify(
        &self,
        algorithm: SignatureAlgorithm,
        public_key: &[u8],
        message: &[u8],
        signature: &Signature,
        _options: &VerificationOptions,
    ) -> impl Future<Output = Result<bool, BearDogError>> + Send {
        let public_key = public_key.to_vec();
        let message = message.to_vec();
        let signature = signature.clone();
        let this = self;
        async move {
            match algorithm {
                SignatureAlgorithm::Ed25519 => {
                    this.verify_ed25519(&public_key, &message, &signature).await
                }
                SignatureAlgorithm::EcdsaP256 { .. } => {
                    this.verify_ecdsa_p256(&public_key, &message, &signature)
                        .await
                }
                SignatureAlgorithm::EcdsaP384 { .. } => {
                    this.verify_ecdsa_p384(&public_key, &message, &signature)
                        .await
                }
                _ => Err(BearDogError::unsupported_operation(format!(
                    "RustCrypto doesn't support verification with: {algorithm}"
                ))),
            }
        }
    }

    fn hash(
        &self,
        algorithm: HashAlgorithm,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let data = data.to_vec();
        let this = self;
        async move {
            match algorithm {
                HashAlgorithm::Sha256 => Ok(this.hash_sha256(&data)),
                HashAlgorithm::Sha384 => Ok(this.hash_sha384(&data)),
                HashAlgorithm::Sha512 => Ok(this.hash_sha512(&data)),
                HashAlgorithm::Blake3 => Ok(this.hash_blake3(&data)),
                _ => Err(BearDogError::unsupported_operation(format!(
                    "RustCrypto doesn't support hashing with: {algorithm}"
                ))),
            }
        }
    }

    fn derive_key(
        &self,
        algorithm: KdfAlgorithm,
        input_key: &[u8],
        salt: &[u8],
        info: &[u8],
        output_length: usize,
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let input_key = input_key.to_vec();
        let salt = salt.to_vec();
        let info = info.to_vec();
        let this = self;
        async move {
            match algorithm {
                KdfAlgorithm::HkdfSha256 => {
                    this.derive_hkdf_sha256(&input_key, &salt, &info, output_length)
                }
                KdfAlgorithm::HkdfSha384 => {
                    this.derive_hkdf_sha384(&input_key, &salt, &info, output_length)
                }
                KdfAlgorithm::HkdfSha512 => {
                    this.derive_hkdf_sha512(&input_key, &salt, &info, output_length)
                }
                KdfAlgorithm::Pbkdf2 {
                    ref hash,
                    iterations,
                } => this.derive_pbkdf2(hash, iterations, &input_key, &salt, output_length),
                KdfAlgorithm::Argon2 { ref variant } => {
                    this.derive_argon2(variant, &input_key, &salt, output_length)
                }
                KdfAlgorithm::Scrypt { n, r, p } => {
                    this.derive_scrypt(n, r, p, &input_key, &salt, output_length)
                }
                KdfAlgorithm::Custom { ref name, .. } => Err(BearDogError::unsupported_operation(
                    format!("Custom KDF '{name}' not supported by RustCrypto provider"),
                )),
            }
        }
    }
}
