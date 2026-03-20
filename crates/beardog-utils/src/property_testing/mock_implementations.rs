// SPDX-License-Identifier: AGPL-3.0-only

//! Mock implementations for property-based testing
//!
//! This module provides test-only mock implementations of cryptographic
//! and security operations for use in property-based testing scenarios.
//!
//! # Warning
//!
//! **These implementations are NOT cryptographically secure and should ONLY be used for testing!**
//!
//! All "cryptographic" operations here are simple deterministic transformations
//! designed to enable fast, reproducible property-based tests without requiring
//! real cryptographic libraries or operations.

use super::{BearDogError, Debug, HashMap, PropertyBasedTestFramework};

/// Mock cryptographic key pair for testing purposes only
///
/// **NOT CRYPTOGRAPHICALLY SECURE** - Use only in tests!
#[derive(Debug, Clone)]
pub struct MockKeyPair {
    /// Mock private key bytes (not real cryptography)
    pub private_key: Vec<u8>,
    /// Mock public key bytes (not real cryptography)
    pub public_key: Vec<u8>,
}

impl PropertyBasedTestFramework {
    /// Generate a mock hash for testing purposes
    ///
    /// **NOT CRYPTOGRAPHICALLY SECURE** - Uses simple hashing for deterministic tests.
    ///
    /// # Errors
    /// Returns error if hashing operation fails (unlikely in this mock implementation)
    pub fn mock_hash(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        let hash_value = hasher.finish();

        // Convert to 32-byte hash
        let mut hash_bytes = vec![0u8; 32];
        hash_bytes[..8].copy_from_slice(&hash_value.to_be_bytes());

        // Add some pseudo-randomness based on data
        for (i, &byte) in data.iter().take(24).enumerate() {
            hash_bytes[i + 8] = byte.wrapping_add(i as u8);
        }

        Ok(hash_bytes)
    }

    /// Mock encryption using simple XOR pattern (testing only)
    ///
    /// **NOT CRYPTOGRAPHICALLY SECURE** - Uses XOR with position for deterministic testing.
    ///
    /// # Errors
    /// Returns error if encryption operation fails
    pub fn mock_encrypt(&self, _key: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        // Simple XOR "encryption" for testing
        let mut encrypted = data.to_vec();
        for (i, byte) in encrypted.iter_mut().enumerate() {
            *byte ^= (i % 256) as u8; // Simple pattern for testing
        }
        Ok(encrypted)
    }

    /// Mock decryption reversing the XOR pattern (testing only)
    ///
    /// **NOT CRYPTOGRAPHICALLY SECURE** - Reverses XOR encryption for deterministic testing.
    ///
    /// # Errors
    /// Returns error if decryption operation fails
    pub fn mock_decrypt(&self, _key: &[u8], encrypted: &[u8]) -> Result<Vec<u8>, BearDogError> {
        // Reverse the XOR "encryption"
        let mut decrypted = encrypted.to_vec();
        for (i, byte) in decrypted.iter_mut().enumerate() {
            *byte ^= (i % 256) as u8;
        }
        Ok(decrypted)
    }

    /// Generate a mock cryptographic key pair for testing
    ///
    /// **NOT CRYPTOGRAPHICALLY SECURE** - Uses simple random bytes for deterministic testing.
    ///
    /// # Errors
    /// Returns error if key generation fails
    pub fn mock_generate_keypair(&self) -> Result<MockKeyPair, BearDogError> {
        Ok(MockKeyPair {
            private_key: self.generate_random_bytes(32)?,
            public_key: self.generate_random_bytes(32)?,
        })
    }

    /// Generate a mock digital signature for testing
    ///
    /// **NOT CRYPTOGRAPHICALLY SECURE** - Uses simple hash truncation for deterministic testing.
    ///
    /// # Errors
    /// Returns error if signature generation fails
    pub fn mock_sign(&self, _private_key: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        // Simple mock signature
        let hash = self.mock_hash(data)?;
        Ok(hash[..16].to_vec()) // 16-byte "signature"
    }

    /// Verify a mock digital signature for testing
    ///
    /// **NOT CRYPTOGRAPHICALLY SECURE** - Uses simple comparison for deterministic testing.
    ///
    /// # Errors
    /// Returns error if verification operation fails
    pub fn mock_verify(
        &self,
        _public_key: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        let expected_signature = self.mock_sign(&[], data)?;
        Ok(expected_signature == signature)
    }

