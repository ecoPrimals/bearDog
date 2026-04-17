// SPDX-License-Identifier: AGPL-3.0-or-later
//! Symmetric encryption tests (ChaCha20-Poly1305).

#![cfg(test)]

use super::*;

fn create_test_key_chacha() -> Vec<u8> {
    vec![0x42; 32] // 256-bit key
}

#[tokio::test]
async fn test_chacha20_encrypt_decrypt_round_trip() {
    let provider = RustCryptoProvider::new();
    let key = create_test_key_chacha();
    let plaintext = b"ChaCha20-Poly1305 test";

    let alg = SymmetricAlgorithm::ChaCha20Poly1305;
    let opts = EncryptionOptions::default();

    let encrypted = provider
        .encrypt_symmetric(alg.clone(), &key, plaintext, &opts)
        .await
        .unwrap();
    assert_eq!(encrypted.algorithm, "ChaCha20-Poly1305");

    let decrypt_opts = DecryptionOptions::default();
    let decrypted = provider
        .decrypt_symmetric(alg, &key, &encrypted, &decrypt_opts)
        .await
        .unwrap();
    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_chacha20_invalid_key_length() {
    let provider = RustCryptoProvider::new();
    let bad_key = vec![0x42; 16]; // Wrong size (should be 32)
    let plaintext = b"Test";

    let alg = SymmetricAlgorithm::ChaCha20Poly1305;
    let opts = EncryptionOptions::default();

    let result = provider
        .encrypt_symmetric(alg, &bad_key, plaintext, &opts)
        .await;
    assert!(result.is_err());
}
