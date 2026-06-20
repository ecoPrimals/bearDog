// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_errors::BearDogError;

pub(super) fn attempt_encryption(data: &[u8], key: &[u8]) -> Result<Vec<u8>, BearDogError> {
    if key.is_empty() {
        return Err(BearDogError::Security {
            message: "Empty encryption key".to_string(),
            category: beardog_errors::SecurityErrorCategory::Encryption,
        });
    }
    if key.len() < 32 {
        return Err(BearDogError::Security {
            message: "Key too short".to_string(),
            category: beardog_errors::SecurityErrorCategory::Encryption,
        });
    }
    // Stub: simulate encryption
    let mut result = data.to_vec();
    for byte in &mut result {
        *byte ^= key[0]; // Simple XOR for testing
    }
    Ok(result)
}

pub(super) fn attempt_decryption(data: &[u8], key: &[u8]) -> Result<Vec<u8>, BearDogError> {
    if data.len() < 16 {
        return Err(BearDogError::Security {
            message: "Encrypted data too short".to_string(),
            category: beardog_errors::SecurityErrorCategory::Encryption,
        });
    }
    // Stub: detect if data looks corrupted (mostly 0xFF pattern is suspicious for 128-byte data)
    if data.len() == 128 {
        let mut ff_count = 0usize;
        for &b in data {
            if b == 0xFF {
                ff_count += 1;
            }
        }
        if ff_count >= 120 {
            // If 120+ of 128 bytes are 0xFF, likely corrupted
            return Err(BearDogError::Security {
                message: "Corrupted encrypted data detected".to_string(),
                category: beardog_errors::SecurityErrorCategory::Encryption,
            });
        }
    }
    // Stub: simulate decryption
    attempt_encryption(data, key)
}

pub(super) fn generate_test_key() -> Result<Vec<u8>, BearDogError> {
    use rand::Rng;
    let mut rng = rand::rng();
    let key: Vec<u8> = (0..32).map(|_| rng.random()).collect();
    Ok(key)
}

pub(super) fn delete_nonexistent_key(_key_id: &str) -> Result<(), BearDogError> {
    Err(BearDogError::Business {
        message: "Key not found".to_string(),
        category: beardog_errors::BusinessErrorCategory::Validation,
    })
}

pub(super) fn simulate_failed_key_rotation(_key: &[u8]) -> Result<Vec<u8>, BearDogError> {
    Err(BearDogError::System {
        message: "Key rotation failed".to_string(),
        category: beardog_errors::SystemErrorCategory::General,
    })
}

pub(super) fn access_key(_key_id: &str) -> Result<Vec<u8>, BearDogError> {
    Ok(vec![1u8; 32])
}

pub(super) fn compute_test_hash(data: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use sha2::{Digest, Sha256};
    let hash = Sha256::digest(data);
    Ok(hash.to_vec())
}

pub(super) fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

pub(super) fn secure_zero(data: &mut [u8]) {
    // Note: In real code, use zeroize crate or similar
    // This is a test stub that simulates secure zeroing
    for byte in data {
        *byte = 0;
    }
    // Prevent compiler from optimizing away the zeroing
    std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
}

pub(super) fn authenticate_user(_username: &str, password: &str) -> Result<(), BearDogError> {
    if password.is_empty() {
        return Err(BearDogError::Security {
            message: "Empty password".to_string(),
            category: beardog_errors::SecurityErrorCategory::Authentication,
        });
    }
    Ok(())
}

pub(super) fn verify_signature(data: &[u8], signature: &[u8]) -> Result<(), BearDogError> {
    if signature.is_empty() {
        return Err(BearDogError::Security {
            message: "Empty signature".to_string(),
            category: beardog_errors::SecurityErrorCategory::Encryption,
        });
    }
    if signature.len() != 64 {
        return Err(BearDogError::Security {
            message: "Invalid signature length".to_string(),
            category: beardog_errors::SecurityErrorCategory::Encryption,
        });
    }
    // Stub: would verify signature
    let _ = data; // Use the parameter
    Ok(())
}

pub(super) fn generate_test_signature(_data: &[u8]) -> Result<Vec<u8>, BearDogError> {
    Ok(vec![0u8; 64])
}

pub(super) fn verify_signature_with_data(
    _data: &[u8],
    _signature: &[u8],
    expected_data: &[u8],
) -> Result<(), BearDogError> {
    // Simulate that signature was for different data
    if _data != expected_data {
        return Err(BearDogError::Security {
            message: "Signature verification failed".to_string(),
            category: beardog_errors::SecurityErrorCategory::Encryption,
        });
    }
    Ok(())
}

pub(super) fn generate_random_bytes(size: usize) -> Result<Vec<u8>, BearDogError> {
    use rand::Rng;
    let mut rng = rand::rng();
    let bytes: Vec<u8> = (0..size).map(|_| rng.random()).collect();
    Ok(bytes)
}

pub(super) fn create_config_with_key_size(_size: usize) -> TestConfig {
    TestConfig {
        key_size: _size,
        timeout: 60,
        max_attempts: 3,
    }
}

pub(super) fn create_config_with_timeout(_timeout: u64) -> TestConfig {
    TestConfig {
        key_size: 32,
        timeout: _timeout,
        max_attempts: 3,
    }
}

pub(super) fn create_config_with_max_attempts(_attempts: i32) -> TestConfig {
    TestConfig {
        key_size: 32,
        timeout: 60,
        max_attempts: _attempts,
    }
}

pub(super) fn validate_config(config: &TestConfig) -> Result<(), BearDogError> {
    if config.key_size < 16 || config.key_size > 64 {
        return Err(BearDogError::Configuration {
            message: "Invalid key size".to_string(),
            category: beardog_errors::ConfigurationErrorCategory::Validation,
        });
    }
    if config.timeout == 0 {
        return Err(BearDogError::Configuration {
            message: "Timeout cannot be zero".to_string(),
            category: beardog_errors::ConfigurationErrorCategory::Validation,
        });
    }
    if config.max_attempts < 0 {
        return Err(BearDogError::Configuration {
            message: "Max attempts cannot be negative".to_string(),
            category: beardog_errors::ConfigurationErrorCategory::Validation,
        });
    }
    Ok(())
}

pub(super) fn simulate_key_gen_failure() -> Result<Vec<u8>, BearDogError> {
    Err(BearDogError::System {
        message: "Key generation failed".to_string(),
        category: beardog_errors::SystemErrorCategory::General,
    })
}

#[derive(Debug)]
pub(super) struct TestConfig {
    pub key_size: usize,
    pub timeout: u64,
    pub max_attempts: i32,
}