    /// Derive a mock key from input material for testing
    ///
    /// **NOT CRYPTOGRAPHICALLY SECURE** - Uses simple hash extension for deterministic testing.
    ///
    /// # Errors
    /// Returns error if key derivation fails
    pub fn mock_derive_key(
        &self,
        input_key: &[u8],
        salt: &[u8],
        output_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        // Simple key derivation using hash
        let mut combined = Vec::new();
        combined.extend_from_slice(input_key);
        combined.extend_from_slice(salt);

        let base_hash = self.mock_hash(&combined)?;
        let mut derived_key = Vec::with_capacity(output_length);

        // Extend the hash to the required length
        for i in 0..output_length {
            derived_key.push(base_hash[i % base_hash.len()]);
        }

        Ok(derived_key)
    }

    /// Sanitize input data by removing control characters (for testing)
    ///
    /// Removes null bytes and control characters to simulate input validation.
    ///
    /// # Errors
    /// Returns error if sanitization fails
    pub fn mock_sanitize_input(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        // Remove null bytes and control characters
        let sanitized: Vec<u8> = data
            .iter()
            .filter(|&&byte| byte != 0 && (32..=126).contains(&byte))
            .copied()
            .collect();
        Ok(sanitized)
    }

    /// Simulates business validation: empty input yields a [`BearDogError::Business`] error.
    pub fn mock_process_with_errors(&self, _data: &[u8]) -> Result<String, BearDogError> {
        // Simulate processing that might fail
        if _data.is_empty() {
            return Err(BearDogError::Business {
                message: "Empty input data".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }
        Ok("Processed successfully".to_string())
    }

    /// Minimal `key = "value"` line parser for property tests (not a full TOML implementation).
    pub fn mock_parse_toml(
        &self,
        toml_data: &str,
    ) -> Result<HashMap<String, String>, BearDogError> {
        // Simple key=value parser for testing
        let mut config = HashMap::new();

        for line in toml_data.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value = line[eq_pos + 1..].trim().trim_matches('"').to_string();
                config.insert(key, value);
            }
        }

        Ok(config)
    }

    /// [`Self::mock_parse_toml`] plus injected defaults for `default_value` and `timeout`.
    pub fn mock_parse_config_with_defaults(
        &self,
        config_data: &str,
    ) -> Result<HashMap<String, String>, BearDogError> {
        let mut config = self.mock_parse_toml(config_data)?;

        // Add defaults for missing keys
        config
            .entry("default_value".to_string())
            .or_insert_with(|| "default".to_string());
        config
            .entry("timeout".to_string())
            .or_insert_with(|| "30".to_string());

        Ok(config)
    }

    /// Deterministic pseudo-random bytes from time + `length` (not suitable for crypto).
    pub fn generate_random_bytes(&self, length: usize) -> Result<Vec<u8>, BearDogError> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut bytes = Vec::with_capacity(length);
        let mut hasher = DefaultHasher::new();

        // Use current timestamp and length as seed
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;

        seed.hash(&mut hasher);
        length.hash(&mut hasher);

        let mut current_hash = hasher.finish();

        for _ in 0..length {
            bytes.push((current_hash & 0xFF) as u8);
            current_hash = current_hash
                .wrapping_mul(1_103_515_245)
                .wrapping_add(12_345);
        }

        Ok(bytes)
    }

    /// Rotates through benign, XSS, SQLi, path-traversal, large, and empty API payloads.
    pub fn generate_api_input_data(&self, index: usize) -> Result<Vec<u8>, BearDogError> {
        let test_inputs = [
            b"valid_input".to_vec(),
            b"<script>alert('xss')</script>".to_vec(),
            b"'; DROP TABLE users; --".to_vec(),
            b"../../../etc/passwd".to_vec(),
            vec![0u8; 1000], // Large input
            vec![],          // Empty input
        ];

        Ok(test_inputs[index % test_inputs.len()].clone())
    }

    /// Rotates through valid, multi-key, broken, comment-only, and empty config snippets.
    pub fn generate_config_data(&self, index: usize) -> Result<Vec<u8>, BearDogError> {
        const TEST_PORT: u16 = 8080;
        let test_configs = [
            b"key = \"value\"".to_vec(),
            format!("port = {TEST_PORT}\nhost = \"localhost\"").into_bytes(),
            b"invalid_toml = [unclosed".to_vec(),
            b"# Comment only".to_vec(),
            b"".to_vec(), // Empty config
        ];

        Ok(test_configs[index % test_configs.len()].clone())
    }
}
