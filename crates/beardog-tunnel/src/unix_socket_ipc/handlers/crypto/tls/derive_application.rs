// SPDX-License-Identifier: AGPL-3.0-only

//! `handle_tls_derive_application_secrets` — TLS 1.3 application traffic keys.

use base64::Engine;
use serde_json::Value;
use tracing::{debug, info};

use super::helpers::{derive_application_secrets_sha256, derive_application_secrets_sha384};

/// # Errors
///
/// Returns an error if hashing fails.
/// Handle `tls.derive_application_secrets` method
///
/// Derives TLS 1.3 APPLICATION traffic secrets using the full RFC 8446 key schedule.
/// This is the SECOND key derivation stage - used for encrypting HTTP application data.
///
/// # RFC 8446 Compliance
///
/// This function implements the **application secret derivation stage** of the TLS 1.3
/// key schedule (RFC 8446 Section 7.1):
///
/// ```text
/// Handshake Secret (input)
///     ↓
/// Derive-Secret("derived", "")
///     ↓
/// HKDF-Extract(0) → Master Secret
///     ↓
/// Derive-Secret("c ap traffic", transcript) → Client App Secret
/// Derive-Secret("s ap traffic", transcript) → Server App Secret
///     ↓
/// HKDF-Expand-Label("key") → Encryption Keys
/// HKDF-Expand-Label("iv") → IVs
/// ```
///
/// # Parameters (RFC 8446 Compliant)
///
/// - `handshake_secret`: Base64-encoded handshake secret (32 or 48 bytes depending on cipher) from previous stage
/// - `transcript_hash`: Base64-encoded hash of all handshake messages (32 or 48 bytes depending on cipher)
/// - `cipher_suite` (optional): TLS cipher suite ID (default: 0x1303 = ChaCha20-Poly1305)
///   - 0x1301: Uses SHA-256 (32-byte hashes)
///   - 0x1302: Uses SHA-384 (48-byte hashes)
///   - 0x1303: Uses SHA-256 (32-byte hashes)
///
/// # Returns
///
/// - `client_write_key`: Base64-encoded client encryption key (16 or 32 bytes)
/// - `server_write_key`: Base64-encoded server encryption key (16 or 32 bytes)
/// - `client_write_iv`: Base64-encoded client IV/nonce (12 bytes)
/// - `server_write_iv`: Base64-encoded server IV/nonce (12 bytes)
/// - `client_application_secret`: Base64-encoded client traffic secret (32 bytes)
/// - `server_application_secret`: Base64-encoded server traffic secret (32 bytes)
///
/// # Difference from `tls.derive_handshake_secrets`
///
/// - `tls.derive_handshake_secrets`: ECDH → Handshake Secret → Handshake Keys
/// - `tls.derive_application_secrets`: Handshake Secret → Master Secret → App Keys
///
/// Both follow RFC 8446, but at different stages of the key schedule.
///
/// # Example
///
/// ```json
/// {
///   "method": "tls.derive_application_secrets",
///   "params": {
///     "handshake_secret": "base64_encoded_32_bytes",
///     "transcript_hash": "base64_encoded_sha256_of_all_handshake_messages",
///     "cipher_suite": 4865
///   }
/// }
/// ```
pub async fn handle_tls_derive_application_secrets(
    params: Option<&Value>,
) -> Result<Value, String> {
    // EXECUTION TRACE: Log immediately to confirm function entry
    info!("🚀 ENTERED handle_tls_derive_application_secrets (RFC 8446 Compliant)");

    let params = params.ok_or("Missing params for tls.derive_application_secrets")?;
    info!("✅ Parameters parsed successfully");

    // Extract RFC 8446 compliant parameters
    let handshake_secret_b64 = params
        .get("handshake_secret")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: handshake_secret (from derive_handshake_secrets)")?;

    // REQUIRED: transcript_hash (RFC 8446 Section 7.1)
    let transcript_hash_b64 = params
        .get("transcript_hash")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: transcript_hash (SHA-256 of all handshake messages)")?;

    // Extract cipher_suite (for dynamic key length derivation)
    #[expect(
        clippy::cast_possible_truncation,
        reason = "IANA TLS cipher suite identifier"
    )]
    let cipher_suite = params
        .get("cipher_suite")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0x1303) as u16; // Default to ChaCha20-Poly1305

    info!("🔐 Cipher suite: 0x{:04x}", cipher_suite);

    // Decode RFC 8446 compliant parameters
    let handshake_secret = base64::engine::general_purpose::STANDARD
        .decode(handshake_secret_b64)
        .map_err(|e| format!("Invalid base64 handshake_secret: {e}"))?;

    let transcript_hash = base64::engine::general_purpose::STANDARD
        .decode(transcript_hash_b64)
        .map_err(|e| format!("Invalid base64 transcript_hash: {e}"))?;

    // Determine hash algorithm and key length based on cipher suite (RFC 8446)
    let (hash_algo, hash_len, key_len, iv_len) = match cipher_suite {
        0x1301 => {
            info!(
                "  → Cipher suite: 0x1301 (TLS_AES_128_GCM_SHA256) - using SHA-256, 16-byte keys"
            );
            ("SHA-256", 32, 16, 12)
        }
        0x1302 => {
            info!(
                "  → Cipher suite: 0x1302 (TLS_AES_256_GCM_SHA384) - using SHA-384, 32-byte keys"
            );
            ("SHA-384", 48, 32, 12)
        }
        0x1303 => {
            info!(
                "  → Cipher suite: 0x1303 (TLS_CHACHA20_POLY1305_SHA256) - using SHA-256, 32-byte keys"
            );
            ("SHA-256", 32, 32, 12)
        }
        _ => {
            return Err(format!(
                "Unsupported TLS 1.3 cipher suite: 0x{cipher_suite:04x}. Supported: 0x1301, 0x1302, 0x1303"
            ));
        }
    };

    // Validate parameter sizes (RFC 8446)
    if handshake_secret.len() != hash_len {
        return Err(format!(
            "handshake_secret must be {} bytes for {} (got {} bytes)",
            hash_len,
            hash_algo,
            handshake_secret.len()
        ));
    }

    if transcript_hash.len() != hash_len {
        return Err(format!(
            "transcript_hash must be {} bytes for {} (got {} bytes)",
            hash_len,
            hash_algo,
            transcript_hash.len()
        ));
    }

    info!(
        "✅ Base64 decoding complete: handshake_secret={} bytes, transcript_hash={} bytes",
        handshake_secret.len(),
        transcript_hash.len()
    );

    // EXECUTION TRACE: About to enter comprehensive debug logging
    info!("🎯 CHECKPOINT: Starting RFC 8446 compliant key derivation...");

    // VERSION MARKER: v0.19.0+ RFC 8446 Compliant Application Secret Derivation with SHA-384 Support
    info!("════════════════════════════════════════════════════════════");
    info!("🔍 BEARDOG v0.19.0+ APPLICATION KEY DERIVATION - RFC 8446 COMPLIANT (SHA-384 READY)");
    info!("════════════════════════════════════════════════════════════");
    info!("RFC 8446 Section 7.1: Application Secret Derivation");
    info!(
        "  • Handshake secret: {} bytes (from derive_handshake_secrets)",
        handshake_secret.len()
    );
    info!(
        "  • Transcript hash: {} bytes ({} of all handshake messages)",
        transcript_hash.len(),
        hash_algo
    );
    info!(
        "  • Transcript hash (hex): {}",
        hex::encode(&transcript_hash)
    );
    info!(
        "  • Cipher suite: 0x{:04x} → hash: {}, key_len: {} bytes",
        cipher_suite, hash_algo, key_len
    );

    // Dispatch to hash-specific derivation based on cipher suite
    let (
        client_app_secret,
        server_app_secret,
        client_write_key,
        server_write_key,
        client_write_iv,
        server_write_iv,
    ) = match cipher_suite {
        0x1301 | 0x1303 => {
            // SHA-256 based cipher suites
            derive_application_secrets_sha256(
                &handshake_secret,
                &transcript_hash,
                hash_len,
                key_len,
            )?
        }
        0x1302 => {
            // SHA-384 based cipher suite
            derive_application_secrets_sha384(
                &handshake_secret,
                &transcript_hash,
                hash_len,
                key_len,
            )?
        }
        _ => {
            return Err(format!(
                "unsupported cipher suite variant reached in TLS 1.3 application key derivation: 0x{cipher_suite:04x} (expected 0x1301, 0x1302, or 0x1303 after validation)"
            ));
        }
    };

    info!("────────────────────────────────────────────────────────────");
    info!("RFC 8446 Key Schedule - Application Stage ({}):", hash_algo);
    info!("────────────────────────────────────────────────────────────");
    info!(
        "  ✅ Client Application Traffic Secret ({} bytes):",
        client_app_secret.len()
    );
    info!("         {}", hex::encode(&client_app_secret));
    info!("");
    info!(
        "  ✅ Server Application Traffic Secret ({} bytes):",
        server_app_secret.len()
    );
    info!("         {}", hex::encode(&server_app_secret));
    info!("");
    info!("  ✅ Client Write Key ({} bytes):", client_write_key.len());
    info!("         {}", hex::encode(&client_write_key));
    info!("  ✅ Client Write IV ({} bytes):", client_write_iv.len());
    info!("         {}", hex::encode(&client_write_iv));
    info!("");
    info!("  ✅ Server Write Key ({} bytes):", server_write_key.len());
    info!("         {}", hex::encode(&server_write_key));
    info!("  ✅ Server Write IV ({} bytes):", server_write_iv.len());
    info!("         {}", hex::encode(&server_write_iv));

    // Encode results to base64
    let client_write_key_b64 = base64::engine::general_purpose::STANDARD.encode(&client_write_key);
    let server_write_key_b64 = base64::engine::general_purpose::STANDARD.encode(&server_write_key);
    let client_write_iv_b64 = base64::engine::general_purpose::STANDARD.encode(&client_write_iv);
    let server_write_iv_b64 = base64::engine::general_purpose::STANDARD.encode(&server_write_iv);

    // Also encode traffic secrets (for key updates and debugging, RFC 8446 Section 7.2)
    let client_app_secret_b64 =
        base64::engine::general_purpose::STANDARD.encode(&client_app_secret);
    let server_app_secret_b64 =
        base64::engine::general_purpose::STANDARD.encode(&server_app_secret);

    info!("════════════════════════════════════════════════════════════");
    info!("✅ TLS 1.3 APPLICATION secrets derived successfully! (RFC 8446 Section 7.1)");
    info!("   Cipher suite: 0x{:04x}", cipher_suite);
    info!("   Hash algorithm: {}", hash_algo);
    info!("   Key length: {} bytes", key_len);
    info!("   IV length: {} bytes", iv_len);
    info!("   Mode: RFC 8446 Full Compliance (SHA-384 Ready)");
    info!("════════════════════════════════════════════════════════════");

    // NOTE: SSLKEYLOGFILE export requires client_random, which is not available in this
    // RFC 8446 compliant API. If you need Wireshark decryption, export from
    // handle_tls_derive_handshake_secrets or pass client_random as an optional parameter.
    debug!("ℹ️  SSLKEYLOGFILE export skipped (client_random not in RFC 8446 compliant API)");

    let algorithm = format!("HKDF-{hash_algo}");

    Ok(serde_json::json!({
        "client_write_key": client_write_key_b64,
        "server_write_key": server_write_key_b64,
        "client_write_iv": client_write_iv_b64,
        "server_write_iv": server_write_iv_b64,
        "client_application_secret": client_app_secret_b64,  // For key updates (RFC 8446 Section 7.2)
        "server_application_secret": server_app_secret_b64,  // For key updates (RFC 8446 Section 7.2)
        "algorithm": algorithm,
        "hash_algorithm": hash_algo,
        "hash_length": hash_len,
        "rfc": "RFC 8446 Section 7.1",
        "mode": "RFC 8446 Full Compliance (SHA-384 Ready)",
        "stage": "application",
        "key_length": key_len,
        "iv_length": iv_len,
        "cipher_suite": cipher_suite
    }))
}
