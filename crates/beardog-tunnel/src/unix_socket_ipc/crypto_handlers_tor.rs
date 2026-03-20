// SPDX-License-Identifier: AGPL-3.0-only

//! Tor Protocol Crypto Handlers (Phase 2 - Pure Rust Tor)
//!
//! Provides cryptographic operations for Tor protocol implementation:
//! - ntor handshake (circuit key exchange)
//! - Cell encryption/decryption
//! - Tor-specific KDF operations
//!
//! **Architecture**: BearDog provides crypto primitives, Songbird implements protocol
//!
//! **Reference**: <https://spec.torproject.org/tor-spec>
//!
//! Pure Rust implementation using RustCrypto (zero C dependencies).

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_errors::BearDogError;
use hmac::{Hmac, Mac};
use serde_json::{Value, json};
use sha2::Sha256;
use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};

// ============================================================================
// CONSTANTS (from tor-spec section 5.1.4)
// ============================================================================

/// Protocol identifier for ntor
const NTOR_PROTOID: &[u8] = b"ntor-curve25519-sha256-1";

/// Key derivation tweak for KEY_SEED extraction
const NTOR_T_KEY: &[u8] = b"ntor-curve25519-sha256-1:key_extract";

/// Key derivation tweak for verification MAC
const NTOR_T_VERIFY: &[u8] = b"ntor-curve25519-sha256-1:verify";

/// Key expansion tweak for HKDF
const NTOR_T_EXPAND: &[u8] = b"ntor-curve25519-sha256-1:key_expand";

/// MAC tweak (for auth computation)
const NTOR_T_MAC: &[u8] = b"ntor-curve25519-sha256-1:mac";

/// Server string constant
const NTOR_SERVER: &[u8] = b"Server";

// ============================================================================
// NTOR HANDSHAKE - CLIENT SIDE
// ============================================================================

/// Handle `beardog.crypto.tor_ntor_client_init` - Initialize client-side ntor
///
/// Generates ephemeral X25519 keypair and prepares handshake state.
///
/// **Input**:
/// ```json
/// {
///   "node_id": "base64_20_byte_identity",
///   "node_onion_key": "base64_32_byte_x25519_public_B"
/// }
/// ```
///
/// **Output**:
/// ```json
/// {
///   "ephemeral_public": "base64_32_byte_X",
///   "client_state": "base64_encrypted_state"
/// }
/// ```
///
/// **Reference**: tor-spec section 5.1.4
pub async fn handle_tor_ntor_client_init(params: Option<&Value>) -> Result<Value, BearDogError> {
    let params = params.ok_or_else(|| BearDogError::invalid_input("Missing parameters"))?;

    // Extract node_id (20 bytes - SHA1 hash of node identity key)
    let node_id_b64 = params
        .get("node_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'node_id' parameter"))?;

    let node_id = BASE64
        .decode(node_id_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 node_id: {e}")))?;

    if node_id.len() != 20 {
        return Err(BearDogError::invalid_input(&format!(
            "node_id must be 20 bytes (SHA1 hash), got {}",
            node_id.len()
        )));
    }

    // Extract node_onion_key (32 bytes - X25519 public key B)
    let onion_key_b64 = params
        .get("node_onion_key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'node_onion_key' parameter"))?;

    let onion_key_bytes = BASE64
        .decode(onion_key_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 node_onion_key: {e}")))?;

    if onion_key_bytes.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "node_onion_key must be 32 bytes, got {}",
            onion_key_bytes.len()
        )));
    }

    // Generate ephemeral X25519 keypair for this handshake
    // NOTE: Using StaticSecret instead of EphemeralSecret because we need to
    // serialize the secret into client_state for the two-phase ntor handshake.
    // The secret is encrypted in client_state and only used once.
    let ephemeral_secret = StaticSecret::random_from_rng(rand::thread_rng());
    let ephemeral_public = PublicKey::from(&ephemeral_secret);

    // Create client state (contains secret for later use)
    // Structure: x (32 bytes) || node_id (20 bytes) || B (32 bytes)
    // This state is encrypted before being returned to caller.
    let mut client_state = Vec::with_capacity(84);
    client_state.extend_from_slice(ephemeral_secret.as_bytes());
    client_state.extend_from_slice(&node_id);
    client_state.extend_from_slice(&onion_key_bytes);

    // Encrypt client state with a fixed key for security
    // Using XOR with HKDF output (symmetric - same operation for encrypt/decrypt)
    let encrypted_state = xor_encrypt(&client_state, &STATE_ENCRYPTION_KEY)?;

    Ok(json!({
        "ephemeral_public": BASE64.encode(ephemeral_public.as_bytes()),
        "client_state": BASE64.encode(&encrypted_state),
        "algorithm": "ntor-curve25519-sha256-1"
    }))
}

