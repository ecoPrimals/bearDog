// SPDX-License-Identifier: AGPL-3.0-only

//! Password Hashing Handlers (Phase 6 - Final Critical Piece!)
//!
//! Provides secure password hashing for authentication systems.
//! Essential for any system that stores user passwords.
//!
//! **Modern**: Argon2id (OWASP recommended since 2023)
//! **Legacy**: PBKDF2 (still common in iOS/macOS/legacy systems)
//!
//! Pure Rust implementations using `RustCrypto` crates (zero C dependencies).
//!
//! # Password Hashing Best Practices
//!
//! - **ALWAYS use Argon2id for new systems** (memory-hard, GPU/ASIC resistant)
//! - **NEVER store passwords in plaintext!**
//! - **NEVER use fast hashes (MD5/SHA) for passwords!**
//! - Use high iteration counts for PBKDF2 (minimum 100,000+)
//! - Use random salts (automatically handled by Argon2id)
//!
//! # Argon2id Overview
//!
//! Argon2id is the winner of the Password Hashing Competition (2015).
//! It combines:
//! - **Memory-hardness**: Expensive RAM requirements resist GPUs/ASICs
//! - **Time cost**: Configurable iteration count
//! - **Parallelism**: Multi-thread support
//! - **Side-channel resistance**: Constant-time operations
//!
//! # PBKDF2 Overview
//!
//! PBKDF2 (Password-Based Key Derivation Function 2) is a legacy standard.
//! Still used by:
//! - iOS/macOS password storage
//! - WPA2/WPA3 `WiFi` encryption
//! - Legacy enterprise systems
//!
//! **Not recommended for new systems** - use Argon2id instead!

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_errors::BearDogError;
use pbkdf2::pbkdf2_hmac;
use rand::rngs::OsRng;
use serde_json::{Value, json};
use sha2::Sha256;

/// # Errors
///
/// Returns an error if hashing fails.
/// Handle `crypto.argon2id_hash` - Argon2id password hashing
///
/// Hashes a password using Argon2id (OWASP recommended).
/// Automatically generates a random salt and outputs the PHC string format.
///
/// **Input**:
/// ```json
/// {
///   "password": "plaintext_password_string"
/// }
/// ```
///
/// **Output**:
/// ```json
/// {
///   "hash": "$argon2id$v=19$m=19456,t=2,p=1$...",
///   "algorithm": "argon2id",
///   "version": 19,
///   "params": {
///     "memory_cost_kb": 19456,
///     "time_cost": 2,
///     "parallelism": 1
///   }
/// }
/// ```
///
/// **Security**: Hash is safe to store in database (includes salt and params)
/// **Performance**: ~50-100ms (intentionally slow to resist brute-force)
pub fn handle_argon2id_hash(params: &Value) -> Result<Value, BearDogError> {
    // Extract password
    let password = params
        .get("password")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'password' parameter"))?;

    if password.is_empty() {
        return Err(BearDogError::invalid_input("Password cannot be empty"));
    }

    // Generate random salt
    let salt = SaltString::generate(&mut OsRng);

    // Create Argon2 instance with default parameters (secure defaults)
    // Memory cost: 19456 KiB (~19 MB)
    // Time cost: 2 iterations
    // Parallelism: 1 thread
    let argon2 = Argon2::default();

    // Hash password
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| BearDogError::system(format!("Argon2id hashing failed: {e}")))?;

    // Get PHC string format (includes algorithm, version, params, salt, and hash)
    let hash_string = password_hash.to_string();

    Ok(json!({
        "hash": hash_string,
        "algorithm": "argon2id",
        "version": 19,
        "params": {
            "memory_cost_kb": 19456,
            "time_cost": 2,
            "parallelism": 1
        }
    }))
}

/// # Errors
///
/// Returns an error if verification fails in the underlying HSM provider.
/// Handle `crypto.argon2id_verify` - Argon2id password verification
///
/// Verifies a password against an Argon2id hash.
/// Safe against timing attacks (constant-time comparison).
///
/// **Input**:
/// ```json
/// {
///   "password": "plaintext_password_string",
///   "hash": "$argon2id$v=19$m=19456,t=2,p=1$..."
/// }
/// ```
///
/// **Output**:
/// ```json
/// {
///   "valid": true,
///   "algorithm": "argon2id"
/// }
/// ```
///
/// **Security**: Always returns in constant time to prevent timing attacks
/// **Performance**: ~50-100ms (same as hashing)
pub fn handle_argon2id_verify(params: &Value) -> Result<Value, BearDogError> {
    // Extract password
    let password = params
        .get("password")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'password' parameter"))?;

    // Extract hash
    let hash_string = params
        .get("hash")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'hash' parameter"))?;

    // Parse the PHC string
    let parsed_hash = PasswordHash::new(hash_string)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid Argon2 hash format: {e}")))?;

    // Verify password (constant-time comparison)
    let argon2 = Argon2::default();
    let is_valid = argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(json!({
        "valid": is_valid,
        "algorithm": "argon2id"
    }))
}

