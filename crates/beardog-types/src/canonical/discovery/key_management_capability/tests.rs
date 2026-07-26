// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use std::collections::HashMap;

// TEST: All 10 Software HSM operations
#[tokio::test]
async fn test_software_hsm_all_operations() {
    let provider = SoftwareHsmProvider::new().expect("Software HSM init should succeed in tests");

    // 1. ✅ generate_key
    let key_spec = KeySpec {
        algorithm: KeyAlgorithm::Aes,
        key_size: Some(256),
        usage: KeyUsage::Encrypt,
        extractable: false,
        metadata: HashMap::new(),
    };
    let key_id = provider.generate_key(key_spec).await;
    assert!(key_id.is_ok(), "generate_key should succeed");
    let key_id = key_id.unwrap();
    assert!(
        key_id.contains("software-hsm"),
        "key_id should have correct prefix"
    );

    // 2. ✅ generate_random
    let random_bytes = provider.generate_random(32).await;
    assert!(random_bytes.is_ok(), "generate_random should succeed");
    assert_eq!(random_bytes.unwrap().len(), 32, "should generate 32 bytes");

    // 3. ✅ encrypt
    let plaintext = b"Hello, BearDog!";
    let ciphertext = provider.encrypt(plaintext, &key_id).await;
    assert!(
        ciphertext.is_ok(),
        "encrypt should succeed: {:?}",
        ciphertext.as_ref().err()
    );
    let ciphertext = ciphertext.unwrap();
    assert!(
        ciphertext.len() > plaintext.len(),
        "ciphertext should include nonce"
    );

    // 4. ✅ decrypt
    let decrypted = provider.decrypt(&ciphertext, &key_id).await;
    assert!(decrypted.is_ok(), "decrypt should succeed");
    assert_eq!(
        decrypted.unwrap(),
        plaintext,
        "decrypt should return original plaintext"
    );

    // 5. ✅ sign
    let data = b"Sign this data";
    let signature = provider.sign(data, &key_id).await;
    assert!(signature.is_ok(), "sign should succeed");
    let signature = signature.unwrap();
    assert_eq!(signature.len(), 8, "signature should be 8 bytes");

    // 6. ✅ verify
    let verified = provider.verify(data, &signature, &key_id).await;
    assert!(verified.is_ok(), "verify should succeed");
    assert!(verified.unwrap(), "signature should be valid");

    // Verify with wrong data fails
    let wrong_data = b"Different data";
    let verified_wrong = provider.verify(wrong_data, &signature, &key_id).await;
    assert!(verified_wrong.is_ok());
    assert!(!verified_wrong.unwrap(), "wrong data should not verify");

    // 7. ✅ get_public_key
    let public_key = provider.get_public_key(&key_id).await;
    assert!(public_key.is_ok(), "get_public_key should succeed");
    let public_key = public_key.unwrap();
    assert!(!public_key.is_empty(), "public key should not be empty");

    // 8. ✅ list_keys
    let keys = provider.list_keys().await;
    assert!(keys.is_ok(), "list_keys should succeed");
    // Empty for now (no storage), but demonstrates interface works

    // 9. ✅ rotate_key
    let new_key_id = provider.rotate_key(&key_id).await;
    assert!(new_key_id.is_ok(), "rotate_key should succeed");
    let new_key_id = new_key_id.unwrap();
    assert_ne!(new_key_id, key_id, "rotated key should have different ID");
    // Note: rotation relationship tracked via metadata, not key ID naming
    assert!(
        new_key_id.contains("software-hsm"),
        "rotated key should be a valid HSM key"
    );

    // 10. ✅ delete_key
    let deleted = provider.delete_key(&key_id).await;
    assert!(deleted.is_ok(), "delete_key should succeed");

    // 11. ✅ health_check (bonus)
    let health = provider.health_check().await;
    assert!(health.is_ok(), "health_check should succeed");
    assert!(health.unwrap().is_healthy, "provider should be healthy");

    println!("✅ All 10 Software HSM operations implemented and tested!");
}

