// SPDX-License-Identifier: AGPL-3.0-or-later

//! Crypto fault injection tests: adversarial inputs, key corruption, and edge cases.
//!
//! These tests exercise JSON-RPC crypto handlers with intentionally malformed or
//! boundary-condition inputs to verify stable error handling.

use base64::{Engine, engine::general_purpose::STANDARD as B64};
use serde_json::json;

use crate::unix_socket_ipc::handlers::crypto::{
    handle_blake3_hash, handle_chacha20_poly1305_decrypt, handle_chacha20_poly1305_encrypt,
    handle_ed25519_generate_keypair, handle_sign_ed25519, handle_verify_ed25519,
    handle_x25519_derive_secret, handle_x25519_generate_ephemeral,
};

fn b64_zeros(n: usize) -> String {
    B64.encode(vec![0u8; n])
}

fn b64_ones(n: usize) -> String {
    B64.encode(vec![0xFFu8; n])
}

// -- Blake3 / SHA-style: malformed base64 --

#[tokio::test]
async fn blake3_hash_malformed_base64_returns_error() {
    let params = json!({ "data": "not-valid-base64!!!" });
    let err = handle_blake3_hash(Some(&params))
        .await
        .expect_err("malformed base64 should fail");
    assert!(
        err.to_lowercase().contains("base64") || err.to_lowercase().contains("invalid"),
        "unexpected error: {err}"
    );
}

#[tokio::test]
async fn blake3_hash_missing_data_field_returns_error() {
    let params = json!({});
    let err = handle_blake3_hash(Some(&params))
        .await
        .expect_err("missing data field should fail");
    assert!(!err.is_empty());
}

#[tokio::test]
async fn blake3_hash_null_params_returns_error() {
    let err = handle_blake3_hash(None)
        .await
        .expect_err("null params should fail");
    assert!(!err.is_empty());
}

// -- ChaCha20-Poly1305: wrong key size --

#[tokio::test]
async fn chacha_encrypt_wrong_key_size_16_bytes() {
    let params = json!({
        "plaintext": B64.encode(b"hello"),
        "key": b64_zeros(16),
    });
    let err = handle_chacha20_poly1305_encrypt(Some(&params))
        .await
        .expect_err("16-byte key should be rejected");
    let msg = err.to_lowercase();
    assert!(
        msg.contains("key") || msg.contains("32") || msg.contains("length"),
        "unexpected error: {err}"
    );
}

// -- ChaCha20-Poly1305: all-zero key (valid length, weak key) --

#[tokio::test]
async fn chacha_encrypt_zero_key_succeeds_without_panic() {
    let params = json!({
        "plaintext": B64.encode(b"Hello"),
        "key": b64_zeros(32),
    });
    let result = handle_chacha20_poly1305_encrypt(Some(&params)).await;
    assert!(result.is_ok(), "zero key should still encrypt: {result:?}");
}

// -- ChaCha20-Poly1305: all-0xFF key --

#[tokio::test]
async fn chacha_encrypt_all_ones_key_succeeds() {
    let params = json!({
        "plaintext": B64.encode(b"World"),
        "key": b64_ones(32),
    });
    let result = handle_chacha20_poly1305_encrypt(Some(&params)).await;
    assert!(result.is_ok(), "0xFF key should still encrypt: {result:?}");
}

// -- ChaCha20-Poly1305 decrypt: wrong tag (authentication failure) --

#[tokio::test]
async fn chacha_decrypt_wrong_tag_fails_authentication() {
    let key = b64_zeros(32);
    let enc_params = json!({
        "plaintext": B64.encode(b"secret"),
        "key": &key,
    });
    let encrypted = handle_chacha20_poly1305_encrypt(Some(&enc_params))
        .await
        .expect("encrypt should succeed");

    let ciphertext = encrypted["ciphertext"].as_str().expect("ciphertext field");
    let nonce = encrypted["nonce"].as_str().expect("nonce field");

    let wrong_tag = b64_zeros(16);
    let dec_params = json!({
        "ciphertext": ciphertext,
        "key": &key,
        "nonce": nonce,
        "tag": &wrong_tag,
    });
    let err = handle_chacha20_poly1305_decrypt(Some(&dec_params))
        .await
        .expect_err("wrong tag should fail authentication");
    assert!(
        !err.is_empty(),
        "should produce a meaningful error on wrong tag"
    );
}

// -- ChaCha20-Poly1305 decrypt: truncated ciphertext --

