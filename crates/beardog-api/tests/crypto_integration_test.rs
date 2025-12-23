//! Integration tests for real crypto operations via HTTP API
//!
//! Tests verify that:
//! 1. Mock implementations have been replaced with real crypto
//! 2. Encrypt/decrypt round-trips work
//! 3. Sign/verify round-trips work
//! 4. Base64 encoding/decoding works correctly

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use base64::{engine::general_purpose::STANDARD, Engine};
use beardog_api::{create_router, ApiResponse};
use beardog_core::core::BearDogCore;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt; // for oneshot

#[tokio::test]
async fn test_aes_gcm_encrypt_decrypt_roundtrip() {
    // Setup
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    // Original plaintext
    let plaintext = b"Hello, BearDog! This is secret data.";
    let plaintext_b64 = STANDARD.encode(plaintext);

    // 1. Encrypt
    let encrypt_request = Request::builder()
        .method("POST")
        .uri("/api/v1/crypto/aes-gcm/encrypt")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&json!({
                "data": plaintext_b64,
                "key_id": "test-key-123",
            }))
            .expect("Failed to serialize"),
        ))
        .expect("Failed to build request");

    let response = app
        .clone()
        .oneshot(encrypt_request)
        .await
        .expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let encrypt_response: ApiResponse<serde_json::Value> =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    assert!(encrypt_response.success);
    let encrypt_data = encrypt_response
        .data
        .as_ref()
        .expect("Missing encryption data");

    let ciphertext = encrypt_data["ciphertext"]
        .as_str()
        .expect("Missing ciphertext");
    let nonce = encrypt_data["nonce"].as_str().expect("Missing nonce");
    let tag = encrypt_data["tag"].as_str().expect("Missing tag");

    // Verify ciphertext is different from plaintext
    let ciphertext_bytes = STANDARD.decode(ciphertext).expect("Invalid base64");
    assert_ne!(ciphertext_bytes, plaintext);
    assert!(!ciphertext_bytes.is_empty());

    // 2. Decrypt
    let decrypt_request = Request::builder()
        .method("POST")
        .uri("/api/v1/crypto/aes-gcm/decrypt")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&json!({
                "ciphertext": ciphertext,
                "nonce": nonce,
                "tag": tag,
                "key_id": "test-key-123",
            }))
            .expect("Failed to serialize"),
        ))
        .expect("Failed to build request");

    let response = app
        .oneshot(decrypt_request)
        .await
        .expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let decrypt_response: ApiResponse<serde_json::Value> =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    assert!(decrypt_response.success);
    let decrypt_data = decrypt_response
        .data
        .as_ref()
        .expect("Missing decryption data");

    let decrypted_b64 = decrypt_data["plaintext"]
        .as_str()
        .expect("Missing plaintext");
    let decrypted = STANDARD.decode(decrypted_b64).expect("Invalid base64");

    // Verify roundtrip: decrypted == original plaintext
    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_ed25519_sign_verify_roundtrip() {
    // Setup
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    // Original message
    let message = b"This is an important message from BearDog.";
    let message_b64 = STANDARD.encode(message);

    // 1. Sign
    let sign_request = Request::builder()
        .method("POST")
        .uri("/api/v1/crypto/ed25519/sign")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&json!({
                "message": message_b64,
                "key_id": "test-signing-key",
            }))
            .expect("Failed to serialize"),
        ))
        .expect("Failed to build request");

    let response = app
        .clone()
        .oneshot(sign_request)
        .await
        .expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let sign_response: ApiResponse<serde_json::Value> =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    assert!(sign_response.success);
    let sign_data = sign_response.data.as_ref().expect("Missing sign data");

    let signature = sign_data["signature"].as_str().expect("Missing signature");
    let algorithm = sign_data["algorithm"].as_str().expect("Missing algorithm");

    assert_eq!(algorithm, "ed25519");
    assert!(!signature.is_empty());

    // Verify signature is 64 bytes (Ed25519 signature size)
    let signature_bytes = STANDARD.decode(signature).expect("Invalid base64");
    assert_eq!(signature_bytes.len(), 64);

    // 2. Get public key (for this test, we'll derive it from the key_id the same way the service does)
    // The service uses HKDF for key derivation, we must match that exactly
    use ed25519_dalek::SigningKey;

    // Match the exact key derivation in BearDogCryptoService::derive_signing_key
    // Uses: hkdf_sha256(key_id, salt, info, 32)
    let key_id = b"test-signing-key";
    let salt = b"beardog-signing-key-derivation-v1";
    let info = format!("{}:sign:{}", "beardog", "test-signing-key"); // service_name:sign:key_id

    // Inline HKDF to avoid adding dependency (same as crypto service uses)
    use hkdf::Hkdf;
    use sha2::Sha256;
    let hk = Hkdf::<Sha256>::new(Some(salt), key_id);
    let mut derived = [0u8; 32];
    hk.expand(info.as_bytes(), &mut derived)
        .expect("HKDF expand failed");

    let signing_key = SigningKey::from_bytes(&derived);
    let verifying_key = signing_key.verifying_key();
    let public_key_b64 = STANDARD.encode(verifying_key.as_bytes());

    // 3. Verify
    let verify_request = Request::builder()
        .method("POST")
        .uri("/api/v1/crypto/ed25519/verify")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&json!({
                "message": message_b64,
                "signature": signature,
                "public_key": public_key_b64,
            }))
            .expect("Failed to serialize"),
        ))
        .expect("Failed to build request");

    let response = app
        .oneshot(verify_request)
        .await
        .expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let verify_response: ApiResponse<serde_json::Value> =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    assert!(verify_response.success);
    let verify_data = verify_response
        .data
        .as_ref()
        .expect("Missing verification data");

    let valid = verify_data["valid"].as_bool().expect("Missing valid field");

    // Verify signature is valid
    assert!(valid);
}

