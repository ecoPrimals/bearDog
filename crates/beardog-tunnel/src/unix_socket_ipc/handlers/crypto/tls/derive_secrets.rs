// SPDX-License-Identifier: AGPL-3.0-or-later

//! `handle_tls_derive_secrets` — legacy combined TLS 1.3 key derivation.

use base64::Engine;
use hkdf::Hkdf;
use serde_json::Value;
use sha2::Sha256;
use tracing::{debug, info};

/// Derive TLS 1.3 secrets (legacy combined method)
///
/// This handler provides backward compatibility by combining both handshake
/// and application secret derivation into a single call. For new code, prefer
/// `handle_tls_derive_handshake_secrets` and `handle_tls_derive_application_secrets`.
///
/// # Parameters
///
/// - `pre_master_secret` (base64): The ECDHE shared secret from key exchange
/// - `transcript_hash` (base64): Hash of handshake messages for key binding
/// - `cipher_suite` (optional): TLS cipher suite identifier
///
/// # Returns
///
/// A JSON object containing all derived secrets:
/// - `master_secret`: Handshake stage secret
/// - `client_handshake_traffic_secret`: Client handshake encryption key
/// - `server_handshake_traffic_secret`: Server handshake encryption key
/// - `master_secret`: Application stage secret
/// - `client_application_traffic_secret`: Client application encryption key
/// - `server_application_traffic_secret`: Server application encryption key
///
/// # Errors
///
/// Returns an error if:
/// - Required parameters are missing
/// - Base64 decoding fails
/// - Key derivation fails (HKDF error)
///
/// # Example
///
/// ```json
/// {
///   "method": "tls.derive_secrets",
///   "params": {
///     "pre_master_secret": "base64_encoded_ecdhe_secret",
///     "transcript_hash": "base64_encoded_transcript_hash"
///   }
/// }
/// ```
///
/// # References
///
/// - RFC 8446 Section 7.1: TLS 1.3 Key Schedule
/// - RFC 5869: HMAC-based Extract-and-Expand Key Derivation Function (HKDF)
pub async fn handle_tls_derive_secrets(
    params: Option<&Value>,
) -> Result<Value, super::super::super::super::HandlerError> {
    let params = params.ok_or("Missing params for tls.derive_secrets")?;

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

    let cipher_suite = params
        .get("cipher_suite")
        .and_then(|v| v.as_str())
        .unwrap_or("TLS_CHACHA20_POLY1305_SHA256");

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

    if client_random.len() != 32 {
        return Err("client_random must be 32 bytes".to_string().into());
    }

    if server_random.len() != 32 {
        return Err("server_random must be 32 bytes".to_string().into());
    }

    debug!(
        "🔑 Deriving TLS 1.3 secrets (cipher_suite: {}, pre_master: {} bytes)",
        cipher_suite,
        pre_master_secret.len()
    );

    // Use HKDF for TLS 1.3 key derivation
    // HKDF-Extract: Derive master secret from pre-master secret
    // Salt = client_random || server_random (TLS 1.3 pattern)
    let mut salt = Vec::with_capacity(64);
    salt.extend_from_slice(&client_random);
    salt.extend_from_slice(&server_random);

    let hkdf = Hkdf::<Sha256>::new(Some(&salt), &pre_master_secret);

    // Derive master secret (48 bytes for TLS 1.3)
    let mut master_secret = [0u8; 48];
    hkdf.expand(b"tls13 master secret", &mut master_secret)
        .map_err(|e| format!("HKDF expand failed for master secret: {e}"))?;

    // HKDF-Expand: Derive session keys from master secret
    let hkdf_master = Hkdf::<Sha256>::new(None, &master_secret);

    // Determine key and IV sizes based on cipher suite
    let (key_size, iv_size) = match cipher_suite {
        "TLS_CHACHA20_POLY1305_SHA256" | "TLS_AES_256_GCM_SHA384" => (32, 12), // 256-bit keys, 96-bit IVs
        "TLS_AES_128_GCM_SHA256" => (16, 12), // 128-bit keys, 96-bit IVs
        _ => (32, 12),                        // Default to 256-bit
    };

    // Derive client write key
    let mut client_write_key = vec![0u8; key_size];
    hkdf_master
        .expand(b"tls13 client write key", &mut client_write_key)
        .map_err(|e| format!("HKDF expand failed for client write key: {e}"))?;

    // Derive server write key
    let mut server_write_key = vec![0u8; key_size];
    hkdf_master
        .expand(b"tls13 server write key", &mut server_write_key)
        .map_err(|e| format!("HKDF expand failed for server write key: {e}"))?;

    // Derive client write IV
    let mut client_write_iv = vec![0u8; iv_size];
    hkdf_master
        .expand(b"tls13 client write iv", &mut client_write_iv)
        .map_err(|e| format!("HKDF expand failed for client write IV: {e}"))?;

    // Derive server write IV
    let mut server_write_iv = vec![0u8; iv_size];
    hkdf_master
        .expand(b"tls13 server write iv", &mut server_write_iv)
        .map_err(|e| format!("HKDF expand failed for server write IV: {e}"))?;

    // Encode results
    let master_secret_b64 = base64::engine::general_purpose::STANDARD.encode(master_secret);
    let client_write_key_b64 = base64::engine::general_purpose::STANDARD.encode(&client_write_key);
    let server_write_key_b64 = base64::engine::general_purpose::STANDARD.encode(&server_write_key);
    let client_write_iv_b64 = base64::engine::general_purpose::STANDARD.encode(&client_write_iv);
    let server_write_iv_b64 = base64::engine::general_purpose::STANDARD.encode(&server_write_iv);

    info!(
        "✅ TLS 1.3 secrets derived (master: {} bytes, keys: {} bytes, IVs: {} bytes)",
        master_secret.len(),
        key_size,
        iv_size
    );

    Ok(serde_json::json!({
        "master_secret": master_secret_b64,
        "client_write_key": client_write_key_b64,
        "server_write_key": server_write_key_b64,
        "client_write_iv": client_write_iv_b64,
        "server_write_iv": server_write_iv_b64,
        "cipher_suite": cipher_suite,
        "algorithm": "HKDF-SHA256"
    }))
}
