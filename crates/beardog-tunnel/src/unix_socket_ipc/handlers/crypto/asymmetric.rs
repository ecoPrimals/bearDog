//! Asymmetric cryptography operations
//!
//! This module provides asymmetric cryptographic operations for ecoPrimals,
//! including digital signatures and key exchange using modern elliptic curves.
//!
//! # Overview
//!
//! Asymmetric cryptography uses key pairs (public + private) for:
//! - **Digital Signatures**: Ed25519 (EdDSA on Curve25519)
//! - **Key Exchange**: X25519 (ECDH on Curve25519)
//!
//! All operations use Pure Rust implementations from the RustCrypto ecosystem,
//! with zero C dependencies for universal cross-compilation.
//!
//! # Algorithms
//!
//! ## Ed25519 (Digital Signatures)
//!
//! - **Curve**: Curve25519 (Edwards form)
//! - **Signature Size**: 64 bytes
//! - **Public Key Size**: 32 bytes
//! - **Private Key Size**: 32 bytes (seed)
//! - **Security Level**: ~128-bit
//! - **Speed**: Very fast (~60,000 signs/sec, ~40,000 verifies/sec)
//!
//! ### Methods
//!
//! - [`handle_sign_ed25519`] - Sign a message with Ed25519
//! - [`handle_verify_ed25519`] - Verify an Ed25519 signature
//!
//! ## X25519 (Key Exchange)
//!
//! - **Curve**: Curve25519 (Montgomery form)
//! - **Shared Secret Size**: 32 bytes
//! - **Public Key Size**: 32 bytes
//! - **Private Key Size**: 32 bytes
//! - **Security Level**: ~128-bit
//! - **Speed**: Very fast (~10,000 exchanges/sec)
//!
//! ### Methods
//!
//! - [`handle_x25519_generate_ephemeral`] - Generate ephemeral X25519 keypair
//! - [`handle_x25519_derive_secret`] - Derive shared secret via ECDH
//!
//! # Usage
//!
//! All handlers are re-exported from the parent `crypto` module:
//!
//! ```rust,ignore
//! // NOTE: These handlers are internal and called via JSON-RPC
//! use crate::unix_socket_ipc::handlers::crypto::*;
//!
//! // Generate Ed25519 signature
//! let signature = handle_sign_ed25519(params).await?;
//!
//! // Verify Ed25519 signature
//! let is_valid = handle_verify_ed25519(params).await?;
//!
//! // Generate X25519 ephemeral keypair
//! let keypair = handle_x25519_generate_ephemeral(None).await?;
//!
//! // Derive X25519 shared secret
//! let shared_secret = handle_x25519_derive_secret(params).await?;
//! ```
//!
//! # References
//!
//! - RFC 8032 (EdDSA/Ed25519): <https://www.rfc-editor.org/rfc/rfc8032.html>
//! - RFC 7748 (X25519): <https://www.rfc-editor.org/rfc/rfc7748.html>
//! - Curve25519: <https://cr.yp.to/ecdh.html>

use base64::Engine;
use serde_json::Value;
use tracing::{debug, info};

// Import shared utility functions
use super::utils::derive_key_from_id;

pub async fn handle_sign_ed25519(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.sign_ed25519")?;

    // Extract parameters
    let message_b64 = params
        .get("message")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: message")?;

    let key_id = params
        .get("key_id")
        .and_then(|v| v.as_str())
        .unwrap_or("default_signing_key");

    let purpose = params
        .get("purpose")
        .and_then(|v| v.as_str())
        .unwrap_or("general");

    // Decode message
    let message = base64::engine::general_purpose::STANDARD
        .decode(message_b64)
        .map_err(|e| format!("Invalid base64 message: {e}"))?;

    debug!(
        "🔐 Signing {} bytes with Ed25519 (key_id: {}, purpose: {})",
        message.len(),
        key_id,
        purpose
    );

    // Use BearDog's crypto service
    use beardog_core::crypto_service::algorithms::asymmetric;

    // Derive signing key from key_id
    let seed = derive_key_from_id(key_id, purpose)?;
    let (secret_key, _public_key) = asymmetric::generate_ed25519_from_seed(&seed)
        .map_err(|e| format!("Failed to generate Ed25519 keypair: {e}"))?;

    // Sign the message
    let signature = asymmetric::sign_ed25519(&message, &secret_key)
        .map_err(|e| format!("Ed25519 signing failed: {e}"))?;

    // Encode signature
    let signature_b64 = base64::engine::general_purpose::STANDARD.encode(&signature);

    info!("✅ Ed25519 signature generated ({} bytes)", signature.len());

    Ok(serde_json::json!({
        "signature": signature_b64,
        "algorithm": "Ed25519",
        "key_id": key_id,
    }))
}