/// # Errors
///
/// Returns an error if hashing fails.
/// Handle `crypto.pbkdf2_sha256` - PBKDF2-HMAC-SHA256 key derivation
///
/// Derives a key from a password using PBKDF2 with SHA-256.
/// Legacy algorithm, still common in iOS/macOS/WiFi.
///
/// **Input**:
/// ```json
/// {
///   "password": "plaintext_password_string",
///   "salt": "base64_encoded_salt",
///   "iterations": 100000,
///   "output_length": 32
/// }
/// ```
///
/// **Output**:
/// ```json
/// {
///   "derived_key": "base64_encoded_derived_key",
///   "algorithm": "pbkdf2-hmac-sha256",
///   "iterations": 100000,
///   "output_bytes": 32
/// }
/// ```
///
/// **Security**: Use minimum 100,000 iterations (OWASP 2023 recommendation)
/// **Performance**: ~50ms for 100k iterations (intentionally slow)
/// **Recommendation**: Use Argon2id for new systems!
pub fn handle_pbkdf2_sha256(params: &Value) -> Result<Value, BearDogError> {
    // Extract password
    let password = params
        .get("password")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'password' parameter"))?;

    // Extract salt
    let salt_b64 = params
        .get("salt")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'salt' parameter"))?;

    let salt = BASE64
        .decode(salt_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 salt: {e}")))?;

    // Extract iterations (default to 100,000 minimum)
    #[expect(
        clippy::cast_possible_truncation,
        reason = "PBKDF2 iterations capped by API validation"
    )]
    let iterations = params
        .get("iterations")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(100_000) as u32;

    if iterations < 100_000 {
        return Err(BearDogError::invalid_input(
            "PBKDF2 iterations must be at least 100,000 (OWASP 2023 recommendation)",
        ));
    }

    // Extract output length (default to 32 bytes)
    #[expect(
        clippy::cast_possible_truncation,
        reason = "PBKDF2 output length validated to 1..=1024"
    )]
    let output_length = params
        .get("output_length")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(32) as usize;

    if output_length == 0 || output_length > 1024 {
        return Err(BearDogError::invalid_input(
            "output_length must be between 1 and 1024 bytes",
        ));
    }

    // Derive key using PBKDF2-HMAC-SHA256
    let mut derived_key = vec![0u8; output_length];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, iterations, &mut derived_key);

    // Encode output
    let derived_key_b64 = BASE64.encode(&derived_key);

    Ok(json!({
        "derived_key": derived_key_b64,
        "algorithm": "pbkdf2-hmac-sha256",
        "iterations": iterations,
        "output_bytes": output_length
    }))
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn json_str<'a>(v: &'a Value, key: &'static str) -> Result<&'a str, BearDogError> {
        v.get(key)
            .and_then(|x| x.as_str())
            .ok_or_else(|| BearDogError::invalid_input("missing JSON string field"))
    }

    fn json_bool(v: &Value, key: &'static str) -> Result<bool, BearDogError> {
        v.get(key)
            .and_then(|x| x.as_bool())
            .ok_or_else(|| BearDogError::invalid_input("missing JSON bool field"))
    }

    fn json_u64(v: &Value, key: &'static str) -> Result<u64, BearDogError> {
        v.get(key)
            .and_then(|x| x.as_u64())
            .ok_or_else(|| BearDogError::invalid_input("missing JSON u64 field"))
    }

    fn b64_decode(s: &str) -> Result<Vec<u8>, BearDogError> {
        BASE64
            .decode(s)
            .map_err(|e| BearDogError::invalid_input(&format!("base64: {e}")))
    }

    #[test]
    fn test_argon2id_hash_and_verify() -> Result<(), BearDogError> {
        let password = "SuperSecretPassword123!";
        let hash_params = json!({ "password": password });
        let hash_result = handle_argon2id_hash(&hash_params)?;
        let hash = json_str(&hash_result, "hash")?;
        assert!(hash.starts_with("$argon2id$"));
        assert_eq!(json_str(&hash_result, "algorithm")?, "argon2id");
        let verify_params = json!({ "password": password, "hash": hash });
        let verify_result = handle_argon2id_verify(&verify_params)?;
        assert!(json_bool(&verify_result, "valid")?);
        Ok(())
    }

    #[test]
    fn test_argon2id_verify_wrong_password() -> Result<(), BearDogError> {
        let correct_password = "Correct123!";
        let wrong_password = "Wrong456!";
        let hash_params = json!({ "password": correct_password });
        let hash_result = handle_argon2id_hash(&hash_params)?;
        let hash = json_str(&hash_result, "hash")?;
        let verify_params = json!({
            "password": wrong_password,
            "hash": hash
        });
        let verify_result = handle_argon2id_verify(&verify_params)?;
        assert!(!json_bool(&verify_result, "valid")?);
        Ok(())
    }

    #[test]
    fn test_argon2id_different_hashes_for_same_password() -> Result<(), BearDogError> {
        let password = "TestPassword";
        let hash_params = json!({ "password": password });
        let hash1 = handle_argon2id_hash(&hash_params)?;
        let hash2 = handle_argon2id_hash(&hash_params)?;
        let hash1_str = json_str(&hash1, "hash")?;
        let hash2_str = json_str(&hash2, "hash")?;
        assert_ne!(hash1_str, hash2_str);
        let verify1 = handle_argon2id_verify(&json!({
            "password": password,
            "hash": hash1_str
        }))?;
        let verify2 = handle_argon2id_verify(&json!({
            "password": password,
            "hash": hash2_str
        }))?;
        assert!(json_bool(&verify1, "valid")?);
        assert!(json_bool(&verify2, "valid")?);
        Ok(())
    }

    #[test]
    fn test_argon2id_empty_password() {
        let params = json!({ "password": "" });
        let result = handle_argon2id_hash(&params);
        assert!(result.is_err());
        let msg = result.err().map(|e| e.to_string()).unwrap_or_default();
        assert!(msg.contains("cannot be empty"));
    }

    #[test]
    fn test_pbkdf2_sha256_basic() -> Result<(), BearDogError> {
        let password = "password";
        let salt = b"salt";
        let iterations = 100_000u64;
        let params = json!({
            "password": password,
            "salt": BASE64.encode(salt),
            "iterations": iterations,
            "output_length": 32
        });
        let result = handle_pbkdf2_sha256(&params)?;
        let derived_key = b64_decode(json_str(&result, "derived_key")?)?;
        assert_eq!(derived_key.len(), 32);
        assert_eq!(json_str(&result, "algorithm")?, "pbkdf2-hmac-sha256");
        assert_eq!(json_u64(&result, "iterations")?, iterations);
        Ok(())
    }

    #[test]
    fn test_pbkdf2_sha256_deterministic() -> Result<(), BearDogError> {
        let password = "TestPass";
        let salt = b"TestSalt";
        let iterations = 100_000;
        let params = json!({
            "password": password,
            "salt": BASE64.encode(salt),
            "iterations": iterations,
            "output_length": 32
        });
        let result1 = handle_pbkdf2_sha256(&params)?;
        let result2 = handle_pbkdf2_sha256(&params)?;
        assert_eq!(
            json_str(&result1, "derived_key")?,
            json_str(&result2, "derived_key")?
        );
        Ok(())
    }

    #[test]
    fn test_pbkdf2_sha256_different_salts() -> Result<(), BearDogError> {
        let password = "SamePassword";
        let iterations = 100_000;
        let params1 = json!({
            "password": password,
            "salt": BASE64.encode(b"salt1"),
            "iterations": iterations,
            "output_length": 32
        });
        let params2 = json!({
            "password": password,
            "salt": BASE64.encode(b"salt2"),
            "iterations": iterations,
            "output_length": 32
        });
        let result1 = handle_pbkdf2_sha256(&params1)?;
        let result2 = handle_pbkdf2_sha256(&params2)?;
        assert_ne!(
            json_str(&result1, "derived_key")?,
            json_str(&result2, "derived_key")?
        );
        Ok(())
    }

    #[test]
    fn test_pbkdf2_sha256_low_iterations_rejected() {
        let params = json!({
            "password": "password",
            "salt": BASE64.encode(b"salt"),
            "iterations": 10_000
        });
        let result = handle_pbkdf2_sha256(&params);
        assert!(result.is_err());
        let msg = result.err().map(|e| e.to_string()).unwrap_or_default();
        assert!(msg.contains("100,000"));
    }

    #[test]
    fn test_pbkdf2_sha256_custom_output_length() -> Result<(), BearDogError> {
        for output_length in [16u64, 32, 48, 64] {
            let params = json!({
                "password": "test",
                "salt": BASE64.encode(b"salt"),
                "iterations": 100_000,
                "output_length": output_length
            });
            let result = handle_pbkdf2_sha256(&params)?;
            let derived_key = b64_decode(json_str(&result, "derived_key")?)?;
            assert_eq!(derived_key.len(), output_length as usize);
        }
        Ok(())
    }
}
