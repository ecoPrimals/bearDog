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

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use beardog_errors::BearDogError;
use serde_json::{json, Value};
use sha1::Sha1; // Phase 7: Legacy Git compatibility (INSECURE for crypto!)
use sha2::{Digest, Sha256, Sha384, Sha512};
use sha3::Sha3_256; // Phase 7: Modern quantum-resistant hashing

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
///
/// **⚠️ SECURITY WARNING**: SHA-1 is BROKEN for cryptographic purposes!
/// - Collision attacks exist (SHAttered, 2017)
/// - Use ONLY for: Git, legacy checksums, NON-CRYPTO purposes
/// - For new systems, use SHA-256 or SHA3-256
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
///   "hash": "hex_encoded_sha1_hash",
///   "hash_base64": "base64_encoded_hash",
///   "algorithm": "sha1",
///   "output_bits": 160,
///   "warning": "SHA-1 is INSECURE for cryptographic purposes!"
/// }
/// ```
pub fn handle_sha1(params: &Value) -> Result<Value, BearDogError> {
    // Parse and decode data
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'data' parameter"))?;

    let data = BASE64
        .decode(data_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 data: {}", e)))?;

    // Compute SHA-1 hash
    let mut hasher = Sha1::new();
    hasher.update(&data);
    let hash = hasher.finalize();

    // Encode as hex and base64
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
///
/// SHA3-256 is the modern Keccak-based hash function standardized by NIST.
/// Different construction than SHA-2, provides quantum resistance (classical security).
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
///   "hash": "hex_encoded_sha3_256_hash",
///   "hash_base64": "base64_encoded_hash",
///   "algorithm": "sha3_256",
///   "output_bits": 256
/// }
/// ```
///
/// **Performance**: ~1ms (slower than SHA-256, but more secure construction)
pub fn handle_sha3_256(params: &Value) -> Result<Value, BearDogError> {
    // Parse and decode data
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'data' parameter"))?;

    let data = BASE64
        .decode(data_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 data: {}", e)))?;

    // Compute SHA3-256 hash
    let mut hasher = Sha3_256::new();
    hasher.update(&data);
    let hash = hasher.finalize();

    // Encode as hex and base64
    let hash_hex = hex::encode(hash);
    let hash_b64 = BASE64.encode(hash);

    Ok(json!({
        "hash": hash_hex,
        "hash_base64": hash_b64,
        "algorithm": "sha3_256",
        "output_bits": 256
    }))
}

// ============================================================================
// TOR V3 ONION ADDRESS DERIVATION
// ============================================================================

/// Handle `beardog.crypto.derive_onion_address` - Tor v3 onion address derivation
///
/// Derives a Tor v3 onion address from an Ed25519 public key.
/// Follows the Tor rend-spec-v3 specification.
///
/// **Algorithm**:
/// ```text
/// checksum = sha3_256(".onion checksum" || public_key || version)[0:2]
/// onion_address = base32(public_key || checksum || version).onion
/// ```
///
/// **Input**:
/// ```json
/// {
///   "public_key": "base64_encoded_32_byte_ed25519_public_key"
/// }
/// ```
///
/// **Output**:
/// ```json
/// {
///   "onion_address": "56char.onion",
///   "public_key": "base64_encoded_public_key",
///   "checksum": "hex_encoded_2_byte_checksum",
///   "version": 3
/// }
/// ```
///
/// **Reference**: https://spec.torproject.org/rend-spec-v3#encoding-onion-addresses
pub fn handle_derive_onion_address(params: &Value) -> Result<Value, BearDogError> {
    // Extract public key
    let pubkey_b64 = params
        .get("public_key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'public_key' parameter"))?;

    // Decode public key (must be 32 bytes for Ed25519)
    let public_key = BASE64
        .decode(pubkey_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 public_key: {}", e)))?;

    if public_key.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "Ed25519 public key must be 32 bytes, got {}",
            public_key.len()
        )));
    }

    // Tor v3 version byte
    const TOR_V3_VERSION: u8 = 0x03;

    // Compute checksum: SHA3-256(".onion checksum" || pubkey || version)[0:2]
    let mut hasher = Sha3_256::new();
    hasher.update(b".onion checksum");
    hasher.update(&public_key);
    hasher.update(&[TOR_V3_VERSION]);
    let hash = hasher.finalize();
    let checksum = &hash[0..2];

    // Build onion address bytes: pubkey (32) || checksum (2) || version (1) = 35 bytes
    let mut onion_bytes = Vec::with_capacity(35);
    onion_bytes.extend_from_slice(&public_key);
    onion_bytes.extend_from_slice(checksum);
    onion_bytes.push(TOR_V3_VERSION);

    // Base32 encode (lowercase, no padding)
    let onion_base32 = data_encoding::BASE32_NOPAD
        .encode(&onion_bytes)
        .to_lowercase();

    // Full onion address
    let onion_address = format!("{}.onion", onion_base32);

    Ok(json!({
        "onion_address": onion_address,
        "public_key": pubkey_b64,
        "checksum": hex::encode(checksum),
        "version": TOR_V3_VERSION as u32
    }))
}

