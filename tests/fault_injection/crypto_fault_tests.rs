// SPDX-License-Identifier: AGPL-3.0-or-later

//! Crypto fault injection tests — Ed25519, AEAD, signature, and size-limit paths.

use beardog_core::crypto_service::algorithms::asymmetric::verify_ed25519;
use beardog_core::crypto_service::{BearDogCryptoService, CryptoService, CryptoServiceConfig};
use beardog_errors::BearDogError;
use beardog_types::crypto_service::{
    CryptoAlgorithm, DecryptOptions, EncryptOptions, SignOptions, Signature, SignatureAlgorithm,
    SignatureMetadata, VerifyOptions,
};
use std::time::SystemTime;

#[tokio::test]
async fn crypto_fault_invalid_ed25519_key_length_returns_error() {
    let data = b"message";
    let sig = [0u8; 64];
    let short_pk = [0u8; 31];
    let err: BearDogError = verify_ed25519(data, &sig, &short_pk).expect_err("short public key");
    assert!(!err.to_string().is_empty());
}

#[tokio::test]
async fn crypto_fault_tampered_ciphertext_decrypt_returns_error() {
    let mut cfg = CryptoServiceConfig::default();
    cfg.max_data_size = 4096;
    let service = BearDogCryptoService::new(cfg).expect("crypto service init");

    let encrypted = service
        .encrypt(
            b"hello",
            CryptoAlgorithm::Aes256Gcm,
            EncryptOptions {
                key_id: "fault-test-key".to_string(),
                ..Default::default()
            },
        )
        .await
        .expect("encrypt should succeed");

    let mut tampered = encrypted.clone();
    tampered.ciphertext = vec![0xFF; encrypted.ciphertext.len()];

    let bad = service
        .decrypt(
            &tampered,
            DecryptOptions {
                key_id: "fault-test-key".to_string(),
                ..Default::default()
            },
        )
        .await;
    assert!(bad.is_err(), "tampered ciphertext must fail AEAD decrypt");
}

#[tokio::test]
async fn crypto_fault_invalid_signature_length_returns_error() {
    let data = b"signed-payload";
    let sig = Signature {
        signature: vec![0u8; 8],
        algorithm: SignatureAlgorithm::Ed25519,
        metadata: SignatureMetadata {
            timestamp: SystemTime::UNIX_EPOCH,
            key_id: None,
            context: None,
        },
    };
    let service = BearDogCryptoService::new(CryptoServiceConfig::default()).expect("crypto init");
    let v = service
        .verify(
            data,
            &sig,
            VerifyOptions {
                public_key: vec![0u8; 32],
                context: None,
            },
        )
        .await;
    assert!(v.is_err(), "short signature must be rejected");
}

#[tokio::test]
async fn crypto_fault_invalid_signature_returns_false() {
    let seed_b = [9u8; 32];
    let (_, pk_b) =
        beardog_core::crypto_service::algorithms::asymmetric::generate_ed25519_from_seed(&seed_b)
            .expect("keypair B");

    let service = BearDogCryptoService::new(CryptoServiceConfig::default()).expect("crypto init");
    let signature = service
        .sign(
            b"doc",
            SignatureAlgorithm::Ed25519,
            SignOptions {
                key_id: "fault-test-sign".to_string(),
                context: None,
            },
        )
        .await
        .expect("sign");

    let ok = service
        .verify(
            b"doc",
            &signature,
            VerifyOptions {
                public_key: pk_b.to_vec(),
                context: None,
            },
        )
        .await
        .expect("verify completes");
    assert!(!ok, "signature must not verify under unrelated public key");
    assert_eq!(pk_b.len(), 32);
}

#[tokio::test]
async fn crypto_fault_oversized_plaintext_rejected() {
    let mut cfg = CryptoServiceConfig::default();
    cfg.max_data_size = 32;
    let service = BearDogCryptoService::new(cfg).expect("crypto init");
    let big = vec![0u8; 64];
    let err = service
        .encrypt(
            &big,
            CryptoAlgorithm::Aes256Gcm,
            EncryptOptions {
                key_id: "big".to_string(),
                ..Default::default()
            },
        )
        .await;
    assert!(err.is_err(), "oversized input must be rejected");
}
