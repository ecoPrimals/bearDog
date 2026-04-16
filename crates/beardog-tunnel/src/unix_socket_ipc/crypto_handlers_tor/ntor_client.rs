// SPDX-License-Identifier: AGPL-3.0-or-later

//! NTOR handshake — client side.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_errors::BearDogError;
use serde_json::{Value, json};
use x25519_dalek::{PublicKey, StaticSecret};

use super::constants::{NTOR_PROTOID, NTOR_SERVER, NTOR_T_KEY, NTOR_T_MAC, NTOR_T_VERIFY};
use super::primitives::{
    STATE_ENCRYPTION_KEY, constant_time_compare, derive_circuit_keys, hmac_sha256, xor_encrypt,
};

/// Handle `beardog.crypto.tor_ntor_client_init` - Initialize client-side ntor.
///
/// # Errors
///
/// Returns an error on invalid parameters or key generation failure.
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
    let ephemeral_secret = StaticSecret::random_from_rng(chacha20poly1305::aead::OsRng);
    let ephemeral_public = PublicKey::from(&ephemeral_secret);

    // Create client state (contains secret for later use)
    let mut client_state = Vec::with_capacity(84);
    client_state.extend_from_slice(ephemeral_secret.as_bytes());
    client_state.extend_from_slice(&node_id);
    client_state.extend_from_slice(&onion_key_bytes);

    let encrypted_state = xor_encrypt(&client_state, &STATE_ENCRYPTION_KEY)?;

    Ok(json!({
        "ephemeral_public": BASE64.encode(ephemeral_public.as_bytes()),
        "client_state": BASE64.encode(&encrypted_state),
        "algorithm": "ntor-curve25519-sha256-1"
    }))
}

/// Handle `beardog.crypto.tor_ntor_client_finish` - Complete client-side ntor.
///
/// # Errors
///
/// Returns an error on invalid state, authentication failure, or key derivation error.
pub async fn handle_tor_ntor_client_finish(params: Option<&Value>) -> Result<Value, BearDogError> {
    let params = params.ok_or_else(|| BearDogError::invalid_input("Missing parameters"))?;

    let state_b64 = params
        .get("client_state")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'client_state' parameter"))?;

    let encrypted_state = BASE64
        .decode(state_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 client_state: {e}")))?;

    let client_state = xor_encrypt(&encrypted_state, &STATE_ENCRYPTION_KEY)?;

    if client_state.len() != 84 {
        return Err(BearDogError::invalid_input("Invalid client_state length"));
    }

    let x_bytes: [u8; 32] = client_state[0..32]
        .try_into()
        .map_err(|_| BearDogError::crypto_error("Invalid ephemeral secret in ntor client state"))?;
    let node_id = &client_state[32..52];
    let b_bytes: [u8; 32] = client_state[52..84]
        .try_into()
        .map_err(|_| BearDogError::crypto_error("Invalid onion key in ntor client state"))?;

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

    let x = StaticSecret::from(x_bytes);
    let x_public = PublicKey::from(&x);
    let y_arr: [u8; 32] = y_bytes
        .clone()
        .try_into()
        .map_err(|_| BearDogError::invalid_input("server_public must be exactly 32 bytes"))?;
    let y = PublicKey::from(y_arr);
    let b = PublicKey::from(b_bytes);

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

    let key_seed = hmac_sha256(NTOR_T_KEY, &secret_input)?;
    let verify = hmac_sha256(NTOR_T_VERIFY, &secret_input)?;

    let mut auth_input = Vec::new();
    auth_input.extend_from_slice(&verify);
    auth_input.extend_from_slice(node_id);
    auth_input.extend_from_slice(&b_bytes);
    auth_input.extend_from_slice(&y_bytes);
    auth_input.extend_from_slice(x_public.as_bytes());
    auth_input.extend_from_slice(NTOR_PROTOID);
    auth_input.extend_from_slice(NTOR_SERVER);

    let expected_auth = hmac_sha256(NTOR_T_MAC, &auth_input)?;

    if !constant_time_compare(&server_auth, &expected_auth) {
        return Ok(json!({
            "valid": false,
            "error": "ntor handshake verification failed"
        }));
    }

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
