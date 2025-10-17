//! Software HSM cryptography implementation

use beardog_errors::BearDogError;

/// Software cryptography provider
#[derive(Debug, Clone, Default)]
pub struct SoftwareCryptoProvider;

impl SoftwareCryptoProvider {
    /// Create new software crypto provider
    pub fn new() -> Self {
        Self
    }

    /// Generate random bytes
    ///
    /// Stub implementation for cryptographic random number generation
    pub fn generate_random(&self, len: usize) -> Result<Vec<u8>, BearDogError> {
        // TODO: Use proper cryptographic RNG
        Ok(vec![0; len])
    }

    /// Hash data using SHA-256
    ///
    /// Stub implementation for hashing
    pub fn hash_sha256(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        // TODO: Implement actual SHA-256 hashing
        Ok(vec![0; 32])
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
}
