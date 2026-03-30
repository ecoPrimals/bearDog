// SPDX-License-Identifier: AGPL-3.0-only

//! RSA Crypto Handlers
//!
//! Pure Rust RSA signature operations using `RustCrypto`'s rsa crate.
//!
//! # Supported Algorithms
//!
//! - **RSA PKCS#1 v1.5**: Legacy padding scheme (2048, 3072, 4096-bit)
//! - **RSA-PSS**: Modern probabilistic padding (2048, 3072, 4096-bit)
//!
//! # RPC Methods
//!
//! ## RSA PKCS#1 v1.5 (Legacy/Enterprise Support)
//! - `crypto.sign_rsa_pkcs1_sha256` - Sign with RSA PKCS#1 v1.5 + SHA-256
//! - `crypto.verify_rsa_pkcs1_sha256` - Verify RSA PKCS#1 v1.5 signature
//!
//! ## RSA-PSS (Modern, Recommended)
//! - `crypto.sign_rsa_pss_sha256` - Sign with RSA-PSS + SHA-256
//! - `crypto.verify_rsa_pss_sha256` - Verify RSA-PSS signature
//!
//! # Architecture
//!
//! All operations are:
//! - **Pure Rust**: Zero C dependencies (using `RustCrypto` rsa crate)
//! - **No unchecked memory patterns**: Memory-safe implementation
//! - **Zeroized**: Private keys cleared after use
//! - **Capability-based**: Key sizes configurable, no hardcoded preferences
//! - **Production-ready**: Used by major Rust projects
//!
//! # Performance
//!
//! Target latencies (Pure Rust):
//! - RSA-2048 sign: ~2-5ms
//! - RSA-2048 verify: ~100-200μs
//! - RSA-3072 sign: ~8-12ms
//! - RSA-3072 verify: ~200-300μs
//! - RSA-4096 sign: ~15-25ms
//! - RSA-4096 verify: ~300-500μs
//!
//! Note: RSA signing is slower than ECDSA due to mathematical operations.
//! Verification is fast. For most use cases, ECDSA P-256 is preferred.
//!
//! # Security
//!
//! - PKCS#1 v1.5: Legacy, widely supported, potential padding oracle vulnerabilities
//! - RSA-PSS: Modern, recommended, eliminates padding oracle issues
//! - Minimum key size: 2048 bits (128-bit security)
//! - Recommended: 3072 bits (128-bit security, future-proof)
//! - Maximum: 4096 bits (152-bit security, high-security/government)

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use rand::rngs::OsRng;
use rsa::pkcs1v15::{SigningKey as Pkcs1SigningKey, VerifyingKey as Pkcs1VerifyingKey};
use rsa::pss::{SigningKey as PssSigningKey, VerifyingKey as PssVerifyingKey};
use rsa::signature::{RandomizedSigner, SignatureEncoding, Verifier};
use rsa::{RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;
use tracing::{debug, info};
use zeroize::Zeroizing;

// ============================================================================
// RSA PKCS#1 v1.5 (Legacy Support)
// ============================================================================

/// # Errors
///
/// Returns an error if signing fails in the underlying HSM provider.
/// Sign data with RSA PKCS#1 v1.5 + SHA-256
///
/// # RPC Method
///
/// `crypto.sign_rsa_pkcs1_sha256`
///
/// # Parameters
///
/// ```json
/// {
///   "data": "base64_encoded_data_to_sign",
///   "key_size": 2048  // Optional: 2048, 3072, or 4096 (default: 2048)
/// }
/// ```
///
/// # Returns
///
/// ```json
/// {
///   "signature": "base64_encoded_signature",
///   "public_key_pem": "PEM_encoded_public_key",
///   "algorithm": "rsa_pkcs1_sha256",
///   "key_size": 2048,
///   "hash": "SHA-256"
/// }
/// ```
///
/// # Notes
///
/// - Generates ephemeral RSA keypair (`OsRng` - Tier 1 entropy)
/// - Padding: PKCS#1 v1.5 (legacy, widely supported)
/// - Hash: SHA-256 (fixed for consistency)
/// - Key sizes: 2048 (default), 3072, 4096
/// - Pure Rust implementation (`RustCrypto` rsa crate)
///
/// # Security
///
/// - Legacy padding scheme with potential padding oracle vulnerabilities
/// - Use RSA-PSS for new applications
/// - Minimum 2048 bits required (128-bit security)
///
/// # Performance
///
/// - 2048-bit: ~2-5ms sign (slower than ECDSA)
/// - 3072-bit: ~8-12ms sign
/// - 4096-bit: ~15-25ms sign
pub async fn handle_sign_rsa_pkcs1_sha256(
    params: Option<&serde_json::Value>,
) -> Result<serde_json::Value, String> {
    info!("🔐 RSA PKCS#1 v1.5: Signing data");

    // Extract and validate parameters
    let params = params.ok_or("Missing parameters for RSA PKCS#1 signing")?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'data' parameter")?;

    // Capability-based key size selection (no hardcoding)
    #[expect(
        clippy::cast_possible_truncation,
        reason = "RSA modulus size validated to 2048/3072/4096"
    )]
    let key_size = params
        .get("key_size")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(2048) as usize;

    // Validate key size (security requirement, not hardcoding)
    if ![2048, 3072, 4096].contains(&key_size) {
        return Err(format!(
            "Invalid key size: {key_size}. Supported: 2048, 3072, 4096"
        ));
    }

    // Decode input data
    let data = BASE64
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    debug!(
        "📝 Data to sign: {} bytes, key size: {} bits",
        data.len(),
        key_size
    );

    // Generate ephemeral RSA keypair (Pure Rust, no unchecked memory patterns)
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, key_size)
        .map_err(|e| format!("Failed to generate RSA key: {e}"))?;

    let public_key = RsaPublicKey::from(&private_key);

    // Create PKCS#1 v1.5 signing key
    let signing_key = Pkcs1SigningKey::<Sha256>::new(private_key);

    // Sign data (Pure Rust, memory-safe)
    let signature = signing_key.sign_with_rng(&mut rng, &data).to_vec();

    // Encode public key as PEM (standard format for interoperability)
    let public_key_pem =
        rsa::pkcs8::EncodePublicKey::to_public_key_pem(&public_key, rsa::pkcs8::LineEnding::LF)
            .map_err(|e| format!("Failed to encode public key: {e}"))?;

    // Encode signature as base64
    let signature_b64 = BASE64.encode(&signature);

    info!(
        "✅ RSA PKCS#1: Signed {} bytes with {}-bit key, signature {} bytes",
        data.len(),
        key_size,
        signature.len()
    );

    Ok(serde_json::json!({
        "signature": signature_b64,
        "public_key_pem": public_key_pem,
        "algorithm": "rsa_pkcs1_sha256",
        "key_size": key_size,
        "hash": "SHA-256"
    }))
}

