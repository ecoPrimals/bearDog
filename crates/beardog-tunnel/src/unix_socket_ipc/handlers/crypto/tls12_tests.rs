// SPDX-License-Identifier: AGPL-3.0-only

//! TLS 1.2 handler tests
//!
//! Tests for ECDHE key generation, AES-GCM encryption roundtrips,
//! and TLS 1.2 PRF key derivation.

use super::*;

#[tokio::test]
async fn test_p256_keypair_generation() {
    let result = handle_ecdhe_p256_generate(None).await.unwrap();
    assert!(result.get("public_key").is_some());
    assert!(result.get("secret_key").is_some());
    assert_eq!(result.get("algorithm").unwrap(), "P-256");
}

#[tokio::test]
async fn test_p384_keypair_generation() {
    let result = handle_ecdhe_p384_generate(None).await.unwrap();
    assert!(result.get("public_key").is_some());
    assert!(result.get("secret_key").is_some());
    assert_eq!(result.get("algorithm").unwrap(), "P-384");
}

#[tokio::test]
async fn test_aes_128_gcm_roundtrip() {
    let key_b64 = BASE64.encode([0u8; 16]); // 128-bit key
    let nonce_b64 = BASE64.encode([1u8; 12]); // 96-bit nonce
    let plaintext_b64 = BASE64.encode(b"Hello, TLS 1.2!");

    // Encrypt
    let encrypt_params = serde_json::json!({
        "key": key_b64,
        "nonce": nonce_b64,
        "plaintext": plaintext_b64,
    });

    let encrypted = handle_aes_128_gcm_encrypt(Some(&encrypt_params))
        .await
        .unwrap();
    let ciphertext_b64 = encrypted.get("ciphertext").unwrap().as_str().unwrap();
    let tag_b64 = encrypted.get("tag").unwrap().as_str().unwrap();

    // Decrypt
    let decrypt_params = serde_json::json!({
        "key": key_b64,
        "nonce": nonce_b64,
        "ciphertext": ciphertext_b64,
        "tag": tag_b64,
    });

    let decrypted = handle_aes_128_gcm_decrypt(Some(&decrypt_params))
        .await
        .unwrap();
    assert_eq!(
        decrypted.get("plaintext").unwrap().as_str().unwrap(),
        plaintext_b64
    );
    assert_eq!(decrypted.get("verified").unwrap(), true);
}

#[tokio::test]
async fn test_aes_128_gcm_encrypt_rejects_wrong_key_length() {
    let key_b64 = BASE64.encode([0u8; 8]); // too short
    let nonce_b64 = BASE64.encode([1u8; 12]);
    let plaintext_b64 = BASE64.encode(b"hi");
    let params = serde_json::json!({
        "key": key_b64,
        "nonce": nonce_b64,
        "plaintext": plaintext_b64,
    });
    let err = handle_aes_128_gcm_encrypt(Some(&params)).await.unwrap_err();
    assert!(err.contains("16") || err.contains("key length"));
}

#[tokio::test]
async fn test_aes_128_gcm_encrypt_rejects_bad_nonce_length() {
    let key_b64 = BASE64.encode([0u8; 16]);
    let nonce_b64 = BASE64.encode([1u8; 8]); // GCM expects 12
    let plaintext_b64 = BASE64.encode(b"hi");
    let params = serde_json::json!({
        "key": key_b64,
        "nonce": nonce_b64,
        "plaintext": plaintext_b64,
    });
    let err = handle_aes_128_gcm_encrypt(Some(&params)).await.unwrap_err();
    assert!(err.contains("12") || err.contains("nonce"));
}

#[tokio::test]
async fn test_tls12_prf_rejects_unknown_hash() {
    let params = serde_json::json!({
        "secret": BASE64.encode(b"s"),
        "label": "l",
        "seed": BASE64.encode(b"seed"),
        "output_len": 16,
        "hash": "sha999",
    });
    let err = handle_tls12_prf(Some(&params)).await.unwrap_err();
    assert!(err.contains("Unsupported hash"));
}

#[tokio::test]
async fn test_tls12_prf_sha384_smoke() {
    let params = serde_json::json!({
        "secret": BASE64.encode(b"secret"),
        "label": "master secret",
        "seed": BASE64.encode(b"seed"),
        "output_len": 32,
        "hash": "sha384",
    });
    let out = handle_tls12_prf(Some(&params)).await.expect("prf");
    assert_eq!(out.get("algorithm").unwrap(), "TLS12-PRF-SHA384");
}

#[tokio::test]
async fn test_tls12_prf_sha256() {
    let secret_b64 = BASE64.encode(b"secret");
    let seed_b64 = BASE64.encode(b"seed");

    let params = serde_json::json!({
        "secret": secret_b64,
        "label": "test label",
        "seed": seed_b64,
        "output_len": 48,
        "hash": "sha256",
    });

    let result = handle_tls12_prf(Some(&params)).await.unwrap();
    let output_b64 = result.get("output").unwrap().as_str().unwrap();
    let output = BASE64.decode(output_b64).unwrap();

    assert_eq!(output.len(), 48);
    assert_eq!(result.get("algorithm").unwrap(), "TLS12-PRF-SHA256");
}

#[tokio::test]
async fn test_p256_ecdh_shared_secret_agreement() {
    let a = handle_ecdhe_p256_generate(None).await.unwrap();
    let b = handle_ecdhe_p256_generate(None).await.unwrap();

    let a_secret = a.get("secret_key").unwrap().as_str().unwrap();
    let a_pub = a.get("public_key").unwrap().as_str().unwrap();
    let b_secret = b.get("secret_key").unwrap().as_str().unwrap();
    let b_pub = b.get("public_key").unwrap().as_str().unwrap();

    let ab = serde_json::json!({
        "our_secret": a_secret,
        "their_public": b_pub,
    });
    let ba = serde_json::json!({
        "our_secret": b_secret,
        "their_public": a_pub,
    });

    let s1 = handle_ecdhe_p256_compute_shared(Some(&ab)).await.unwrap();
    let s2 = handle_ecdhe_p256_compute_shared(Some(&ba)).await.unwrap();
    assert_eq!(
        s1.get("shared_secret").unwrap().as_str().unwrap(),
        s2.get("shared_secret").unwrap().as_str().unwrap()
    );
}

#[tokio::test]
async fn test_p256_compute_shared_missing_params() {
    assert!(handle_ecdhe_p256_compute_shared(None).await.is_err());
}

#[tokio::test]
async fn test_tls12_prf_sha384() {
    let secret_b64 = BASE64.encode(b"secret");
    let seed_b64 = BASE64.encode(b"seed");

    let params = serde_json::json!({
        "secret": secret_b64,
        "label": "test label",
        "seed": seed_b64,
        "output_len": 48,
        "hash": "sha384",
    });

    let result = handle_tls12_prf(Some(&params)).await.unwrap();
    let output_b64 = result.get("output").unwrap().as_str().unwrap();
    let output = BASE64.decode(output_b64).unwrap();
    assert_eq!(output.len(), 48);
}
