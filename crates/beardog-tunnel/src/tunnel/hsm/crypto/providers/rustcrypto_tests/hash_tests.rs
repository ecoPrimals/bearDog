// SPDX-License-Identifier: AGPL-3.0-or-later
//! Hash tests.

#![cfg(test)]

use super::*;

#[tokio::test]
async fn test_hash_sha256() {
    let provider = RustCryptoProvider::new();
    let data = b"Hello, World!";

    let hash = provider.hash(HashAlgorithm::Sha256, data).await.unwrap();
    assert_eq!(hash.len(), 32); // SHA-256 produces 32 bytes

    // Same input should produce same hash
    let hash2 = provider.hash(HashAlgorithm::Sha256, data).await.unwrap();
    assert_eq!(hash, hash2);
}

#[tokio::test]
async fn test_hash_sha384() {
    let provider = RustCryptoProvider::new();
    let data = b"SHA-384 test";

    let hash = provider.hash(HashAlgorithm::Sha384, data).await.unwrap();
    assert_eq!(hash.len(), 48); // SHA-384 produces 48 bytes
}

#[tokio::test]
async fn test_hash_sha512() {
    let provider = RustCryptoProvider::new();
    let data = b"SHA-512 test";

    let hash = provider.hash(HashAlgorithm::Sha512, data).await.unwrap();
    assert_eq!(hash.len(), 64); // SHA-512 produces 64 bytes
}

#[tokio::test]
async fn test_hash_blake3() {
    let provider = RustCryptoProvider::new();
    let data = b"BLAKE3 test";

    let hash = provider.hash(HashAlgorithm::Blake3, data).await.unwrap();
    assert_eq!(hash.len(), 32); // BLAKE3 produces 32 bytes by default
}

#[tokio::test]
async fn test_hash_unsupported_algorithm() {
    let provider = RustCryptoProvider::new();
    let data = b"Test";

    let result = provider.hash(HashAlgorithm::Sha3_256, data).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_hash_empty_data() {
    let provider = RustCryptoProvider::new();
    let data = b"";

    let hash = provider.hash(HashAlgorithm::Sha256, data).await.unwrap();
    assert_eq!(hash.len(), 32);
}

#[tokio::test]
async fn test_hash_large_data() {
    let provider = RustCryptoProvider::new();
    let data = vec![0x42; 100000]; // 100KB

    let hash = provider.hash(HashAlgorithm::Sha256, &data).await.unwrap();
    assert_eq!(hash.len(), 32);
}

#[tokio::test]
async fn test_hash_deterministic() {
    let provider = RustCryptoProvider::new();
    let data = b"Determinism test";

    let hash1 = provider.hash(HashAlgorithm::Sha256, data).await.unwrap();
    let hash2 = provider.hash(HashAlgorithm::Sha256, data).await.unwrap();
    let hash3 = provider.hash(HashAlgorithm::Sha256, data).await.unwrap();

    assert_eq!(hash1, hash2);
    assert_eq!(hash2, hash3);
}
