// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]

//! Fault / error-path tests.

use base64::Engine;
use beardog_tunnel::unix_socket_ipc::handlers::crypto::*;
use serde_json::json;

#[tokio::test]
async fn test_fault_missing_required_param_message() {
    let result = handle_sign_ed25519(Some(&json!({
        "key_id": "test",
        "purpose": "test"
        // Missing "message"
    })))
    .await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("Missing required parameter: message")
    );
}

#[tokio::test]
async fn test_fault_invalid_ed25519_key_length() {
    // Ed25519 keys are 32 bytes, provide wrong length
    let wrong_key = vec![0u8; 16]; // Only 16 bytes
    let message_b64 = base64::engine::general_purpose::STANDARD.encode(b"test");
    let signature_b64 = base64::engine::general_purpose::STANDARD.encode(vec![0u8; 64]);

    let result = handle_verify_ed25519(Some(&json!({
        "message": message_b64,
        "signature": signature_b64,
        "public_key": base64::engine::general_purpose::STANDARD.encode(&wrong_key)
    })))
    .await;

    assert!(result.is_err(), "Invalid key length should fail");
}

#[tokio::test]
async fn test_fault_invalid_signature_verification() {
    // Valid format but wrong signature
    let message_b64 = base64::engine::general_purpose::STANDARD.encode(b"original message");
    let wrong_signature = base64::engine::general_purpose::STANDARD.encode(vec![0u8; 64]);
    let public_key = base64::engine::general_purpose::STANDARD.encode(vec![1u8; 32]);

    let result = handle_verify_ed25519(Some(&json!({
        "message": message_b64,
        "signature": wrong_signature,
        "public_key": public_key
    })))
    .await;

    // Should succeed but return valid: false
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["valid"], false);
}

#[tokio::test]
async fn test_fault_invalid_x25519_key_length() {
    let wrong_key = vec![0u8; 16]; // Should be 32 bytes

    let result = handle_x25519_derive_secret(Some(&json!({
        "our_secret": base64::engine::general_purpose::STANDARD.encode(&wrong_key),
        "their_public": base64::engine::general_purpose::STANDARD.encode(vec![1u8; 32])
    })))
    .await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("32 bytes"));
}

#[tokio::test]
async fn test_fault_chacha20_invalid_key_length() {
    let wrong_key = vec![0u8; 16]; // Should be 32 bytes
    let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(b"test");
    let nonce = base64::engine::general_purpose::STANDARD.encode(vec![0u8; 12]);

    let result = handle_chacha20_poly1305_encrypt(Some(&json!({
        "plaintext": plaintext_b64,
        "key": base64::engine::general_purpose::STANDARD.encode(&wrong_key),
        "nonce": nonce
    })))
    .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_fault_chacha20_invalid_key_size() {
    // Test with invalid key size (nonce is auto-generated)
    let wrong_key = vec![0u8; 16]; // Should be 32 bytes
    let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(b"test");

    let result = handle_chacha20_poly1305_encrypt(Some(&json!({
        "plaintext": plaintext_b64,
        "key": base64::engine::general_purpose::STANDARD.encode(&wrong_key)
    })))
    .await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("32 bytes"));
}

#[tokio::test]
async fn test_fault_chacha20_corrupted_ciphertext() {
    // Create valid ciphertext then corrupt it
    let key = vec![0x42; 32];
    let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(b"test data");

    let encrypt_result = handle_chacha20_poly1305_encrypt(Some(&json!({
        "plaintext": plaintext_b64,
        "key": base64::engine::general_purpose::STANDARD.encode(&key)
    })))
    .await
    .unwrap();

    // Corrupt the ciphertext
    let mut ciphertext = base64::engine::general_purpose::STANDARD
        .decode(encrypt_result["ciphertext"].as_str().unwrap())
        .unwrap();
    ciphertext[0] ^= 0xFF; // Flip bits

    let decrypt_result = handle_chacha20_poly1305_decrypt(Some(&json!({
        "ciphertext": base64::engine::general_purpose::STANDARD.encode(&ciphertext),
        "key": base64::engine::general_purpose::STANDARD.encode(&key),
        "nonce": encrypt_result["nonce"].as_str().unwrap(),
        "tag": encrypt_result["tag"].as_str().unwrap()
    })))
    .await;

    assert!(
        decrypt_result.is_err(),
        "Corrupted ciphertext should fail authentication"
    );
}

#[tokio::test]
async fn test_fault_chacha20_wrong_key_decryption() {
    // Encrypt with one key, decrypt with another
    let key1 = vec![0x42; 32];
    let key2 = vec![0x43; 32];
    let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(b"secret");

    let encrypt_result = handle_chacha20_poly1305_encrypt(Some(&json!({
        "plaintext": plaintext_b64,
        "key": base64::engine::general_purpose::STANDARD.encode(&key1)
    })))
    .await
    .unwrap();

    let decrypt_result = handle_chacha20_poly1305_decrypt(Some(&json!({
        "ciphertext": encrypt_result["ciphertext"],
        "key": base64::engine::general_purpose::STANDARD.encode(&key2), // Wrong key!
        "nonce": encrypt_result["nonce"],
        "tag": encrypt_result["tag"]
    })))
    .await;

    assert!(
        decrypt_result.is_err(),
        "Wrong key should fail authentication"
    );
}

#[tokio::test]
async fn test_fault_blake3_missing_data() {
    let result = handle_blake3_hash(Some(&json!({}))).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Missing"));
}

#[tokio::test]
async fn test_fault_hmac_missing_key() {
    let data_b64 = base64::engine::general_purpose::STANDARD.encode(b"data");

    let result = handle_hmac_sha256(Some(&json!({"data": data_b64}))).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("key"));
}

#[tokio::test]
async fn test_fault_hmac_missing_data() {
    let key_b64 = base64::engine::general_purpose::STANDARD.encode(b"key");

    let result = handle_hmac_sha256(Some(&json!({"key": key_b64}))).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("data"));
}

#[tokio::test]
async fn test_fault_concurrent_error_handling() {
    // Fire off many failing operations concurrently
    let handles: Vec<_> = (0..50)
        .map(|_| {
            tokio::spawn(async {
                // All of these should fail gracefully
                let _ = handle_sign_ed25519(None).await;
                let _ = handle_verify_ed25519(Some(&json!({}))).await;
                let _ = handle_x25519_derive_secret(Some(&json!({}))).await;
                let _ = handle_blake3_hash(None).await;
                let _ = handle_hmac_sha256(Some(&json!({}))).await;
                "completed"
            })
        })
        .collect();

    for handle in handles {
        let result = handle.await.expect("Task should complete");
        assert_eq!(result, "completed");
    }
}
