// SPDX-License-Identifier: AGPL-3.0-only

//! Key-derivation helpers for CLI key generation (PBKDF2, Argon2, HKDF).

use beardog_errors::BearDogError;

/// KDF configuration
#[derive(Debug, Clone)]
pub struct KdfConfig {
    /// KDF name: `pbkdf2`, `argon2`, or `hkdf`
    pub kdf_type: String,
    /// PBKDF2 iteration count
    pub iterations: Option<u32>,
    /// Argon2 memory cost (KiB)
    pub memory: Option<u32>,
    /// Argon2 time cost
    pub time: Option<u32>,
}

impl KdfConfig {
    /// Build configuration for [`KdfConfig::derive_key`].
    pub const fn new(
        kdf_type: String,
        iterations: Option<u32>,
        memory: Option<u32>,
        time: Option<u32>,
    ) -> Self {
        Self {
            kdf_type,
            iterations,
            memory,
            time,
        }
    }

    /// Derive key material from password/seed using configured KDF
    pub fn derive_key(
        &self,
        password: &[u8],
        salt: &[u8],
        key_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        match self.kdf_type.to_lowercase().as_str() {
            "pbkdf2" => {
                let iterations = self.iterations.unwrap_or(100_000);
                println!("   Using PBKDF2-HMAC-SHA256 ({iterations} iterations)");
                self.derive_key_pbkdf2(password, salt, iterations, key_length)
            }
            "argon2" => {
                let memory = self.memory.unwrap_or(65536); // 64 MB
                let time = self.time.unwrap_or(3);
                println!("   Using Argon2id (memory: {memory}KB, time: {time})");
                self.derive_key_argon2(password, salt, memory, time, key_length)
            }
            "hkdf" => {
                println!("   Using HKDF-SHA256");
                self.derive_key_hkdf(password, salt, key_length)
            }
            _ => Err(BearDogError::validation(&format!(
                "Unknown KDF type '{}'. Supported: pbkdf2, argon2, hkdf",
                self.kdf_type
            ))),
        }
    }

    /// Derive key using Argon2
    fn derive_key_argon2(
        &self,
        password: &[u8],
        salt: &[u8],
        memory_kib: u32,
        time_cost: u32,
        key_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        use argon2::{
            Argon2, Params,
            password_hash::{PasswordHasher, SaltString},
        };

        // Create params
        let params = Params::new(memory_kib, time_cost, 1, Some(key_length))
            .map_err(|e| BearDogError::crypto_error(format!("Argon2 params failed: {e}")))?;

        // Create Argon2 instance
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

        // Convert salt to SaltString
        let salt_string = SaltString::encode_b64(salt)
            .map_err(|e| BearDogError::crypto_error(format!("Salt encoding failed: {e}")))?;

        // Hash password
        let hash = argon2
            .hash_password(password, &salt_string)
            .map_err(|e| BearDogError::crypto_error(format!("Argon2 hashing failed: {e}")))?;

        // Extract hash bytes
        let hash_bytes = hash
            .hash
            .ok_or_else(|| BearDogError::crypto_error("Argon2 produced no hash"))?;

        Ok(hash_bytes.as_bytes()[..key_length].to_vec())
    }

    /// Derive key using PBKDF2
    fn derive_key_pbkdf2(
        &self,
        password: &[u8],
        salt: &[u8],
        iterations: u32,
        key_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        use pbkdf2::pbkdf2_hmac;
        use sha2::Sha256;

        if iterations < 1000 {
            return Err(BearDogError::validation(
                "PBKDF2 iterations must be at least 1000",
            ));
        }

        let mut key = vec![0u8; key_length];
        pbkdf2_hmac::<Sha256>(password, salt, iterations, &mut key);
        Ok(key)
    }

    /// Derive key using HKDF
    fn derive_key_hkdf(
        &self,
        password: &[u8],
        salt: &[u8],
        key_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        use hkdf::Hkdf;
        use sha2::Sha256;

        let hk = Hkdf::<Sha256>::new(Some(salt), password);
        let mut okm = vec![0u8; key_length];
        hk.expand(b"beardog_key_generation", &mut okm)
            .map_err(|e| BearDogError::crypto_error(format!("HKDF expansion failed: {e}")))?;

        Ok(okm)
    }

    /// Get KDF metadata for storage
    #[allow(
        dead_code,
        reason = "Public metadata helper for future persisted key records"
    )]
    pub fn to_metadata(&self) -> serde_json::Value {
        serde_json::json!({
            "kdf_type": self.kdf_type,
            "iterations": self.iterations,
            "memory_kib": self.memory,
            "time_cost": self.time,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pbkdf2_derivation() {
        let config = KdfConfig::new("pbkdf2".to_string(), Some(1000), None, None);
        let password = b"test_password";
        let salt = b"test_salt_16byte";

        let key = config.derive_key(password, salt, 32).unwrap();
        assert_eq!(key.len(), 32);

        // Should be deterministic
        let key2 = config.derive_key(password, salt, 32).unwrap();
        assert_eq!(key, key2);
    }

    #[test]
    fn test_hkdf_derivation() {
        let config = KdfConfig::new("hkdf".to_string(), None, None, None);
        let password = b"test_password";
        let salt = b"test_salt";

        let key = config.derive_key(password, salt, 32).unwrap();
        assert_eq!(key.len(), 32);

        // Should be deterministic
        let key2 = config.derive_key(password, salt, 32).unwrap();
        assert_eq!(key, key2);
    }

    #[test]
    fn test_different_passwords_produce_different_keys() {
        let config = KdfConfig::new("pbkdf2".to_string(), Some(1000), None, None);
        let salt = b"same_salt";

        let key1 = config.derive_key(b"password1", salt, 32).unwrap();
        let key2 = config.derive_key(b"password2", salt, 32).unwrap();

        assert_ne!(key1, key2);
    }
}
