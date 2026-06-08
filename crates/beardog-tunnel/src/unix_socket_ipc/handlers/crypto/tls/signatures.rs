// SPDX-License-Identifier: AGPL-3.0-or-later

//! TLS 1.3 Signature Operations and Finished MAC
//!
//! This module implements cryptographic signature operations for TLS 1.3 handshakes,
//! including Ed25519 signing for `CertificateVerify` and HMAC computation for the Finished message.
//!
//! # Handlers
//!
//! - [`handle_tls_sign_handshake`] - Sign handshake transcripts with Ed25519
//! - [`handle_tls_compute_finished_verify_data`] - Compute Finished message HMAC
//!
//! # References
//!
//! - RFC 8446 Section 4.4.3: Certificate Verify
//! - RFC 8446 Section 4.4.4: Finished
//! - RFC 8032: Edwards-Curve Digital Signature Algorithm (`EdDSA`)
//! - RFC 2104: HMAC

use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use beardog_core::crypto_service::algorithms::asymmetric;
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::{Sha256, Sha384};
use tracing::{debug, info};

type HmacSha256 = Hmac<Sha256>;
type HmacSha384 = Hmac<Sha384>;

// Import shared utility for key derivation
use super::super::utils::derive_key_from_id;

/// Handle `tls.sign_handshake` method
///
/// Signs TLS handshake messages with Ed25519 for ClientKeyExchange/CertificateVerify.
/// This is used in TLS 1.3 to prove possession of the private key.
///
/// # Parameters
///
/// - `message`: Base64-encoded handshake messages to sign
/// - `algorithm`: Signature algorithm (default: "ed25519")
/// - `key_id`: Key identifier for TLS signing key (optional)
/// - `purpose`: Purpose string for key derivation (default: "`tls_handshake`")
///
/// # Returns
///
/// - `signature`: Base64-encoded Ed25519 signature (64 bytes)
/// - `algorithm`: Algorithm used ("ed25519")
/// - `key_id`: The key identifier used for signing
///
/// # Errors
///
/// Returns an error if:
/// - `message` is missing or invalid base64
/// - Key derivation fails
/// - Ed25519 signing operation fails
///
/// # Example
///
/// ```json
/// {
///   "method": "tls.sign_handshake",
///   "params": {
///     "message": "base64_encoded_handshake_data",
///     "algorithm": "ed25519",
///     "key_id": "tls_signing_key"
///   }
/// }
/// ```
///
/// # Security Notes
///
/// - Uses deterministic Ed25519 (RFC 8032)
/// - Signing key is derived from primal identity
/// - Each signature binds to the handshake transcript
///
/// # References
///
/// - RFC 8032: Edwards-Curve Digital Signature Algorithm (`EdDSA`)
/// - RFC 8446 Section 4.4.3: Certificate Verify
pub async fn handle_tls_sign_handshake(
    params: Option<&Value>,
) -> Result<Value, super::super::super::HandlerError> {
    let params = params.ok_or("Missing params for tls.sign_handshake")?;

    // Extract parameters
    let message_b64 = params
        .get("message")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: message")?;

    let algorithm = params
        .get("algorithm")
        .and_then(|v| v.as_str())
        .unwrap_or("ed25519");

    let key_id = params
        .get("key_id")
        .and_then(|v| v.as_str())
        .unwrap_or("tls_signing_key");

    let purpose = params
        .get("purpose")
        .and_then(|v| v.as_str())
        .unwrap_or("tls_handshake");

    // Decode message
    let message = base64::engine::general_purpose::STANDARD
        .decode(message_b64)
        .map_err(|e| format!("Invalid base64 message: {e}"))?;

    debug!(
        "✍️  Signing TLS handshake ({} bytes) with {} (key_id: {}, purpose: {})",
        message.len(),
        algorithm,
        key_id,
        purpose
    );

    // Only Ed25519 is supported for now (most common in modern TLS)
    if algorithm != "ed25519" {
        return Err(format!("Unsupported algorithm: {algorithm}").into());
    }

    // Derive TLS-specific signing key (includes "tls_handshake" in context)
    let seed = derive_key_from_id(key_id, purpose)?;
    let (secret_key, _public_key) = asymmetric::generate_ed25519_from_seed(&seed)
        .map_err(|e| format!("Failed to generate Ed25519 keypair: {e}"))?;

    // Sign the handshake messages
    let signature = asymmetric::sign_ed25519(&message, &secret_key)
        .map_err(|e| format!("Ed25519 signing failed: {e}"))?;

    // Encode signature
    let signature_b64 = base64::engine::general_purpose::STANDARD.encode(&signature);

    info!(
        "✅ TLS handshake signature generated ({} bytes, algorithm: {})",
        signature.len(),
        algorithm
    );

    Ok(serde_json::json!({
        "signature": signature_b64,
        "algorithm": "Ed25519",
        "key_id": key_id,
        "purpose": purpose
    }))
}