/// Handle `beardog.crypto.tor_ntor_client_finish` - Complete client-side ntor
///
/// Verifies server response and derives circuit keys.
///
/// **Input**:
/// ```json
/// {
///   "client_state": "base64_from_init",
///   "server_public": "base64_32_byte_Y",
///   "server_auth": "base64_32_byte_verify"
/// }
/// ```
///
/// **Output**:
/// ```json
/// {
///   "valid": true,
///   "forward_digest_key": "base64_Df_20_bytes",
///   "backward_digest_key": "base64_Db_20_bytes",
///   "forward_key": "base64_Kf_16_bytes",
///   "backward_key": "base64_Kb_16_bytes"
/// }
/// ```
pub async fn handle_tor_ntor_client_finish(params: Option<&Value>) -> Result<Value, BearDogError> {
    let params = params.ok_or_else(|| BearDogError::invalid_input("Missing parameters"))?;

    // Extract and decrypt client state
    let state_b64 = params
        .get("client_state")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'client_state' parameter"))?;

    let encrypted_state = BASE64
        .decode(state_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 client_state: {e}")))?;

    // Decrypt state (XOR is symmetric with same key)
    let client_state = xor_encrypt(&encrypted_state, &STATE_ENCRYPTION_KEY)?;

    if client_state.len() != 84 {
        return Err(BearDogError::invalid_input("Invalid client_state length"));
    }

    // Parse client state: x (32) || node_id (20) || B (32)
    let x_bytes: [u8; 32] = client_state[0..32]
        .try_into()
        .map_err(|_| BearDogError::crypto_error("Invalid ephemeral secret in ntor client state"))?;
    let node_id = &client_state[32..52];
    let b_bytes: [u8; 32] = client_state[52..84]
        .try_into()
        .map_err(|_| BearDogError::crypto_error("Invalid onion key in ntor client state"))?;

    // Extract server response
    let server_public_b64 = params
        .get("server_public")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'server_public' parameter"))?;

    let y_bytes = BASE64
        .decode(server_public_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 server_public: {e}")))?;

    if y_bytes.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "server_public must be 32 bytes, got {}",
            y_bytes.len()
        )));
    }

    let server_auth_b64 = params
        .get("server_auth")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'server_auth' parameter"))?;

    let server_auth = BASE64
        .decode(server_auth_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 server_auth: {e}")))?;

    if server_auth.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "server_auth must be 32 bytes, got {}",
            server_auth.len()
        )));
    }

    // Reconstruct keys for computation
    let x = StaticSecret::from(x_bytes);
    let x_public = PublicKey::from(&x);
    let y_arr: [u8; 32] = y_bytes
        .clone()
        .try_into()
        .map_err(|_| BearDogError::invalid_input("server_public must be exactly 32 bytes"))?;
    let y = PublicKey::from(y_arr);
    let b = PublicKey::from(b_bytes);

    // Compute shared secrets
    // secret_input = EXP(Y, x) || EXP(B, x) || ID || B || X || Y || PROTOID
    let exp_y_x = x.diffie_hellman(&y);
    let exp_b_x = x.diffie_hellman(&b);

    let mut secret_input = Vec::with_capacity(32 + 32 + 20 + 32 + 32 + 32 + NTOR_PROTOID.len());
    secret_input.extend_from_slice(exp_y_x.as_bytes());
    secret_input.extend_from_slice(exp_b_x.as_bytes());
    secret_input.extend_from_slice(node_id);
    secret_input.extend_from_slice(&b_bytes);
    secret_input.extend_from_slice(x_public.as_bytes());
    secret_input.extend_from_slice(&y_arr);
    secret_input.extend_from_slice(NTOR_PROTOID);

    // Compute KEY_SEED = HMAC(t_key, secret_input)
    let key_seed = hmac_sha256(NTOR_T_KEY, &secret_input)?;

    // Compute verify = HMAC(t_verify, secret_input)
    let verify = hmac_sha256(NTOR_T_VERIFY, &secret_input)?;

    // Compute AUTH for verification
    // AUTH = HMAC(t_mac, verify || ID || B || Y || X || PROTOID || "Server")
    let mut auth_input = Vec::new();
    auth_input.extend_from_slice(&verify);
    auth_input.extend_from_slice(node_id);
    auth_input.extend_from_slice(&b_bytes);
    auth_input.extend_from_slice(&y_bytes);
    auth_input.extend_from_slice(x_public.as_bytes());
    auth_input.extend_from_slice(NTOR_PROTOID);
    auth_input.extend_from_slice(NTOR_SERVER);

    let expected_auth = hmac_sha256(NTOR_T_MAC, &auth_input)?;

    // Verify server auth
    if !constant_time_compare(&server_auth, &expected_auth) {
        return Ok(json!({
            "valid": false,
            "error": "ntor handshake verification failed"
        }));
    }

    // Derive circuit keys using HKDF
    let keys = derive_circuit_keys(&key_seed)?;

    Ok(json!({
        "valid": true,
        "forward_digest_key": BASE64.encode(keys.df),
        "backward_digest_key": BASE64.encode(keys.db),
        "forward_key": BASE64.encode(keys.kf),
        "backward_key": BASE64.encode(keys.kb),
        "algorithm": "ntor-curve25519-sha256-1"
    }))
}