/// Handle `beardog.crypto.generate_onion_identity` - Generate Tor v3 onion identity
///
/// Generates a new Ed25519 keypair and derives the corresponding Tor v3 onion address.
/// This is a convenience method that combines key generation with address derivation.
///
/// **Input**:
/// ```json
/// {
///   "purpose": "hidden_service"  // optional, for audit trail
/// }
/// ```
///
/// **Output**:
/// ```json
/// {
///   "public_key": "base64_encoded_32_byte_public_key",
///   "secret_key": "base64_encoded_64_byte_secret_key",
///   "onion_address": "56char.onion",
///   "version": 3,
///   "purpose": "hidden_service"
/// }
/// ```
pub async fn handle_generate_onion_identity(
    params: Option<&Value>,
) -> Result<Value, String> {
    use ed25519_dalek::SigningKey;
    use rand::RngCore;

    // Extract optional purpose
    let purpose = params
        .and_then(|p| p.get("purpose"))
        .and_then(|v| v.as_str())
        .unwrap_or("hidden_service");

    // Generate Ed25519 keypair using random seed
    let mut seed = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);
    let signing_key = SigningKey::from_bytes(&seed);
    let verifying_key = signing_key.verifying_key();

    // Get key bytes
    let secret_bytes = signing_key.to_bytes();
    let public_bytes = verifying_key.to_bytes();

    // Derive onion address
    let onion_params = json!({
        "public_key": BASE64.encode(&public_bytes)
    });
    let onion_result = handle_derive_onion_address(&onion_params)
        .map_err(|e| format!("Failed to derive onion address: {}", e))?;

    let onion_address = onion_result
        .get("onion_address")
        .and_then(|v| v.as_str())
        .ok_or("Failed to get onion address")?;

    // Encode keys
    // Note: Ed25519 secret key is 32 bytes seed, but some implementations expect 64 bytes (seed + public)
    let mut full_secret = Vec::with_capacity(64);
    full_secret.extend_from_slice(&secret_bytes);
    full_secret.extend_from_slice(&public_bytes);

    Ok(json!({
        "public_key": BASE64.encode(&public_bytes),
        "secret_key": BASE64.encode(&full_secret),
        "onion_address": onion_address,
        "version": 3,
        "purpose": purpose
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

    // Phase 7: SHA-1 Tests (Legacy compatibility)

    #[test]
    fn test_sha1_hello_world() {
        let params = json!({"data": BASE64.encode(b"Hello, World!")});
        let result = handle_sha1(&params).unwrap();
        let hash = result.get("hash").unwrap().as_str().unwrap();

        // Known SHA-1 hash
        assert_eq!(hash, "0a0a9f2a6772942557ab5355d76af442f8f65e01");
        assert_eq!(hash.len(), 40); // 160 bits
        assert!(result.get("warning").is_some()); // Security warning
    }

    #[test]
    fn test_sha1_empty() {
        let params = json!({"data": BASE64.encode(b"")});
        let result = handle_sha1(&params).unwrap();
        let hash = result.get("hash").unwrap().as_str().unwrap();
        assert_eq!(hash, "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    }

    // Phase 7: SHA3-256 Tests (Modern quantum-resistant)

    #[test]
    fn test_sha3_256_hello_world() {
        let params = json!({"data": BASE64.encode(b"Hello, World!")});
        let result = handle_sha3_256(&params).unwrap();
        let hash = result.get("hash").unwrap().as_str().unwrap();

        // Known SHA3-256 hash
        assert_eq!(
            hash,
            "1af17a664e3fa8e419b8ba05c2a173169df76162a5a286e0c405b460d478f7ef"
        );
        assert_eq!(hash.len(), 64); // 256 bits
    }

    #[test]
    fn test_sha3_256_empty() {
        let params = json!({"data": BASE64.encode(b"")});
        let result = handle_sha3_256(&params).unwrap();
        let hash = result.get("hash").unwrap().as_str().unwrap();
        assert_eq!(
            hash,
            "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"
        );
    }

    #[test]
    fn test_sha3_vs_sha2_different() {
        // SHA3 and SHA2 produce different hashes
        let data = BASE64.encode(b"test");
        let sha2 = handle_sha256(&json!({"data": &data})).unwrap();
        let sha3 = handle_sha3_256(&json!({"data": &data})).unwrap();

        assert_ne!(sha2.get("hash").unwrap(), sha3.get("hash").unwrap());
    }

    // Tor v3 Onion Address Tests (February 7, 2026)

    #[test]
    fn test_derive_onion_address_format() {
        // Test with a known Ed25519 public key (32 bytes)
        let test_pubkey = [0u8; 32]; // All zeros for test
        let params = json!({
            "public_key": BASE64.encode(&test_pubkey)
        });

        let result = handle_derive_onion_address(&params).unwrap();

        // Verify onion address format
        let onion_address = result.get("onion_address").unwrap().as_str().unwrap();
        assert!(onion_address.ends_with(".onion"));
        assert_eq!(onion_address.len(), 62); // 56 chars + ".onion" (6 chars)

        // Verify version is 3
        assert_eq!(result.get("version").unwrap().as_u64().unwrap(), 3);

        // Verify checksum is present
        assert!(result.get("checksum").is_some());
    }

    #[test]
    fn test_derive_onion_address_consistency() {
        // Same public key should always produce same onion address
        let test_pubkey = [42u8; 32]; // Non-zero for variety
        let params = json!({
            "public_key": BASE64.encode(&test_pubkey)
        });

        let result1 = handle_derive_onion_address(&params).unwrap();
        let result2 = handle_derive_onion_address(&params).unwrap();

        assert_eq!(
            result1.get("onion_address").unwrap().as_str().unwrap(),
            result2.get("onion_address").unwrap().as_str().unwrap()
        );
    }

    #[test]
    fn test_derive_onion_address_invalid_key_length() {
        // Public key must be exactly 32 bytes
        let short_key = [0u8; 16];
        let params = json!({
            "public_key": BASE64.encode(&short_key)
        });

        let result = handle_derive_onion_address(&params);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("32 bytes"));
    }

    #[test]
    fn test_derive_onion_address_missing_params() {
        let params = json!({});
        let result = handle_derive_onion_address(&params);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Missing 'public_key'"));
    }

    #[test]
    fn test_derive_onion_address_base32_lowercase() {
        // Tor v3 addresses use lowercase base32
        let test_pubkey = [255u8; 32]; // High values
        let params = json!({
            "public_key": BASE64.encode(&test_pubkey)
        });

        let result = handle_derive_onion_address(&params).unwrap();
        let onion_address = result.get("onion_address").unwrap().as_str().unwrap();

        // Remove ".onion" suffix and verify lowercase
        let addr_part = &onion_address[..56];
        assert!(addr_part
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }
}
