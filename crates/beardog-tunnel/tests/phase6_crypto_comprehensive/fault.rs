// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]

//! Fault injection — invalid inputs, tampering, and recovery.

use crate::common::*;

#[test]
fn test_fault_corrupted_base64_sha256() {
    // Test with invalid base64
    let result = handle_sha256(&json!({
        "data": "Not@Valid#Base64!"
    }));

    assert!(result.is_err());
}

#[test]
fn test_fault_missing_required_parameters() {
    // Test missing parameters for various operations

    // SHA missing data
    assert!(handle_sha256(&json!({})).is_err());

    // ECDH derive missing private_key
    assert!(
        handle_ecdh_p256_derive(&json!({
            "peer_public_key": "somekey"
        }))
        .is_err()
    );

    // AES encrypt missing key
    assert!(
        handle_aes256_gcm_encrypt(&json!({
            "plaintext": BASE64.encode(b"test")
        }))
        .is_err()
    );

    // Password hash missing password
    assert!(handle_argon2id_hash(&json!({})).is_err());
}

#[test]
fn test_fault_invalid_key_sizes() {
    // Test with wrong key sizes

    // AES-256 with 16-byte key (should fail)
    let result = handle_aes256_gcm_encrypt(&json!({
        "plaintext": BASE64.encode(b"test"),
        "key": BASE64.encode(vec![0u8; 16])
    }));
    assert!(result.is_err());

    // AES-128 with 32-byte key (should fail)
    let result = handle_aes128_gcm_encrypt(&json!({
        "plaintext": BASE64.encode(b"test"),
        "key": BASE64.encode(vec![0u8; 32])
    }));
    assert!(result.is_err());
}

#[test]
fn test_fault_tampered_ciphertext() {
    // Encrypt data, tamper with it, attempt decrypt
    let key = vec![0x42u8; 32];
    let plaintext = b"Original data";

    let encrypt_result = handle_aes256_gcm_encrypt(&json!({
        "plaintext": BASE64.encode(plaintext),
        "key": BASE64.encode(&key)
    }))
    .unwrap();

    let ciphertext_b64 = encrypt_result.get("ciphertext").unwrap().as_str().unwrap();
    let nonce = encrypt_result.get("nonce").unwrap().as_str().unwrap();

    // Tamper with ciphertext
    let mut ciphertext = BASE64.decode(ciphertext_b64).unwrap();
    if !ciphertext.is_empty() {
        ciphertext[0] ^= 0xFF; // Flip bits
    }

    // Attempt decrypt - should fail authentication
    let decrypt_result = handle_aes256_gcm_decrypt(&json!({
        "ciphertext": BASE64.encode(&ciphertext),
        "key": BASE64.encode(&key),
        "nonce": nonce
    }));

    assert!(decrypt_result.is_err());
}

#[test]
fn test_fault_wrong_nonce_size() {
    // Test with invalid nonce size (GCM requires 12 bytes)
    let key = vec![0x42u8; 32];
    let wrong_nonce = vec![0x99u8; 16]; // 16 bytes instead of 12

    let result = handle_aes256_gcm_encrypt(&json!({
        "plaintext": BASE64.encode(b"test"),
        "key": BASE64.encode(&key),
        "nonce": BASE64.encode(&wrong_nonce)
    }));

    assert!(result.is_err());
}

#[test]
fn test_fault_invalid_private_key_format() {
    // Test ECDH with corrupted private key
    let result = handle_ecdh_p256_derive(&json!({
        "private_key": BASE64.encode(vec![0u8; 10]), // Too short
        "peer_public_key": BASE64.encode(vec![0u8; 65])
    }));

    assert!(result.is_err());
}

#[test]
fn test_fault_invalid_public_key_format() {
    // Test ECDH with corrupted public key
    let gen_result = handle_ecdh_p256_generate(&json!({})).unwrap();
    let private_key = gen_result.get("private_key").unwrap().as_str().unwrap();

    let result = handle_ecdh_p256_derive(&json!({
        "private_key": private_key,
        "peer_public_key": BASE64.encode(vec![0xFF; 33]) // Invalid format
    }));

    assert!(result.is_err());
}

