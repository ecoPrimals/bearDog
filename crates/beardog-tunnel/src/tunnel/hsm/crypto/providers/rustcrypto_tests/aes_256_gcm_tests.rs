// SPDX-License-Identifier: AGPL-3.0-or-later
//! Symmetric encryption tests (AES-256-GCM).

#![cfg(test)]

use super::*;

fn create_test_key_256() -> Vec<u8> {
    vec![0x42; 32] // 256-bit key
}

#[tokio::test]
async fn test_aes_256_gcm_encrypt_decrypt_round_trip() {
    let provider = RustCryptoProvider::new();
    let key = create_test_key_256();
    let plaintext = b"Hello, World! This is a test message.";

    let alg = SymmetricAlgorithm::Aes256Gcm;
    let opts = EncryptionOptions::default();

    // Encrypt
    let encrypted = provider
        .encrypt_symmetric(alg.clone(), &key, plaintext, &opts)
        .await
        .unwrap();
    assert!(!encrypted.ciphertext.is_empty());
    assert!(encrypted.nonce.is_some());
    assert_eq!(encrypted.algorithm, "AES-256-GCM");

    // Decrypt
    let decrypt_opts = DecryptionOptions::default();
    let decrypted = provider
        .decrypt_symmetric(alg, &key, &encrypted, &decrypt_opts)
        .await
        .unwrap();
    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_aes_256_gcm_with_custom_nonce() {
    let provider = RustCryptoProvider::new();
    let key = create_test_key_256();
    let plaintext = b"Custom nonce test";
    let custom_nonce = vec![0x11; 12]; // 96-bit nonce

    let alg = SymmetricAlgorithm::Aes256Gcm;
    let opts = EncryptionOptions {
        nonce: Some(custom_nonce.clone()),
        associated_data: None,
    };

    let encrypted = provider
        .encrypt_symmetric(alg, &key, plaintext, &opts)
        .await
        .unwrap();
    assert_eq!(encrypted.nonce, Some(custom_nonce));
}

#[tokio::test]
async fn test_aes_256_gcm_invalid_key_length() {
    let provider = RustCryptoProvider::new();
    let bad_key = vec![0x42; 16]; // Wrong size (should be 32)
    let plaintext = b"Test";

    let alg = SymmetricAlgorithm::Aes256Gcm;
    let opts = EncryptionOptions::default();

    let result = provider
        .encrypt_symmetric(alg, &bad_key, plaintext, &opts)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_aes_256_gcm_decrypt_without_nonce() {
    let provider = RustCryptoProvider::new();
    let key = create_test_key_256();

    let encrypted = EncryptedData {
        algorithm: "AES-256-GCM".to_string(),
        ciphertext: vec![1, 2, 3],
        nonce: None, // Missing nonce
        tag: None,
    };

    let alg = SymmetricAlgorithm::Aes256Gcm;
    let opts = DecryptionOptions::default();
    let result = provider
        .decrypt_symmetric(alg, &key, &encrypted, &opts)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_aes_256_gcm_tampered_ciphertext() {
    let provider = RustCryptoProvider::new();
    let key = create_test_key_256();
    let plaintext = b"Original message";

    let alg = SymmetricAlgorithm::Aes256Gcm;
    let opts = EncryptionOptions::default();

    // Encrypt
    let mut encrypted = provider
        .encrypt_symmetric(alg.clone(), &key, plaintext, &opts)
        .await
        .unwrap();

    // Tamper with ciphertext
    if !encrypted.ciphertext.is_empty() {
        encrypted.ciphertext[0] ^= 0xFF;
    }

    // Decrypt should fail (authentication failure)
    let decrypt_opts = DecryptionOptions::default();
    let result = provider
        .decrypt_symmetric(alg, &key, &encrypted, &decrypt_opts)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_aes_256_gcm_empty_plaintext() {
    let provider = RustCryptoProvider::new();
    let key = create_test_key_256();
    let plaintext = b"";

    let alg = SymmetricAlgorithm::Aes256Gcm;
    let opts = EncryptionOptions::default();

    let encrypted = provider
        .encrypt_symmetric(alg.clone(), &key, plaintext, &opts)
        .await
        .unwrap();

    let decrypt_opts = DecryptionOptions::default();
    let decrypted = provider
        .decrypt_symmetric(alg, &key, &encrypted, &decrypt_opts)
        .await
        .unwrap();
    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_aes_256_gcm_large_plaintext() {
    let provider = RustCryptoProvider::new();
    let key = create_test_key_256();
    let plaintext = vec![0x42; 10000]; // 10KB

    let alg = SymmetricAlgorithm::Aes256Gcm;
    let opts = EncryptionOptions::default();

    let encrypted = provider
        .encrypt_symmetric(alg.clone(), &key, &plaintext, &opts)
        .await
        .unwrap();

    let decrypt_opts = DecryptionOptions::default();
    let decrypted = provider
        .decrypt_symmetric(alg, &key, &encrypted, &decrypt_opts)
        .await
        .unwrap();
    assert_eq!(decrypted, plaintext);
}
