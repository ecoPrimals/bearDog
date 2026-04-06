// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]

//! ChaCha20-Poly1305 unit tests.

use base64::Engine;
use beardog_tunnel::unix_socket_ipc::handlers::crypto::*;
use serde_json::json;

#[tokio::test]
async fn test_chacha20_poly1305_empty_plaintext() {
    let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(b"");
    let key = vec![0x42; 32]; // 256-bit key

    let params = json!({
        "plaintext": plaintext_b64,
        "key": base64::engine::general_purpose::STANDARD.encode(&key)
    });

    let result = handle_chacha20_poly1305_encrypt(Some(&params)).await;
    assert!(result.is_ok(), "Empty plaintext encryption failed");

    // Empty plaintext produces 0-byte ciphertext + separate 16-byte tag
    let response = result.unwrap();
    let ciphertext_b64 = response["ciphertext"].as_str().unwrap();
    let ciphertext = base64::engine::general_purpose::STANDARD
        .decode(ciphertext_b64)
        .unwrap();
    assert_eq!(
        ciphertext.len(),
        0,
        "Empty plaintext should produce 0-byte ciphertext"
    );

    let tag_b64 = response["tag"].as_str().unwrap();
    let tag = base64::engine::general_purpose::STANDARD
        .decode(tag_b64)
        .unwrap();
    assert_eq!(tag.len(), 16, "Tag should be 16 bytes");
}

#[tokio::test]
async fn test_chacha20_poly1305_large_plaintext() {
    // Large plaintext (10MB)
    let large_plaintext = vec![0x55; 10 * 1024 * 1024];
    let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(&large_plaintext);
    let key = vec![0x42; 32];
    let nonce = vec![0x13; 12];

    let params = json!({
        "plaintext": plaintext_b64,
        "key": base64::engine::general_purpose::STANDARD.encode(&key),
        "nonce": base64::engine::general_purpose::STANDARD.encode(&nonce)
    });

    let result = handle_chacha20_poly1305_encrypt(Some(&params)).await;
    assert!(result.is_ok(), "Large plaintext encryption failed");
}

#[tokio::test]
async fn test_chacha20_poly1305_encrypt_decrypt_roundtrip() {
    let plaintext = b"Sensitive data to encrypt";
    let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(plaintext);
    let key = vec![0x42; 32];

    // Encrypt
    let encrypt_params = json!({
        "plaintext": plaintext_b64,
        "key": base64::engine::general_purpose::STANDARD.encode(&key)
    });

    let encrypt_result = handle_chacha20_poly1305_encrypt(Some(&encrypt_params))
        .await
        .unwrap();
    let ciphertext_b64 = encrypt_result["ciphertext"].as_str().unwrap();
    let nonce_b64 = encrypt_result["nonce"].as_str().unwrap();
    let tag_b64 = encrypt_result["tag"].as_str().unwrap();

    // Decrypt
    let decrypt_params = json!({
        "ciphertext": ciphertext_b64,
        "key": base64::engine::general_purpose::STANDARD.encode(&key),
        "nonce": nonce_b64,
        "tag": tag_b64
    });

    let decrypt_result = handle_chacha20_poly1305_decrypt(Some(&decrypt_params))
        .await
        .unwrap();
    let decrypted_b64 = decrypt_result["plaintext"].as_str().unwrap();
    let decrypted = base64::engine::general_purpose::STANDARD
        .decode(decrypted_b64)
        .unwrap();

    assert_eq!(
        decrypted,
        plaintext.to_vec(),
        "Decrypted plaintext must match original"
    );
}

#[tokio::test]
async fn test_chacha20_poly1305_different_nonces() {
    // Same key + plaintext, nonces are generated randomly = different ciphertexts
    let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(b"test");
    let key = base64::engine::general_purpose::STANDARD.encode(vec![0x42; 32]);

    let result1 = handle_chacha20_poly1305_encrypt(Some(&json!({
        "plaintext": plaintext_b64,
        "key": key
    })))
    .await
    .unwrap();

    let result2 = handle_chacha20_poly1305_encrypt(Some(&json!({
        "plaintext": plaintext_b64,
        "key": key
    })))
    .await
    .unwrap();

    // Random nonces mean different ciphertexts
    assert_ne!(
        result1["nonce"], result2["nonce"],
        "Random nonces should be different"
    );
    assert_ne!(
        result1["ciphertext"], result2["ciphertext"],
        "Different nonces must produce different ciphertexts"
    );
}