#[test]
fn test_fault_invalid_argon2_hash_format() {
    // Test password verification with invalid hash format
    let result = handle_argon2id_verify(&json!({
        "password": "test",
        "hash": "not_a_valid_phc_string"
    }));

    assert!(result.is_err());
}

#[test]
fn test_fault_pbkdf2_invalid_salt() {
    // Test PBKDF2 with invalid salt encoding
    let result = handle_pbkdf2_sha256(&json!({
        "password": "test",
        "salt": "Invalid@Base64!",
        "iterations": 100_000
    }));

    assert!(result.is_err());
}

#[test]
fn test_fault_null_json_values() {
    // Test with null JSON values
    assert!(handle_sha256(&json!({"data": null})).is_err());
    assert!(
        handle_ecdh_p256_derive(&json!({
            "private_key": null,
            "peer_public_key": null
        }))
        .is_err()
    );
}

#[test]
fn test_fault_extreme_pbkdf2_iterations() {
    // Test with extremely high iterations (should work but be slow)
    // We'll use a reasonable high value for testing
    let result = handle_pbkdf2_sha256(&json!({
        "password": "test",
        "salt": BASE64.encode(b"salt"),
        "iterations": 1_000_000, // 1 million iterations
        "output_length": 32
    }));

    // Should succeed but take a while
    assert!(result.is_ok());
}

#[test]
fn test_fault_recovery_after_errors() {
    // Test that system recovers gracefully after errors

    // Cause an error
    let _ = handle_sha256(&json!({"data": "Invalid!"}));

    // Should still work fine
    let result = handle_sha256(&json!({
        "data": BASE64.encode(b"test")
    }));
    assert!(result.is_ok());
}

#[test]
fn test_fault_concurrent_errors_dont_affect_others() {
    // Test that errors in one thread don't affect others
    use std::thread;

    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                if i % 2 == 0 {
                    // Even threads: cause errors
                    handle_sha256(&json!({"data": "Invalid!"})).is_err()
                } else {
                    // Odd threads: succeed
                    handle_sha256(&json!({
                        "data": BASE64.encode(b"valid")
                    }))
                    .is_ok()
                }
            })
        })
        .collect();

    for handle in handles {
        assert!(handle.join().unwrap());
    }
}

#[test]
fn test_fault_aes_decrypt_with_wrong_key() {
    // Encrypt with one key, decrypt with another
    let key1 = vec![0x42u8; 32];
    let key2 = vec![0x99u8; 32];
    let plaintext = b"Secret";

    let encrypt_result = handle_aes256_gcm_encrypt(&json!({
        "plaintext": BASE64.encode(plaintext),
        "key": BASE64.encode(&key1)
    }))
    .unwrap();

    let ciphertext = encrypt_result.get("ciphertext").unwrap().as_str().unwrap();
    let nonce = encrypt_result.get("nonce").unwrap().as_str().unwrap();

    // Try to decrypt with wrong key
    let decrypt_result = handle_aes256_gcm_decrypt(&json!({
        "ciphertext": ciphertext,
        "key": BASE64.encode(&key2),
        "nonce": nonce
    }));

    assert!(decrypt_result.is_err());
}

#[test]
fn test_fault_password_timing_attack_resistance() {
    // This is a basic test - true timing attack resistance requires more sophisticated testing
    // We just verify that verification completes regardless of correctness

    let password = "CorrectPassword";
    let hash_result = handle_argon2id_hash(&json!({"password": password})).unwrap();
    let hash = hash_result.get("hash").unwrap().as_str().unwrap();

    // Verify correct password
    let start1 = std::time::Instant::now();
    let _ = handle_argon2id_verify(&json!({
        "password": password,
        "hash": hash
    }));
    let time1 = start1.elapsed();

    // Verify wrong password
    let start2 = std::time::Instant::now();
    let _ = handle_argon2id_verify(&json!({
        "password": "WrongPassword",
        "hash": hash
    }));
    let time2 = start2.elapsed();

    // Times should be relatively similar (within 10x factor)
    // This is a weak test but demonstrates the concept
    let ratio = time1.as_micros() as f64 / time2.as_micros() as f64;
    assert!(
        ratio > 0.1 && ratio < 10.0,
        "Timing should be similar for constant-time ops"
    );
}
