// SPDX-License-Identifier: AGPL-3.0-or-later
//! Asymmetric encryption tests (not implemented).

#![cfg(test)]

use super::*;

#[tokio::test]
async fn test_encrypt_asymmetric_not_implemented() {
    let provider = RustCryptoProvider::new();
    let alg = AsymmetricAlgorithm::EciesP256;
    let opts = EncryptionOptions::default();

    let result = provider.encrypt_asymmetric(alg, &[], b"test", &opts).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_decrypt_asymmetric_not_implemented() {
    let provider = RustCryptoProvider::new();
    let alg = AsymmetricAlgorithm::EciesP256;
    let encrypted = EncryptedData {
        algorithm: "ECIES-P256".to_string(),
        ciphertext: vec![],
        nonce: None,
        tag: None,
    };
    let opts = DecryptionOptions::default();

    let result = provider
        .decrypt_asymmetric(alg, &[], &encrypted, &opts)
        .await;
    assert!(result.is_err());
}
