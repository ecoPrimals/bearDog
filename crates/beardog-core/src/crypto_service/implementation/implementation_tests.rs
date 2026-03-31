// SPDX-License-Identifier: AGPL-3.0-only

use super::BearDogCryptoService;
use crate::crypto_service::CryptoService;
use crate::crypto_service::algorithms::asymmetric;
use crate::crypto_service::types::CryptoServiceConfig;
use beardog_types::crypto_service::{
    CryptoAlgorithm, DecryptOptions, EncryptOptions, KeyAlgorithm, KeyGenOptions, SignOptions,
    SignatureAlgorithm, VerifyOptions,
};

fn test_config() -> CryptoServiceConfig {
    CryptoServiceConfig {
        service_name: "test-service".to_string(),
        hsm_enabled: false,
        genetic_enabled: true,
        max_data_size: 1024 * 1024,
        audit_enabled: false, // Disable for tests
    }
}

#[tokio::test]
async fn test_encrypt_decrypt_aes256() {
    let service =
        BearDogCryptoService::new(test_config()).expect("BearDogCryptoService::new in test");
    let plaintext = b"Hello, BearDog!";

    let encrypted = service
        .encrypt(
            plaintext,
            CryptoAlgorithm::Aes256Gcm,
            EncryptOptions {
                key_id: "test-key".to_string(),
                associated_data: None,
            },
        )
        .await
        .expect("encrypt in test");

    let decrypted = service
        .decrypt(
            &encrypted,
            DecryptOptions {
                key_id: "test-key".to_string(),
                associated_data: None,
            },
        )
        .await
        .expect("decrypt in test");

    assert_eq!(plaintext, decrypted.as_slice());
}

#[tokio::test]
async fn test_sign_verify_ed25519() {
    let service =
        BearDogCryptoService::new(test_config()).expect("BearDogCryptoService::new in test");
    let data = b"Message to sign";

    // Sign
    let signature = service
        .sign(
            data,
            SignatureAlgorithm::Ed25519,
            SignOptions {
                key_id: "signing-key".to_string(),
                context: None,
            },
        )
        .await
        .expect("sign in test");

    // Get public key (in production, this would be stored/retrieved separately)
    let key = service
        .derive_signing_key("signing-key")
        .expect("derive_signing_key in test");
    let (_secret_key, public_key) =
        asymmetric::generate_ed25519_from_seed(&key).expect("ed25519 from derived key in test");

    // Verify
    let valid = service
        .verify(
            data,
            &signature,
            VerifyOptions {
                public_key: public_key.to_vec(),
                context: None,
            },
        )
        .await
        .expect("verify in test");

    assert!(valid);
}

#[tokio::test]
async fn test_capabilities() {
    let service =
        BearDogCryptoService::new(test_config()).expect("BearDogCryptoService::new in test");

    let caps = service
        .get_capabilities()
        .await
        .expect("get_capabilities in test");

    assert_eq!(caps.service_name, "test-service");
    assert!(!caps.supported_algorithms.is_empty());
    assert!(!caps.features.is_empty());
}

#[tokio::test]
async fn test_health() {
    let service =
        BearDogCryptoService::new(test_config()).expect("BearDogCryptoService::new in test");

    let health = service.get_health().await.expect("get_health in test");

    assert!(health.healthy);
    assert_eq!(health.operations_completed, 0);
}