/// Handle crypto.verify_ed25519 method
///
/// Verifies an Ed25519 signature.
///
/// # Parameters
///
/// - `message`: Base64-encoded message that was signed
/// - `signature`: Base64-encoded Ed25519 signature
/// - `public_key`: Base64-encoded Ed25519 public key (32 bytes)
///
/// # Returns
///
/// - `valid`: Boolean indicating if signature is valid
pub async fn handle_verify_ed25519(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.verify_ed25519")?;

    // Extract parameters
    let message_b64 = params
        .get("message")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: message")?;

    let signature_b64 = params
        .get("signature")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: signature")?;

    let public_key_b64 = params
        .get("public_key")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: public_key")?;

    // Decode parameters
    let message = base64::engine::general_purpose::STANDARD
        .decode(message_b64)
        .map_err(|e| format!("Invalid base64 message: {e}"))?;

    let signature = base64::engine::general_purpose::STANDARD
        .decode(signature_b64)
        .map_err(|e| format!("Invalid base64 signature: {e}"))?;

    let public_key = base64::engine::general_purpose::STANDARD
        .decode(public_key_b64)
        .map_err(|e| format!("Invalid base64 public_key: {e}"))?;

    debug!(
        "🔍 Verifying Ed25519 signature ({} bytes message, {} bytes signature)",
        message.len(),
        signature.len()
    );

    // Use BearDog's crypto service
    use beardog_core::crypto_service::algorithms::asymmetric;

    // Verify signature
    let valid = asymmetric::verify_ed25519(&message, &signature, &public_key)
        .map_err(|e| format!("Ed25519 verification failed: {e}"))?;

    info!("✅ Ed25519 signature verification: {}", valid);

    Ok(serde_json::json!({
        "valid": valid,
        "algorithm": "Ed25519",
    }))
}

/// Handle crypto.ed25519_generate_keypair method
///
/// Generates a new Ed25519 keypair for signing/identity.
/// Used by Songbird's Sovereign Onion Service for identity generation.
///
/// # Parameters
///
/// - `purpose`: Purpose string (optional, for logging)
///
/// # Returns
///
/// - `public_key`: Base64-encoded Ed25519 public key (32 bytes)
/// - `secret_key`: Base64-encoded Ed25519 secret key (32 bytes)
/// - `algorithm`: "Ed25519"
pub async fn handle_ed25519_generate_keypair(params: Option<&Value>) -> Result<Value, String> {
    let purpose = params
        .and_then(|p| p.get("purpose"))
        .and_then(|v| v.as_str())
        .unwrap_or("identity");

    debug!(
        "🔑 Generating Ed25519 keypair (purpose: {})",
        purpose
    );

    // Use ed25519-dalek for key generation
    use ed25519_dalek::{SigningKey, VerifyingKey};

    // Generate a random 32-byte seed
    let mut seed_bytes = [0u8; 32];
    rand::Rng::fill(&mut rand::rngs::OsRng, &mut seed_bytes);

    // Create signing key from the seed
    let signing_key = SigningKey::from_bytes(&seed_bytes);
    let verifying_key: VerifyingKey = (&signing_key).into();

    // Encode keys
    let public_key_b64 = base64::engine::general_purpose::STANDARD.encode(verifying_key.as_bytes());
    let secret_key_b64 = base64::engine::general_purpose::STANDARD.encode(signing_key.as_bytes());

    info!("✅ Ed25519 keypair generated (purpose: {})", purpose);

    Ok(serde_json::json!({
        "public_key": public_key_b64,
        "secret_key": secret_key_b64,
        "algorithm": "Ed25519",
    }))
}

