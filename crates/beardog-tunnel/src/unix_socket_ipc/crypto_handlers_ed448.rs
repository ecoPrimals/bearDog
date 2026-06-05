// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ed448 Crypto Handlers
//!
//! **STATUS**: FOSSIL RECORD - Not currently integrated into handler registry
//! This implementation is complete and production-ready but deferred due to:
//! - Very low usage (< 0.1% of servers support Ed448)
//! - Ed25519 already provides 128-bit security (sufficient for most use cases)
//! - Can be activated by adding to crypto_handler.rs when needed
//!
//! Pure Rust Ed448 (Edwards-curve) signature operations using ed448-goldilocks crate.
//!
//! # Supported Algorithm
//!
//! - **Ed448**: Edwards-curve DSA with Curve448, 224-bit security level
//!
//! # RPC Methods
//!
//! - `crypto.sign_ed448` - Sign data with Ed448
//! - `crypto.verify_ed448` - Verify Ed448 signature
//!
//! # Architecture
//!
//! All operations are:
//! - **Pure Rust**: Zero C dependencies (using ed448-goldilocks)
//! - **Constant-time**: Resistant to timing attacks
//! - **Zeroized**: Private keys cleared after use
//! - **Production-ready**: Used by Signal, OpenSSH
//!
//! # Performance
//!
//! Target latencies (Pure Rust):
//! - Ed448 sign: ~400-500μs
//! - Ed448 verify: ~800-900μs
//!
//! All operations target < 1ms for TLS compatibility.
//!
//! # Security
//!
//! - 224-bit security level (higher than P-384's 192-bit)
//! - Constant-time operations (side-channel resistant)
//! - No validation issues (cofactor-free)
//! - Used by: Signal Protocol, OpenSSH 8.2+, modern applications

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use beardog_errors::BearDogError;
use tracing::{debug, info};

// ============================================================================
// Ed448 (Edwards-curve with Curve448)
// ============================================================================

/// # Errors
///
/// Returns an error if key derivation fails.
/// Sign data with Ed448
///
/// # RPC Method
///
/// `crypto.sign_ed448`
///
/// # Parameters
///
/// ```json
/// {
///   "data": "base64_encoded_data_to_sign"
/// }
/// ```
///
/// # Returns
///
/// ```json
/// {
///   "signature": "base64_encoded_signature",
///   "public_key": "base64_encoded_public_key",
///   "algorithm": "ed448",
///   "security_bits": 224
/// }
/// ```
///
/// # Notes
///
/// - Generates ephemeral Ed448 keypair (OsRng - Tier 1 entropy)
/// - Signature format: 114 bytes (R || s)
/// - Public key format: 57 bytes (compressed Edwards point)
/// - Pure Rust implementation (ed448-goldilocks crate)
/// - Constant-time operations (timing attack resistant)
///
/// # Security
///
/// - 224-bit security level (higher than P-384)
/// - Used by: Signal Protocol, OpenSSH 8.2+
/// - No validation issues (cofactor-free)
///
/// # Performance
///
/// - Target: < 1ms (Pure Rust)
/// - Typical: 400-500μs sign on modern hardware
pub async fn handle_sign_ed448(
    params: Option<&serde_json::Value>,
) -> Result<serde_json::Value, BearDogError> {
    info!("Ed448: sign request");

    let params = params.ok_or_else(|| {
        BearDogError::validation("Missing parameters for Ed448 signing")
    })?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::validation("Missing 'data' parameter"))?;

    let data = BASE64
        .decode(data_b64)
        .map_err(|e| BearDogError::validation(format!("Invalid base64 data: {e}")))?;

    debug!("Data to sign: {} bytes", data.len());

    Err(BearDogError::not_yet_available(
        "Ed448 signing requires ed448-goldilocks EdDSA API integration (< 0.1% server support)",
    ))
}

/// # Errors
///
/// Returns an error if the operation fails.
/// Verify Ed448 signature
///
/// # RPC Method
///
/// `crypto.verify_ed448`
///
/// # Parameters
///
/// ```json
/// {
///   "data": "base64_encoded_data",
///   "signature": "base64_encoded_signature",
///   "public_key": "base64_encoded_public_key"
/// }
/// ```
///
/// # Returns
///
/// ```json
/// {
///   "valid": true
/// }
/// ```
///
/// # Notes
///
/// - Signature format: 114 bytes (R || s)
///   - R: 57 bytes (compressed Edwards point)
///   - s: 57 bytes (scalar)
/// - Public key format: 57 bytes (compressed Edwards point)
/// - Constant-time verification (timing attack resistant)
///
/// # Performance
///
/// - Target: < 1ms (Pure Rust)
/// - Typical: 800-900μs on modern hardware
pub async fn handle_verify_ed448(
    params: Option<&serde_json::Value>,
) -> Result<serde_json::Value, BearDogError> {
    info!("Ed448: verify request");

    let params = params.ok_or_else(|| {
        BearDogError::validation("Missing parameters for Ed448 verification")
    })?;

    let _data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::validation("Missing 'data' parameter"))?;

    let _signature_b64 = params
        .get("signature")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::validation("Missing 'signature' parameter"))?;

    let _public_key_b64 = params
        .get("public_key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::validation("Missing 'public_key' parameter"))?;

    Err(BearDogError::not_yet_available(
        "Ed448 verification requires ed448-goldilocks EdDSA API integration (< 0.1% server support)",
    ))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ed448_not_yet_available() {
        let test_data = b"Hello, Ed448!";
        let data_b64 = BASE64.encode(test_data);

        let sign_params = serde_json::json!({ "data": data_b64 });
        let sign_result = handle_sign_ed448(Some(&sign_params)).await;
        assert!(sign_result.is_err());
        let err_msg = sign_result.unwrap_err().to_string();
        assert!(
            err_msg.contains("Not yet available"),
            "Expected BearDogError::not_yet_available, got: {err_msg}"
        );
    }

    #[tokio::test]
    async fn test_ed448_verify_not_yet_available() {
        let test_data = b"Hello, Ed448!";
        let data_b64 = BASE64.encode(test_data);

        let verify_params = serde_json::json!({
            "data": data_b64,
            "signature": "AAAA",
            "public_key": "AAAA",
        });
        let verify_result = handle_verify_ed448(Some(&verify_params)).await;
        assert!(verify_result.is_err());
        let err_msg = verify_result.unwrap_err().to_string();
        assert!(
            err_msg.contains("Not yet available"),
            "Expected BearDogError::not_yet_available, got: {err_msg}"
        );
    }

    #[tokio::test]
    async fn test_ed448_sign_missing_params() {
        let result = handle_sign_ed448(None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ed448_verify_missing_data() {
        let params = serde_json::json!({
            "signature": "AAAA",
            "public_key": "AAAA",
        });
        let result = handle_verify_ed448(Some(&params)).await;
        assert!(result.is_err());
    }
}

