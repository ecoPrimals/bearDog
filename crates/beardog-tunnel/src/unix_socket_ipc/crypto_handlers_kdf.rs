// SPDX-License-Identifier: AGPL-3.0-only

//! Additional Key Derivation Function (KDF) handlers for BearDog RPC
//!
//! This module implements legacy and memory-hard KDF/password hashing algorithms:
//! - bcrypt: Legacy password hashing (very common in web apps)
//! - scrypt: Memory-hard KDF (Litecoin, legacy systems)
//!
//! All implementations are Pure Rust from RustCrypto.
//!
//! # Phase 7: High-Priority Compatibility
//! - bcrypt: Most requested for legacy auth system integration
//! - scrypt: Cryptocurrency and legacy system compatibility
//!
//! # Security Notes
//! - bcrypt: Secure, but not OWASP 2023 recommended (use Argon2id instead)
//! - scrypt: Memory-hard, good, but Argon2id is preferred
//! - Both use constant-time comparison for verify operations
//! - Random salts are generated using OsRng (CSPRNG)
//!
//! # Performance
//! - bcrypt: ~100-200ms (cost 12, intentionally slow)
//! - scrypt: ~50-100ms (N=16384, r=8, p=1)
//! - Both are CPU/memory intensive by design (brute-force protection)

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use beardog_errors::BearDogError;
use serde_json::{json, Value};
use zeroize::Zeroizing;

/// Handle bcrypt password hashing
///
/// # Input Parameters
/// - `password`: Base64-encoded password bytes
/// - `cost`: Optional work factor (4-31, default: 12)
///
/// # Returns
/// - `hash`: PHC string format (bcrypt hash with embedded salt and cost)
///
/// # Example
/// ```json
/// {
///     "password": "aGVsbG8xMjM=",  // "hello123" in base64
///     "cost": 12
/// }
/// ```
///
/// # Security
/// - Generates random salt using OsRng
/// - Returns PHC format: $2b$12$salt_and_hash
/// - Default cost 12 (~100-200ms on modern CPU)
/// - NOT OWASP 2023 recommended (use Argon2id)
pub fn handle_bcrypt_hash(params: &Value) -> Result<Value, BearDogError> {
    // Parse password
    let password_b64 = params
        .get("password")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            BearDogError::business("Missing or invalid 'password' parameter".to_string())
        })?;

    let password_bytes = BASE64
        .decode(password_b64)
        .map_err(|e| BearDogError::business(format!("Invalid base64 password: {}", e)))?;

    // Use Zeroizing to protect password in memory
    let password = Zeroizing::new(
        String::from_utf8(password_bytes)
            .map_err(|e| BearDogError::business(format!("Invalid UTF-8 password: {}", e)))?,
    );

    // Parse cost (default: 12, range: 4-31)
    let cost = params.get("cost").and_then(|v| v.as_u64()).unwrap_or(12) as u32;

    if !(4..=31).contains(&cost) {
        return Err(BearDogError::business(
            "bcrypt cost must be between 4 and 31".to_string(),
        ));
    }

    // Hash the password
    let hash = bcrypt::hash(&password, cost)
        .map_err(|e| BearDogError::security(format!("bcrypt hash failed: {}", e)))?;

    Ok(json!({
        "hash": hash,
        "algorithm": "bcrypt",
        "cost": cost
    }))
}

/// Handle bcrypt password verification
///
/// # Input Parameters
/// - `password`: Base64-encoded password bytes to verify
/// - `hash`: bcrypt hash string (PHC format)
///
/// # Returns
/// - `valid`: Boolean indicating if password matches hash
///
/// # Example
/// ```json
/// {
///     "password": "aGVsbG8xMjM=",
///     "hash": "$2b$12$salt_and_hash"
/// }
/// ```
///
/// # Security
/// - Uses constant-time comparison
/// - Protects against timing attacks
pub fn handle_bcrypt_verify(params: &Value) -> Result<Value, BearDogError> {
    // Parse password
    let password_b64 = params
        .get("password")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            BearDogError::business("Missing or invalid 'password' parameter".to_string())
        })?;

    let password_bytes = BASE64
        .decode(password_b64)
        .map_err(|e| BearDogError::business(format!("Invalid base64 password: {}", e)))?;

    let password = Zeroizing::new(
        String::from_utf8(password_bytes)
            .map_err(|e| BearDogError::business(format!("Invalid UTF-8 password: {}", e)))?,
    );

    // Parse hash
    let hash = params
        .get("hash")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::business("Missing or invalid 'hash' parameter".to_string()))?;

    // Verify password (constant-time)
    let valid = bcrypt::verify(&password, hash)
        .map_err(|e| BearDogError::security(format!("bcrypt verify failed: {}", e)))?;

    Ok(json!({
        "valid": valid
    }))
}

