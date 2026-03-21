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
async fn test_p256_generate_with_optional_purpose() {
    let params = serde_json::json!({ "purpose": "unit_test_purpose" });
    let result = handle_ecdhe_p256_generate(Some(&params)).await.unwrap();
    assert_eq!(
        result.get("curve").and_then(|v| v.as_str()),
        Some("secp256r1")
    );
}

#[tokio::test]
async fn test_p384_generate_with_optional_purpose() {
    let params = serde_json::json!({ "purpose": "p384_test" });
    let result = handle_ecdhe_p384_generate(Some(&params)).await.unwrap();
    assert_eq!(
        result.get("curve").and_then(|v| v.as_str()),
        Some("secp384r1")
    );
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
async fn test_p256_compute_shared_missing_our_secret() {
    let params = serde_json::json!({ "their_public": BASE64.encode([1u8; 33]) });
    let err = handle_ecdhe_p256_compute_shared(Some(&params))
        .await
        .unwrap_err();
    assert!(err.contains("our_secret"));
}

#[tokio::test]
async fn test_p256_compute_shared_missing_their_public() {
    let sk = handle_ecdhe_p256_generate(None).await.unwrap();
    let b64 = sk.get("secret_key").unwrap().as_str().unwrap();
    let params = serde_json::json!({ "our_secret": b64 });
    let err = handle_ecdhe_p256_compute_shared(Some(&params))
        .await
        .unwrap_err();
    assert!(err.contains("their_public"));
}

#[tokio::test]
async fn test_p256_compute_shared_invalid_base64() {
    let params = serde_json::json!({
        "our_secret": "@@@",
        "their_public": BASE64.encode([1u8; 33]),
    });
    let err = handle_ecdhe_p256_compute_shared(Some(&params))
        .await
        .unwrap_err();
    assert!(err.contains("our_secret") || err.contains("base64"));
}

#[tokio::test]
async fn test_p384_compute_shared_missing_params() {
    let err = handle_ecdhe_p384_compute_shared(None).await.unwrap_err();
    assert!(err.contains("P-384") || err.contains("Missing"));
}

#[tokio::test]
async fn test_aes_128_gcm_encrypt_missing_params_and_fields() {
    assert!(
        handle_aes_128_gcm_encrypt(None)
            .await
            .unwrap_err()
            .contains("Missing")
    );
    let empty = serde_json::json!({});
    let err = handle_aes_128_gcm_encrypt(Some(&empty)).await.unwrap_err();
    assert!(err.contains("key") || err.contains("Missing"));
}

#[tokio::test]
async fn test_aes_128_gcm_decrypt_missing_tag() {
    let key_b64 = BASE64.encode([0u8; 16]);
    let nonce_b64 = BASE64.encode([1u8; 12]);
    let params = serde_json::json!({
        "key": key_b64,
        "nonce": nonce_b64,
        "ciphertext": BASE64.encode(b"x"),
    });
    let err = handle_aes_128_gcm_decrypt(Some(&params)).await.unwrap_err();
    assert!(err.contains("tag"));
}

#[tokio::test]
async fn test_aes_128_gcm_roundtrip_with_aad() {
    let key_b64 = BASE64.encode([0xCu8; 16]);
    let nonce_b64 = BASE64.encode([2u8; 12]);
    let plaintext_b64 = BASE64.encode(b"aad-body");
    let aad_b64 = BASE64.encode(b"extra-aad");
    let enc = serde_json::json!({
        "key": key_b64,
        "nonce": nonce_b64,
        "plaintext": plaintext_b64,
        "aad": aad_b64,
    });
    let encrypted = handle_aes_128_gcm_encrypt(Some(&enc)).await.unwrap();
    let dec = serde_json::json!({
        "key": key_b64,
        "nonce": nonce_b64,
        "ciphertext": encrypted.get("ciphertext").unwrap().as_str().unwrap(),
        "tag": encrypted.get("tag").unwrap().as_str().unwrap(),
        "aad": aad_b64,
    });
    let out = handle_aes_128_gcm_decrypt(Some(&dec)).await.unwrap();
    assert_eq!(
        out.get("plaintext").unwrap().as_str().unwrap(),
        plaintext_b64
    );
}

#[tokio::test]
async fn test_aes_128_gcm_invalid_aad_base64() {
    let key_b64 = BASE64.encode([0u8; 16]);
    let nonce_b64 = BASE64.encode([1u8; 12]);
    let params = serde_json::json!({
        "key": key_b64,
        "nonce": nonce_b64,
        "plaintext": BASE64.encode(b"hi"),
        "aad": "not-valid-b64!!!",
    });
    let err = handle_aes_128_gcm_encrypt(Some(&params)).await.unwrap_err();
    assert!(err.contains("AAD") || err.contains("base64"));
}

#[tokio::test]
async fn test_aes_256_gcm_wrong_key_length() {
    let params = serde_json::json!({
        "key": BASE64.encode([0u8; 16]),
        "nonce": BASE64.encode([1u8; 12]),
        "plaintext": BASE64.encode(b"hi"),
    });
    let err = handle_aes_256_gcm_encrypt(Some(&params)).await.unwrap_err();
    assert!(err.contains("32") || err.contains("key"));
}

#[tokio::test]
async fn test_aes_256_gcm_roundtrip() {
    let key_b64 = BASE64.encode([0xDu8; 32]);
    let nonce_b64 = BASE64.encode([3u8; 12]);
    let plaintext_b64 = BASE64.encode(b"aes256-gcm");
    let enc = serde_json::json!({
        "key": key_b64,
        "nonce": nonce_b64,
        "plaintext": plaintext_b64,
    });
    let encrypted = handle_aes_256_gcm_encrypt(Some(&enc)).await.unwrap();
    let dec = serde_json::json!({
        "key": key_b64,
        "nonce": nonce_b64,
        "ciphertext": encrypted.get("ciphertext").unwrap().as_str().unwrap(),
        "tag": encrypted.get("tag").unwrap().as_str().unwrap(),
    });
    let out = handle_aes_256_gcm_decrypt(Some(&dec)).await.unwrap();
    assert_eq!(
        out.get("plaintext").unwrap().as_str().unwrap(),
        plaintext_b64
    );
    assert_eq!(
        out.get("algorithm").unwrap().as_str().unwrap(),
        "AES-256-GCM"
    );
}

#[tokio::test]
async fn test_aes_128_gcm_decrypt_fails_on_bad_tag() {
    let key_b64 = BASE64.encode([0xEu8; 16]);
    let nonce_b64 = BASE64.encode([4u8; 12]);
    let enc = serde_json::json!({
        "key": key_b64,
        "nonce": nonce_b64,
        "plaintext": BASE64.encode(b"secret"),
    });
    let encrypted = handle_aes_128_gcm_encrypt(Some(&enc)).await.unwrap();
    let mut tag = BASE64
        .decode(encrypted.get("tag").unwrap().as_str().unwrap())
        .unwrap();
    tag[0] ^= 0xFF;
    let dec = serde_json::json!({
        "key": key_b64,
        "nonce": nonce_b64,
        "ciphertext": encrypted.get("ciphertext").unwrap().as_str().unwrap(),
        "tag": BASE64.encode(&tag),
    });
    let err = handle_aes_128_gcm_decrypt(Some(&dec)).await.unwrap_err();
    assert!(err.contains("decryption") || err.contains("verification"));
}

#[tokio::test]
async fn test_tls12_prf_missing_fields() {
    let err = handle_tls12_prf(Some(&serde_json::json!({})))
        .await
        .unwrap_err();
    assert!(err.contains("secret") || err.contains("Missing"));
}

#[tokio::test]
async fn test_tls12_prf_default_hash_sha256() {
    let params = serde_json::json!({
        "secret": BASE64.encode(b"s"),
        "label": "l",
        "seed": BASE64.encode(b"seed"),
        "output_len": 8,
    });
    let out = handle_tls12_prf(Some(&params)).await.unwrap();
    assert_eq!(out.get("algorithm").unwrap(), "TLS12-PRF-SHA256");
}

#[tokio::test]
async fn test_tls12_prf_sha256_large_output() {
    let params = serde_json::json!({
        "secret": BASE64.encode(b"long-secret"),
        "label": "key expansion",
        "seed": BASE64.encode(b"0123456789abcdef"),
        "output_len": 128,
        "hash": "sha256",
    });
    let out = handle_tls12_prf(Some(&params)).await.unwrap();
    let raw = BASE64
        .decode(out.get("output").unwrap().as_str().unwrap())
        .unwrap();
    assert_eq!(raw.len(), 128);
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
