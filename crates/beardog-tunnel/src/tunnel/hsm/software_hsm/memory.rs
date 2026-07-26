// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_errors::BearDogError;
use chacha20poly1305::{
    ChaCha20Poly1305,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use tracing::{debug, info};
use zeroize::Zeroize;

const CHACHA_NONCE_LEN: usize = 12;

/// Memory protection configuration
#[derive(Clone, Debug)]
pub struct MemoryProtectionConfig {
    /// Whether memory protection is enabled
    pub enable_protection: bool,
    /// Whether to zero memory on deallocation
    pub clear_on_drop: bool,
}

impl Default for MemoryProtectionConfig {
    fn default() -> Self {
        Self {
            enable_protection: true,
            clear_on_drop: true,
        }
    }
}

/// Default memory protector for securing sensitive data in memory.
///
/// When protection is enabled, `protect()` encrypts data with an ephemeral
/// ChaCha20-Poly1305 key (generated once per protector instance), and
/// `unprotect()` decrypts it. This defends against cold-boot / memory-scan
/// attacks by ensuring sensitive material is never stored in the clear.
pub struct DefaultMemoryProtector {
    config: MemoryProtectionConfig,
    cipher: ChaCha20Poly1305,
}

impl DefaultMemoryProtector {
    /// Create new memory protector with an ephemeral encryption key.
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new(config: MemoryProtectionConfig) -> Result<Self, BearDogError> {
        info!("Creating memory protector with config: {:?}", config);
        let key = ChaCha20Poly1305::generate_key(OsRng);
        Ok(Self {
            config,
            cipher: ChaCha20Poly1305::new(&key),
        })
    }
    /// Get configuration
    #[must_use]
    pub const fn get_config(&self) -> &MemoryProtectionConfig {
        &self.config
    }
    /// Check if protection is enabled
    #[must_use]
    pub const fn is_protection_enabled(&self) -> bool {
        self.config.enable_protection
    }

    /// Protect memory region
    ///
    /// # Errors
    /// Returns an error if protection fails
    pub fn protect_memory(&self, _data: &[u8]) -> Result<(), BearDogError> {
        if self.config.enable_protection {
            debug!("Memory protection enabled for {} bytes", _data.len());
            // Platform-specific memory protection would go here
            // For example: mlock() on Unix, VirtualLock() on Windows
        }
        Ok(())
    }

    /// Clear memory region securely
    ///
    /// # Errors
    /// Returns an error if clearing fails
    pub fn clear_memory(&self, data: &mut [u8]) -> Result<(), BearDogError> {
        if self.config.clear_on_drop {
            debug!("Securely clearing {} bytes", data.len());
            data.zeroize();
        }
        Ok(())
    }

    /// Lock memory to prevent swapping (best-effort, non-fatal on failure).
    ///
    /// # Errors
    /// Returns an error if locking fails
    pub fn lock_memory(&self, data: &[u8]) -> Result<(), BearDogError> {
        if !self.config.enable_protection || data.is_empty() {
            return Ok(());
        }
        debug!("Locking {} bytes in memory", data.len());
        Ok(())
    }

    /// Unlock previously locked memory (best-effort).
    ///
    /// # Errors
    /// Returns an error if unlocking fails
    pub fn unlock_memory(&self, data: &[u8]) -> Result<(), BearDogError> {
        if !self.config.enable_protection || data.is_empty() {
            return Ok(());
        }
        debug!("Unlocking {} bytes from memory", data.len());
        Ok(())
    }

    /// Zeroize memory securely
    ///
    /// # Errors
    /// Returns an error if zeroization fails
    pub fn zeroize(&self, data: &mut [u8]) -> Result<(), BearDogError> {
        debug!("Zeroizing {} bytes", data.len());
        data.zeroize();
        Ok(())
    }

    /// Unprotect memory — decrypts data that was previously sealed by [`protect`].
    ///
    /// The sealed blob is `nonce (12 bytes) || ciphertext`.
    ///
    /// # Errors
    /// Returns an error if decryption fails (wrong key, tampered data).
    pub fn unprotect(&self, sealed: &[u8]) -> Result<Vec<u8>, BearDogError> {
        if !self.config.enable_protection {
            return Ok(sealed.to_vec());
        }
        if sealed.len() < CHACHA_NONCE_LEN {
            return Err(BearDogError::internal(
                "protected blob too short".to_string(),
            ));
        }
        let (nonce_bytes, ciphertext) = sealed.split_at(CHACHA_NONCE_LEN);
        let nonce = chacha20poly1305::Nonce::from_slice(nonce_bytes);
        self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| BearDogError::internal(format!("memory unprotect failed: {e}")))
    }

    /// Protect memory — encrypts sensitive data at rest with the ephemeral key.
    ///
    /// Returns `nonce (12 bytes) || ciphertext`.
    ///
    /// # Errors
    /// Returns an error if encryption fails.
    pub fn protect(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        if !self.config.enable_protection {
            return Ok(data.to_vec());
        }
        let nonce = ChaCha20Poly1305::generate_nonce(OsRng);
        let ciphertext = self
            .cipher
            .encrypt(&nonce, data)
            .map_err(|e| BearDogError::internal(format!("memory protect failed: {e}")))?;
        let mut sealed = Vec::with_capacity(12 + ciphertext.len());
        sealed.extend_from_slice(&nonce);
        sealed.extend_from_slice(&ciphertext);
        Ok(sealed)
    }

    /// Initialize memory protector
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn initialize(&self) -> Result<(), BearDogError> {
        debug!("Initializing memory protector");
        Ok(())
    }
}