/// Handle scrypt key derivation
///
/// # Input Parameters
/// - `password`: Base64-encoded password bytes
/// - `salt`: Base64-encoded salt (min 8 bytes recommended)
/// - `log_n`: Optional CPU/memory cost parameter (default: 14, i.e., N=16384)
/// - `r`: Optional block size (default: 8)
/// - `p`: Optional parallelization (default: 1)
/// - `key_length`: Optional output length in bytes (default: 32)
///
/// # Returns
/// - `derived_key`: Base64-encoded derived key
/// - `params`: Scrypt parameters used
///
/// # Example
/// ```json
/// {
///     "password": "aGVsbG8xMjM=",
///     "salt": "cmFuZG9tc2FsdA==",
///     "log_n": 14,
///     "r": 8,
///     "p": 1,
///     "key_length": 32
/// }
/// ```
///
/// # Security
/// - Memory-hard algorithm (resists GPU/ASIC attacks)
/// - Default params: N=16384, r=8, p=1 (~50-100ms)
/// - Use random salt (min 8 bytes, 16+ recommended)
/// - OWASP recommends Argon2id over scrypt
pub fn handle_scrypt(params: &Value) -> Result<Value, BearDogError> {
    use scrypt::{scrypt, Params};

    // Parse password
    let password_b64 = params
        .get("password")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            BearDogError::business("Missing or invalid 'password' parameter".to_string())
        })?;

    let password_bytes = Zeroizing::new(
        BASE64
            .decode(password_b64)
            .map_err(|e| BearDogError::business(format!("Invalid base64 password: {}", e)))?,
    );

    // Parse salt
    let salt_b64 = params
        .get("salt")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::business("Missing or invalid 'salt' parameter".to_string()))?;

    let salt = BASE64
        .decode(salt_b64)
        .map_err(|e| BearDogError::business(format!("Invalid base64 salt: {}", e)))?;

    if salt.len() < 8 {
        return Err(BearDogError::business(
            "Salt must be at least 8 bytes".to_string(),
        ));
    }

    // Parse scrypt parameters
    let log_n = params.get("log_n").and_then(|v| v.as_u64()).unwrap_or(14) as u8;
    let r = params.get("r").and_then(|v| v.as_u64()).unwrap_or(8) as u32;
    let p = params.get("p").and_then(|v| v.as_u64()).unwrap_or(1) as u32;
    let key_length = params
        .get("key_length")
        .and_then(|v| v.as_u64())
        .unwrap_or(32) as usize;

    // Validate parameters
    if !(1..=20).contains(&log_n) {
        return Err(BearDogError::business(
            "log_n must be between 1 and 20".to_string(),
        ));
    }

    if key_length == 0 || key_length > 128 {
        return Err(BearDogError::business(
            "key_length must be between 1 and 128".to_string(),
        ));
    }

    // Create scrypt params
    let scrypt_params = Params::new(log_n, r, p)
        .map_err(|e| BearDogError::security(format!("Invalid scrypt params: {}", e)))?;

    // Derive key
    let mut derived_key = Zeroizing::new(vec![0u8; key_length]);
    scrypt(&password_bytes, &salt, &scrypt_params, &mut derived_key)
        .map_err(|e| BearDogError::security(format!("scrypt derivation failed: {}", e)))?;

    Ok(json!({
        "derived_key": BASE64.encode(&*derived_key),
        "algorithm": "scrypt",
        "params": {
            "log_n": log_n,
            "n": 1u64 << log_n,
            "r": r,
            "p": p,
            "key_length": key_length
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_bcrypt_hash_and_verify() {
        let password = BASE64.encode(b"test_password_123");
        let params = json!({
            "password": password,
            "cost": 10  // Lower cost for faster tests
        });

        // Hash password
        let result = handle_bcrypt_hash(&params).unwrap();
        let hash = result.get("hash").unwrap().as_str().unwrap();
        assert!(hash.starts_with("$2b$10$"));

        // Verify correct password
        let verify_params = json!({
            "password": password,
            "hash": hash
        });
        let verify_result = handle_bcrypt_verify(&verify_params).unwrap();
        assert!(verify_result.get("valid").unwrap().as_bool().unwrap());

        // Verify wrong password
        let wrong_password = BASE64.encode(b"wrong_password");
        let wrong_verify_params = json!({
            "password": wrong_password,
            "hash": hash
        });
        let wrong_result = handle_bcrypt_verify(&wrong_verify_params).unwrap();
        assert!(!wrong_result.get("valid").unwrap().as_bool().unwrap());
    }

    #[test]
    fn test_bcrypt_different_costs() {
        let password = BASE64.encode(b"password");

        // Cost 4 (fast)
        let params_4 = json!({"password": password, "cost": 4});
        let result_4 = handle_bcrypt_hash(&params_4).unwrap();
        assert!(result_4
            .get("hash")
            .unwrap()
            .as_str()
            .unwrap()
            .starts_with("$2b$04$"));

        // Cost 12 (default)
        let params_12 = json!({"password": password});
        let result_12 = handle_bcrypt_hash(&params_12).unwrap();
        assert!(result_12
            .get("hash")
            .unwrap()
            .as_str()
            .unwrap()
            .starts_with("$2b$12$"));
    }

    #[test]
    fn test_bcrypt_invalid_cost() {
        let password = BASE64.encode(b"password");

        // Cost too low
        let params_low = json!({"password": password, "cost": 3});
        assert!(handle_bcrypt_hash(&params_low).is_err());

        // Cost too high
        let params_high = json!({"password": password, "cost": 32});
        assert!(handle_bcrypt_hash(&params_high).is_err());
    }

    #[test]
    fn test_scrypt_derivation() {
        let password = BASE64.encode(b"my_password");
        let salt = BASE64.encode(b"random_salt_12345");

        let params = json!({
            "password": password,
            "salt": salt,
            "log_n": 10,  // Low for fast tests (N=1024)
            "r": 8,
            "p": 1,
            "key_length": 32
        });

        let result = handle_scrypt(&params).unwrap();
        let derived_key = result.get("derived_key").unwrap().as_str().unwrap();
        let decoded = BASE64.decode(derived_key).unwrap();
        assert_eq!(decoded.len(), 32);
    }

    #[test]
    fn test_scrypt_deterministic() {
        let password = BASE64.encode(b"password123");
        let salt = BASE64.encode(b"fixed_salt_value");

        let params = json!({
            "password": password,
            "salt": salt,
            "log_n": 10,
            "r": 8,
            "p": 1,
            "key_length": 32
        });

        // Derive twice with same params
        let result1 = handle_scrypt(&params).unwrap();
        let result2 = handle_scrypt(&params).unwrap();

        // Should produce identical keys
        assert_eq!(
            result1.get("derived_key").unwrap(),
            result2.get("derived_key").unwrap()
        );
    }

    #[test]
    fn test_scrypt_different_salts() {
        let password = BASE64.encode(b"password");

        let params1 = json!({
            "password": password,
            "salt": BASE64.encode(b"salt1234"),  // 8 bytes minimum for scrypt
            "log_n": 10
        });

        let params2 = json!({
            "password": password,
            "salt": BASE64.encode(b"salt5678"),  // 8 bytes minimum for scrypt
            "log_n": 10
        });

        let result1 = handle_scrypt(&params1).unwrap();
        let result2 = handle_scrypt(&params2).unwrap();

        // Different salts should produce different keys
        assert_ne!(
            result1.get("derived_key").unwrap(),
            result2.get("derived_key").unwrap()
        );
    }

    #[test]
    fn test_scrypt_variable_key_length() {
        let password = BASE64.encode(b"password");
        let salt = BASE64.encode(b"salt12345678");

        for key_length in [16, 32, 64, 128] {
            let params = json!({
                "password": password,
                "salt": salt,
                "log_n": 10,
                "key_length": key_length
            });

            let result = handle_scrypt(&params).unwrap();
            let derived_key = result.get("derived_key").unwrap().as_str().unwrap();
            let decoded = BASE64.decode(derived_key).unwrap();
            assert_eq!(decoded.len(), key_length);
        }
    }

    #[test]
    fn test_scrypt_invalid_salt() {
        let password = BASE64.encode(b"password");
        let short_salt = BASE64.encode(b"short"); // Only 5 bytes

        let params = json!({
            "password": password,
            "salt": short_salt,
            "log_n": 10
        });

        assert!(handle_scrypt(&params).is_err());
    }
}