/// # Errors
///
/// Returns an error if hashing fails.
/// Verify RSA PKCS#1 v1.5 + SHA-256 signature
///
/// # RPC Method
///
/// `crypto.verify_rsa_pkcs1_sha256`
///
/// # Parameters
///
/// ```json
/// {
///   "data": "base64_encoded_data",
///   "signature": "base64_encoded_signature",
///   "public_key_pem": "PEM_encoded_public_key"
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
/// - Padding: PKCS#1 v1.5 (legacy)
/// - Hash: SHA-256 (fixed)
/// - Public key format: PEM (PKCS#8)
/// - Memory-safe verification (no unchecked memory patterns)
///
/// # Performance
///
/// - Target: < 500μs (Pure Rust)
/// - Typical: 100-300μs depending on key size
pub async fn handle_verify_rsa_pkcs1_sha256(
    params: Option<&serde_json::Value>,
) -> Result<serde_json::Value, String> {
    info!("✅ RSA PKCS#1 v1.5: Verifying signature");

    // Extract and validate parameters
    let params = params.ok_or("Missing parameters for RSA PKCS#1 verification")?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'data' parameter")?;

    let signature_b64 = params
        .get("signature")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'signature' parameter")?;

    let public_key_pem = params
        .get("public_key_pem")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'public_key_pem' parameter")?;

    // Decode inputs
    let data = BASE64
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    let signature_bytes = Zeroizing::new(
        BASE64
            .decode(signature_b64)
            .map_err(|e| format!("Invalid base64 signature: {e}"))?,
    );

    debug!(
        "📝 Verifying: data={} bytes, signature={} bytes",
        data.len(),
        signature_bytes.len()
    );

    // Parse public key from PEM (standard format)
    let public_key = rsa::pkcs8::DecodePublicKey::from_public_key_pem(public_key_pem)
        .map_err(|e| format!("Invalid PEM public key: {e}"))?;

    // Create PKCS#1 v1.5 verifying key
    let verifying_key = Pkcs1VerifyingKey::<Sha256>::new(public_key);

    // Parse signature
    let signature = rsa::pkcs1v15::Signature::try_from(signature_bytes.as_slice())
        .map_err(|e| format!("Invalid signature format: {e}"))?;

    // Verify signature (memory-safe, no unchecked memory patterns)
    let valid = verifying_key.verify(&data, &signature).is_ok();

    if valid {
        info!("✅ RSA PKCS#1: Signature VALID");
    } else {
        info!("❌ RSA PKCS#1: Signature INVALID");
    }

    Ok(serde_json::json!({
        "valid": valid,
        "algorithm": "rsa_pkcs1_sha256"
    }))
}