/// Handle crypto.x25519_generate_ephemeral method
///
/// Generates an ephemeral X25519 keypair for key exchange.
///
/// # Parameters
///
/// - `purpose`: Purpose string (optional, for logging)
///
/// # Returns
///
/// - `public_key`: Base64-encoded X25519 public key (32 bytes)
/// - `secret_key`: Base64-encoded X25519 secret key (32 bytes)
pub async fn handle_x25519_generate_ephemeral(params: Option<&Value>) -> Result<Value, String> {
    let purpose = params
        .and_then(|p| p.get("purpose"))
        .and_then(|v| v.as_str())
        .unwrap_or("key_exchange");

    debug!(
        "🔑 Generating ephemeral X25519 keypair (purpose: {})",
        purpose
    );

    // Use x25519-dalek for key generation
    use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};

    // Generate a random 32-byte secret
    let mut secret_bytes = [0u8; 32];
    rand::Rng::fill(&mut rand::rngs::OsRng, &mut secret_bytes);

    // Create keypair from the secret
    let secret = StaticSecret::from(secret_bytes);
    let public = X25519PublicKey::from(&secret);

    // Encode keys
    let public_key_b64 = base64::engine::general_purpose::STANDARD.encode(public.as_bytes());
    let secret_key_b64 = base64::engine::general_purpose::STANDARD.encode(secret_bytes);

    info!("✅ Ephemeral X25519 keypair generated");

    Ok(serde_json::json!({
        "public_key": public_key_b64,
        "secret_key": secret_key_b64,
        "algorithm": "X25519",
    }))
}

