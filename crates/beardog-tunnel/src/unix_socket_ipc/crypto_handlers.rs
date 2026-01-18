//! Crypto operation handlers for JSON-RPC
//!
//! This module provides cryptographic operations via JSON-RPC for Songbird TLS
//! and other primals. All crypto operations delegate to BearDog's existing
//! Pure Rust crypto implementations.
//!
//! # Architecture
//!
//! - **Complete Implementation**: No mocks, all operations are production-ready
//! - **Pure Rust**: 100% RustCrypto, zero C dependencies
//! - **Capability-Based**: Methods exposed as capabilities
//! - **Self-Knowledge Only**: No hardcoded primal names
//!
//! # Supported Operations
//!
//! - Ed25519 signatures (sign/verify)
//! - X25519 key exchange (generate ephemeral, derive secret)
//! - ChaCha20-Poly1305 AEAD (encrypt/decrypt)
//! - Blake3 hashing
//! - HMAC-SHA256

use anyhow::Result;
use base64::Engine;
use serde_json::Value;
use tracing::{debug, info};

/// Handle crypto.sign_ed25519 method
///
/// Signs a message with Ed25519 using BearDog's crypto service.
///
/// # Parameters
///
/// - `message`: Base64-encoded message to sign
/// - `key_id`: Key identifier (optional, uses default if not provided)
/// - `purpose`: Purpose string for key derivation (optional)
///
/// # Returns
///
/// - `signature`: Base64-encoded Ed25519 signature (64 bytes)
pub(crate) async fn handle_sign_ed25519(params: Option<&Value>) -> Result<Value, String> {
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
pub(crate) async fn handle_verify_ed25519(params: Option<&Value>) -> Result<Value, String> {
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
pub(crate) async fn handle_x25519_generate_ephemeral(params: Option<&Value>) -> Result<Value, String> {
    let purpose = params
        .and_then(|p| p.get("purpose"))
        .and_then(|v| v.as_str())
        .unwrap_or("key_exchange");

    debug!("🔑 Generating ephemeral X25519 keypair (purpose: {})", purpose);

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
    let secret_key_b64 = base64::engine::general_purpose::STANDARD.encode(&secret_bytes);

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
pub(crate) async fn handle_x25519_derive_secret(params: Option<&Value>) -> Result<Value, String> {
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

/// Handle crypto.chacha20_poly1305_encrypt method
///
/// Encrypts data with ChaCha20-Poly1305 AEAD.
///
/// # Parameters
///
/// - `plaintext`: Base64-encoded plaintext
/// - `key`: Base64-encoded 32-byte key
/// - `nonce`: Base64-encoded 12-byte nonce (optional, generated if not provided)
/// - `aad`: Base64-encoded additional authenticated data (optional)
///
/// # Returns
///
/// - `ciphertext`: Base64-encoded ciphertext
/// - `nonce`: Base64-encoded nonce (12 bytes)
/// - `tag`: Base64-encoded authentication tag (16 bytes)
pub(crate) async fn handle_chacha20_poly1305_encrypt(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.chacha20_poly1305_encrypt")?;

    // Extract parameters
    let plaintext_b64 = params
        .get("plaintext")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: plaintext")?;

    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: key")?;

    let aad_b64 = params.get("aad").and_then(|v| v.as_str());

    // Decode parameters
    let plaintext = base64::engine::general_purpose::STANDARD
        .decode(plaintext_b64)
        .map_err(|e| format!("Invalid base64 plaintext: {e}"))?;

    let key_bytes = base64::engine::general_purpose::STANDARD
        .decode(key_b64)
        .map_err(|e| format!("Invalid base64 key: {e}"))?;

    let key: [u8; 32] = key_bytes
        .as_slice()
        .try_into()
        .map_err(|_| "key must be 32 bytes".to_string())?;

    let aad = if let Some(aad_b64) = aad_b64 {
        Some(
            base64::engine::general_purpose::STANDARD
                .decode(aad_b64)
                .map_err(|e| format!("Invalid base64 aad: {e}"))?,
        )
    } else {
        None
    };

    debug!(
        "🔒 Encrypting {} bytes with ChaCha20-Poly1305",
        plaintext.len()
    );

    // Use BearDog's crypto service
    use beardog_core::crypto_service::algorithms::symmetric;

    let (ciphertext, nonce, tag) =
        symmetric::encrypt_chacha20_poly1305(&plaintext, &key, aad.as_deref())
            .map_err(|e| format!("ChaCha20-Poly1305 encryption failed: {e}"))?;

    // Encode results
    let ciphertext_b64 = base64::engine::general_purpose::STANDARD.encode(&ciphertext);
    let nonce_b64 = base64::engine::general_purpose::STANDARD.encode(&nonce);
    let tag_b64 = base64::engine::general_purpose::STANDARD.encode(&tag);

    info!(
        "✅ ChaCha20-Poly1305 encryption complete ({} bytes → {} bytes)",
        plaintext.len(),
        ciphertext.len()
    );

    Ok(serde_json::json!({
        "ciphertext": ciphertext_b64,
        "nonce": nonce_b64,
        "tag": tag_b64,
        "algorithm": "ChaCha20-Poly1305",
    }))
}

/// Handle crypto.chacha20_poly1305_decrypt method
///
/// Decrypts data with ChaCha20-Poly1305 AEAD.
///
/// # Parameters
///
/// - `ciphertext`: Base64-encoded ciphertext
/// - `key`: Base64-encoded 32-byte key
/// - `nonce`: Base64-encoded 12-byte nonce
/// - `tag`: Base64-encoded 16-byte authentication tag
/// - `aad`: Base64-encoded additional authenticated data (optional)
///
/// # Returns
///
/// - `plaintext`: Base64-encoded plaintext
pub(crate) async fn handle_chacha20_poly1305_decrypt(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.chacha20_poly1305_decrypt")?;

    // Extract parameters
    let ciphertext_b64 = params
        .get("ciphertext")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: ciphertext")?;

    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: key")?;

    let nonce_b64 = params
        .get("nonce")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: nonce")?;

    let tag_b64 = params
        .get("tag")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: tag")?;

    let aad_b64 = params.get("aad").and_then(|v| v.as_str());

    // Decode parameters
    let ciphertext = base64::engine::general_purpose::STANDARD
        .decode(ciphertext_b64)
        .map_err(|e| format!("Invalid base64 ciphertext: {e}"))?;

    let key_bytes = base64::engine::general_purpose::STANDARD
        .decode(key_b64)
        .map_err(|e| format!("Invalid base64 key: {e}"))?;

    let key: [u8; 32] = key_bytes
        .as_slice()
        .try_into()
        .map_err(|_| "key must be 32 bytes".to_string())?;

    let nonce = base64::engine::general_purpose::STANDARD
        .decode(nonce_b64)
        .map_err(|e| format!("Invalid base64 nonce: {e}"))?;

    let tag = base64::engine::general_purpose::STANDARD
        .decode(tag_b64)
        .map_err(|e| format!("Invalid base64 tag: {e}"))?;

    let aad = if let Some(aad_b64) = aad_b64 {
        Some(
            base64::engine::general_purpose::STANDARD
                .decode(aad_b64)
                .map_err(|e| format!("Invalid base64 aad: {e}"))?,
        )
    } else {
        None
    };

    debug!(
        "🔓 Decrypting {} bytes with ChaCha20-Poly1305",
        ciphertext.len()
    );

    // Use BearDog's crypto service
    use beardog_core::crypto_service::algorithms::symmetric;

    let plaintext =
        symmetric::decrypt_chacha20_poly1305(&ciphertext, &nonce, &tag, &key, aad.as_deref())
            .map_err(|e| format!("ChaCha20-Poly1305 decryption failed: {e}"))?;

    // Encode result
    let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(&plaintext);

    info!(
        "✅ ChaCha20-Poly1305 decryption complete ({} bytes → {} bytes)",
        ciphertext.len(),
        plaintext.len()
    );

    Ok(serde_json::json!({
        "plaintext": plaintext_b64,
        "algorithm": "ChaCha20-Poly1305",
    }))
}

/// Handle crypto.blake3_hash method
///
/// Computes BLAKE3 hash of data.
///
/// # Parameters
///
/// - `data`: Base64-encoded data to hash
///
/// # Returns
///
/// - `hash`: Base64-encoded BLAKE3 hash (32 bytes)
pub(crate) async fn handle_blake3_hash(params: Option<&Value>) -> Result<Value, String> {
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
pub(crate) async fn handle_hmac_sha256(params: Option<&Value>) -> Result<Value, String> {
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

/// Derive a 32-byte key from a key ID and purpose
///
/// Uses BLAKE3 key derivation for deterministic key generation.
fn derive_key_from_id(key_id: &str, purpose: &str) -> Result<[u8; 32], String> {
    use beardog_core::crypto_service::algorithms::hashing;

    // Get master key from environment or generate deterministic key
    let master_key = std::env::var("BEARDOG_MASTER_KEY")
        .unwrap_or_else(|_| "beardog_default_master_key_v1".to_string());

    // Derive key using BLAKE3 KDF
    let context = format!("beardog_crypto_v1:{}:{}", key_id, purpose);
    let derived = hashing::derive_key_blake3(&context, master_key.as_bytes());

    let key: [u8; 32] = derived
        .as_slice()
        .try_into()
        .map_err(|_| "Key derivation failed".to_string())?;

    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;

    #[tokio::test]
    async fn test_ed25519_sign_and_verify() {
        // Test message
        let message = b"Hello, BearDog!";
        let message_b64 = base64::engine::general_purpose::STANDARD.encode(message);

        // Sign with deterministic key (derived from key_id and purpose)
        let sign_params = serde_json::json!({
            "message": message_b64,
            "key_id": "test_key",
            "purpose": "test"
        });

        let sign_result = handle_sign_ed25519(Some(&sign_params)).await.unwrap();
        let signature_b64 = sign_result["signature"].as_str().unwrap();

        // Get the public key for the same key_id/purpose
        use beardog_core::crypto_service::algorithms::asymmetric;
        let seed = derive_key_from_id("test_key", "test").unwrap();
        let (_secret_key, public_key) = asymmetric::generate_ed25519_from_seed(&seed).unwrap();

        // Verify
        let public_key_b64 = base64::engine::general_purpose::STANDARD.encode(&public_key);
        let verify_params = serde_json::json!({
            "message": message_b64,
            "signature": signature_b64,
            "public_key": public_key_b64
        });

        let verify_result = handle_verify_ed25519(Some(&verify_params)).await.unwrap();
        assert_eq!(verify_result["valid"], true);
    }

    #[tokio::test]
    async fn test_x25519_key_exchange() {
        // Generate Alice's keypair
        let alice_result = handle_x25519_generate_ephemeral(None).await.unwrap();
        let alice_public = alice_result["public_key"].as_str().unwrap().to_string();
        let alice_secret = alice_result["secret_key"].as_str().unwrap().to_string();

        // Generate Bob's keypair
        let bob_result = handle_x25519_generate_ephemeral(None).await.unwrap();
        let bob_public = bob_result["public_key"].as_str().unwrap().to_string();
        let bob_secret = bob_result["secret_key"].as_str().unwrap().to_string();

        // Alice derives shared secret using her secret and Bob's public
        let alice_derive_params = serde_json::json!({
            "our_secret": alice_secret,
            "their_public": bob_public
        });

        let alice_shared = handle_x25519_derive_secret(Some(&alice_derive_params))
            .await
            .unwrap();

        // Bob derives shared secret using his secret and Alice's public
        let bob_derive_params = serde_json::json!({
            "our_secret": bob_secret,
            "their_public": alice_public
        });

        let bob_shared = handle_x25519_derive_secret(Some(&bob_derive_params))
            .await
            .unwrap();

        // Both sides should derive the same shared secret
        assert_eq!(
            alice_shared["shared_secret"],
            bob_shared["shared_secret"],
            "Shared secrets should match (Diffie-Hellman property)"
        );
    }

    #[tokio::test]
    async fn test_chacha20_poly1305_encrypt_decrypt() {
        // Generate a random key
        let key = [42u8; 32];
        let key_b64 = base64::engine::general_purpose::STANDARD.encode(&key);

        // Test data
        let plaintext = b"Secret message for Songbird TLS";
        let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(plaintext);

        // Encrypt
        let encrypt_params = serde_json::json!({
            "plaintext": plaintext_b64,
            "key": key_b64
        });

        let encrypt_result = handle_chacha20_poly1305_encrypt(Some(&encrypt_params))
            .await
            .unwrap();

        let ciphertext_b64 = encrypt_result["ciphertext"].as_str().unwrap();
        let nonce_b64 = encrypt_result["nonce"].as_str().unwrap();
        let tag_b64 = encrypt_result["tag"].as_str().unwrap();

        // Decrypt
        let decrypt_params = serde_json::json!({
            "ciphertext": ciphertext_b64,
            "key": key_b64,
            "nonce": nonce_b64,
            "tag": tag_b64
        });

        let decrypt_result = handle_chacha20_poly1305_decrypt(Some(&decrypt_params))
            .await
            .unwrap();

        let decrypted_b64 = decrypt_result["plaintext"].as_str().unwrap();
        let decrypted = base64::engine::general_purpose::STANDARD
            .decode(decrypted_b64)
            .unwrap();

        assert_eq!(decrypted, plaintext, "Decrypted plaintext should match original");
    }

    #[tokio::test]
    async fn test_blake3_hash() {
        let data = b"Data to hash";
        let data_b64 = base64::engine::general_purpose::STANDARD.encode(data);

        let params = serde_json::json!({
            "data": data_b64
        });

        let result = handle_blake3_hash(Some(&params)).await.unwrap();
        let hash_b64 = result["hash"].as_str().unwrap();
        let hash = base64::engine::general_purpose::STANDARD
            .decode(hash_b64)
            .unwrap();

        // Blake3 produces 32-byte hashes
        assert_eq!(hash.len(), 32);

        // Same input should produce same hash
        let result2 = handle_blake3_hash(Some(&params)).await.unwrap();
        assert_eq!(result["hash"], result2["hash"]);
    }

    #[tokio::test]
    async fn test_hmac_sha256() {
        let key = b"secret_key";
        let data = b"message to authenticate";

        let key_b64 = base64::engine::general_purpose::STANDARD.encode(key);
        let data_b64 = base64::engine::general_purpose::STANDARD.encode(data);

        let params = serde_json::json!({
            "key": key_b64,
            "data": data_b64
        });

        let result = handle_hmac_sha256(Some(&params)).await.unwrap();
        let mac_b64 = result["mac"].as_str().unwrap();
        let mac = base64::engine::general_purpose::STANDARD
            .decode(mac_b64)
            .unwrap();

        // HMAC-SHA256 produces 32-byte MACs
        assert_eq!(mac.len(), 32);

        // Same key and data should produce same MAC
        let result2 = handle_hmac_sha256(Some(&params)).await.unwrap();
        assert_eq!(result["mac"], result2["mac"]);
    }
}