// ============================================================================
// RSA-PSS (Modern, Recommended)
// ============================================================================

/// # Errors
///
/// Returns an error if signing fails in the underlying HSM provider.
/// Sign data with RSA-PSS + SHA-256
///
/// # RPC Method
///
/// `crypto.sign_rsa_pss_sha256`
///
/// # Parameters
///
/// ```json
/// {
///   "data": "base64_encoded_data_to_sign",
///   "key_size": 2048  // Optional: 2048, 3072, or 4096 (default: 2048)
/// }
/// ```
///
/// # Returns
///
/// ```json
/// {
///   "signature": "base64_encoded_signature",
///   "public_key_pem": "PEM_encoded_public_key",
///   "algorithm": "rsa_pss_sha256",
///   "key_size": 2048,
///   "hash": "SHA-256"
/// }
/// ```
///
/// # Notes
///
/// - Generates ephemeral RSA keypair (`OsRng` - Tier 1 entropy)
/// - Padding: PSS (Probabilistic Signature Scheme - modern, secure)
/// - Hash: SHA-256 (fixed for consistency)
/// - Key sizes: 2048 (default), 3072, 4096
/// - Pure Rust implementation (`RustCrypto` rsa crate)
///
/// # Security
///
/// - Modern padding scheme (eliminates padding oracle issues)
/// - Recommended for new applications
/// - Provably secure under RSA assumption
/// - Minimum 2048 bits required (128-bit security)
///
/// # Performance
///
/// - 2048-bit: ~2-5ms sign
/// - 3072-bit: ~8-12ms sign
/// - 4096-bit: ~15-25ms sign
pub async fn handle_sign_rsa_pss_sha256(
    params: Option<&serde_json::Value>,
) -> Result<serde_json::Value, String> {
    info!("🔐 RSA-PSS: Signing data");

    // Extract and validate parameters
    let params = params.ok_or("Missing parameters for RSA-PSS signing")?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'data' parameter")?;

    // Capability-based key size selection (no hardcoding)
    #[expect(
        clippy::cast_possible_truncation,
        reason = "RSA modulus size validated to 2048/3072/4096"
    )]
    let key_size = params
        .get("key_size")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(2048) as usize;

    // Validate key size (security requirement, not hardcoding)
    if ![2048, 3072, 4096].contains(&key_size) {
        return Err(format!(
            "Invalid key size: {key_size}. Supported: 2048, 3072, 4096"
        ));
    }

    // Decode input data
    let data = BASE64
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    debug!(
        "📝 Data to sign: {} bytes, key size: {} bits",
        data.len(),
        key_size
    );

    // Generate ephemeral RSA keypair (Pure Rust, no unchecked memory patterns)
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, key_size)
        .map_err(|e| format!("Failed to generate RSA key: {e}"))?;

    let public_key = RsaPublicKey::from(&private_key);

    // Create RSA-PSS signing key
    let signing_key = PssSigningKey::<Sha256>::new(private_key);

    // Sign data (Pure Rust, memory-safe, probabilistic padding)
    let signature = signing_key.sign_with_rng(&mut rng, &data).to_vec();

    // Encode public key as PEM (standard format for interoperability)
    let public_key_pem =
        rsa::pkcs8::EncodePublicKey::to_public_key_pem(&public_key, rsa::pkcs8::LineEnding::LF)
            .map_err(|e| format!("Failed to encode public key: {e}"))?;

    // Encode signature as base64
    let signature_b64 = BASE64.encode(&signature);

    info!(
        "✅ RSA-PSS: Signed {} bytes with {}-bit key, signature {} bytes",
        data.len(),
        key_size,
        signature.len()
    );

    Ok(serde_json::json!({
        "signature": signature_b64,
        "public_key_pem": public_key_pem,
        "algorithm": "rsa_pss_sha256",
        "key_size": key_size,
        "hash": "SHA-256"
    }))
}

