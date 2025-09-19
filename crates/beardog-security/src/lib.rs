// BearDog Security Crate
//
// Provides comprehensive security functionality including cryptography,
// key management, and hardware security module integration.

#[cfg(test)]
pub mod comprehensive_tests;
pub mod encryption;
pub mod memory_key_manager;
pub mod simd_crypto;

// Re-export main types and functions
pub use encryption::*;
pub use memory_key_manager::*;

use beardog_errors::BearDogError;
use rand::RngCore;
use sha2::{Digest, Sha256, Sha512};

/// Compute SHA-256 hash of input data
pub fn compute_sha256_hash(data: &[u8]) -> Result<Vec<u8>, BearDogError> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    Ok(hasher.finalize().to_vec())
}

/// Compute SHA-512 hash of input data
pub fn compute_sha512_hash(data: &[u8]) -> Result<Vec<u8>, BearDogError> {
    let mut hasher = Sha512::new();
    hasher.update(data);
    Ok(hasher.finalize().to_vec())
}

/// Generate secure random bytes
pub fn generate_secure_random_bytes(size: usize) -> Result<Vec<u8>, BearDogError> {
    let mut bytes = vec![0u8; size];
    rand::thread_rng().fill_bytes(&mut bytes);
    Ok(bytes)
}

/// Derive key from password using a simple PBKDF2-like approach
pub fn derive_key_from_password(
    password: &[u8],
    salt: &[u8],
    iterations: u32,
) -> Result<Vec<u8>, BearDogError> {
    // Simple key derivation (replace with proper PBKDF2 in production)
    let mut result = password.to_vec();
    for _ in 0..iterations {
        let mut hasher = Sha256::new();
        hasher.update(&result);
        hasher.update(salt);
        result = hasher.finalize().to_vec();
    }
    Ok(result)
}

/// Constant-time comparison of two byte arrays
pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

pub fn secure_zero_memory(data: &mut [u8]) {
    // Use volatile write to prevent optimization
    for byte in data {
        unsafe {
            std::ptr::write_volatile(byte, 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_functions() {
        let data = b"test";
        let hash = compute_sha256_hash(data).unwrap();
        assert_eq!(hash.len(), 32);
    }
}