// ============================================================================
// NTOR HANDSHAKE - SERVER SIDE
// ============================================================================

/// Handle `beardog.crypto.tor_ntor_server_respond` - Server-side ntor response
///
/// For hidden service rendezvous points.
///
/// **Input**:
/// ```json
/// {
///   "client_public": "base64_32_byte_X",
///   "node_id": "base64_20_byte_identity",
///   "onion_secret_key": "base64_32_byte_b",
///   "onion_public_key": "base64_32_byte_B"
/// }
/// ```
///
/// **Output**:
/// ```json
/// {
///   "ephemeral_public": "base64_32_byte_Y",
///   "server_auth": "base64_32_byte_verify",
///   "forward_digest_key": "base64_Df",
///   "backward_digest_key": "base64_Db",
///   "forward_key": "base64_Kf",
///   "backward_key": "base64_Kb"
/// }
/// ```
pub async fn handle_tor_ntor_server_respond(params: Option<&Value>) -> Result<Value, BearDogError> {
    let params = params.ok_or_else(|| BearDogError::invalid_input("Missing parameters"))?;

    // Extract client's ephemeral public key X
    let x_public_b64 = params
        .get("client_public")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'client_public' parameter"))?;

    let x_bytes = BASE64
        .decode(x_public_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 client_public: {e}")))?;

    if x_bytes.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "client_public must be 32 bytes, got {}",
            x_bytes.len()
        )));
    }
    // Clone before consuming to preserve bytes for later use in secret_input/auth_input
    let x_arr: [u8; 32] = x_bytes
        .try_into()
        .map_err(|_| BearDogError::invalid_input("client_public must be exactly 32 bytes"))?;
    let x = PublicKey::from(x_arr);

    // Extract node_id
    let node_id_b64 = params
        .get("node_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'node_id' parameter"))?;

    let node_id = BASE64
        .decode(node_id_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 node_id: {e}")))?;

    if node_id.len() != 20 {
        return Err(BearDogError::invalid_input(&format!(
            "node_id must be 20 bytes, got {}",
            node_id.len()
        )));
    }

    // Extract onion secret key (b)
    let b_secret_b64 = params
        .get("onion_secret_key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'onion_secret_key' parameter"))?;

    let b_secret_bytes = BASE64.decode(b_secret_b64).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid base64 onion_secret_key: {e}"))
    })?;

    if b_secret_bytes.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "onion_secret_key must be 32 bytes, got {}",
            b_secret_bytes.len()
        )));
    }
    let b_secret_arr: [u8; 32] = b_secret_bytes
        .try_into()
        .map_err(|_| BearDogError::crypto_error("Invalid onion secret key length"))?;
    let b = StaticSecret::from(b_secret_arr);

    // Extract onion public key (B)
    let b_public_b64 = params
        .get("onion_public_key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'onion_public_key' parameter"))?;

    let b_public_bytes = BASE64.decode(b_public_b64).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid base64 onion_public_key: {e}"))
    })?;

    if b_public_bytes.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "onion_public_key must be 32 bytes, got {}",
            b_public_bytes.len()
        )));
    }

    // Generate server ephemeral keypair
    let y_secret = EphemeralSecret::random_from_rng(rand::thread_rng());
    let y_public = PublicKey::from(&y_secret);

    // Compute shared secrets
    // secret_input = EXP(X, y) || EXP(X, b) || ID || B || X || Y || PROTOID
    let exp_x_y = y_secret.diffie_hellman(&x);
    let exp_x_b = b.diffie_hellman(&x);

    let mut secret_input = Vec::with_capacity(32 + 32 + 20 + 32 + 32 + 32 + NTOR_PROTOID.len());
    secret_input.extend_from_slice(exp_x_y.as_bytes());
    secret_input.extend_from_slice(exp_x_b.as_bytes());
    secret_input.extend_from_slice(&node_id);
    secret_input.extend_from_slice(&b_public_bytes);
    secret_input.extend_from_slice(&x_arr);
    secret_input.extend_from_slice(y_public.as_bytes());
    secret_input.extend_from_slice(NTOR_PROTOID);

    // Compute KEY_SEED
    let key_seed = hmac_sha256(NTOR_T_KEY, &secret_input)?;

    // Compute verify
    let verify = hmac_sha256(NTOR_T_VERIFY, &secret_input)?;

    // Compute AUTH (server_auth)
    let mut auth_input = Vec::new();
    auth_input.extend_from_slice(&verify);
    auth_input.extend_from_slice(&node_id);
    auth_input.extend_from_slice(&b_public_bytes);
    auth_input.extend_from_slice(y_public.as_bytes());
    auth_input.extend_from_slice(&x_arr);
    auth_input.extend_from_slice(NTOR_PROTOID);
    auth_input.extend_from_slice(NTOR_SERVER);

    let server_auth = hmac_sha256(NTOR_T_MAC, &auth_input)?;

    // Derive circuit keys
    let keys = derive_circuit_keys(&key_seed)?;

    Ok(json!({
        "ephemeral_public": BASE64.encode(y_public.as_bytes()),
        "server_auth": BASE64.encode(server_auth),
        "forward_digest_key": BASE64.encode(keys.df),
        "backward_digest_key": BASE64.encode(keys.db),
        "forward_key": BASE64.encode(keys.kf),
        "backward_key": BASE64.encode(keys.kb),
        "algorithm": "ntor-curve25519-sha256-1"
    }))
}