/// # Errors
///
/// Returns an error if hashing fails.
/// Verify RSA-PSS + SHA-256 signature
///
/// # RPC Method
///
/// `crypto.verify_rsa_pss_sha256`
///
/// # Parameters
///
/// ```json
/// {
///   "data": "base64_encoded_data",
///   "signature": "base64_encoded_signature",
///   "public_key_pem": "PEM_encoded_public_key"
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
/// - Padding: PSS (Probabilistic Signature Scheme)
/// - Hash: SHA-256 (fixed)
/// - Public key format: PEM (PKCS#8)
/// - Memory-safe verification (no unchecked memory patterns)
///
/// # Performance
///
/// - Target: < 500μs (Pure Rust)
/// - Typical: 100-300μs depending on key size
pub async fn handle_verify_rsa_pss_sha256(
    params: Option<&serde_json::Value>,
) -> Result<serde_json::Value, String> {
    info!("✅ RSA-PSS: Verifying signature");

    // Extract and validate parameters
    let params = params.ok_or("Missing parameters for RSA-PSS verification")?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'data' parameter")?;

    let signature_b64 = params
        .get("signature")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'signature' parameter")?;

    let public_key_pem = params
        .get("public_key_pem")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'public_key_pem' parameter")?;

    // Decode inputs
    let data = BASE64
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    let signature_bytes = Zeroizing::new(
        BASE64
            .decode(signature_b64)
            .map_err(|e| format!("Invalid base64 signature: {e}"))?,
    );

    debug!(
        "📝 Verifying: data={} bytes, signature={} bytes",
        data.len(),
        signature_bytes.len()
    );

    // Parse public key from PEM (standard format)
    let public_key = rsa::pkcs8::DecodePublicKey::from_public_key_pem(public_key_pem)
        .map_err(|e| format!("Invalid PEM public key: {e}"))?;

    // Create RSA-PSS verifying key
    let verifying_key = PssVerifyingKey::<Sha256>::new(public_key);

    // Parse signature
    let signature = rsa::pss::Signature::try_from(signature_bytes.as_slice())
        .map_err(|e| format!("Invalid signature format: {e}"))?;

    // Verify signature (memory-safe, no unchecked memory patterns)
    let valid = verifying_key.verify(&data, &signature).is_ok();

    if valid {
        info!("✅ RSA-PSS: Signature VALID");
    } else {
        info!("❌ RSA-PSS: Signature INVALID");
    }

    Ok(serde_json::json!({
        "valid": valid,
        "algorithm": "rsa_pss_sha256"
    }))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // RSA PKCS#1 v1.5 Tests
    // ========================================================================

    #[tokio::test]
    async fn test_rsa_pkcs1_2048_sign_and_verify_roundtrip() {
        // Test data
        let test_data = b"Hello, RSA PKCS#1 v1.5!";
        let data_b64 = BASE64.encode(test_data);

        // Sign with 2048-bit key
        let sign_params = serde_json::json!({
            "data": data_b64,
            "key_size": 2048
        });

        let sign_result = handle_sign_rsa_pkcs1_sha256(Some(&sign_params))
            .await
            .expect("Signing should succeed");

        let signature_b64 = sign_result["signature"]
            .as_str()
            .expect("pkcs1 signature string");
        let public_key_pem = sign_result["public_key_pem"]
            .as_str()
            .expect("pkcs1 public key pem");
        assert_eq!(sign_result["key_size"], 2048);
        assert_eq!(sign_result["algorithm"], "rsa_pkcs1_sha256");

        // Verify
        let verify_params = serde_json::json!({
            "data": data_b64,
            "signature": signature_b64,
            "public_key_pem": public_key_pem
        });

        let verify_result = handle_verify_rsa_pkcs1_sha256(Some(&verify_params))
            .await
            .expect("Verification should succeed");

        assert_eq!(verify_result["valid"], true);
        assert_eq!(verify_result["algorithm"], "rsa_pkcs1_sha256");
    }

    #[tokio::test]
    async fn test_rsa_pkcs1_verify_invalid_signature() {
        // Test data
        let test_data = b"Hello, RSA PKCS#1!";
        let data_b64 = BASE64.encode(test_data);

        // Sign
        let sign_params = serde_json::json!({
            "data": data_b64
        });

        let sign_result = handle_sign_rsa_pkcs1_sha256(Some(&sign_params))
            .await
            .expect("Signing should succeed");

        let signature_b64 = sign_result["signature"]
            .as_str()
            .expect("pkcs1 signature string");
        let public_key_pem = sign_result["public_key_pem"]
            .as_str()
            .expect("pkcs1 public key pem");

        // Tamper with data
        let tampered_data = b"Tampered data!";
        let tampered_data_b64 = BASE64.encode(tampered_data);

        // Verify with tampered data
        let verify_params = serde_json::json!({
            "data": tampered_data_b64,
            "signature": signature_b64,
            "public_key_pem": public_key_pem
        });

        let verify_result = handle_verify_rsa_pkcs1_sha256(Some(&verify_params))
            .await
            .expect("Verification should succeed (but return false)");

        assert_eq!(verify_result["valid"], false);
    }

    // ========================================================================
    // RSA-PSS Tests
    // ========================================================================

    #[tokio::test]
    async fn test_rsa_pss_2048_sign_and_verify_roundtrip() {
        // Test data
        let test_data = b"Hello, RSA-PSS!";
        let data_b64 = BASE64.encode(test_data);

        // Sign with 2048-bit key
        let sign_params = serde_json::json!({
            "data": data_b64,
            "key_size": 2048
        });

        let sign_result = handle_sign_rsa_pss_sha256(Some(&sign_params))
            .await
            .expect("Signing should succeed");

        let signature_b64 = sign_result["signature"]
            .as_str()
            .expect("pss signature string");
        let public_key_pem = sign_result["public_key_pem"]
            .as_str()
            .expect("pss public key pem");
        assert_eq!(sign_result["key_size"], 2048);
        assert_eq!(sign_result["algorithm"], "rsa_pss_sha256");

        // Verify
        let verify_params = serde_json::json!({
            "data": data_b64,
            "signature": signature_b64,
            "public_key_pem": public_key_pem
        });

        let verify_result = handle_verify_rsa_pss_sha256(Some(&verify_params))
            .await
            .expect("Verification should succeed");

        assert_eq!(verify_result["valid"], true);
        assert_eq!(verify_result["algorithm"], "rsa_pss_sha256");
    }

    #[tokio::test]
    async fn test_rsa_pss_verify_invalid_signature() {
        // Test data
        let test_data = b"Hello, RSA-PSS!";
        let data_b64 = BASE64.encode(test_data);

        // Sign
        let sign_params = serde_json::json!({
            "data": data_b64
        });

        let sign_result = handle_sign_rsa_pss_sha256(Some(&sign_params))
            .await
            .expect("Signing should succeed");

        let signature_b64 = sign_result["signature"]
            .as_str()
            .expect("pss signature string");
        let public_key_pem = sign_result["public_key_pem"]
            .as_str()
            .expect("pss public key pem");

        // Tamper with data
        let tampered_data = b"Tampered data!";
        let tampered_data_b64 = BASE64.encode(tampered_data);

        // Verify with tampered data
        let verify_params = serde_json::json!({
            "data": tampered_data_b64,
            "signature": signature_b64,
            "public_key_pem": public_key_pem
        });

        let verify_result = handle_verify_rsa_pss_sha256(Some(&verify_params))
            .await
            .expect("Verification should succeed (but return false)");

        assert_eq!(verify_result["valid"], false);
    }

    #[tokio::test]
    async fn test_rsa_pkcs1_invalid_key_size() {
        let test_data = b"Test";
        let data_b64 = BASE64.encode(test_data);

        let sign_params = serde_json::json!({
            "data": data_b64,
            "key_size": 1024  // Invalid (too small)
        });

        let result = handle_sign_rsa_pkcs1_sha256(Some(&sign_params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid key size"));
    }

    #[tokio::test]
    async fn test_rsa_pss_missing_params() {
        let result = handle_sign_rsa_pss_sha256(None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Missing parameters"));
    }
}