#[tokio::test]
async fn test_ed25519_verify_invalid_signature() {
    // Setup
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    let message = b"Original message";
    let message_b64 = STANDARD.encode(message);

    // Create a bogus signature
    let bogus_signature = STANDARD.encode(vec![0u8; 64]);

    // Create a valid public key
    use ed25519_dalek::SigningKey;
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"beardog-signing-key-v1:");
    hasher.update(b"some-key"); // Must match BearDogCryptoService::derive_signing_key_from_id
    let key_bytes: [u8; 32] = hasher.finalize().into();
    let signing_key = SigningKey::from_bytes(&key_bytes);
    let verifying_key = signing_key.verifying_key();
    let public_key_b64 = STANDARD.encode(verifying_key.as_bytes());

    // Verify with bogus signature
    let verify_request = Request::builder()
        .method("POST")
        .uri("/api/v1/crypto/ed25519/verify")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&json!({
                "message": message_b64,
                "signature": bogus_signature,
                "public_key": public_key_b64,
            }))
            .expect("Failed to serialize"),
        ))
        .expect("Failed to build request");

    let response = app
        .oneshot(verify_request)
        .await
        .expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let verify_response: ApiResponse<serde_json::Value> =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    assert!(verify_response.success);
    let verify_data = verify_response
        .data
        .as_ref()
        .expect("Missing verification data");

    let valid = verify_data["valid"].as_bool().expect("Missing valid field");

    // Bogus signature should NOT be valid
    assert!(!valid);
}

#[tokio::test]
async fn test_aes_gcm_with_aad() {
    // Setup
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    let plaintext = b"Secret payload";
    let plaintext_b64 = STANDARD.encode(plaintext);
    let aad = b"Additional authenticated data";
    let aad_b64 = STANDARD.encode(aad);

    // Encrypt with AAD
    let encrypt_request = Request::builder()
        .method("POST")
        .uri("/api/v1/crypto/aes-gcm/encrypt")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&json!({
                "data": plaintext_b64,
                "key_id": "test-key-aad",
                "aad": aad_b64,
            }))
            .expect("Failed to serialize"),
        ))
        .expect("Failed to build request");

    let response = app
        .clone()
        .oneshot(encrypt_request)
        .await
        .expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let encrypt_response: ApiResponse<serde_json::Value> =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    let encrypt_data = encrypt_response.data.as_ref().expect("Missing data");
    let ciphertext = encrypt_data["ciphertext"]
        .as_str()
        .expect("Missing ciphertext");
    let nonce = encrypt_data["nonce"].as_str().expect("Missing nonce");
    let tag = encrypt_data["tag"].as_str().expect("Missing tag");

    // Decrypt with correct AAD
    let decrypt_request = Request::builder()
        .method("POST")
        .uri("/api/v1/crypto/aes-gcm/decrypt")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&json!({
                "ciphertext": ciphertext,
                "nonce": nonce,
                "tag": tag,
                "key_id": "test-key-aad",
                "aad": aad_b64,
            }))
            .expect("Failed to serialize"),
        ))
        .expect("Failed to build request");

    let response = app
        .oneshot(decrypt_request)
        .await
        .expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let decrypt_response: ApiResponse<serde_json::Value> =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    assert!(decrypt_response.success);
    let decrypt_data = decrypt_response.data.as_ref().expect("Missing data");
    let decrypted_b64 = decrypt_data["plaintext"]
        .as_str()
        .expect("Missing plaintext");
    let decrypted = STANDARD.decode(decrypted_b64).expect("Invalid base64");

    // Should successfully decrypt with correct AAD
    assert_eq!(decrypted, plaintext);
}