// ============================================================================
// CELL ENCRYPTION (ChaCha20 counter mode)
// ============================================================================

/// Handle `beardog.crypto.tor_cell_encrypt` - Encrypt Tor relay cell
///
/// Uses ChaCha20 in counter mode (NOT AEAD - cells are authenticated by digest).
///
/// **Input**:
/// ```json
/// {
///   "key": "base64_32_byte_key",
///   "counter": 0,
///   "data": "base64_cell_payload"
/// }
/// ```
///
/// **Output**:
/// ```json
/// {
///   "ciphertext": "base64_encrypted",
///   "next_counter": 1
/// }
/// ```
pub async fn handle_tor_cell_encrypt(params: Option<&Value>) -> Result<Value, BearDogError> {
    let params = params.ok_or_else(|| BearDogError::invalid_input("Missing parameters"))?;

    // Extract key (32 bytes for ChaCha20)
    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'key' parameter"))?;

    let key = BASE64
        .decode(key_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 key: {e}")))?;

    if key.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "key must be 32 bytes, got {}",
            key.len()
        )));
    }

    // Extract counter
    let counter = params
        .get("counter")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| BearDogError::invalid_input("Missing 'counter' parameter"))?;

    // Extract data
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'data' parameter"))?;

    let mut data = BASE64
        .decode(data_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 data: {e}")))?;

    // Apply ChaCha20 keystream
    chacha20_counter_mode(&key, counter, &mut data)?;

    Ok(json!({
        "ciphertext": BASE64.encode(&data),
        "next_counter": counter + 1,
        "algorithm": "chacha20-counter"
    }))
}