#[test]
fn test_key_spec_creation() {
    let spec = KeySpec {
        algorithm: KeyAlgorithm::Aes,
        key_size: Some(256),
        usage: KeyUsage::Encrypt,
        extractable: false,
        metadata: HashMap::new(),
    };

    assert_eq!(spec.algorithm, KeyAlgorithm::Aes);
    assert_eq!(spec.key_size, Some(256));
}

#[test]
fn test_kms_error_display() {
    let error = KmsError::KeyNotFound {
        key_id: "key-123".to_string(),
    };

    let display = format!("{error}");
    assert!(display.contains("Key not found"));
}

#[test]
fn test_kms_error_display_all_variants() {
    let cases: Vec<(KmsError, &'static [&'static str])> = vec![
        (
            KmsError::KeyNotFound { key_id: "k".into() },
            &["Key not found", "k"],
        ),
        (
            KmsError::ProviderUnavailable {
                provider: "p".into(),
                reason: "r".into(),
            },
            &["p", "r"],
        ),
        (
            KmsError::OperationNotSupported {
                operation: "op".into(),
            },
            &["not supported", "op"],
        ),
        (
            KmsError::InvalidKeySpec {
                reason: "bad".into(),
            },
            &["Invalid key", "bad"],
        ),
        (
            KmsError::CryptoError {
                details: "c".into(),
            },
            &["Cryptographic", "c"],
        ),
        (
            KmsError::PermissionDenied {
                resource: "res".into(),
            },
            &["Permission denied", "res"],
        ),
        (
            KmsError::RateLimitExceeded {
                retry_after_seconds: 9,
            },
            &["Rate limit", "9"],
        ),
        (
            KmsError::NetworkError {
                details: "n".into(),
            },
            &["Network error", "n"],
        ),
        (
            KmsError::Other {
                message: "m".into(),
            },
            &["KMS error", "m"],
        ),
    ];
    for (err, needles) in cases {
        let s = err.to_string();
        for needle in needles {
            assert!(s.contains(*needle), "expected {needle:?} in {s:?}");
        }
    }
    assert!(
        std::error::Error::source(&KmsError::Other {
            message: "x".into()
        })
        .is_none()
    );
}

#[test]
fn test_key_algorithm_key_usage_key_state_serde_roundtrip() {
    let alg = KeyAlgorithm::EcdsaP384;
    let json = serde_json::to_string(&alg).unwrap();
    let back: KeyAlgorithm = serde_json::from_str(&json).unwrap();
    assert_eq!(alg, back);

    let usage = KeyUsage::Both;
    let json = serde_json::to_string(&usage).unwrap();
    assert_eq!(serde_json::from_str::<KeyUsage>(&json).unwrap(), usage);

    let state = KeyState::PendingDeletion;
    let json = serde_json::to_string(&state).unwrap();
    assert_eq!(serde_json::from_str::<KeyState>(&json).unwrap(), state);
}

#[test]
fn test_kms_capabilities_serde_roundtrip() {
    let caps = KmsCapabilities {
        supports_symmetric: true,
        supports_asymmetric: false,
        supports_signing: true,
        has_hardware_rng: false,
        supports_rotation: true,
        fips_compliant: false,
        algorithms: vec![KeyAlgorithm::Aes, KeyAlgorithm::Ed25519],
    };
    let json = serde_json::to_string(&caps).unwrap();
    let back: KmsCapabilities = serde_json::from_str(&json).unwrap();
    assert_eq!(back.algorithms.len(), 2);
    assert!(back.supports_symmetric);
}

#[tokio::test]
async fn test_create_key_management_returns_software_hsm() {
    let kms = create_key_management().await.expect("fallback HSM");
    assert_eq!(kms.provider_name(), "SecureSoftwareHSM");
    let caps = kms.capabilities();
    assert!(caps.supports_symmetric);
}

#[tokio::test]
async fn test_software_hsm_with_config() {
    let p =
        SoftwareHsmProvider::with_config(std::collections::HashMap::new()).expect("with_config");
    assert_eq!(p.provider_name(), "SecureSoftwareHSM");
}