/// Handle `tls.compute_finished_verify_data` method
///
/// Computes the TLS 1.3 Finished message `verify_data` for client or server.
/// Implements RFC 8446 Section 4.4.4:
///
/// ```text
/// finished_key = HKDF-Expand-Label(BaseKey, "finished", "", Hash.length)
/// verify_data = HMAC(finished_key, Transcript-Hash(messages))
/// ```
///
/// # Algorithm
///
/// Where:
/// - `BaseKey` is either `client_handshake_traffic_secret` or `server_handshake_traffic_secret`
/// - Transcript-Hash covers all handshake messages up to (but not including) Finished
/// - HMAC uses SHA-256 or SHA-384 depending on cipher suite
///
/// # Parameters
///
/// - `base_key` (base64): The handshake traffic secret (32 bytes for SHA-256)
/// - `transcript_hash` (base64): SHA-256 hash of handshake messages
///
/// # Returns
///
/// A JSON object containing:
/// - `verify_data`: Base64-encoded HMAC (32 bytes for SHA-256 cipher suites)
///
/// # Errors
///
/// Returns an error if:
/// - `base_key` is missing or invalid base64
/// - `transcript_hash` is missing or invalid base64
/// - HKDF-Expand-Label fails
/// - HMAC computation fails
///
/// # Example
///
/// ```json
/// {
///   "method": "tls.compute_finished_verify_data",
///   "params": {
///     "base_key": "base64_client_handshake_traffic_secret",
///     "transcript_hash": "base64_transcript_hash_up_to_certificate_verify"
///   }
/// }
/// ```
///
/// # Security Notes
///
/// - Provides key confirmation and handshake integrity
/// - Binds the identity to the handshake transcript
/// - Prevents downgrade attacks and MITM tampering
/// - Both client and server compute their own Finished messages
///
/// # References
///
/// - RFC 8446 Section 4.4.4: Finished
/// - RFC 5869: HKDF
/// - RFC 2104: HMAC
pub async fn handle_tls_compute_finished_verify_data(
    params: Option<&Value>,
) -> Result<Value, super::super::super::HandlerError> {
    let params = params.ok_or("Missing params for tls.compute_finished_verify_data")?;

    // Extract parameters
    let base_key_b64 = params
        .get("base_key")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: base_key")?;

    let transcript_hash_b64 = params
        .get("transcript_hash")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: transcript_hash")?;

    // NEW: Extract cipher_suite (default to SHA-256 for backwards compatibility)
    #[expect(
        clippy::cast_possible_truncation,
        reason = "IANA TLS cipher suite identifier"
    )]
    let cipher_suite = params
        .get("cipher_suite")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0x1301) as u16;

    // Decode from base64
    let base_key = BASE64_STANDARD
        .decode(base_key_b64)
        .map_err(|e| format!("Invalid base64 for base_key: {e}"))?;

    let transcript_hash = BASE64_STANDARD
        .decode(transcript_hash_b64)
        .map_err(|e| format!("Invalid base64 for transcript_hash: {e}"))?;

    info!("🔐 Computing TLS 1.3 Finished verify_data");
    info!("   Cipher suite: 0x{:04x}", cipher_suite);
    info!("   Base key: {} bytes", base_key.len());
    info!("   Transcript hash: {} bytes", transcript_hash.len());

    // Validate hash length based on cipher suite
    let (expected_len, hash_algo) = match cipher_suite {
        0x1301 | 0x1303 => (32, "SHA-256"), // TLS_AES_128_GCM_SHA256, TLS_CHACHA20_POLY1305_SHA256
        0x1302 => (48, "SHA-384"),          // TLS_AES_256_GCM_SHA384
        _ => {
            return Err(format!(
                "Unsupported TLS 1.3 cipher suite for finished verify_data: 0x{cipher_suite:04x}"
            )
            .into());
        }
    };

    if transcript_hash.len() != expected_len {
        return Err(format!(
            "Invalid transcript_hash length: {} (expected {} for {} in cipher 0x{:04x})",
            transcript_hash.len(),
            expected_len,
            hash_algo,
            cipher_suite
        )
        .into());
    }

    // Step 1: Derive finished_key using cipher-aware HKDF-Expand-Label
    // finished_key = HKDF-Expand-Label(base_key, "finished", "", hash_len)
    // RFC 8446 Section 7.1: Label MUST include "tls13 " prefix!

    let (_finished_key, verify_data) = match cipher_suite {
        0x1301 | 0x1303 => {
            // SHA-256 path
            let hkdf_expand_label_sha256 =
                |secret: &[u8], label: &str, context: &[u8], length: usize| {
                    let hkdf = Hkdf::<Sha256>::from_prk(secret)
                        .map_err(|e| format!("HKDF PRK error: {e}"))?;

                    // RFC 8446 Section 7.1: HkdfLabel structure
                    // CRITICAL: Label must be "tls13 " + label (e.g., "tls13 finished")
                    let mut hkdf_label = Vec::new();
                    super::key_derivation::append_tls13_hkdf_label(
                        &mut hkdf_label,
                        label,
                        context,
                        length,
                    );

                    let mut output = vec![0u8; length];
                    hkdf.expand(&hkdf_label, &mut output)
                        .map_err(|e| format!("HKDF expand error: {e}"))?;

                    Ok::<Vec<u8>, String>(output)
                };

            let finished_key = hkdf_expand_label_sha256(&base_key, "finished", &[], 32)?;
            info!(
                "✅ Derived finished_key (SHA-256): {} bytes",
                finished_key.len()
            );

            // Compute verify_data = HMAC-SHA256(finished_key, transcript_hash)
            let mut mac = HmacSha256::new_from_slice(&finished_key)
                .map_err(|e| format!("HMAC key error: {e}"))?;
            mac.update(&transcript_hash);
            let verify_data = mac.finalize().into_bytes().to_vec();

            (finished_key, verify_data)
        }
        0x1302 => {
            // SHA-384 path
            let hkdf_expand_label_sha384 =
                |secret: &[u8], label: &str, context: &[u8], length: usize| {
                    let hkdf = Hkdf::<Sha384>::from_prk(secret)
                        .map_err(|e| format!("HKDF PRK error: {e}"))?;

                    // RFC 8446 Section 7.1: HkdfLabel structure
                    // CRITICAL: Label must be "tls13 " + label (e.g., "tls13 finished")
                    let mut hkdf_label = Vec::new();
                    super::key_derivation::append_tls13_hkdf_label(
                        &mut hkdf_label,
                        label,
                        context,
                        length,
                    );

                    let mut output = vec![0u8; length];
                    hkdf.expand(&hkdf_label, &mut output)
                        .map_err(|e| format!("HKDF expand error: {e}"))?;

                    Ok::<Vec<u8>, String>(output)
                };

            let finished_key = hkdf_expand_label_sha384(&base_key, "finished", &[], 48)?;
            info!(
                "✅ Derived finished_key (SHA-384): {} bytes",
                finished_key.len()
            );

            // Compute verify_data = HMAC-SHA384(finished_key, transcript_hash)
            let mut mac = HmacSha384::new_from_slice(&finished_key)
                .map_err(|e| format!("HMAC key error: {e}"))?;
            mac.update(&transcript_hash);
            let verify_data = mac.finalize().into_bytes().to_vec();

            (finished_key, verify_data)
        }
        _ => {
            return Err(format!(
                "unsupported TLS 1.3 cipher suite variant in Finished verify_data derivation: 0x{cipher_suite:04x} (expected 0x1301, 0x1302, or 0x1303 after prior validation)"
            ).into());
        }
    };

    info!("✅ Computed verify_data: {} bytes", verify_data.len());
    info!("   Hash algorithm: {}", hash_algo);
    info!("   Verify data (hex): {}", hex::encode(&verify_data));

    Ok(serde_json::json!({
        "verify_data": BASE64_STANDARD.encode(&verify_data),
        "length": verify_data.len(),
        "hash_algorithm": hash_algo,
        "cipher_suite": format!("0x{:04x}", cipher_suite),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sign_handshake_roundtrip() {
        let msg = b"hello tls transcript";
        let params = serde_json::json!({
            "message": BASE64_STANDARD.encode(msg),
            "algorithm": "ed25519",
            "key_id": "test_key",
            "purpose": "tls_handshake",
        });
        let out = handle_tls_sign_handshake(Some(&params))
            .await
            .expect("tls.sign_handshake should succeed with valid test params");
        assert_eq!(out["algorithm"], "Ed25519");
        assert_eq!(out["key_id"], "test_key");
        let sig_b64 = out["signature"]
            .as_str()
            .expect("handler must return signature as string");
        let sig = BASE64_STANDARD
            .decode(sig_b64)
            .expect("signature must be valid base64");
        assert_eq!(sig.len(), 64);
    }

    #[tokio::test]
    async fn sign_handshake_errors() {
        assert!(handle_tls_sign_handshake(None).await.is_err());
        let p = serde_json::json!({
            "message": "!!!",
            "algorithm": "ed25519",
        });
        assert!(handle_tls_sign_handshake(Some(&p)).await.is_err());
        let p2 = serde_json::json!({
            "message": BASE64_STANDARD.encode(b"x"),
            "algorithm": "rsa-pss",
        });
        assert!(handle_tls_sign_handshake(Some(&p2)).await.is_err());
    }

    #[tokio::test]
    async fn finished_verify_data_sha256_suites() {
        let base_key = [7u8; 32];
        let th = [9u8; 32];
        for suite in [0x1301u16, 0x1303u16] {
            let params = serde_json::json!({
                "base_key": BASE64_STANDARD.encode(base_key),
                "transcript_hash": BASE64_STANDARD.encode(th),
                "cipher_suite": suite,
            });
            let out = handle_tls_compute_finished_verify_data(Some(&params))
                .await
                .expect("finished verify_data should succeed for valid SHA-256 suite inputs");
            assert_eq!(out["length"], 32);
            assert_eq!(out["hash_algorithm"], "SHA-256");
        }
    }

    #[tokio::test]
    async fn finished_verify_data_sha384() {
        let base_key = [3u8; 48];
        let th = [0xabu8; 48];
        let params = serde_json::json!({
            "base_key": BASE64_STANDARD.encode(base_key),
            "transcript_hash": BASE64_STANDARD.encode(th),
            "cipher_suite": 0x1302,
        });
        let out = handle_tls_compute_finished_verify_data(Some(&params))
            .await
            .expect("finished verify_data should succeed for SHA-384 suite inputs");
        assert_eq!(out["length"], 48);
        assert_eq!(out["hash_algorithm"], "SHA-384");
    }

    #[tokio::test]
    async fn finished_verify_data_errors() {
        assert!(handle_tls_compute_finished_verify_data(None).await.is_err());
        let p = serde_json::json!({
            "base_key": "x",
            "transcript_hash": base64::engine::general_purpose::STANDARD.encode([0u8; 32]),
        });
        assert!(
            handle_tls_compute_finished_verify_data(Some(&p))
                .await
                .is_err()
        );
        let p2 = serde_json::json!({
            "base_key": base64::engine::general_purpose::STANDARD.encode([0u8; 32]),
            "transcript_hash": base64::engine::general_purpose::STANDARD.encode([0u8; 16]),
            "cipher_suite": 0x1301,
        });
        assert!(
            handle_tls_compute_finished_verify_data(Some(&p2))
                .await
                .is_err()
        );
        let p3 = serde_json::json!({
            "base_key": base64::engine::general_purpose::STANDARD.encode([0u8; 32]),
            "transcript_hash": base64::engine::general_purpose::STANDARD.encode([0u8; 32]),
            "cipher_suite": 0x9999,
        });
        assert!(
            handle_tls_compute_finished_verify_data(Some(&p3))
                .await
                .is_err()
        );
    }
}
