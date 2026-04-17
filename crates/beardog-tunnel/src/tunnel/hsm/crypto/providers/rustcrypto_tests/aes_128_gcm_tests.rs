// SPDX-License-Identifier: AGPL-3.0-or-later
//! Symmetric encryption tests (AES-128-GCM).

#![cfg(test)]

use super::*;

fn create_test_key_128() -> Vec<u8> {
    vec![0x42; 16] // 128-bit key
}

#[tokio::test]
async fn test_aes_128_gcm_encrypt_decrypt_round_trip() {
    let provider = RustCryptoProvider::new();
    let key = create_test_key_128();
    let plaintext = b"AES-128 test message";

    let alg = SymmetricAlgorithm::Aes128Gcm;
    let opts = EncryptionOptions::default();

    let encrypted = provider
        .encrypt_symmetric(alg.clone(), &key, plaintext, &opts)
        .await
        .unwrap();
    assert_eq!(encrypted.algorithm, "AES-128-GCM");

    let decrypt_opts = DecryptionOptions::default();
    let decrypted = provider
        .decrypt_symmetric(alg, &key, &encrypted, &decrypt_opts)
        .await
        .unwrap();
    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_aes_128_gcm_invalid_key_length() {
    let provider = RustCryptoProvider::new();
    let bad_key = vec![0x42; 32]; // Wrong size (should be 16)
    let plaintext = b"Test";

    let alg = SymmetricAlgorithm::Aes128Gcm;
    let opts = EncryptionOptions::default();

    let result = provider
        .encrypt_symmetric(alg, &bad_key, plaintext, &opts)
        .await;
    assert!(result.is_err());
}
