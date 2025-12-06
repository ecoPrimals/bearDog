//! Software HSM cryptography implementation

use beardog_errors::BearDogError;
use rand::RngCore;
use sha2::{Digest, Sha256};

/// Software cryptography provider
#[derive(Debug, Clone, Default)]
pub struct SoftwareCryptoProvider;

impl SoftwareCryptoProvider {
    /// Create new software crypto provider
    pub fn new() -> Self {
        Self
    }

    /// Generate cryptographically secure random bytes
    ///
    /// Uses the system's cryptographically secure random number generator.
    ///
    /// # Errors
    /// Returns an error if the RNG fails to generate random bytes
    pub fn generate_random(&self, len: usize) -> Result<Vec<u8>, BearDogError> {
        let mut bytes = vec![0u8; len];
        rand::thread_rng()
            .try_fill_bytes(&mut bytes)
            .map_err(|e| BearDogError::security(format!("Failed to generate random bytes: {e}"), e.into()))?;
        Ok(bytes)
    }

    /// Hash data using SHA-256
    ///
    /// Computes the SHA-256 hash of the input data.
    ///
    /// # Errors
    /// This function is infallible but returns Result for API consistency
    pub fn hash_sha256(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        Ok(hasher.finalize().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_provider_creation() {
        let provider = SoftwareCryptoProvider::new();
        assert!(provider.generate_random(32).is_ok());
    }

    #[test]
    fn test_generate_random() {
        let provider = SoftwareCryptoProvider::new();
        
        // Generate random bytes
        let random1 = provider.generate_random(32)?;
        let random2 = provider.generate_random(32)?;
        
        // Should be correct length
        assert_eq!(random1.len(), 32);
        assert_eq!(random2.len(), 32);
        
        // Should be different (extremely high probability)
        assert_ne!(random1, random2);
        
        // Should not be all zeros
        assert_ne!(random1, vec![0u8; 32]);
    }

    #[test]
    fn test_hash_sha256() {
        let provider = SoftwareCryptoProvider::new();
        
        // Test known vector
        let data = b"hello world";
        let hash = provider.hash_sha256(data)?;
        
        // SHA-256 should produce 32 bytes
        assert_eq!(hash.len(), 32);
        
        // Same input should produce same hash
        let hash2 = provider.hash_sha256(data)?;
        assert_eq!(hash, hash2);
        
        // Different input should produce different hash
        let different_hash = provider.hash_sha256(b"goodbye world")?;
        assert_ne!(hash, different_hash);
    }
    
    #[test]
    fn test_empty_random_generation() {
        let provider = SoftwareCryptoProvider::new();
        let empty = provider.generate_random(0)?;
        assert_eq!(empty.len(), 0);
    }
    
    #[test]
    fn test_empty_data_hash() {
        let provider = SoftwareCryptoProvider::new();
        let hash = provider.hash_sha256(b"")?;
        assert_eq!(hash.len(), 32);
        
        // SHA-256 of empty string is a known value
        // e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        let expected = vec![
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14,
            0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9, 0x24,
            0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c,
            0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55,
        ];
        assert_eq!(hash, expected);
    }
}
