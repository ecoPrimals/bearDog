// SPDX-License-Identifier: AGPL-3.0-only

use super::*;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;

fn rand32() -> Vec<u8> {
    (0..32).map(|i| (i * 7 + 13) as u8).collect()
}

#[tokio::test]
async fn derive_secrets_happy_path() {
    let pms = B64.encode([5u8; 32]);
    let cr = B64.encode([1u8; 32]);
    let sr = B64.encode([2u8; 32]);
    let params = serde_json::json!({
        "pre_master_secret": pms,
        "client_random": cr,
        "server_random": sr,
        "cipher_suite": "TLS_AES_128_GCM_SHA256",
    });
    let out = handle_tls_derive_secrets(Some(&params))
        .await
        .expect("derive secrets should succeed for valid TLS 1.3 test inputs");
    assert!(out.get("master_secret").is_some());
    assert_eq!(out["cipher_suite"], "TLS_AES_128_GCM_SHA256");
}

#[tokio::test]
async fn derive_secrets_errors() {
    assert!(handle_tls_derive_secrets(None).await.is_err());
    let bad = serde_json::json!({
        "pre_master_secret": "x",
        "client_random": B64.encode([0u8; 32]),
        "server_random": B64.encode([0u8; 32]),
    });
    assert!(handle_tls_derive_secrets(Some(&bad)).await.is_err());
}

#[tokio::test]
async fn derive_handshake_secrets_sha256() {
    let params = serde_json::json!({
        "pre_master_secret": B64.encode([8u8; 32]),
        "client_random": B64.encode(rand32()),
        "server_random": B64.encode(rand32()),
        "transcript_hash": B64.encode([3u8; 32]),
        "cipher_suite": 0x1303u64,
    });
    let out = handle_tls_derive_handshake_secrets(Some(&params))
        .await
        .expect("handshake secrets should succeed for SHA-256 ChaCha20 suite");
    assert!(out.get("client_write_key").is_some());
    assert!(out.get("hash_algorithm").is_some());
}

#[tokio::test]
async fn derive_handshake_missing_cipher_fails() {
    let params = serde_json::json!({
        "pre_master_secret": B64.encode([8u8; 32]),
        "client_random": B64.encode(rand32()),
        "server_random": B64.encode(rand32()),
        "transcript_hash": B64.encode([3u8; 32]),
    });
    assert!(
        handle_tls_derive_handshake_secrets(Some(&params))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn derive_application_secrets_chacha_poly1305() {
    let params = serde_json::json!({
        "handshake_secret": B64.encode([9u8; 32]),
        "transcript_hash": B64.encode([3u8; 32]),
        "cipher_suite": 0x1303u64,
    });
    let out = handle_tls_derive_application_secrets(Some(&params))
        .await
        .expect("application secrets");
    assert!(out.get("client_write_key").is_some());
    assert_eq!(out["cipher_suite"], 0x1303);
}

#[tokio::test]
async fn derive_application_secrets_missing_handshake_secret() {
    let params = serde_json::json!({
        "transcript_hash": B64.encode([3u8; 32]),
    });
    assert!(
        handle_tls_derive_application_secrets(Some(&params))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn derive_application_secrets_unsupported_cipher_suite() {
    let params = serde_json::json!({
        "handshake_secret": B64.encode([9u8; 32]),
        "transcript_hash": B64.encode([3u8; 32]),
        "cipher_suite": 0x9999u64,
    });
    assert!(
        handle_tls_derive_application_secrets(Some(&params))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn derive_application_secrets_wrong_handshake_length_for_suite() {
    let params = serde_json::json!({
        "handshake_secret": B64.encode([9u8; 16]),
        "transcript_hash": B64.encode([3u8; 32]),
        "cipher_suite": 0x1303u64,
    });
    assert!(
        handle_tls_derive_application_secrets(Some(&params))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn derive_handshake_secrets_sha384_cipher_0x1302() {
    let params = serde_json::json!({
        "pre_master_secret": B64.encode([11u8; 32]),
        "client_random": B64.encode(rand32()),
        "server_random": B64.encode(rand32()),
        "transcript_hash": B64.encode([4u8; 48]),
        "cipher_suite": 0x1302u64,
    });
    let out = handle_tls_derive_handshake_secrets(Some(&params))
        .await
        .expect("handshake secrets sha384");
    assert!(out.get("client_write_key").is_some());
    assert_eq!(out["hash_algorithm"], "SHA-384");
}

#[tokio::test]
async fn derive_application_secrets_sha384_cipher_0x1302() {
    let params = serde_json::json!({
        "handshake_secret": B64.encode([12u8; 48]),
        "transcript_hash": B64.encode([4u8; 48]),
        "cipher_suite": 0x1302u64,
    });
    let out = handle_tls_derive_application_secrets(Some(&params))
        .await
        .expect("application secrets sha384");
    assert!(out.get("server_write_key").is_some());
    assert_eq!(out["hash_algorithm"], "SHA-384");
}
