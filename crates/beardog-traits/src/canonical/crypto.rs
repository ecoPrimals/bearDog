// SPDX-License-Identifier: AGPL-3.0-only

use super::base::BaseProvider;
use beardog_errors::BearDogError;
use beardog_types::canonical::crypto::{CryptoKeyPair, HashAlgorithm, KeyPairAlgorithm};

// Essential crypto algorithm types for canonical trait system
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SymmetricAlgorithm {
    /// Represents aes256 variant
    Aes256,
    /// Represents cha cha20 variant
    ChaCha20,
    /// Represents aes128 variant
    Aes128,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SignatureAlgorithm {
    /// Represents ed25519 variant
    Ed25519,
    /// Represents ecdsa p256 variant
    EcdsaP256,
    /// Represents rsa2048 variant
    Rsa2048,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AsymmetricAlgorithm {
    /// Represents ed25519 variant
    Ed25519,
    /// Represents x25519 variant
    X25519,
    /// Represents ecdsa p256 variant
    EcdsaP256,
    /// Represents rsa2048 variant
    Rsa2048,
}

pub trait CryptoProvider: BaseProvider {
    fn generate_random(
        length: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    fn generate_key_pair(
        algorithm: KeyPairAlgorithm,
    ) -> impl std::future::Future<Output = Result<CryptoKeyPair, BearDogError>> + Send;

    fn hash_data(
        data: &[u8],
        algorithm: HashAlgorithm,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    fn derive_key_pbkdf2(
        password: &str,
        salt: &[u8],
        iterations: u32,
        key_length: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    fn encrypt_symmetric(
        plaintext: &[u8],
        key: &[u8],
        algorithm: SymmetricAlgorithm,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    fn decrypt_symmetric(
        ciphertext: &[u8],
        key: &[u8],
        algorithm: SymmetricAlgorithm,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    fn sign_with_key(
        data: &[u8],
        private_key: &[u8],
        algorithm: SignatureAlgorithm,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    fn verify_with_key(
        data: &[u8],
        signature: &[u8],
        public_key: &[u8],
        algorithm: SignatureAlgorithm,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;

    fn encrypt_asymmetric(
        plaintext: &[u8],
        public_key: &[u8],
        algorithm: AsymmetricAlgorithm,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    fn decrypt_asymmetric(
        ciphertext: &[u8],
        private_key: &[u8],
        algorithm: AsymmetricAlgorithm,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;
}