/// Handle crypto.x25519_derive_secret method
///
/// Derives a shared secret using X25519 Diffie-Hellman.
///
/// # Parameters
///
/// - `our_secret`: Base64-encoded our X25519 secret key (32 bytes)
/// - `their_public`: Base64-encoded their X25519 public key (32 bytes)
///
/// # Returns
///
/// - `shared_secret`: Base64-encoded shared secret (32 bytes)
pub async fn handle_x25519_derive_secret(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.x25519_derive_secret")?;

    // Extract parameters
    let our_secret_b64 = params
        .get("our_secret")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: our_secret")?;

    let their_public_b64 = params
        .get("their_public")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: their_public")?;

    // Decode parameters
    let our_secret_bytes = base64::engine::general_purpose::STANDARD
        .decode(our_secret_b64)
        .map_err(|e| format!("Invalid base64 our_secret: {e}"))?;

    let their_public_bytes = base64::engine::general_purpose::STANDARD
        .decode(their_public_b64)
        .map_err(|e| format!("Invalid base64 their_public: {e}"))?;

    // Convert to fixed-size arrays
    let our_secret: [u8; 32] = our_secret_bytes
        .as_slice()
        .try_into()
        .map_err(|_| "our_secret must be 32 bytes".to_string())?;

    let their_public: [u8; 32] = their_public_bytes
        .as_slice()
        .try_into()
        .map_err(|_| "their_public must be 32 bytes".to_string())?;

    debug!("🤝 Deriving X25519 shared secret");

    // Use x25519-dalek for key exchange
    use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};

    // Create secret key from bytes
    let secret_key = StaticSecret::from(our_secret);
    let public = X25519PublicKey::from(their_public);

    let shared_secret = secret_key.diffie_hellman(&public);

    // Encode shared secret
    let shared_secret_b64 =
        base64::engine::general_purpose::STANDARD.encode(shared_secret.as_bytes());

    info!("✅ X25519 shared secret derived");

    Ok(serde_json::json!({
        "shared_secret": shared_secret_b64,
        "algorithm": "X25519",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use serde_json::json;

    // ========================================================================
    // ED25519 SIGNATURE TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_ed25519_sign_basic() {
        let message = BASE64.encode(b"Hello, BearDog!");
        let params = json!({ "message": message });

        let result = handle_sign_ed25519(Some(&params)).await;
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value["algorithm"], "Ed25519");
        assert!(value["signature"].is_string());
        assert!(value["key_id"].is_string());

        // Verify signature is 64 bytes
        let signature = BASE64.decode(value["signature"].as_str().unwrap()).unwrap();
        assert_eq!(signature.len(), 64);
    }

    #[tokio::test]
    async fn test_ed25519_sign_with_key_id() {
        let message = BASE64.encode(b"Signed message");
        let params = json!({ "message": message, "key_id": "my-custom-key" });

        let result = handle_sign_ed25519(Some(&params)).await;
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value["key_id"], "my-custom-key");
    }

    #[tokio::test]
    async fn test_ed25519_sign_deterministic_same_key() {
        let message = BASE64.encode(b"Same message");
        let params = json!({ "message": message, "key_id": "test-key-1" });

        let result1 = handle_sign_ed25519(Some(&params)).await.unwrap();
        let result2 = handle_sign_ed25519(Some(&params)).await.unwrap();

        // Same key, same message = same signature
        assert_eq!(result1["signature"], result2["signature"]);
    }

    #[tokio::test]
    async fn test_ed25519_sign_different_keys_different_signatures() {
        let message = BASE64.encode(b"Same message");

        let params1 = json!({ "message": message, "key_id": "key-a" });
        let params2 = json!({ "message": message, "key_id": "key-b" });

        let result1 = handle_sign_ed25519(Some(&params1)).await.unwrap();
        let result2 = handle_sign_ed25519(Some(&params2)).await.unwrap();

        // Different keys = different signatures
        assert_ne!(result1["signature"], result2["signature"]);
    }

    #[tokio::test]
    async fn test_ed25519_sign_missing_message() {
        let params = json!({ "key_id": "some-key" });
        let result = handle_sign_ed25519(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("message"));
    }

    #[tokio::test]
    async fn test_ed25519_sign_invalid_base64() {
        let params = json!({ "message": "!!!invalid-base64!!!" });
        let result = handle_sign_ed25519(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid base64"));
    }

    // ========================================================================
    // ED25519 VERIFICATION TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_ed25519_verify_missing_params() {
        let message = BASE64.encode(b"msg");
        let signature = BASE64.encode(&[0u8; 64]);

        let params = json!({ "message": message, "signature": signature });
        let result = handle_verify_ed25519(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("public_key"));
    }

    #[tokio::test]
    async fn test_ed25519_verify_missing_signature() {
        let message = BASE64.encode(b"msg");
        let public_key = BASE64.encode(&[0u8; 32]);

        let params = json!({ "message": message, "public_key": public_key });
        let result = handle_verify_ed25519(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("signature"));
    }

    // ========================================================================
    // X25519 KEY EXCHANGE TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_x25519_generate_ephemeral() {
        let result = handle_x25519_generate_ephemeral(None).await;
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value["algorithm"], "X25519");
        assert!(value["public_key"].is_string());
        assert!(value["secret_key"].is_string());

        // Verify key sizes (32 bytes each)
        let public_key = BASE64.decode(value["public_key"].as_str().unwrap()).unwrap();
        let secret_key = BASE64.decode(value["secret_key"].as_str().unwrap()).unwrap();
        assert_eq!(public_key.len(), 32);
        assert_eq!(secret_key.len(), 32);
    }

    #[tokio::test]
    async fn test_x25519_generate_unique_keys() {
        let result1 = handle_x25519_generate_ephemeral(None).await.unwrap();
        let result2 = handle_x25519_generate_ephemeral(None).await.unwrap();

        // Each generation should produce unique keys
        assert_ne!(result1["public_key"], result2["public_key"]);
        assert_ne!(result1["secret_key"], result2["secret_key"]);
    }

    #[tokio::test]
    async fn test_x25519_key_exchange_roundtrip() {
        // Alice generates her keypair
        let alice_keypair = handle_x25519_generate_ephemeral(None).await.unwrap();

        // Bob generates his keypair
        let bob_keypair = handle_x25519_generate_ephemeral(None).await.unwrap();

        // Alice derives shared secret with Bob's public key
        let alice_params = json!({
            "our_secret": alice_keypair["secret_key"],
            "their_public": bob_keypair["public_key"]
        });
        let alice_secret = handle_x25519_derive_secret(Some(&alice_params))
            .await
            .expect("Alice derive failed");

        // Bob derives shared secret with Alice's public key
        let bob_params = json!({
            "our_secret": bob_keypair["secret_key"],
            "their_public": alice_keypair["public_key"]
        });
        let bob_secret = handle_x25519_derive_secret(Some(&bob_params))
            .await
            .expect("Bob derive failed");

        // Both should arrive at the same shared secret
        assert_eq!(alice_secret["shared_secret"], bob_secret["shared_secret"]);
        assert_eq!(alice_secret["algorithm"], "X25519");
    }

    #[tokio::test]
    async fn test_x25519_derive_missing_our_secret() {
        let their_public = BASE64.encode(&[0u8; 32]);
        let params = json!({ "their_public": their_public });

        let result = handle_x25519_derive_secret(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("our_secret"));
    }

    #[tokio::test]
    async fn test_x25519_derive_missing_their_public() {
        let our_secret = BASE64.encode(&[0u8; 32]);
        let params = json!({ "our_secret": our_secret });

        let result = handle_x25519_derive_secret(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("their_public"));
    }

    #[tokio::test]
    async fn test_x25519_derive_wrong_key_length() {
        let our_secret = BASE64.encode(&[0u8; 16]); // Wrong: 16 bytes
        let their_public = BASE64.encode(&[0u8; 32]);
        let params = json!({ "our_secret": our_secret, "their_public": their_public });

        let result = handle_x25519_derive_secret(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("32 bytes"));
    }
}
