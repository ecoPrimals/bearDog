// SPDX-License-Identifier: AGPL-3.0-only

//! SHA-2 / SHA-1 / SHA3-256 hash handlers for Unix socket crypto IPC.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_errors::BearDogError;
use serde_json::{Value, json};
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha384, Sha512};
use sha3::Sha3_256;

/// Handle `crypto.sha256` - SHA-256 hashing
pub fn handle_sha256(params: &Value) -> Result<Value, BearDogError> {
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'data' parameter"))?;

    let data = BASE64
        .decode(data_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 data: {e}")))?;

    let mut hasher = Sha256::new();
    hasher.update(&data);
    let hash = hasher.finalize();

    let hash_hex = hex::encode(hash);
    let hash_b64 = BASE64.encode(hash);

    Ok(json!({
        "hash": hash_hex,
        "hash_base64": hash_b64,
        "algorithm": "sha256",
        "output_bits": 256
    }))
}

/// Handle `crypto.sha384` - SHA-384 hashing
pub fn handle_sha384(params: &Value) -> Result<Value, BearDogError> {
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'data' parameter"))?;

    let data = BASE64
        .decode(data_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 data: {e}")))?;

    let mut hasher = Sha384::new();
    hasher.update(&data);
    let hash = hasher.finalize();

    let hash_hex = hex::encode(hash);
    let hash_b64 = BASE64.encode(hash);

    Ok(json!({
        "hash": hash_hex,
        "hash_base64": hash_b64,
        "algorithm": "sha384",
        "output_bits": 384
    }))
}

/// Handle `crypto.sha512` - SHA-512 hashing
pub fn handle_sha512(params: &Value) -> Result<Value, BearDogError> {
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'data' parameter"))?;

    let data = BASE64
        .decode(data_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 data: {e}")))?;

    let mut hasher = Sha512::new();
    hasher.update(&data);
    let hash = hasher.finalize();

    let hash_hex = hex::encode(hash);
    let hash_b64 = BASE64.encode(hash);

    Ok(json!({
        "hash": hash_hex,
        "hash_base64": hash_b64,
        "algorithm": "sha512",
        "output_bits": 512
    }))
}

/// Handle `crypto.sha1` - SHA-1 hashing (LEGACY ONLY - INSECURE!)
pub fn handle_sha1(params: &Value) -> Result<Value, BearDogError> {
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'data' parameter"))?;

    let data = BASE64
        .decode(data_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 data: {e}")))?;

    let mut hasher = Sha1::new();
    hasher.update(&data);
    let hash = hasher.finalize();

    let hash_hex = hex::encode(hash);
    let hash_b64 = BASE64.encode(hash);

    Ok(json!({
        "hash": hash_hex,
        "hash_base64": hash_b64,
        "algorithm": "sha1",
        "output_bits": 160,
        "warning": "SHA-1 is INSECURE for cryptographic purposes!"
    }))
}

/// Handle `crypto.sha3_256` - SHA3-256 hashing (Modern quantum-resistant)
pub fn handle_sha3_256(params: &Value) -> Result<Value, BearDogError> {
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'data' parameter"))?;

    let data = BASE64
        .decode(data_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 data: {e}")))?;

    let mut hasher = Sha3_256::new();
    hasher.update(&data);
    let hash = hasher.finalize();

    let hash_hex = hex::encode(hash);
    let hash_b64 = BASE64.encode(hash);

    Ok(json!({
        "hash": hash_hex,
        "hash_base64": hash_b64,
        "algorithm": "sha3_256",
        "output_bits": 256
    }))
}
