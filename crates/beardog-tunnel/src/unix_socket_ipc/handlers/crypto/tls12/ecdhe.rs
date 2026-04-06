// SPDX-License-Identifier: AGPL-3.0-or-later

//! TLS 1.2 ECDHE with NIST P-256 and P-384 (secp256r1 / secp384r1).

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde_json::Value;
use tracing::{debug, info};

// =============================================================================
// ECDHE with NIST P-256 (secp256r1)
// =============================================================================

/// # Errors
///
/// Returns an error if key derivation fails.
/// Handle crypto.ecdhe.p256.generate method
///
/// Generates an ephemeral P-256 (secp256r1) keypair for ECDHE key exchange.
///
/// # Parameters
///
/// - `purpose`: Optional purpose string (for logging)
///
/// # Returns
///
/// - `public_key`: Base64-encoded P-256 public key (compressed, 33 bytes)
/// - `secret_key`: Base64-encoded P-256 secret key (32 bytes)
/// - `algorithm`: "P-256" (NIST secp256r1)
pub async fn handle_ecdhe_p256_generate(params: Option<&Value>) -> Result<Value, String> {
    let purpose = params
        .and_then(|p| p.get("purpose"))
        .and_then(|v| v.as_str())
        .unwrap_or("tls12_ecdhe");

    debug!(
        "🔑 Generating ephemeral P-256 keypair (purpose: {})",
        purpose
    );

    use p256::elliptic_curve::SecretKey;
    use rand::RngCore;
    use zeroize::Zeroizing;

    // Generate a random 32-byte secret
    let mut private_key_bytes = Zeroizing::new([0u8; 32]);
    rand::rng().fill_bytes(&mut *private_key_bytes);

    // Create secret key from bytes
    let secret_key: SecretKey<p256::NistP256> = SecretKey::from_slice(&private_key_bytes[..])
        .map_err(|e| format!("Failed to create P-256 secret key: {e}"))?;

    // Derive public key from secret
    let public_key = secret_key.public_key();
    let public_key_bytes = public_key.to_sec1_bytes();

    // Encode keys
    let private_b64 = BASE64.encode(&private_key_bytes[..]);
    let public_b64 = BASE64.encode(&public_key_bytes);

    info!(
        "✅ Ephemeral P-256 keypair generated ({} bytes public)",
        public_key_bytes.len()
    );

    Ok(serde_json::json!({
        "public_key": public_b64,
        "secret_key": private_b64,
        "algorithm": "P-256",
        "curve": "secp256r1",
    }))
}

/// # Errors
///
/// Returns an error if the ECDH operation fails.
/// Handle `crypto.ecdhe.p256.compute_shared` method
///
/// Computes ECDH shared secret using P-256.
///
/// # Parameters
///
/// - `our_secret`: Base64-encoded P-256 secret key (32 bytes)
/// - `their_public`: Base64-encoded P-256 public key (33 or 65 bytes)
///
/// # Returns
///
/// - `shared_secret`: Base64-encoded shared secret (32 bytes)
/// - `algorithm`: "P-256"
pub async fn handle_ecdhe_p256_compute_shared(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing parameters for P-256 ECDH")?;

    let our_secret_b64 = params
        .get("our_secret")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'our_secret' parameter")?;

    let their_public_b64 = params
        .get("their_public")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'their_public' parameter")?;

    debug!("🔐 Computing P-256 ECDH shared secret");

    // Decode keys
    let our_secret_bytes = BASE64
        .decode(our_secret_b64)
        .map_err(|e| format!("Invalid our_secret base64: {e}"))?;

    let their_public_bytes = BASE64
        .decode(their_public_b64)
        .map_err(|e| format!("Invalid their_public base64: {e}"))?;

    // Parse keys
    use p256::PublicKey;
    use p256::elliptic_curve::SecretKey;

    let secret_key: SecretKey<p256::NistP256> = SecretKey::from_slice(&our_secret_bytes)
        .map_err(|e| format!("Invalid P-256 secret key: {e}"))?;

    let peer_public_key = PublicKey::from_sec1_bytes(&their_public_bytes)
        .map_err(|e| format!("Invalid P-256 public key: {e}"))?;

    // Perform ECDH
    use p256::ecdh::diffie_hellman;
    let shared_secret = diffie_hellman(secret_key.to_nonzero_scalar(), peer_public_key.as_affine());

    // Encode shared secret
    let shared_secret_b64 = BASE64.encode(shared_secret.raw_secret_bytes());

    info!("✅ P-256 ECDH shared secret computed (32 bytes)");

    Ok(serde_json::json!({
        "shared_secret": shared_secret_b64,
        "algorithm": "P-256",
    }))
}

