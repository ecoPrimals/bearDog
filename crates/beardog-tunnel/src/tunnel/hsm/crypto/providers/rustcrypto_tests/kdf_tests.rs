// SPDX-License-Identifier: AGPL-3.0-or-later
//! KDF tests (HKDF).

#![cfg(test)]

use super::*;

#[tokio::test]
async fn test_hkdf_sha256() {
    let provider = RustCryptoProvider::new();
    let input_key = b"input key material";
    let salt = b"salt value";
    let info = b"context info";
    let output_length = 32;

    let derived = provider
        .derive_key(
            KdfAlgorithm::HkdfSha256,
            input_key,
            salt,
            info,
            output_length,
        )
        .await
        .unwrap();

    assert_eq!(derived.len(), output_length);
}

#[tokio::test]
async fn test_hkdf_sha384() {
    let provider = RustCryptoProvider::new();
    let input_key = b"input key material";
    let salt = b"salt value";
    let info = b"context info";
    let output_length = 48;

    let derived = provider
        .derive_key(
            KdfAlgorithm::HkdfSha384,
            input_key,
            salt,
            info,
            output_length,
        )
        .await
        .unwrap();

    assert_eq!(derived.len(), output_length);
}

#[tokio::test]
async fn test_hkdf_sha512() {
    let provider = RustCryptoProvider::new();
    let input_key = b"input key material";
    let salt = b"salt value";
    let info = b"context info";
    let output_length = 64;

    let derived = provider
        .derive_key(
            KdfAlgorithm::HkdfSha512,
            input_key,
            salt,
            info,
            output_length,
        )
        .await
        .unwrap();

    assert_eq!(derived.len(), output_length);
}

#[tokio::test]
async fn test_hkdf_deterministic() {
    let provider = RustCryptoProvider::new();
    let input_key = b"input key material";
    let salt = b"salt value";
    let info = b"context info";
    let output_length = 32;

    let derived1 = provider
        .derive_key(
            KdfAlgorithm::HkdfSha256,
            input_key,
            salt,
            info,
            output_length,
        )
        .await
        .unwrap();

    let derived2 = provider
        .derive_key(
            KdfAlgorithm::HkdfSha256,
            input_key,
            salt,
            info,
            output_length,
        )
        .await
        .unwrap();

    assert_eq!(derived1, derived2);
}

#[tokio::test]
async fn test_hkdf_different_info_produces_different_keys() {
    let provider = RustCryptoProvider::new();
    let input_key = b"input key material";
    let salt = b"salt value";
    let info1 = b"context info 1";
    let info2 = b"context info 2";
    let output_length = 32;

    let derived1 = provider
        .derive_key(
            KdfAlgorithm::HkdfSha256,
            input_key,
            salt,
            info1,
            output_length,
        )
        .await
        .unwrap();

    let derived2 = provider
        .derive_key(
            KdfAlgorithm::HkdfSha256,
            input_key,
            salt,
            info2,
            output_length,
        )
        .await
        .unwrap();

    assert_ne!(derived1, derived2);
}

#[tokio::test]
async fn test_hkdf_empty_salt() {
    let provider = RustCryptoProvider::new();
    let input_key = b"input key material";
    let salt = b"";
    let info = b"context info";
    let output_length = 32;

    let derived = provider
        .derive_key(
            KdfAlgorithm::HkdfSha256,
            input_key,
            salt,
            info,
            output_length,
        )
        .await
        .unwrap();

    assert_eq!(derived.len(), output_length);
}

#[tokio::test]
async fn test_kdf_unsupported_algorithm() {
    let provider = RustCryptoProvider::new();
    let input_key = b"test";
    let salt = b"salt";
    let info = b"info";

    // PBKDF2 with Blake3 hash is not supported
    let result = provider
        .derive_key(
            KdfAlgorithm::Pbkdf2 {
                hash: HashAlgorithm::Blake3,
                iterations: 10000,
            },
            input_key,
            salt,
            info,
            32,
        )
        .await;

    assert!(result.is_err());
}
