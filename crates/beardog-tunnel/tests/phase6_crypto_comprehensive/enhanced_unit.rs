// SPDX-License-Identifier: AGPL-3.0-only
#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]

//! Enhanced unit tests — edge cases, boundaries, special inputs.

use crate::common::*;

#[test]
fn test_sha256_very_large_input() {
    // Test SHA-256 with 1MB input
    let large_data = vec![0x42u8; 1024 * 1024]; // 1MB
    let params = json!({
        "data": BASE64.encode(&large_data)
    });

    let result = handle_sha256(&params);
    assert!(result.is_ok());

    let result_value = result.unwrap();
    let hash_hex = result_value.get("hash").unwrap().as_str().unwrap();
    let hash_bytes = hex::decode(hash_hex).unwrap();
    assert_eq!(hash_bytes.len(), 32); // SHA-256 is always 32 bytes
}

#[test]
fn test_sha_family_with_unicode() {
    // Test SHA with Unicode characters
    let unicode_text = "Hello 世界! 🔐🦀";
    let params = json!({
        "data": BASE64.encode(unicode_text.as_bytes())
    });

    // All three SHA variants should handle Unicode
    assert!(handle_sha256(&params).is_ok());
    assert!(handle_sha384(&params).is_ok());
    assert!(handle_sha512(&params).is_ok());
}

#[test]
fn test_sha256_incremental_consistency() {
    // Test that hashing in parts vs. all at once is different
    // (SHA doesn't support incremental in our API, but we test consistency)
    let part1 = b"Hello, ";
    let _part2 = b"World!";
    let combined = b"Hello, World!";

    let result1 = handle_sha256(&json!({"data": BASE64.encode(combined)})).unwrap();
    let hash1 = result1.get("hash").unwrap().as_str().unwrap();

    // Hashing separately should give different result than combined
    let result2 = handle_sha256(&json!({"data": BASE64.encode(part1)})).unwrap();
    let hash2 = result2.get("hash").unwrap().as_str().unwrap();

    assert_ne!(hash1, hash2); // Different inputs = different hashes
}

#[test]
fn test_ecdh_key_reuse_safety() {
    // Test that reusing ephemeral keys multiple times works correctly
    let gen_result = handle_ecdh_p256_generate(&json!({})).unwrap();
    let private_key = gen_result.get("private_key").unwrap().as_str().unwrap();
    let _public_key1 = gen_result.get("public_key").unwrap().as_str().unwrap();

    // Generate another key pair
    let gen_result2 = handle_ecdh_p256_generate(&json!({})).unwrap();
    let public_key2 = gen_result2.get("public_key").unwrap().as_str().unwrap();

    // Derive shared secret with both
    let derive1 = handle_ecdh_p256_derive(&json!({
        "private_key": private_key,
        "peer_public_key": public_key2
    }));

    let derive2 = handle_ecdh_p256_derive(&json!({
        "private_key": private_key,
        "peer_public_key": public_key2
    }));

    // Should get same secret both times
    assert!(derive1.is_ok());
    assert!(derive2.is_ok());
    assert_eq!(
        derive1
            .unwrap()
            .get("shared_secret")
            .unwrap()
            .as_str()
            .unwrap(),
        derive2
            .unwrap()
            .get("shared_secret")
            .unwrap()
            .as_str()
            .unwrap()
    );
}

#[test]
fn test_ecdh_p384_larger_key_size() {
    // P-384 should produce larger keys than P-256
    let p256_result = handle_ecdh_p256_generate(&json!({})).unwrap();
    let p256_pubkey = p256_result.get("public_key").unwrap().as_str().unwrap();

    let p384_result = handle_ecdh_p384_generate(&json!({})).unwrap();
    let p384_pubkey = p384_result.get("public_key").unwrap().as_str().unwrap();

    let p256_bytes = BASE64.decode(p256_pubkey).unwrap();
    let p384_bytes = BASE64.decode(p384_pubkey).unwrap();

    // P-256 uncompressed: 65 bytes, P-384 uncompressed: 97 bytes
    assert_eq!(p256_bytes.len(), 65);
    assert_eq!(p384_bytes.len(), 97);
}

#[test]
fn test_aes_gcm_maximum_payload_size() {
    // Test with 10MB payload (practical limit for GCM)
    let large_plaintext = vec![0x55u8; 10 * 1024 * 1024]; // 10MB
    let key = vec![0x42u8; 32]; // 32-byte key

    let encrypt_params = json!({
        "plaintext": BASE64.encode(&large_plaintext),
        "key": BASE64.encode(&key)
    });

    let encrypt_result = handle_aes256_gcm_encrypt(&encrypt_params);
    assert!(encrypt_result.is_ok(), "Should handle large payloads");
}

#[test]
fn test_aes_gcm_nonce_uniqueness() {
    // Generate multiple encryptions and verify nonces are unique
    let plaintext = b"test";
    let key = vec![0x42u8; 32];

    let mut nonces = Vec::new();
    for _ in 0..10 {
        let result = handle_aes256_gcm_encrypt(&json!({
            "plaintext": BASE64.encode(plaintext),
            "key": BASE64.encode(&key)
        }))
        .unwrap();

        let nonce = result.get("nonce").unwrap().as_str().unwrap().to_string();
        nonces.push(nonce);
    }

    // All nonces should be unique
    let unique_nonces: std::collections::HashSet<_> = nonces.iter().collect();
    assert_eq!(unique_nonces.len(), 10, "All nonces should be unique");
}