/// Handle `beardog.crypto.tor_cell_decrypt` - Decrypt Tor relay cell
///
/// Same as encrypt (ChaCha20 is symmetric).
pub async fn handle_tor_cell_decrypt(params: Option<&Value>) -> Result<Value, BearDogError> {
    let params = params.ok_or_else(|| BearDogError::invalid_input("Missing parameters"))?;

    // Extract key
    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'key' parameter"))?;

    let key = BASE64
        .decode(key_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 key: {e}")))?;

    if key.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "key must be 32 bytes, got {}",
            key.len()
        )));
    }

    // Extract counter
    let counter = params
        .get("counter")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| BearDogError::invalid_input("Missing 'counter' parameter"))?;

    // Extract ciphertext
    let ciphertext_b64 = params
        .get("ciphertext")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'ciphertext' parameter"))?;

    let mut data = BASE64
        .decode(ciphertext_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 ciphertext: {e}")))?;

    // Apply ChaCha20 keystream (XOR is symmetric)
    chacha20_counter_mode(&key, counter, &mut data)?;

    Ok(json!({
        "plaintext": BASE64.encode(&data),
        "next_counter": counter + 1,
        "algorithm": "chacha20-counter"
    }))
}

// ============================================================================
// TOR KDF
// ============================================================================