#[tokio::test]
async fn test_data_size_limit() {
    let service =
        BearDogCryptoService::new(test_config()).expect("BearDogCryptoService::new in test");

    // Create data larger than limit (1MB)
    let large_data = vec![0u8; 2 * 1024 * 1024];

    let result = service
        .encrypt(
            &large_data,
            CryptoAlgorithm::Aes256Gcm,
            EncryptOptions {
                key_id: "test-key".to_string(),
                associated_data: None,
            },
        )
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_encrypt_chacha20_roundtrip() {
    let service =
        BearDogCryptoService::new(test_config()).expect("BearDogCryptoService::new in test");
    let plaintext = b"chacha-payload";
    let enc = service
        .encrypt(
            plaintext,
            CryptoAlgorithm::ChaCha20Poly1305,
            EncryptOptions {
                key_id: "k-chacha".to_string(),
                associated_data: Some(b"ad".to_vec()),
            },
        )
        .await
        .expect("encrypt chacha");
    let dec = service
        .decrypt(
            &enc,
            DecryptOptions {
                key_id: "k-chacha".to_string(),
                associated_data: Some(b"ad".to_vec()),
            },
        )
        .await
        .expect("decrypt chacha");
    assert_eq!(dec, plaintext);
}

#[tokio::test]
async fn test_encrypt_aes128_roundtrip() {
    let service =
        BearDogCryptoService::new(test_config()).expect("BearDogCryptoService::new in test");
    let plaintext = b"aes128";
    let enc = service
        .encrypt(
            plaintext,
            CryptoAlgorithm::Aes128Gcm,
            EncryptOptions {
                key_id: "k-aes128".to_string(),
                associated_data: None,
            },
        )
        .await
        .expect("encrypt aes128");
    let dec = service
        .decrypt(
            &enc,
            DecryptOptions {
                key_id: String::new(),
                associated_data: None,
            },
        )
        .await
        .expect("decrypt uses metadata key_id");
    assert_eq!(dec, plaintext);
}

#[tokio::test]
async fn test_decrypt_requires_tag() {
    let service =
        BearDogCryptoService::new(test_config()).expect("BearDogCryptoService::new in test");
    let mut bad = service
        .encrypt(
            b"x",
            CryptoAlgorithm::Aes256Gcm,
            EncryptOptions {
                key_id: "k".to_string(),
                associated_data: None,
            },
        )
        .await
        .expect("encrypt");
    bad.metadata.tag = None;
    let err = service
        .decrypt(
            &bad,
            DecryptOptions {
                key_id: "k".to_string(),
                associated_data: None,
            },
        )
        .await
        .expect_err("tag required");
    assert!(err.to_string().contains("tag") || err.to_string().contains("Authentication"));
}

#[tokio::test]
async fn test_sign_ecdsa_p256_produces_der_signature() {
    let service =
        BearDogCryptoService::new(test_config()).expect("BearDogCryptoService::new in test");
    let msg = b"ecdsa-msg";
    let sig = service
        .sign(
            msg,
            SignatureAlgorithm::EcdsaP256,
            SignOptions {
                key_id: "ecdsa-key".to_string(),
                context: None,
            },
        )
        .await
        .expect("sign ecdsa");
    assert!(!sig.signature.is_empty());
    assert_eq!(sig.algorithm, SignatureAlgorithm::EcdsaP256);
}

#[tokio::test]
async fn test_generate_rsa_key_invalid_size_errors() {
    let service =
        BearDogCryptoService::new(test_config()).expect("BearDogCryptoService::new in test");
    let err = service
        .generate_rsa_key("bad-rsa", 1024)
        .expect_err("invalid rsa size");
    assert!(err.to_string().contains("2048") || err.to_string().contains("Invalid"));
}

#[tokio::test]
async fn test_generate_rsa_key_2048_stores_public_key() {
    let service =
        BearDogCryptoService::new(test_config()).expect("BearDogCryptoService::new in test");
    let pk = service
        .generate_rsa_key("unit-rsa-2048", 2048)
        .expect("generate rsa 2048");
    assert!(!pk.is_empty());
    let loaded = service
        .get_public_key("unit-rsa-2048")
        .expect("get public rsa");
    assert_eq!(loaded, pk);
}

#[tokio::test]
async fn test_generate_key_with_explicit_key_id() {
    let service =
        BearDogCryptoService::new(test_config()).expect("BearDogCryptoService::new in test");
    let info = service
        .generate_key(
            KeyAlgorithm::Ed25519,
            KeyGenOptions {
                key_id: Some("fixed-id-1".to_string()),
                use_hsm: false,
                use_genetic: false,
                purpose: None,
                ..Default::default()
            },
        )
        .await
        .expect("generate_key");
    assert_eq!(info.key_id, "fixed-id-1");
}

#[tokio::test]
async fn test_get_capabilities_includes_audit_feature_when_enabled() {
    let mut cfg = test_config();
    cfg.audit_enabled = true;
    let service = BearDogCryptoService::new(cfg).expect("BearDogCryptoService::new in test");
    let caps = service.get_capabilities().await.expect("caps");
    assert!(caps.features.iter().any(|f| f == "audit"));
}

#[tokio::test]
async fn test_get_public_key_missing_errors() {
    let service =
        BearDogCryptoService::new(test_config()).expect("BearDogCryptoService::new in test");
    let err = service
        .get_public_key("no-such-key")
        .expect_err("missing pubkey");
    assert!(err.to_string().contains("not found") || err.to_string().contains("Public key"));
}
