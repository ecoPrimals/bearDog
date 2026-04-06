// SPDX-License-Identifier: AGPL-3.0-or-later

//! Utility functions for crypto handlers
//!
//! This module contains shared helper functions used across multiple
//! crypto handler domains.
//!
//! # Helpers Provided
//!
//! - `derive_key_from_id()` - BLAKE3 key derivation from key ID + purpose
//! - `decode_base64_field()` - Base64 decode with field-specific error messages
//! - `require_params()` - Extract required JSON-RPC parameters
//! - `extract_str_param()` - Extract optional string parameter from JSON
//! - `deserialize_request()` - Deserialize JSON params with method-specific errors

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_core::crypto_service::algorithms::hashing;
use beardog_errors::BearDogError;
use rand::RngCore;
use serde::de::DeserializeOwned;
use serde_json::Value;

/// Derive a 32-byte key from a key ID and purpose
///
/// Uses BLAKE3 key derivation for deterministic key generation.
/// This is used by various crypto handlers to derive signing keys,
/// encryption keys, and other cryptographic material from a key identifier.
///
/// # Parameters
///
/// - `key_id`: The key identifier (e.g., "default", "prod", "test")
/// - `purpose`: The purpose of the key (e.g., "signature", "encryption", `tls_handshake`)
///
/// # Returns
///
/// A 32-byte key derived deterministically from the inputs.
///
/// # Security Note
///
/// The master key is read from the `BEARDOG_MASTER_KEY` environment variable.
/// If unset, an ephemeral random key is generated per process lifetime.
/// Ephemeral keys mean derived material does NOT survive restarts.
/// In production, always set a strong, random `BEARDOG_MASTER_KEY`.
///
/// # Example
///
/// ```rust,ignore
/// let key = derive_key_from_id("prod", "signature")?;
/// // Use key for signing operations
/// ```
/// Exposes [`derive_key_from_id`] to integration tests in other modules (e.g. crypto RPC routers).
#[cfg(test)]
pub(crate) fn derive_key_from_id_for_tests(
    key_id: &str,
    purpose: &str,
) -> Result<[u8; 32], String> {
    derive_key_from_id(key_id, purpose)
}

pub(super) fn derive_key_from_id(key_id: &str, purpose: &str) -> Result<[u8; 32], String> {
    use std::sync::OnceLock;

    static EPHEMERAL_KEY: OnceLock<String> = OnceLock::new();

    let master_key = beardog_errors::process_env::var("BEARDOG_MASTER_KEY").unwrap_or_else(|_| {
        EPHEMERAL_KEY
            .get_or_init(|| {
                let mut buf = [0u8; 32];
                rand::rng().fill_bytes(&mut buf);
                let hex = hex::encode(buf);
                tracing::warn!(
                    "BEARDOG_MASTER_KEY not set — using ephemeral random key. \
                         Keys will not survive restarts. Set BEARDOG_MASTER_KEY for persistence."
                );
                hex
            })
            .clone()
    });

    // Derive key using BLAKE3 KDF
    let context = format!("beardog_crypto_v1:{key_id}:{purpose}");
    let derived = hashing::derive_key_blake3(&context, master_key.as_bytes());

    let key: [u8; 32] = derived
        .as_slice()
        .try_into()
        .map_err(|_| "Key derivation failed".to_string())?;

    Ok(key)
}

// ============================================================================
// Base64 Helpers
// ============================================================================

/// # Errors
///
/// Returns an error if serialization fails.
/// Decode base64 with a descriptive field name for error messages
///
/// # Example
///
/// ```rust,ignore
/// let data = decode_base64_field("ciphertext", &request.ciphertext)?;
/// ```
pub fn decode_base64_field(field_name: &str, input: &str) -> Result<Vec<u8>, BearDogError> {
    BASE64.decode(input).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid {field_name} (not valid base64): {e}"))
    })
}

/// # Errors
///
/// Returns an error if serialization fails.
/// Decode base64 returning String error (for handlers using Result<_, String>)
///
/// This is a transition helper while migrating to proper error types.
pub fn decode_base64_field_str(field_name: &str, input: &str) -> Result<Vec<u8>, String> {
    BASE64
        .decode(input)
        .map_err(|e| format!("Invalid {field_name} (not valid base64): {e}"))
}

// ============================================================================
// Parameter Extraction Helpers
// ============================================================================

/// # Errors
///
/// Returns an error if serialization fails.
/// Require parameters from a JSON-RPC request
///
/// # Example
///
/// ```rust,ignore
/// let params = require_params(params)?;
/// let key_id = extract_str_param(&params, "key_id")?;
/// ```
pub fn require_params(params: Option<&Value>) -> Result<&Value, String> {
    params.ok_or_else(|| "Missing required parameters".to_string())
}

/// Extract an optional string parameter from JSON
///
/// Returns `None` if the field doesn't exist or isn't a string.
pub fn extract_str_param<'a>(params: &'a Value, field: &str) -> Option<&'a str> {
    params.get(field).and_then(|v| v.as_str())
}