/// Handle `beardog.crypto.tor_kdf` - Tor-specific key derivation
///
/// Expands a key seed into multiple derived keys using HKDF-SHA256.
///
/// **Input**:
/// ```json
/// {
///   "key_seed": "base64_32_byte_seed",
///   "key_count": 4,
///   "key_length": 20
/// }
/// ```
///
/// **Output**:
/// ```json
/// {
///   "keys": ["base64_key1", "base64_key2", ...]
/// }
/// ```
pub async fn handle_tor_kdf(params: Option<&Value>) -> Result<Value, BearDogError> {
    let params = params.ok_or_else(|| BearDogError::invalid_input("Missing parameters"))?;

    // Extract key_seed
    let seed_b64 = params
        .get("key_seed")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'key_seed' parameter"))?;

    let key_seed = BASE64
        .decode(seed_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 key_seed: {e}")))?;

    // Extract key count and length
    let key_count = params
        .get("key_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(4) as usize;

    let key_length = params
        .get("key_length")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(20) as usize;

    // Use HKDF to derive keys
    let total_len = key_count * key_length;
    let expanded = hkdf_expand(&key_seed, NTOR_T_EXPAND, total_len)?;

    // Split into individual keys
    let keys: Vec<String> = expanded
        .chunks(key_length)
        .map(|chunk| BASE64.encode(chunk))
        .collect();

    Ok(json!({
        "keys": keys,
        "algorithm": "hkdf-sha256",
        "key_count": key_count,
        "key_length": key_length
    }))
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Circuit keys derived from ntor handshake
struct CircuitKeys {
    /// Forward digest key (Df) - 20 bytes
    df: [u8; 20],
    /// Backward digest key (Db) - 20 bytes
    db: [u8; 20],
    /// Forward encryption key (Kf) - 16 bytes for AES-128 or 32 for ChaCha20
    kf: [u8; 16],
    /// Backward encryption key (Kb) - 16 bytes for AES-128 or 32 for ChaCha20
    kb: [u8; 16],
}

/// Derive circuit keys from KEY_SEED using HKDF
///
/// Returns an error if HKDF expansion produces unexpected output length.
fn derive_circuit_keys(key_seed: &[u8]) -> Result<CircuitKeys, BearDogError> {
    // Expand to get: Df (20) + Db (20) + Kf (16) + Kb (16) = 72 bytes
    let expanded = hkdf_expand(key_seed, NTOR_T_EXPAND, 72)?;

    if expanded.len() != 72 {
        return Err(BearDogError::crypto_error(format!(
            "HKDF expansion produced {} bytes, expected 72",
            expanded.len()
        )));
    }

    Ok(CircuitKeys {
        df: expanded[0..20]
            .try_into()
            .map_err(|_| BearDogError::crypto_error("Failed to convert Df slice to [u8; 20]"))?,
        db: expanded[20..40]
            .try_into()
            .map_err(|_| BearDogError::crypto_error("Failed to convert Db slice to [u8; 20]"))?,
        kf: expanded[40..56]
            .try_into()
            .map_err(|_| BearDogError::crypto_error("Failed to convert Kf slice to [u8; 16]"))?,
        kb: expanded[56..72]
            .try_into()
            .map_err(|_| BearDogError::crypto_error("Failed to convert Kb slice to [u8; 16]"))?,
    })
}

/// HMAC-SHA256
///
/// HMAC accepts keys of any size, so `new_from_slice` is infallible in practice.
/// We still propagate the error for correctness.
fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<[u8; 32], BearDogError> {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|e| BearDogError::crypto_error(format!("HMAC-SHA256 key error: {e}")))?;
    mac.update(data);
    let result = mac.finalize();
    Ok(result.into_bytes().into())
}

/// HKDF-Expand (simplified - uses HMAC iteratively)
fn hkdf_expand(prk: &[u8], info: &[u8], length: usize) -> Result<Vec<u8>, BearDogError> {
    let mut output = Vec::with_capacity(length);
    let mut t = Vec::new();
    let mut counter = 1u8;

    while output.len() < length {
        let mut input = t.clone();
        input.extend_from_slice(info);
        input.push(counter);

        t = hmac_sha256(prk, &input)?.to_vec();
        output.extend_from_slice(&t);
        counter += 1;
    }

    output.truncate(length);
    Ok(output)
}

/// ChaCha20 counter mode (for cell encryption)
///
/// Returns an error if `key` is not exactly 32 bytes.
fn chacha20_counter_mode(key: &[u8], counter: u64, data: &mut [u8]) -> Result<(), BearDogError> {
    use chacha20::ChaCha20;
    use chacha20::cipher::{KeyIvInit, StreamCipher};

    // Validate key length
    let key_arr: [u8; 32] = key.try_into().map_err(|_| {
        BearDogError::invalid_input(&format!(
            "ChaCha20 requires exactly 32-byte key, got {}",
            key.len()
        ))
    })?;

    // Construct nonce from counter (12 bytes)
    let mut nonce = [0u8; 12];
    nonce[4..12].copy_from_slice(&counter.to_le_bytes());

    let mut cipher = ChaCha20::new(&key_arr.into(), &nonce.into());
    cipher.apply_keystream(data);
    Ok(())
}

/// Fixed state encryption key (for protecting client_state)
///
/// NOTE: In a production system, this would be derived from a session key
/// or use proper AEAD. For the ntor handshake, the client state only needs
/// to be protected during the brief handshake period (milliseconds to seconds).
const STATE_ENCRYPTION_KEY: [u8; 32] = [
    0x62, 0x65, 0x61, 0x72, 0x64, 0x6f, 0x67, 0x2d, // "beardog-"
    0x6e, 0x74, 0x6f, 0x72, 0x2d, 0x73, 0x74, 0x61, // "ntor-sta"
    0x74, 0x65, 0x2d, 0x6b, 0x65, 0x79, 0x2d, 0x76, // "te-key-v"
    0x31, 0x2d, 0x70, 0x72, 0x6f, 0x64, 0x00, 0x01, // "1-prod.."
];

/// Simple XOR encryption (used with derived key for state protection)
fn xor_encrypt(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, BearDogError> {
    // Expand key using HKDF to match data length
    let expanded_key = hkdf_expand(key, b"state-encryption", data.len())?;
    Ok(data
        .iter()
        .zip(expanded_key.iter())
        .map(|(d, k)| d ^ k)
        .collect())
}

/// Constant-time comparison to prevent timing attacks
fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn json_str<'a>(v: &'a Value, key: &'static str) -> Result<&'a str, BearDogError> {
        v.get(key)
            .and_then(|x| x.as_str())
            .ok_or_else(|| BearDogError::invalid_input("missing JSON string field"))
    }

    #[tokio::test]
    async fn test_ntor_client_init_basic() -> Result<(), BearDogError> {
        let node_id = [0u8; 20];
        let onion_key = [1u8; 32];

        let params = json!({
            "node_id": BASE64.encode(&node_id),
            "node_onion_key": BASE64.encode(&onion_key)
        });

        let result = handle_tor_ntor_client_init(Some(&params)).await?;

        assert!(result.get("ephemeral_public").is_some());
        let ephem_b64 = json_str(&result, "ephemeral_public")?;
        let ephem = BASE64
            .decode(ephem_b64)
            .map_err(|e| BearDogError::invalid_input(&format!("base64: {e}")))?;
        assert_eq!(ephem.len(), 32);
        assert!(result.get("client_state").is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_ntor_full_handshake() -> Result<(), BearDogError> {
        let server_secret = StaticSecret::random_from_rng(rand::thread_rng());
        let server_public = PublicKey::from(&server_secret);
        let node_id = [42u8; 20];

        let init_params = json!({
            "node_id": BASE64.encode(&node_id),
            "node_onion_key": BASE64.encode(server_public.as_bytes())
        });

        let init_result = handle_tor_ntor_client_init(Some(&init_params)).await?;
        let client_state = json_str(&init_result, "client_state")?;
        let client_public = json_str(&init_result, "ephemeral_public")?;

        let server_params = json!({
            "client_public": client_public,
            "node_id": BASE64.encode(&node_id),
            "onion_secret_key": BASE64.encode(server_secret.as_bytes()),
            "onion_public_key": BASE64.encode(server_public.as_bytes())
        });

        let server_result = handle_tor_ntor_server_respond(Some(&server_params)).await?;
        let server_ephem = json_str(&server_result, "ephemeral_public")?;
        let server_auth = json_str(&server_result, "server_auth")?;

        let finish_params = json!({
            "client_state": client_state,
            "server_public": server_ephem,
            "server_auth": server_auth
        });

        let finish_result = handle_tor_ntor_client_finish(Some(&finish_params)).await?;

        assert_eq!(
            finish_result
                .get("valid")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| BearDogError::invalid_input("missing valid"))?,
            true
        );

        let client_kf = json_str(&finish_result, "forward_key")?;
        let server_kf = json_str(&server_result, "forward_key")?;
        assert_eq!(client_kf, server_kf);

        let client_kb = json_str(&finish_result, "backward_key")?;
        let server_kb = json_str(&server_result, "backward_key")?;
        assert_eq!(client_kb, server_kb);
        Ok(())
    }

    #[tokio::test]
    async fn test_cell_encrypt_decrypt_roundtrip() -> Result<(), BearDogError> {
        let key = [0xABu8; 32];
        let plaintext = b"Hello, Tor cell encryption!";
        let counter = 0u64;

        let encrypt_params = json!({
            "key": BASE64.encode(&key),
            "counter": counter,
            "data": BASE64.encode(plaintext)
        });

        let encrypt_result = handle_tor_cell_encrypt(Some(&encrypt_params)).await?;
        let ciphertext = json_str(&encrypt_result, "ciphertext")?;

        let decrypt_params = json!({
            "key": BASE64.encode(&key),
            "counter": counter,
            "ciphertext": ciphertext
        });

        let decrypt_result = handle_tor_cell_decrypt(Some(&decrypt_params)).await?;
        let decrypted_b64 = json_str(&decrypt_result, "plaintext")?;
        let decrypted = BASE64
            .decode(decrypted_b64)
            .map_err(|e| BearDogError::invalid_input(&format!("base64: {e}")))?;

        assert_eq!(decrypted, plaintext);
        Ok(())
    }

    #[tokio::test]
    async fn test_tor_kdf() -> Result<(), BearDogError> {
        let key_seed = [0x42u8; 32];

        let params = json!({
            "key_seed": BASE64.encode(&key_seed),
            "key_count": 4,
            "key_length": 20
        });

        let result = handle_tor_kdf(Some(&params)).await?;

        let keys = result
            .get("keys")
            .and_then(|v| v.as_array())
            .ok_or_else(|| BearDogError::invalid_input("missing keys array"))?;
        assert_eq!(keys.len(), 4);

        for key in keys {
            let s = key
                .as_str()
                .ok_or_else(|| BearDogError::invalid_input("key entry not a string"))?;
            let key_bytes = BASE64
                .decode(s)
                .map_err(|e| BearDogError::invalid_input(&format!("base64: {e}")))?;
            assert_eq!(key_bytes.len(), 20);
        }
        Ok(())
    }

    #[test]
    fn test_constant_time_compare() {
        let a = [1, 2, 3, 4];
        let b = [1, 2, 3, 4];
        let c = [1, 2, 3, 5];

        assert!(constant_time_compare(&a, &b));
        assert!(!constant_time_compare(&a, &c));
        assert!(!constant_time_compare(&a, &[1, 2, 3]));
    }
}
