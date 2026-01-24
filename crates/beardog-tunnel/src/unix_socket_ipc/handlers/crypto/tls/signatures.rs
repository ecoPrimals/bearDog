//! TLS 1.3 Signature Operations and Finished MAC
//!
//! This module implements cryptographic signature operations for TLS 1.3 handshakes,
//! including Ed25519 signing for CertificateVerify and HMAC computation for the Finished message.
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
//! - RFC 8032: Edwards-Curve Digital Signature Algorithm (EdDSA)
//! - RFC 2104: HMAC

use base64::Engine;
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::{Digest, Sha256};
use tracing::{debug, info};

// Import shared utility for key derivation
use super::super::utils::derive_key_from_id;

/// Handle tls.sign_handshake method
///
/// Signs TLS handshake messages with Ed25519 for ClientKeyExchange/CertificateVerify.
/// This is used in TLS 1.3 to prove possession of the private key.
///
/// # Parameters
///
/// - `message`: Base64-encoded handshake messages to sign
/// - `algorithm`: Signature algorithm (default: "ed25519")
/// - `key_id`: Key identifier for TLS signing key (optional)
/// - `purpose`: Purpose string for key derivation (default: "tls_handshake")
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
/// - RFC 8032: Edwards-Curve Digital Signature Algorithm (EdDSA)
/// - RFC 8446 Section 4.4.3: Certificate Verify
pub async fn handle_tls_sign_handshake(params: Option<&Value>) -> Result<Value, String> {
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
        return Err(format!("Unsupported algorithm: {}", algorithm));
    }

    // Use BearDog's crypto service for Ed25519 signing
    use beardog_core::crypto_service::algorithms::asymmetric;

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

/// Handle tls.compute_finished_verify_data method
///
/// Computes the TLS 1.3 Finished message verify_data for client or server.
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
/// - BaseKey is either `client_handshake_traffic_secret` or `server_handshake_traffic_secret`
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
) -> Result<Value, String> {
    use base64::prelude::*;

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

    // Decode from base64
    let base_key = BASE64_STANDARD
        .decode(base_key_b64)
        .map_err(|e| format!("Invalid base64 for base_key: {}", e))?;

    let transcript_hash = BASE64_STANDARD
        .decode(transcript_hash_b64)
        .map_err(|e| format!("Invalid base64 for transcript_hash: {}", e))?;

    info!("🔐 Computing TLS 1.3 Finished verify_data");
    info!("   Base key: {} bytes", base_key.len());
    info!("   Transcript hash: {} bytes", transcript_hash.len());

    // Validate inputs
    if transcript_hash.len() != 32 {
        return Err(format!(
            "Invalid transcript_hash length: {} (expected 32 for SHA-256)",
            transcript_hash.len()
        ));
    }

    // Step 1: Derive finished_key using HKDF-Expand-Label
    // finished_key = HKDF-Expand-Label(base_key, "finished", "", 32)
    // RFC 8446 Section 7.1: Label MUST include "tls13 " prefix!
    let hkdf_expand_label = |secret: &[u8], label: &str, context: &[u8], length: usize| {
        let hkdf =
            Hkdf::<Sha256>::from_prk(secret).map_err(|e| format!("HKDF PRK error: {}", e))?;

        // RFC 8446 Section 7.1: HkdfLabel structure
        // CRITICAL: Label must be "tls13 " + label (e.g., "tls13 finished")
        let tls13_label = format!("tls13 {}", label);
        let mut hkdf_label = Vec::new();
        hkdf_label.extend_from_slice(&(length as u16).to_be_bytes());
        hkdf_label.push(tls13_label.len() as u8);
        hkdf_label.extend_from_slice(tls13_label.as_bytes());
        hkdf_label.push(context.len() as u8);
        hkdf_label.extend_from_slice(context);

        let mut output = vec![0u8; length];
        hkdf.expand(&hkdf_label, &mut output)
            .map_err(|e| format!("HKDF expand error: {}", e))?;

        Ok::<Vec<u8>, String>(output)
    };

    // FIX: Use "finished" which becomes "tls13 finished" via hkdf_expand_label
    let finished_key = hkdf_expand_label(&base_key, "finished", &[], 32)?;
    info!("✅ Derived finished_key: {} bytes", finished_key.len());

    // Step 2: Compute verify_data = HMAC-SHA256(finished_key, transcript_hash)
    type HmacSha256 = Hmac<Sha256>;
    let mut mac =
        HmacSha256::new_from_slice(&finished_key).map_err(|e| format!("HMAC key error: {}", e))?;

    mac.update(&transcript_hash);
    let verify_data = mac.finalize().into_bytes().to_vec();

    info!("✅ Computed verify_data: {} bytes", verify_data.len());
    info!("   Verify data (hex): {}", hex::encode(&verify_data));

    Ok(serde_json::json!({
        "verify_data": BASE64_STANDARD.encode(&verify_data),
        "length": verify_data.len(),
    }))
}
