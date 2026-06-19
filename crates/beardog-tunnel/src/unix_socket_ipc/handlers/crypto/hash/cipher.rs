// SPDX-License-Identifier: AGPL-3.0-or-later

use base64::Engine;
use serde_json::Value;
use sha2::{Digest, Sha256, Sha384};
use tracing::{debug, info};

/// Handle `crypto.hash_for_cipher` method
///
/// Cipher-aware hashing for TLS 1.3 - selects hash algorithm based on cipher suite.
///
/// # TLS 1.3 Cipher Suites (RFC 8446)
///
/// - **0x1301** (`TLS_AES_128_GCM_SHA256)`: Uses SHA-256 (32 bytes)
/// - **0x1302** (`TLS_AES_256_GCM_SHA384)`: Uses SHA-384 (48 bytes)
/// - **0x1303** (`TLS_CHACHA20_POLY1305_SHA256)`: Uses SHA-256 (32 bytes)
///
/// # Parameters
///
/// - `data` (base64): Data to hash
/// - `cipher_suite` (number): TLS 1.3 cipher suite ID
///
/// # Returns
///
/// - `hash` (base64): Hash output (32 or 48 bytes depending on cipher)
/// - `algorithm` (string): Hash algorithm used ("SHA-256" or "SHA-384")
/// - `cipher_suite` (number): Echo of input cipher suite
///
/// # Example
///
/// ```json
/// {
///   "method": "crypto.hash_for_cipher",
///   "params": {
///     "data": "base64_encoded_data",
///     "cipher_suite": 4866
///   }
/// }
/// ```
///
/// # TRUE PRIMAL Pattern
///
/// This method enables callers to hash data without knowing which algorithm to use.
/// `BearDog` owns the crypto decisions; callers pass `cipher_suite`.
///
/// # Errors
///
/// Returns `Err` if params are missing, data is not valid base64, or the cipher suite is unrecognized.
pub async fn handle_hash_for_cipher(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.hash_for_cipher")?;

    // Extract parameters
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: data")?;

    #[expect(
        clippy::cast_possible_truncation,
        reason = "IANA TLS cipher suite identifier"
    )]
    let cipher_suite = params
        .get("cipher_suite")
        .and_then(serde_json::Value::as_u64)
        .ok_or("Missing required parameter: cipher_suite")? as u16;

    // Decode data
    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    debug!(
        "🔨 Hashing {} bytes for cipher suite 0x{:04x}",
        data.len(),
        cipher_suite
    );

    // Select hash algorithm based on cipher suite (RFC 8446)
    let (hash, algorithm) = match cipher_suite {
        0x1301 => {
            // TLS_AES_128_GCM_SHA256
            info!("  → Using SHA-256 for cipher suite 0x1301 (TLS_AES_128_GCM_SHA256)");
            (Sha256::digest(&data).to_vec(), "SHA-256")
        }
        0x1302 => {
            // TLS_AES_256_GCM_SHA384
            info!("  → Using SHA-384 for cipher suite 0x1302 (TLS_AES_256_GCM_SHA384)");
            (Sha384::digest(&data).to_vec(), "SHA-384")
        }
        0x1303 => {
            // TLS_CHACHA20_POLY1305_SHA256
            info!("  → Using SHA-256 for cipher suite 0x1303 (TLS_CHACHA20_POLY1305_SHA256)");
            (Sha256::digest(&data).to_vec(), "SHA-256")
        }
        _ => {
            return Err(format!(
                "Unsupported TLS 1.3 cipher suite: 0x{cipher_suite:04x}. Supported: 0x1301 (AES-128-GCM), 0x1302 (AES-256-GCM), 0x1303 (ChaCha20-Poly1305)"
            ));
        }
    };

    // Encode hash
    let hash_b64 = base64::engine::general_purpose::STANDARD.encode(&hash);

    info!(
        "✅ Hash computed: {} bytes using {} (cipher 0x{:04x})",
        hash.len(),
        algorithm,
        cipher_suite
    );

    Ok(serde_json::json!({
        "hash": hash_b64,
        "algorithm": algorithm,
        "hash_length": hash.len(),
        "cipher_suite": cipher_suite
    }))
}