// =============================================================================
// ECDHE with NIST P-384 (secp384r1)
// =============================================================================

/// # Errors
///
/// Returns an error if key derivation fails.
/// Handle crypto.ecdhe.p384.generate method
///
/// Generates an ephemeral P-384 (secp384r1) keypair for ECDHE key exchange.
///
/// # Parameters
///
/// - `purpose`: Optional purpose string (for logging)
///
/// # Returns
///
/// - `public_key`: Base64-encoded P-384 public key (compressed, 49 bytes)
/// - `secret_key`: Base64-encoded P-384 secret key (48 bytes)
/// - `algorithm`: "P-384" (NIST secp384r1)
pub async fn handle_ecdhe_p384_generate(params: Option<&Value>) -> Result<Value, String> {
    let purpose = params
        .and_then(|p| p.get("purpose"))
        .and_then(|v| v.as_str())
        .unwrap_or("tls12_ecdhe");

    debug!(
        "🔑 Generating ephemeral P-384 keypair (purpose: {})",
        purpose
    );

    use p384::elliptic_curve::SecretKey;
    use rand::RngCore;
    use zeroize::Zeroizing;

    // Generate a random 48-byte secret (P-384)
    let mut private_key_bytes = Zeroizing::new([0u8; 48]);
    rand::rng().fill_bytes(&mut *private_key_bytes);

    // Create secret key from bytes
    let secret_key: SecretKey<p384::NistP384> = SecretKey::from_slice(&private_key_bytes[..])
        .map_err(|e| format!("Failed to create P-384 secret key: {e}"))?;

    // Derive public key from secret
    let public_key = secret_key.public_key();
    let public_key_bytes = public_key.to_sec1_bytes();

    // Encode keys
    let private_b64 = BASE64.encode(&private_key_bytes[..]);
    let public_b64 = BASE64.encode(&public_key_bytes);

    info!(
        "✅ Ephemeral P-384 keypair generated ({} bytes public)",
        public_key_bytes.len()
    );

    Ok(serde_json::json!({
        "public_key": public_b64,
        "secret_key": private_b64,
        "algorithm": "P-384",
        "curve": "secp384r1",
    }))
}

/// # Errors
///
/// Returns an error if encryption fails.
/// Handle `crypto.ecdhe.p384.compute_shared` method
///
/// Computes ECDH shared secret using P-384.
///
/// # Parameters
///
/// - `our_secret`: Base64-encoded P-384 secret key (48 bytes)
/// - `their_public`: Base64-encoded P-384 public key (49 or 97 bytes)
///
/// # Returns
///
/// - `shared_secret`: Base64-encoded shared secret (48 bytes)
/// - `algorithm`: "P-384"
pub async fn handle_ecdhe_p384_compute_shared(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing parameters for P-384 ECDH")?;

    let our_secret_b64 = params
        .get("our_secret")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'our_secret' parameter")?;

    let their_public_b64 = params
        .get("their_public")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'their_public' parameter")?;

    debug!("🔐 Computing P-384 ECDH shared secret");

    // Decode keys
    let our_secret_bytes = BASE64
        .decode(our_secret_b64)
        .map_err(|e| format!("Invalid our_secret base64: {e}"))?;

    let their_public_bytes = BASE64
        .decode(their_public_b64)
        .map_err(|e| format!("Invalid their_public base64: {e}"))?;

    // Parse keys
    use p384::PublicKey;
    use p384::elliptic_curve::SecretKey;

    let secret_key: SecretKey<p384::NistP384> = SecretKey::from_slice(&our_secret_bytes)
        .map_err(|e| format!("Invalid P-384 secret key: {e}"))?;

    let peer_public_key = PublicKey::from_sec1_bytes(&their_public_bytes)
        .map_err(|e| format!("Invalid P-384 public key: {e}"))?;

    // Perform ECDH
    use p384::ecdh::diffie_hellman;
    let shared_secret = diffie_hellman(secret_key.to_nonzero_scalar(), peer_public_key.as_affine());

    // Encode shared secret
    let shared_secret_b64 = BASE64.encode(shared_secret.raw_secret_bytes());

    info!("✅ P-384 ECDH shared secret computed (48 bytes)");

    Ok(serde_json::json!({
        "shared_secret": shared_secret_b64,
        "algorithm": "P-384",
    }))
}
