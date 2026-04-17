// SPDX-License-Identifier: AGPL-3.0-or-later
//! Provider basics tests.

#![cfg(test)]

use super::*;

#[test]
fn test_provider_new() {
    let provider = RustCryptoProvider::new();
    assert_eq!(provider.provider_name(), "RustCrypto");
}

#[test]
fn test_provider_default() {
    let provider = RustCryptoProvider::default();
    assert_eq!(provider.provider_name(), "RustCrypto");
}

#[test]
fn test_provider_name() {
    let provider = RustCryptoProvider::new();
    assert_eq!(provider.provider_name(), "RustCrypto");
}

#[test]
fn test_provider_version() {
    let provider = RustCryptoProvider::new();
    let version = provider.provider_version();
    assert!(!version.is_empty());
    // Should be semantic version format
    assert!(version.contains('.'));
}

#[tokio::test]
async fn test_discover_capabilities() {
    let provider = RustCryptoProvider::new();
    let caps = provider.discover_capabilities().await.unwrap();

    assert_eq!(caps.provider_name, "RustCrypto");
    assert!(!caps.provider_version.is_empty());

    // Should have symmetric algorithms
    assert!(!caps.symmetric_algorithms.is_empty());
    assert!(
        caps.symmetric_algorithms
            .contains(&SymmetricAlgorithm::Aes256Gcm)
    );
    assert!(
        caps.symmetric_algorithms
            .contains(&SymmetricAlgorithm::ChaCha20Poly1305)
    );

    // Should have signature algorithms
    assert!(!caps.signature_algorithms.is_empty());
    assert!(
        caps.signature_algorithms
            .contains(&SignatureAlgorithm::Ed25519)
    );

    // Should have hash algorithms
    assert!(!caps.hash_algorithms.is_empty());
    assert!(caps.hash_algorithms.contains(&HashAlgorithm::Sha256));

    // Should have KDF algorithms
    assert!(!caps.kdf_algorithms.is_empty());
    assert!(caps.kdf_algorithms.contains(&KdfAlgorithm::HkdfSha256));
}

#[tokio::test]
async fn test_supports_algorithm_symmetric() {
    let provider = RustCryptoProvider::new();

    // Supported
    let alg = CryptoAlgorithm::Symmetric(SymmetricAlgorithm::Aes256Gcm);
    assert!(provider.supports_algorithm(&alg).await);

    let alg = CryptoAlgorithm::Symmetric(SymmetricAlgorithm::ChaCha20Poly1305);
    assert!(provider.supports_algorithm(&alg).await);
}

#[tokio::test]
async fn test_supports_algorithm_signature() {
    let provider = RustCryptoProvider::new();

    let alg = CryptoAlgorithm::Signature(SignatureAlgorithm::Ed25519);
    assert!(provider.supports_algorithm(&alg).await);

    let alg = CryptoAlgorithm::Signature(SignatureAlgorithm::EcdsaP256 {
        hash: HashAlgorithm::Sha256,
    });
    assert!(provider.supports_algorithm(&alg).await);
}

#[tokio::test]
async fn test_supports_algorithm_hash() {
    let provider = RustCryptoProvider::new();

    let alg = CryptoAlgorithm::Hash(HashAlgorithm::Sha256);
    assert!(provider.supports_algorithm(&alg).await);

    let alg = CryptoAlgorithm::Hash(HashAlgorithm::Blake3);
    assert!(provider.supports_algorithm(&alg).await);
}

#[tokio::test]
async fn test_supports_algorithm_asymmetric_not_supported() {
    let provider = RustCryptoProvider::new();

    let alg = CryptoAlgorithm::Asymmetric(AsymmetricAlgorithm::EciesP256);
    assert!(!provider.supports_algorithm(&alg).await);
}

#[test]
fn test_provider_clone() {
    let provider1 = RustCryptoProvider::new();
    let provider2 = provider1.clone();
    assert_eq!(provider1.provider_name(), provider2.provider_name());
}
