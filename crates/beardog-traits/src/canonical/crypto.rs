// SPDX-License-Identifier: AGPL-3.0-only

//! Symmetric, signature, and asymmetric algorithm enums plus the legacy [`CryptoProvider`] API.

use super::base::BaseProvider;
use beardog_errors::BearDogError;
use beardog_types::canonical::crypto::{CryptoKeyPair, HashAlgorithm, KeyPairAlgorithm};

/// Supported symmetric ciphers for [`CryptoProvider::encrypt_symmetric`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SymmetricAlgorithm {
    /// Represents aes256 variant
    Aes256,
    /// Represents cha cha20 variant
    ChaCha20,
    /// Represents aes128 variant
    Aes128,
}

/// Digital signature algorithms for [`CryptoProvider::sign_with_key`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SignatureAlgorithm {
    /// Represents ed25519 variant
    Ed25519,
    /// Represents ecdsa p256 variant
    EcdsaP256,
    /// Represents rsa2048 variant
    Rsa2048,
}

/// KEM / encryption algorithms for [`CryptoProvider::encrypt_asymmetric`].
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

/// Software crypto operations used by the canonical provider stack.
pub trait CryptoProvider: BaseProvider {
    /// Fills `length` cryptographically strong random bytes.
    fn generate_random(
        length: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Generates a key pair for the given algorithm.
    fn generate_key_pair(
        algorithm: KeyPairAlgorithm,
    ) -> impl std::future::Future<Output = Result<CryptoKeyPair, BearDogError>> + Send;

    /// Computes a digest over `data` using `algorithm`.
    fn hash_data(
        data: &[u8],
        algorithm: HashAlgorithm,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// PBKDF2-based key derivation.
    fn derive_key_pbkdf2(
        password: &str,
        salt: &[u8],
        iterations: u32,
        key_length: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Symmetrically encrypts `plaintext` with `key`.
    fn encrypt_symmetric(
        plaintext: &[u8],
        key: &[u8],
        algorithm: SymmetricAlgorithm,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Symmetrically decrypts `ciphertext` with `key`.
    fn decrypt_symmetric(
        ciphertext: &[u8],
        key: &[u8],
        algorithm: SymmetricAlgorithm,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Signs `data` with `private_key` using `algorithm`.
    fn sign_with_key(
        data: &[u8],
        private_key: &[u8],
        algorithm: SignatureAlgorithm,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Verifies `signature` over `data` with `public_key`.
    fn verify_with_key(
        data: &[u8],
        signature: &[u8],
        public_key: &[u8],
        algorithm: SignatureAlgorithm,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;

    /// Public-key encrypts `plaintext`.
    fn encrypt_asymmetric(
        plaintext: &[u8],
        public_key: &[u8],
        algorithm: AsymmetricAlgorithm,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Decrypts ciphertext produced by [`Self::encrypt_asymmetric`].
    fn decrypt_asymmetric(
        ciphertext: &[u8],
        private_key: &[u8],
        algorithm: AsymmetricAlgorithm,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;
}