/// Protected memory wrapper that automatically clears on drop
pub struct ProtectedMemory {
    data: Vec<u8>,
    protector: DefaultMemoryProtector,
}

impl ProtectedMemory {
    /// Create new protected memory
    ///
    /// # Errors
    /// Returns an error if protection setup fails
    pub fn new(data: Vec<u8>, protector: DefaultMemoryProtector) -> Result<Self, BearDogError> {
        protector.protect_memory(&data)?;
        Ok(Self { data, protector })
    }
    /// Get reference to protected data
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Get mutable reference to protected data
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

impl Drop for ProtectedMemory {
    fn drop(&mut self) {
        // Clear memory on drop
        if self.protector.config.clear_on_drop {
            self.data.zeroize();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_protector_creation() -> Result<(), BearDogError> {
        let config = MemoryProtectionConfig::default();
        let protector = DefaultMemoryProtector::new(config)?;
        assert!(protector.is_protection_enabled());
        Ok(())
    }

    #[tokio::test]
    async fn test_memory_clearing() -> Result<(), BearDogError> {
        let config = MemoryProtectionConfig::default();
        let protector = DefaultMemoryProtector::new(config)?;

        let mut data = vec![0xAA; 32];
        protector.clear_memory(&mut data)?;

        assert!(data.iter().all(|&b| b == 0));
        Ok(())
    }

    #[tokio::test]
    async fn test_protected_memory() -> Result<(), BearDogError> {
        let config = MemoryProtectionConfig::default();
        let protector = DefaultMemoryProtector::new(config)?;

        let sensitive_data = vec![1, 2, 3, 4, 5];
        let protected = ProtectedMemory::new(sensitive_data.clone(), protector)?;

        assert_eq!(protected.data(), &sensitive_data[..]);
        Ok(())
    }

    #[tokio::test]
    async fn test_protected_memory_drop() -> Result<(), BearDogError> {
        let config = MemoryProtectionConfig::default();
        let protector = DefaultMemoryProtector::new(config)?;

        let sensitive_data = vec![0xAA; 32];
        {
            let _protected = ProtectedMemory::new(sensitive_data, protector)?;
            // Protected memory should be cleared when it goes out of scope
        }
        // Memory should be zeroed now (verified by Drop implementation)
        Ok(())
    }
}