/// # Errors
///
/// Returns an error if serialization fails.
/// Extract a required string parameter from JSON
///
/// Returns an error if the field doesn't exist or isn't a string.
pub fn require_str_param<'a>(params: &'a Value, field: &str) -> Result<&'a str, String> {
    extract_str_param(params, field)
        .ok_or_else(|| format!("Missing or invalid '{field}' parameter"))
}

/// Extract an optional integer parameter from JSON
pub fn extract_u64_param(params: &Value, field: &str) -> Option<u64> {
    params.get(field).and_then(serde_json::Value::as_u64)
}

// ============================================================================
// Deserialization Helpers
// ============================================================================

/// # Errors
///
/// Returns an error if serialization fails.
/// Deserialize JSON parameters into a typed request with method-specific errors
///
/// # Example
///
/// ```rust,ignore
/// let request: SignRequest = deserialize_request(params, "crypto.sign_ed25519")?;
/// ```
pub fn deserialize_request<T: DeserializeOwned>(
    params: Value,
    method_name: &str,
) -> Result<T, BearDogError> {
    serde_json::from_value(params)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid {method_name} params: {e}")))
}

/// # Errors
///
/// Returns an error if serialization fails.
/// Deserialize with String error (transition helper)
pub fn deserialize_request_str<T: DeserializeOwned>(
    params: Value,
    method_name: &str,
) -> Result<T, String> {
    serde_json::from_value(params).map_err(|e| format!("Invalid {method_name} params: {e}"))
}

// ============================================================================
// Error Conversion Helpers
// ============================================================================

/// Convert any Display error to a `BearDogError` with context
///
/// # Example
///
/// ```rust,ignore
/// let result = operation().map_err(|e| to_security_error("encryption", e))?;
/// ```
pub fn to_security_error<E: std::fmt::Display>(operation: &str, error: E) -> BearDogError {
    BearDogError::security(format!("{operation} failed: {error}"))
}

/// Convert any Display error to String with context (transition helper)
pub fn to_error_string<E: std::fmt::Display>(operation: &str, error: E) -> String {
    format!("{operation} failed: {error}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json::json;

    #[test]
    fn test_derive_key_from_id() {
        let key = derive_key_from_id("test_key", "test").expect("derive_key_from_id in test");
        assert_eq!(key.len(), 32);

        // Same inputs should produce same key (deterministic)
        let key2 = derive_key_from_id("test_key", "test").expect("derive_key_from_id in test");
        assert_eq!(key, key2);

        // Different inputs should produce different keys
        let key3 = derive_key_from_id("test_key", "different").expect("derive_key_from_id in test");
        assert_ne!(key, key3);
    }

    #[test]
    fn test_decode_base64_field_valid() {
        let encoded = BASE64.encode(b"hello world");
        let result = decode_base64_field("test_data", &encoded);
        assert!(result.is_ok());
        assert_eq!(result.expect("decode_base64_field in test"), b"hello world");
    }

    #[test]
    fn test_decode_base64_field_invalid() {
        let result = decode_base64_field("test_data", "not-valid-base64!!!");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("test_data"));
        assert!(err.to_string().contains("not valid base64"));
    }

    #[test]
    fn test_require_params_some() {
        let value = json!({"key": "value"});
        let result = require_params(Some(&value));
        assert!(result.is_ok());
    }

    #[test]
    fn test_require_params_none() {
        let result = require_params(None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Missing"));
    }

    #[test]
    fn test_extract_str_param() {
        let params = json!({"name": "test", "count": 42});

        assert_eq!(extract_str_param(&params, "name"), Some("test"));
        assert_eq!(extract_str_param(&params, "count"), None); // Not a string
        assert_eq!(extract_str_param(&params, "missing"), None);
    }

    #[test]
    fn test_require_str_param() {
        let params = json!({"name": "test"});

        assert_eq!(
            require_str_param(&params, "name").expect("require_str_param in test"),
            "test"
        );
        assert!(require_str_param(&params, "missing").is_err());
    }

    #[test]
    fn test_extract_u64_param() {
        let params = json!({"count": 42, "name": "test"});

        assert_eq!(extract_u64_param(&params, "count"), Some(42));
        assert_eq!(extract_u64_param(&params, "name"), None);
        assert_eq!(extract_u64_param(&params, "missing"), None);
    }

    #[derive(Debug, Deserialize)]
    struct TestRequest {
        key_id: String,
        data: String,
    }

    #[test]
    fn test_deserialize_request_valid() {
        let params = json!({"key_id": "test", "data": "hello"});
        let result: Result<TestRequest, _> = deserialize_request(params, "test.method");
        assert!(result.is_ok());
        let req = result.expect("deserialize_request in test");
        assert_eq!(req.key_id, "test");
        assert_eq!(req.data, "hello");
    }

    #[test]
    fn test_deserialize_request_invalid() {
        let params = json!({"wrong_field": "test"});
        let result: Result<TestRequest, _> = deserialize_request(params, "test.method");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("test.method"));
    }

    #[test]
    fn test_to_security_error() {
        let err = to_security_error("encryption", "key not found");
        assert!(err.to_string().contains("encryption"));
        assert!(err.to_string().contains("key not found"));
    }

    #[test]
    fn test_to_error_string() {
        let err = to_error_string("decryption", "invalid tag");
        assert!(err.contains("decryption"));
        assert!(err.contains("invalid tag"));
    }
}
