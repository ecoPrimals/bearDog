//! SHA Hashing Handlers (Phase 6 - Critical Production Gaps)
//!
//! Provides standalone SHA-256, SHA-384, and SHA-512 hashing for universal compatibility.
//! These are the most widely used cryptographic hash functions.
//!
//! **Usage**: 95%+ of systems (Bitcoin, TLS, file integrity, etc.)
//! **Performance**: < 1ms for typical payloads
//! **Security**: SHA-256/384/512 are secure (SHA-1 is legacy only!)
//!
//! Pure Rust implementation using RustCrypto `sha2` crate (zero C dependencies).

use beardog_errors::BearDogError;
use serde_json::{json, Value};
use sha2::{Digest, Sha256, Sha384, Sha512};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;

/// Handle `crypto.sha256` - SHA-256 hashing
///
/// SHA-256 is the most widely used cryptographic hash function.
/// Used in Bitcoin, TLS certificates, file integrity, HMAC, and countless other systems.
///
/// **Input**:
/// ```json
/// {
///   "data": "base64_encoded_data"
/// }
/// ```
///
/// **Output**:
/// ```json
/// {
///   "hash": "hex_encoded_sha256_hash",
///   "hash_base64": "base64_encoded_hash",
///   "algorithm": "sha256",
///   "output_bits": 256
/// }
/// ```
///
/// **Performance**: < 500μs for typical payloads (hardware accelerated on x86)
/// **Security**: Secure (no known practical attacks)
pub fn handle_sha256(params: &Value) -> Result<Value, BearDogError> {
    // Extract data parameter
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'data' parameter"))?;

    // Decode base64 data
    let data = BASE64
        .decode(data_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 data: {}", e)))?;

    // Compute SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let hash = hasher.finalize();

    // Encode as hex and base64
    let hash_hex = hex::encode(&hash);
    let hash_b64 = BASE64.encode(&hash);

    Ok(json!({
        "hash": hash_hex,
        "hash_base64": hash_b64,
        "algorithm": "sha256",
        "output_bits": 256
    }))
}

/// Handle `crypto.sha384` - SHA-384 hashing
///
/// SHA-384 is a truncated version of SHA-512, providing 384-bit security.
/// Used in high-security systems and protocols requiring collision resistance.
///
/// **Input**:
/// ```json
/// {
///   "data": "base64_encoded_data"
/// }
/// ```
///
/// **Output**:
/// ```json
/// {
///   "hash": "hex_encoded_sha384_hash",
///   "hash_base64": "base64_encoded_hash",
///   "algorithm": "sha384",
///   "output_bits": 384
/// }
/// ```
///
/// **Performance**: < 600μs for typical payloads
/// **Security**: Secure (higher collision resistance than SHA-256)
pub fn handle_sha384(params: &Value) -> Result<Value, BearDogError> {
    // Extract data parameter
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'data' parameter"))?;

    // Decode base64 data
    let data = BASE64
        .decode(data_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 data: {}", e)))?;

    // Compute SHA-384 hash
    let mut hasher = Sha384::new();
    hasher.update(&data);
    let hash = hasher.finalize();

    // Encode as hex and base64
    let hash_hex = hex::encode(&hash);
    let hash_b64 = BASE64.encode(&hash);

    Ok(json!({
        "hash": hash_hex,
        "hash_base64": hash_b64,
        "algorithm": "sha384",
        "output_bits": 384
    }))
}

/// Handle `crypto.sha512` - SHA-512 hashing
///
/// SHA-512 provides 512-bit security and is used in cryptographic protocols
/// requiring maximum collision resistance.
///
/// **Input**:
/// ```json
/// {
///   "data": "base64_encoded_data"
/// }
/// ```
///
/// **Output**:
/// ```json
/// {
///   "hash": "hex_encoded_sha512_hash",
///   "hash_base64": "base64_encoded_hash",
///   "algorithm": "sha512",
///   "output_bits": 512
/// }
/// ```
///
/// **Performance**: < 700μs for typical payloads
/// **Security**: Secure (highest collision resistance in SHA-2 family)
pub fn handle_sha512(params: &Value) -> Result<Value, BearDogError> {
    // Extract data parameter
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'data' parameter"))?;

    // Decode base64 data
    let data = BASE64
        .decode(data_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 data: {}", e)))?;

    // Compute SHA-512 hash
    let mut hasher = Sha512::new();
    hasher.update(&data);
    let hash = hasher.finalize();

    // Encode as hex and base64
    let hash_hex = hex::encode(&hash);
    let hash_b64 = BASE64.encode(&hash);

    Ok(json!({
        "hash": hash_hex,
        "hash_base64": hash_b64,
        "algorithm": "sha512",
        "output_bits": 512
    }))
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sha256_empty_string() {
        // SHA-256 of empty string (known test vector)
        let params = json!({
            "data": BASE64.encode(b"")
        });

        let result = handle_sha256(&params).unwrap();
        let hash = result.get("hash").unwrap().as_str().unwrap();

        // Known SHA-256 of empty string
        assert_eq!(
            hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_sha256_hello_world() {
        // SHA-256 of "Hello, World!"
        let params = json!({
            "data": BASE64.encode(b"Hello, World!")
        });

        let result = handle_sha256(&params).unwrap();
        let hash = result.get("hash").unwrap().as_str().unwrap();

        // Known SHA-256 of "Hello, World!"
        assert_eq!(
            hash,
            "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
        );
        assert_eq!(result.get("algorithm").unwrap().as_str().unwrap(), "sha256");
        assert_eq!(result.get("output_bits").unwrap().as_u64().unwrap(), 256);
    }

    #[test]
    fn test_sha256_bitcoin_genesis() {
        // SHA-256 of Bitcoin genesis block message (first 32 bytes)
        let message = b"The Times 03/Jan/2009 Chancellor";
        let params = json!({
            "data": BASE64.encode(message)
        });

        let result = handle_sha256(&params).unwrap();
        let hash_hex = result.get("hash").unwrap().as_str().unwrap();
        let hash_b64 = result.get("hash_base64").unwrap().as_str().unwrap();

        // Verify both hex and base64 outputs are present
        assert_eq!(hash_hex.len(), 64); // 32 bytes = 64 hex chars
        assert!(!hash_b64.is_empty());

        // Decode and verify they match
        let decoded_b64 = BASE64.decode(hash_b64).unwrap();
        assert_eq!(hex::encode(&decoded_b64), hash_hex);
    }

    #[test]
    fn test_sha384_empty_string() {
        // SHA-384 of empty string (known test vector)
        let params = json!({
            "data": BASE64.encode(b"")
        });

        let result = handle_sha384(&params).unwrap();
        let hash = result.get("hash").unwrap().as_str().unwrap();

        // Known SHA-384 of empty string
        assert_eq!(
            hash,
            "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b"
        );
    }

    #[test]
    fn test_sha384_hello_world() {
        // SHA-384 of "Hello, World!"
        let params = json!({
            "data": BASE64.encode(b"Hello, World!")
        });

        let result = handle_sha384(&params).unwrap();
        let hash = result.get("hash").unwrap().as_str().unwrap();

        // Known SHA-384 of "Hello, World!"
        assert_eq!(
            hash,
            "5485cc9b3365b4305dfb4e8337e0a598a574f8242bf17289e0dd6c20a3cd44a089de16ab4ab308f63e44b1170eb5f515"
        );
        assert_eq!(result.get("algorithm").unwrap().as_str().unwrap(), "sha384");
        assert_eq!(result.get("output_bits").unwrap().as_u64().unwrap(), 384);
    }

    #[test]
    fn test_sha512_empty_string() {
        // SHA-512 of empty string (known test vector)
        let params = json!({
            "data": BASE64.encode(b"")
        });

        let result = handle_sha512(&params).unwrap();
        let hash = result.get("hash").unwrap().as_str().unwrap();

        // Known SHA-512 of empty string
        assert_eq!(
            hash,
            "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"
        );
    }

    #[test]
    fn test_sha512_hello_world() {
        // SHA-512 of "Hello, World!"
        let params = json!({
            "data": BASE64.encode(b"Hello, World!")
        });

        let result = handle_sha512(&params).unwrap();
        let hash = result.get("hash").unwrap().as_str().unwrap();

        // Known SHA-512 of "Hello, World!"
        assert_eq!(
            hash,
            "374d794a95cdcfd8b35993185fef9ba368f160d8daf432d08ba9f1ed1e5abe6cc69291e0fa2fe0006a52570ef18c19def4e617c33ce52ef0a6e5fbe318cb0387"
        );
        assert_eq!(result.get("algorithm").unwrap().as_str().unwrap(), "sha512");
        assert_eq!(result.get("output_bits").unwrap().as_u64().unwrap(), 512);
    }

    #[test]
    fn test_sha256_invalid_base64() {
        // Test with invalid base64
        let params = json!({
            "data": "not-valid-base64!!!"
        });

        let result = handle_sha256(&params);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid base64 data"));
    }

    #[test]
    fn test_sha256_missing_data() {
        // Test with missing data parameter
        let params = json!({});

        let result = handle_sha256(&params);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing 'data'"));
    }

    #[test]
    fn test_sha_family_consistency() {
        // Verify all three algorithms process the same data correctly
        let test_data = b"Consistency test across SHA family";
        let params = json!({
            "data": BASE64.encode(test_data)
        });

        let sha256_result = handle_sha256(&params).unwrap();
        let sha384_result = handle_sha384(&params).unwrap();
        let sha512_result = handle_sha512(&params).unwrap();

        // Verify output lengths
        assert_eq!(
            sha256_result.get("hash").unwrap().as_str().unwrap().len(),
            64
        ); // 32 bytes
        assert_eq!(
            sha384_result.get("hash").unwrap().as_str().unwrap().len(),
            96
        ); // 48 bytes
        assert_eq!(
            sha512_result.get("hash").unwrap().as_str().unwrap().len(),
            128
        ); // 64 bytes

        // Verify all have both hex and base64 outputs
        assert!(sha256_result.get("hash_base64").is_some());
        assert!(sha384_result.get("hash_base64").is_some());
        assert!(sha512_result.get("hash_base64").is_some());
    }
}

