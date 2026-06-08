// SPDX-License-Identifier: AGPL-3.0-or-later

//! `handle_tls_derive_handshake_secrets` — TLS 1.3 handshake traffic keys.

use base64::Engine;
use serde_json::Value;
use tracing::{debug, info, warn};

use super::super::super::sslkeylog::export_to_sslkeylogfile;
use super::helpers::{derive_handshake_secrets_sha256, derive_handshake_secrets_sha384};

/// # Errors
///
/// Returns an error if hashing fails.
/// Handle `tls.derive_handshake_secrets` method
///
/// Derives TLS 1.3 HANDSHAKE traffic secrets using the full RFC 8446 key schedule.
/// This is the FIRST key derivation stage - used for encrypting handshake messages.
///
/// # Parameters
///
/// - `pre_master_secret`: Base64-encoded ECDH shared secret (32 bytes for X25519)
/// - `client_random`: Base64-encoded `ClientHello` random (32 bytes)
/// - `server_random`: Base64-encoded `ServerHello` random (32 bytes)
/// - `transcript_hash`: Base64-encoded SHA-256(ClientHello + `ServerHello`) (32 bytes)
///
/// # Returns
///
/// - `client_write_key`: Base64-encoded client key (32 bytes for `ChaCha20`)
/// - `client_write_iv`: Base64-encoded client IV/nonce (12 bytes)
/// - `server_write_key`: Base64-encoded server key (32 bytes for `ChaCha20`)
/// - `server_write_iv`: Base64-encoded server IV/nonce (12 bytes)
///
/// # Difference from `tls.derive_application_secrets`
///
/// - `tls.derive_handshake_secrets`: Derives HANDSHAKE traffic keys (for handshake messages)
/// - `tls.derive_application_secrets`: Derives APPLICATION traffic keys (for HTTP data)
///
/// Both follow RFC 8446, but at different stages of the key schedule.
pub async fn handle_tls_derive_handshake_secrets(
    params: Option<&Value>,
) -> Result<Value, super::super::super::super::HandlerError> {
    const IV_LEN: usize = 12; // AEAD nonce size (same for all cipher suites)

    let params = params.ok_or("Missing params for tls.derive_handshake_secrets")?;

    // Extract parameters
    let pre_master_secret_b64 = params
        .get("pre_master_secret")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: pre_master_secret")?;

    let client_random_b64 = params
        .get("client_random")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: client_random")?;

    let server_random_b64 = params
        .get("server_random")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: server_random")?;

    // REQUIRED: transcript_hash (RFC 8446 compliance)
    let transcript_hash_b64 = params
        .get("transcript_hash")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: transcript_hash")?;

    // REQUIRED: cipher_suite (RFC 8446 Section 7.3 - determines key length!)
    #[expect(
        clippy::cast_possible_truncation,
        reason = "IANA TLS cipher suite identifier"
    )]
    let cipher_suite = params
        .get("cipher_suite")
        .and_then(serde_json::Value::as_u64)
        .ok_or("Missing required parameter: cipher_suite")? as u16;

    // Decode parameters
    let pre_master_secret = base64::engine::general_purpose::STANDARD
        .decode(pre_master_secret_b64)
        .map_err(|e| format!("Invalid base64 pre_master_secret: {e}"))?;

    let client_random = base64::engine::general_purpose::STANDARD
        .decode(client_random_b64)
        .map_err(|e| format!("Invalid base64 client_random: {e}"))?;

    let server_random = base64::engine::general_purpose::STANDARD
        .decode(server_random_b64)
        .map_err(|e| format!("Invalid base64 server_random: {e}"))?;

    let transcript_hash = base64::engine::general_purpose::STANDARD
        .decode(transcript_hash_b64)
        .map_err(|e| format!("Invalid base64 transcript_hash: {e}"))?;

    // Validate parameter sizes
    if client_random.len() != 32 {
        return Err("client_random must be 32 bytes".to_string().into());
    }

    if server_random.len() != 32 {
        return Err("server_random must be 32 bytes".to_string().into());
    }

    // Determine hash algorithm and key length based on cipher suite (RFC 8446)
    let (hash_algo, hash_len, key_len) = match cipher_suite {
        0x1301 => {
            info!(
                "  → Cipher suite: 0x1301 (TLS_AES_128_GCM_SHA256) - using SHA-256, 16-byte keys"
            );
            ("SHA-256", 32, 16) // SHA-256 hash (32 bytes), AES-128-GCM keys (16 bytes)
        }
        0x1302 => {
            info!(
                "  → Cipher suite: 0x1302 (TLS_AES_256_GCM_SHA384) - using SHA-384, 32-byte keys"
            );
            ("SHA-384", 48, 32) // SHA-384 hash (48 bytes), AES-256-GCM keys (32 bytes)
        }
        0x1303 => {
            info!(
                "  → Cipher suite: 0x1303 (TLS_CHACHA20_POLY1305_SHA256) - using SHA-256, 32-byte keys"
            );
            ("SHA-256", 32, 32) // SHA-256 hash (32 bytes), ChaCha20-Poly1305 keys (32 bytes)
        }
        _ => {
            return Err(format!(
                "Unsupported TLS 1.3 cipher suite: 0x{cipher_suite:04x}. Supported: 0x1301 (AES-128-GCM-SHA256), 0x1302 (AES-256-GCM-SHA384), 0x1303 (ChaCha20-Poly1305-SHA256)"
            ).into());
        }
    };

    // Validate transcript_hash size based on hash algorithm
    if transcript_hash.len() != hash_len {
        return Err(format!(
            "transcript_hash must be {} bytes for {} (got {} bytes)",
            hash_len,
            hash_algo,
            transcript_hash.len()
        )
        .into());
    }

    debug!("🔑 Deriving TLS 1.3 HANDSHAKE secrets (RFC 8446 Section 7.1)");
    debug!(
        "  → pre_master: {} bytes (ECDH shared secret)",
        pre_master_secret.len()
    );
    debug!("  → client_random: {} bytes", client_random.len());
    debug!("  → server_random: {} bytes", server_random.len());
    debug!(
        "  → transcript_hash: {} bytes ({})",
        transcript_hash.len(),
        hash_algo
    );
    debug!(
        "  → cipher_suite: 0x{:04x} → hash: {}, key_len: {} bytes",
        cipher_suite, hash_algo, key_len
    );

    // Dispatch to hash-specific derivation based on cipher suite
    let (
        handshake_secret_bytes,
        client_handshake_secret,
        server_handshake_secret,
        client_write_key,
        server_write_key,
        client_write_iv,
        server_write_iv,
    ) = match cipher_suite {
        0x1301 | 0x1303 => {
            // SHA-256 based cipher suites
            derive_handshake_secrets_sha256(
                &pre_master_secret,
                &transcript_hash,
                hash_len,
                key_len,
            )?
        }
        0x1302 => {
            // SHA-384 based cipher suite
            derive_handshake_secrets_sha384(
                &pre_master_secret,
                &transcript_hash,
                hash_len,
                key_len,
            )?
        }
        _ => {
            return Err(format!(
                "unsupported cipher suite variant reached in TLS 1.3 handshake key derivation: 0x{cipher_suite:04x} (expected 0x1301, 0x1302, or 0x1303 after validation)"
            ).into());
        }
    };

    debug!(
        "  ✅ Keys and IVs derived (hash: {}, key: {} bytes, IV: {} bytes)",
        hash_algo, key_len, IV_LEN
    );

    // HEX DUMPS for derived keys (cross-verify with RFC 8448 and other implementations)
    info!("🔍 BEARDOG DERIVED HANDSHAKE KEYS - FULL HEX DUMPS:");
    info!("   client_write_key: {}", hex::encode(&client_write_key));
    info!("   server_write_key: {}", hex::encode(&server_write_key));
    info!("   client_write_iv: {}", hex::encode(&client_write_iv));
    info!("   server_write_iv: {}", hex::encode(&server_write_iv));

    // Encode to base64
    let client_write_key_b64 = base64::engine::general_purpose::STANDARD.encode(&client_write_key);
    let server_write_key_b64 = base64::engine::general_purpose::STANDARD.encode(&server_write_key);
    let client_write_iv_b64 = base64::engine::general_purpose::STANDARD.encode(&client_write_iv);
    let server_write_iv_b64 = base64::engine::general_purpose::STANDARD.encode(&server_write_iv);

    // Also encode the traffic secrets (needed for Finished message computation, RFC 8446 Section 4.4.4)
    let client_handshake_secret_b64 =
        base64::engine::general_purpose::STANDARD.encode(&client_handshake_secret);
    let server_handshake_secret_b64 =
        base64::engine::general_purpose::STANDARD.encode(&server_handshake_secret);

    // CRITICAL: Also encode the raw handshake_secret (needed for application secrets derivation!)
    // This is the intermediate value in the TLS 1.3 key schedule that feeds into Master Secret
    let handshake_secret_b64 =
        base64::engine::general_purpose::STANDARD.encode(&handshake_secret_bytes);

    info!(
        "✅ TLS 1.3 HANDSHAKE secrets derived (cipher: 0x{:04x}, hash: {}, keys: {} bytes, IVs: {} bytes, RFC 8446 Section 7.3 compliant)",
        cipher_suite, hash_algo, key_len, IV_LEN
    );

    // Export to SSLKEYLOGFILE for Wireshark decryption (if SSLKEYLOGFILE env var is set)
    if let Err(e) = export_to_sslkeylogfile(
        &client_random,
        Some((&client_handshake_secret, &server_handshake_secret)),
        None, // No application secrets yet (will be exported in handle_tls_derive_application_secrets)
    ) {
        warn!("⚠️  Failed to export to SSLKEYLOGFILE: {}", e);
    }

    let algorithm = format!("HKDF-{hash_algo}");

    Ok(serde_json::json!({
        "client_write_key": client_write_key_b64,
        "server_write_key": server_write_key_b64,
        "client_write_iv": client_write_iv_b64,
        "server_write_iv": server_write_iv_b64,
        "client_handshake_secret": client_handshake_secret_b64,  // For Finished message (RFC 8446 Section 4.4.4)
        "server_handshake_secret": server_handshake_secret_b64,  // For Finished message (RFC 8446 Section 4.4.4)
        "handshake_secret": handshake_secret_b64,  // CRITICAL: For application secrets derivation (RFC 8446 Section 7.1)
        "algorithm": algorithm,
        "hash_algorithm": hash_algo,
        "hash_length": hash_len,
        "key_length": key_len,
        "rfc": "RFC 8446 Section 7.1",
        "stage": "handshake",
        "cipher_suite": cipher_suite,
        "mode": "RFC 8446 Full Compliance"
    }))
}
