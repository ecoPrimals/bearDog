// SPDX-License-Identifier: AGPL-3.0-only
#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]

//! End-to-end integration tests — full handler flows across primitives.

use crate::common::*;

#[test]
fn test_e2e_ecdh_to_aes_encryption_flow() {
    // Simulate TLS-like flow: ECDH key exchange → AES-GCM encryption

    // Step 1: Alice generates keypair
    let alice_gen = handle_ecdh_p256_generate(&json!({})).unwrap();
    let alice_private = alice_gen.get("private_key").unwrap().as_str().unwrap();
    let alice_public = alice_gen.get("public_key").unwrap().as_str().unwrap();

    // Step 2: Bob generates keypair
    let bob_gen = handle_ecdh_p256_generate(&json!({})).unwrap();
    let bob_private = bob_gen.get("private_key").unwrap().as_str().unwrap();
    let bob_public = bob_gen.get("public_key").unwrap().as_str().unwrap();

    // Step 3: Both derive shared secret
    let alice_derive = handle_ecdh_p256_derive(&json!({
        "private_key": alice_private,
        "peer_public_key": bob_public
    }))
    .unwrap();

    let bob_derive = handle_ecdh_p256_derive(&json!({
        "private_key": bob_private,
        "peer_public_key": alice_public
    }))
    .unwrap();

    let alice_secret = alice_derive.get("shared_secret").unwrap().as_str().unwrap();
    let bob_secret = bob_derive.get("shared_secret").unwrap().as_str().unwrap();

    // Secrets should match
    assert_eq!(alice_secret, bob_secret);

    // Step 4: Use shared secret as AES key (hash it to get 32 bytes)
    let key_material = BASE64.decode(alice_secret).unwrap();
    let key_hash_result = handle_sha256(&json!({
        "data": BASE64.encode(&key_material)
    }))
    .unwrap();
    let aes_key_hex = key_hash_result.get("hash").unwrap().as_str().unwrap();
    // SHA returns hex, AES needs base64, so decode hex and re-encode as base64
    let aes_key_bytes = hex::decode(aes_key_hex).unwrap();
    let aes_key = BASE64.encode(&aes_key_bytes);

    // Step 5: Alice encrypts message
    let plaintext = b"Secret message from Alice to Bob!";
    let encrypt_result = handle_aes256_gcm_encrypt(&json!({
        "plaintext": BASE64.encode(plaintext),
        "key": aes_key
    }))
    .unwrap();

    let ciphertext = encrypt_result.get("ciphertext").unwrap().as_str().unwrap();
    let nonce = encrypt_result.get("nonce").unwrap().as_str().unwrap();

    // Step 6: Bob decrypts message
    let decrypt_result = handle_aes256_gcm_decrypt(&json!({
        "ciphertext": ciphertext,
        "key": aes_key,
        "nonce": nonce
    }))
    .unwrap();

    let decrypted_b64 = decrypt_result.get("plaintext").unwrap().as_str().unwrap();
    let decrypted = BASE64.decode(decrypted_b64).unwrap();

    assert_eq!(&decrypted, plaintext);
}

#[test]
fn test_e2e_password_registration_and_login_flow() {
    // Simulate user registration and login

    // Step 1: User registers with password
    let password = "MySecurePassword123!";
    let hash_result = handle_argon2id_hash(&json!({
        "password": password
    }))
    .unwrap();

    let stored_hash = hash_result
        .get("hash")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    // Step 2: User attempts login with correct password
    let login_result = handle_argon2id_verify(&json!({
        "password": password,
        "hash": stored_hash
    }))
    .unwrap();

    assert!(login_result.get("valid").unwrap().as_bool().unwrap());

    // Step 3: User attempts login with wrong password
    let wrong_login = handle_argon2id_verify(&json!({
        "password": "WrongPassword!",
        "hash": stored_hash
    }))
    .unwrap();

    assert!(!wrong_login.get("valid").unwrap().as_bool().unwrap());
}

#[test]
fn test_e2e_multi_layer_encryption() {
    // Test encryption with multiple layers (SHA → AES → SHA)

    let original_data = b"Multi-layer encryption test data";

    // Layer 1: Hash original data
    let hash1 = handle_sha256(&json!({
        "data": BASE64.encode(original_data)
    }))
    .unwrap();
    let hash1_value = hash1.get("hash").unwrap().as_str().unwrap();

    // Layer 2: Encrypt the hash
    let key = vec![0x42u8; 32];
    let encrypt_result = handle_aes256_gcm_encrypt(&json!({
        "plaintext": hash1_value,
        "key": BASE64.encode(&key)
    }))
    .unwrap();

    let ciphertext = encrypt_result.get("ciphertext").unwrap().as_str().unwrap();
    let nonce = encrypt_result.get("nonce").unwrap().as_str().unwrap();

    // Layer 3: Hash the ciphertext
    let _hash2 = handle_sha256(&json!({
        "data": ciphertext
    }))
    .unwrap();

    // Verify we can reverse the process
    let decrypt_result = handle_aes256_gcm_decrypt(&json!({
        "ciphertext": ciphertext,
        "key": BASE64.encode(&key),
        "nonce": nonce
    }))
    .unwrap();

    let decrypted_hash = decrypt_result.get("plaintext").unwrap().as_str().unwrap();
    assert_eq!(decrypted_hash, hash1_value);
}

#[test]
fn test_e2e_concurrent_user_sessions() {
    // Simulate 10 concurrent user sessions
    use std::thread;

    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                let password = format!("User{i}Password");

                // Each user hashes their password
                let hash_result = handle_argon2id_hash(&json!({
                    "password": password
                }))
                .unwrap();

                let hash = hash_result
                    .get("hash")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();

                // Each user verifies their password
                let verify_result = handle_argon2id_verify(&json!({
                    "password": password,
                    "hash": hash
                }))
                .unwrap();

                verify_result.get("valid").unwrap().as_bool().unwrap()
            })
        })
        .collect();

    // All should succeed
    for handle in handles {
        assert!(handle.join().unwrap());
    }
}

#[test]
fn test_e2e_data_integrity_chain() {
    // Test data integrity through multiple operations
    let original = b"Important data that must maintain integrity";

    // Hash it
    let hash_result = handle_sha256(&json!({
        "data": BASE64.encode(original)
    }))
    .unwrap();
    let hash_value = hash_result.get("hash").unwrap().as_str().unwrap();

    // Encrypt original data
    let key = vec![0x42u8; 32];
    let encrypt_result = handle_aes256_gcm_encrypt(&json!({
        "plaintext": BASE64.encode(original),
        "key": BASE64.encode(&key)
    }))
    .unwrap();

    let ciphertext = encrypt_result.get("ciphertext").unwrap().as_str().unwrap();
    let nonce = encrypt_result.get("nonce").unwrap().as_str().unwrap();

    // Decrypt
    let decrypt_result = handle_aes256_gcm_decrypt(&json!({
        "ciphertext": ciphertext,
        "key": BASE64.encode(&key),
        "nonce": nonce
    }))
    .unwrap();

    let decrypted_b64 = decrypt_result.get("plaintext").unwrap().as_str().unwrap();
    let decrypted = BASE64.decode(decrypted_b64).unwrap();

    // Verify integrity by hashing decrypted data
    let verify_hash = handle_sha256(&json!({
        "data": BASE64.encode(&decrypted)
    }))
    .unwrap();
    let verify_hash_value = verify_hash.get("hash").unwrap().as_str().unwrap();

    assert_eq!(hash_value, verify_hash_value);
    assert_eq!(&decrypted, original);
}
