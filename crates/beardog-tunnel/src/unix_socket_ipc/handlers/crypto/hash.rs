//! Hashing and MAC operations
//!
//! This module provides cryptographic hashing and message authentication code
//! (MAC) operations for ecoPrimals.
//!
//! # Overview
//!
//! Hashing operations:
//! - **Integrity**: Verify data hasn't been corrupted
//! - **Content-Addressed Storage**: Use hash as identifier
//! - **Key Derivation**: Derive keys from passwords/secrets
//! - **Digital Signatures**: Hash messages before signing
//!
//! Message Authentication Codes (MACs):
//! - **Authentication**: Verify message sender
//! - **Integrity**: Detect tampering
//! - **Key-Based**: Requires shared secret key
//!
//! # Algorithms
//!
//! ## Blake3
//!
//! - **Type**: Cryptographic hash function
//! - **Output Size**: 32 bytes (256-bit, default)
//! - **Speed**: Extremely fast (~3 GB/s single-core, highly parallelizable)
//! - **Security**: 128-bit collision resistance
//! - **Use Case**: Primary hash for BearDog (faster than SHA-256)
//!
//! ### Methods
//!
//! - [`handle_blake3_hash`] - Compute Blake3 hash
//!
//! ## HMAC-SHA256
//!
//! - **Type**: Hash-based Message Authentication Code
//! - **Hash**: SHA-256
//! - **Key Size**: Variable (recommend 32 bytes)
//! - **Output Size**: 32 bytes (256-bit)
//! - **Security**: Keyed MAC for authentication
//! - **Use Case**: TLS Finished messages, API authentication
//!
//! ### Methods
//!
//! - [`handle_hmac_sha256`] - Compute HMAC-SHA256
//!
//! # Usage
//!
//! All handlers are re-exported from the parent `crypto` module:
//!
//! ```rust,ignore
//! // NOTE: These handlers are internal and called via JSON-RPC
//! use crate::unix_socket_ipc::handlers::crypto::*;
//!
//! // Compute Blake3 hash
//! let hash = handle_blake3_hash(params).await?;
//!
//! // Compute HMAC-SHA256
//! let mac = handle_hmac_sha256(params).await?;
//! ```
//!
//! # Security Notes
//!
//! - **Don't Hash Passwords**: Use Argon2id, bcrypt, or scrypt instead
//! - **Use MACs for Authentication**: Plain hashes don't authenticate
//! - **Key Management**: Protect HMAC keys like encryption keys
//!
//! # References
//!
//! - Blake3: <https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf>
//! - RFC 2104 (HMAC): <https://www.rfc-editor.org/rfc/rfc2104.html>
//! - FIPS 180-4 (SHA-256): <https://csrc.nist.gov/publications/detail/fips/180/4/final>

use base64::Engine;
use serde_json::Value;
use sha2::{Digest, Sha256, Sha384};
use tracing::{debug, info};

/// Handle crypto.hash_for_cipher method
///
/// Cipher-aware hashing for TLS 1.3 - selects hash algorithm based on cipher suite.
///
/// # TLS 1.3 Cipher Suites (RFC 8446)
///
/// - **0x1301** (TLS_AES_128_GCM_SHA256): Uses SHA-256 (32 bytes)
/// - **0x1302** (TLS_AES_256_GCM_SHA384): Uses SHA-384 (48 bytes)
/// - **0x1303** (TLS_CHACHA20_POLY1305_SHA256): Uses SHA-256 (32 bytes)
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
/// This method enables Songbird to hash data without knowing which algorithm to use.
/// BearDog owns the crypto decisions, Songbird just passes cipher_suite.
pub async fn handle_hash_for_cipher(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.hash_for_cipher")?;

    // Extract parameters
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: data")?;

    let cipher_suite = params
        .get("cipher_suite")
        .and_then(|v| v.as_u64())
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
                "Unsupported TLS 1.3 cipher suite: 0x{:04x}. Supported: 0x1301 (AES-128-GCM), 0x1302 (AES-256-GCM), 0x1303 (ChaCha20-Poly1305)",
                cipher_suite
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

pub async fn handle_blake3_hash(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.blake3_hash")?;

    // Extract parameters
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: data")?;

    // Decode data
    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    debug!("🔨 Hashing {} bytes with BLAKE3", data.len());

    // Use BearDog's crypto service
    use beardog_core::crypto_service::algorithms::hashing;

    let hash = hashing::hash_blake3(&data);

    // Encode hash
    let hash_b64 = base64::engine::general_purpose::STANDARD.encode(&hash);

    info!("✅ BLAKE3 hash computed ({} bytes)", hash.len());

    Ok(serde_json::json!({
        "hash": hash_b64,
        "algorithm": "BLAKE3",
    }))
}

/// Handle crypto.hmac_sha256 method
///
/// Computes HMAC-SHA256 authentication tag.
///
/// # Parameters
///
/// - `key`: Base64-encoded secret key
/// - `data`: Base64-encoded data to authenticate
///
/// # Returns
///
/// - `mac`: Base64-encoded HMAC-SHA256 tag (32 bytes)
pub async fn handle_hmac_sha256(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.hmac_sha256")?;

    // Extract parameters
    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: key")?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: data")?;

    // Decode parameters
    let key = base64::engine::general_purpose::STANDARD
        .decode(key_b64)
        .map_err(|e| format!("Invalid base64 key: {e}"))?;

    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    debug!("🔐 Computing HMAC-SHA256 for {} bytes", data.len());

    // Use BearDog's crypto service
    use beardog_core::crypto_service::algorithms::hashing;

    let mac = hashing::hmac_sha256(&key, &data)
        .map_err(|e| format!("HMAC-SHA256 computation failed: {e}"))?;

    // Encode MAC
    let mac_b64 = base64::engine::general_purpose::STANDARD.encode(&mac);

    info!("✅ HMAC-SHA256 computed ({} bytes)", mac.len());

    Ok(serde_json::json!({
        "mac": mac_b64,
        "algorithm": "HMAC-SHA256",
    }))
}