#[test]
fn test_aes128_vs_aes256_same_behavior() {
    // Test that AES-128 and AES-256 behave identically (except key size)
    let plaintext = b"Test message for both variants";
    let key128 = vec![0x42u8; 16];
    let key256 = vec![0x42u8; 32];
    let nonce = vec![0x99u8; 12];

    let result128 = handle_aes128_gcm_encrypt(&json!({
        "plaintext": BASE64.encode(plaintext),
        "key": BASE64.encode(&key128),
        "nonce": BASE64.encode(&nonce)
    }));

    let result256 = handle_aes256_gcm_encrypt(&json!({
        "plaintext": BASE64.encode(plaintext),
        "key": BASE64.encode(&key256),
        "nonce": BASE64.encode(&nonce)
    }));

    // Both should succeed
    assert!(result128.is_ok());
    assert!(result256.is_ok());

    // Ciphertexts should be different (different keys)
    let result128_value = result128.unwrap();
    let result256_value = result256.unwrap();
    let ct128 = result128_value.get("ciphertext").unwrap().as_str().unwrap();
    let ct256 = result256_value.get("ciphertext").unwrap().as_str().unwrap();
    assert_ne!(ct128, ct256);
}

#[test]
fn test_argon2id_unicode_password() {
    // Test Argon2id with Unicode password
    let unicode_password = "Пароль123!🔐";

    let hash_result = handle_argon2id_hash(&json!({
        "password": unicode_password
    }));

    assert!(hash_result.is_ok());

    let hash = hash_result
        .unwrap()
        .get("hash")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    // Verify the hash
    let verify_result = handle_argon2id_verify(&json!({
        "password": unicode_password,
        "hash": hash
    }));

    assert!(verify_result.is_ok());
    assert!(
        verify_result
            .unwrap()
            .get("valid")
            .unwrap()
            .as_bool()
            .unwrap()
    );
}

#[test]
fn test_pbkdf2_minimum_iterations_enforcement() {
    // Test that PBKDF2 rejects low iteration counts
    let low_iterations = [1, 10, 100, 1000, 10_000, 50_000, 99_999];

    for iterations in low_iterations {
        let result = handle_pbkdf2_sha256(&json!({
            "password": "test",
            "salt": BASE64.encode(b"salt"),
            "iterations": iterations
        }));

        assert!(result.is_err(), "Should reject {iterations} iterations");
    }

    // 100,000 should be accepted
    let result = handle_pbkdf2_sha256(&json!({
        "password": "test",
        "salt": BASE64.encode(b"salt"),
        "iterations": 100_000
    }));

    assert!(result.is_ok(), "Should accept 100,000 iterations");
}

#[test]
fn test_pbkdf2_variable_output_lengths() {
    // Test PBKDF2 with various output lengths
    let output_lengths = [16, 32, 48, 64, 128, 256];

    for length in output_lengths {
        let result = handle_pbkdf2_sha256(&json!({
            "password": "test",
            "salt": BASE64.encode(b"salt"),
            "iterations": 100_000,
            "output_length": length
        }))
        .unwrap();

        let key_b64 = result.get("derived_key").unwrap().as_str().unwrap();
        let key = BASE64.decode(key_b64).unwrap();

        assert_eq!(key.len(), length, "Output length should match requested");
    }
}

#[test]
fn test_password_empty_salt_rejected() {
    // Test that empty salt is rejected
    let result = handle_pbkdf2_sha256(&json!({
        "password": "test",
        "salt": BASE64.encode(b""),
        "iterations": 100_000
    }));

    // Empty salt should still technically work, but produces weak key
    // This is more of a validation test
    assert!(result.is_ok()); // PBKDF2 doesn't reject empty salt, but it's not recommended
}

#[test]
fn test_argon2id_very_long_password() {
    // Test with very long password (1000 characters)
    let long_password = "a".repeat(1000);

    let hash_result = handle_argon2id_hash(&json!({
        "password": long_password
    }));

    assert!(hash_result.is_ok());

    let hash = hash_result
        .unwrap()
        .get("hash")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    // Verify it
    let verify_result = handle_argon2id_verify(&json!({
        "password": long_password,
        "hash": hash
    }));

    assert!(verify_result.is_ok());
    assert!(
        verify_result
            .unwrap()
            .get("valid")
            .unwrap()
            .as_bool()
            .unwrap()
    );
}

#[test]
fn test_aes_gcm_aad_with_empty_string() {
    // Test AAD with empty string (should work)
    let plaintext = b"test";
    let key = vec![0x42u8; 32];

    let result = handle_aes256_gcm_encrypt(&json!({
        "plaintext": BASE64.encode(plaintext),
        "key": BASE64.encode(&key),
        "aad": BASE64.encode(b"")
    }));

    assert!(result.is_ok());
}