#[tokio::test]
async fn chacha_decrypt_truncated_ciphertext_fails() {
    let key = b64_ones(32);
    let enc_params = json!({
        "plaintext": B64.encode(b"a long message to make ciphertext big enough to truncate"),
        "key": &key,
    });
    let encrypted = handle_chacha20_poly1305_encrypt(Some(&enc_params))
        .await
        .expect("encrypt");

    let ct_bytes = B64
        .decode(encrypted["ciphertext"].as_str().expect("ct"))
        .expect("decode ct");
    let truncated = B64.encode(&ct_bytes[..ct_bytes.len() / 2]);
    let nonce = encrypted["nonce"].as_str().expect("nonce");
    let tag = encrypted["tag"].as_str().expect("tag");

    let dec_params = json!({
        "ciphertext": &truncated,
        "key": &key,
        "nonce": nonce,
        "tag": tag,
    });
    let err = handle_chacha20_poly1305_decrypt(Some(&dec_params))
        .await
        .expect_err("truncated ciphertext should fail");
    assert!(!err.is_empty());
}

// -- X25519: malformed base64 in our_secret --

#[tokio::test]
async fn x25519_derive_malformed_our_secret() {
    let params = json!({
        "our_secret": "!!!not-base64!!!",
        "their_public": b64_zeros(32),
    });
    let err = handle_x25519_derive_secret(Some(&params))
        .await
        .expect_err("malformed base64 should fail");
    assert!(
        err.to_lowercase().contains("base64") || err.to_lowercase().contains("secret"),
        "unexpected: {err}"
    );
}

// -- X25519: wrong length after decode --

#[tokio::test]
async fn x25519_derive_wrong_length_our_secret() {
    let params = json!({
        "our_secret": b64_zeros(16),
        "their_public": b64_zeros(32),
    });
    let err = handle_x25519_derive_secret(Some(&params))
        .await
        .expect_err("16-byte secret should be rejected");
    let msg = err.to_lowercase();
    assert!(
        msg.contains("32") || msg.contains("length") || msg.contains("secret"),
        "unexpected: {err}"
    );
}

// -- Ed25519: verify with corrupted signature --

#[tokio::test]
async fn ed25519_verify_corrupted_signature_fails() {
    let keypair = handle_ed25519_generate_keypair(None).await.expect("keygen");
    let public_key = keypair["public_key"].as_str().expect("pub key");

    let message = B64.encode(b"important data");
    let sign_params = json!({
        "message": &message,
    });
    let signed = handle_sign_ed25519(Some(&sign_params)).await.expect("sign");
    let signature = signed["signature"].as_str().expect("sig");

    let mut sig_bytes = B64.decode(signature).expect("decode sig");
    sig_bytes[0] ^= 0xFF;
    let corrupted_sig = B64.encode(&sig_bytes);

    let verify_params = json!({
        "message": &message,
        "signature": &corrupted_sig,
        "public_key": public_key,
    });
    let result = handle_verify_ed25519(Some(&verify_params)).await;
    match result {
        Ok(val) => {
            assert_eq!(
                val.get("valid").and_then(|v| v.as_bool()),
                Some(false),
                "corrupted signature should not verify"
            );
        }
        Err(_) => {} // also acceptable
    }
}

// -- X25519: all-zero public key (low-order point) --

#[tokio::test]
async fn x25519_derive_with_zero_public_key() {
    let ephemeral = handle_x25519_generate_ephemeral(None)
        .await
        .expect("generate ephemeral");
    let our_secret = ephemeral["secret_key"].as_str().expect("secret");

    let params = json!({
        "our_secret": our_secret,
        "their_public": b64_zeros(32),
    });
    let result = handle_x25519_derive_secret(Some(&params)).await;
    // X25519 with all-zero public key produces all-zero shared secret
    // per RFC 7748; the handler should either succeed (protocol-level)
    // or reject it as a low-order point
    match result {
        Ok(val) => {
            assert!(val.get("shared_secret").is_some());
        }
        Err(e) => {
            assert!(
                e.to_lowercase().contains("low") || e.to_lowercase().contains("zero"),
                "unexpected rejection: {e}"
            );
        }
    }
}

// -- ChaCha20-Poly1305 encrypt: empty plaintext --

#[tokio::test]
async fn chacha_encrypt_empty_plaintext() {
    let params = json!({
        "plaintext": B64.encode(b""),
        "key": b64_zeros(32),
    });
    let result = handle_chacha20_poly1305_encrypt(Some(&params)).await;
    assert!(
        result.is_ok(),
        "empty plaintext should be valid AEAD input: {result:?}"
    );
}

// -- ChaCha20-Poly1305 encrypt: missing params --

#[tokio::test]
async fn chacha_encrypt_null_params_returns_error() {
    let err = handle_chacha20_poly1305_encrypt(None)
        .await
        .expect_err("null params should fail");
    assert!(!err.is_empty());
}
