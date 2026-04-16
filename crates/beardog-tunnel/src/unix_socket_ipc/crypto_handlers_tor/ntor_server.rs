// SPDX-License-Identifier: AGPL-3.0-or-later

//! NTOR handshake — server side.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_errors::BearDogError;
use serde_json::{Value, json};
use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};

use super::constants::{NTOR_PROTOID, NTOR_SERVER, NTOR_T_KEY, NTOR_T_MAC, NTOR_T_VERIFY};
use super::primitives::{derive_circuit_keys, hmac_sha256};

/// Handle `beardog.crypto.tor_ntor_server_respond` - Server-side ntor response.
///
/// # Errors
///
/// Returns an error on invalid parameters, key exchange failure, or MAC computation error.
pub async fn handle_tor_ntor_server_respond(params: Option<&Value>) -> Result<Value, BearDogError> {
    let params = params.ok_or_else(|| BearDogError::invalid_input("Missing parameters"))?;

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
    let x_arr: [u8; 32] = x_bytes
        .try_into()
        .map_err(|_| BearDogError::invalid_input("client_public must be exactly 32 bytes"))?;
    let x = PublicKey::from(x_arr);

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

    let y_secret = EphemeralSecret::random_from_rng(chacha20poly1305::aead::OsRng);
    let y_public = PublicKey::from(&y_secret);

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

    let key_seed = hmac_sha256(NTOR_T_KEY, &secret_input)?;
    let verify = hmac_sha256(NTOR_T_VERIFY, &secret_input)?;

    let mut auth_input = Vec::new();
    auth_input.extend_from_slice(&verify);
    auth_input.extend_from_slice(&node_id);
    auth_input.extend_from_slice(&b_public_bytes);
    auth_input.extend_from_slice(y_public.as_bytes());
    auth_input.extend_from_slice(&x_arr);
    auth_input.extend_from_slice(NTOR_PROTOID);
    auth_input.extend_from_slice(NTOR_SERVER);

    let server_auth = hmac_sha256(NTOR_T_MAC, &auth_input)?;

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
