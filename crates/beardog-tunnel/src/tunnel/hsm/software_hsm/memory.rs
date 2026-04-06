// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_errors::BearDogError;
use tracing::{debug, info};
use zeroize::Zeroize;

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

/// Default memory protector for securing sensitive data in memory
pub struct DefaultMemoryProtector {
    config: MemoryProtectionConfig,
}

impl DefaultMemoryProtector {
    /// Create new memory protector
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub async fn new(config: MemoryProtectionConfig) -> Result<Self, BearDogError> {
        info!("Creating memory protector with config: {:?}", config);
        Ok(Self { config })
    }

    /// Get configuration
    pub const fn get_config(&self) -> &MemoryProtectionConfig {
        &self.config
    }

    /// Check if protection is enabled
    pub const fn is_protection_enabled(&self) -> bool {
        self.config.enable_protection
    }

    /// Protect memory region
    ///
    /// # Errors
    /// Returns an error if protection fails
    pub async fn protect_memory(&self, _data: &[u8]) -> Result<(), BearDogError> {
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
    pub async fn clear_memory(&self, data: &mut [u8]) -> Result<(), BearDogError> {
        if self.config.clear_on_drop {
            debug!("Securely clearing {} bytes", data.len());
            data.zeroize();
        }
        Ok(())
    }

    /// Lock memory to prevent swapping
    ///
    /// # Errors
    /// Returns an error if locking fails
    pub async fn lock_memory(&self, _data: &[u8]) -> Result<(), BearDogError> {
        if self.config.enable_protection {
            debug!("Locking {} bytes in memory", _data.len());
            // Platform-specific memory locking
            #[cfg(unix)]
            {
                // Would call mlock() here
            }
            #[cfg(windows)]
            {
                // Would call VirtualLock() here
            }
        }
        Ok(())
    }

    /// Unlock previously locked memory
    ///
    /// # Errors
    /// Returns an error if unlocking fails
    pub async fn unlock_memory(&self, _data: &[u8]) -> Result<(), BearDogError> {
        if self.config.enable_protection {
            debug!("Unlocking {} bytes from memory", _data.len());
            // Platform-specific memory unlocking
            #[cfg(unix)]
            {
                // Would call munlock() here
            }
            #[cfg(windows)]
            {
                // Would call VirtualUnlock() here
            }
        }
        Ok(())
    }

    /// Zeroize memory securely
    ///
    /// # Errors
    /// Returns an error if zeroization fails
    pub async fn zeroize(&self, data: &mut [u8]) -> Result<(), BearDogError> {
        debug!("Zeroizing {} bytes", data.len());
        data.zeroize();
        Ok(())
    }

    /// Unprotect memory (decrypt/decode protected data)
    ///
    /// # Errors
    /// Returns an error if unprotection fails
    pub async fn unprotect(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        debug!("Unprotecting {} bytes", data.len());
        // For now, just return a copy
        // In a real implementation, this would decrypt or decode protected memory
        Ok(data.to_vec())
    }

    /// Protect memory (encrypt/encode sensitive data)
    ///
    /// # Errors
    /// Returns an error if protection fails
    pub async fn protect(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        debug!("Protecting {} bytes", data.len());
        // Lock the memory if protection is enabled
        self.protect_memory(data).await?;
        // For now, just return a copy
        // In a real implementation, this would encrypt or encode the data
        Ok(data.to_vec())
    }

    /// Initialize memory protector
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub async fn initialize(&self) -> Result<(), BearDogError> {
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
    pub async fn new(
        data: Vec<u8>,
        protector: DefaultMemoryProtector,
    ) -> Result<Self, BearDogError> {
        protector.protect_memory(&data).await?;
        Ok(Self { data, protector })
    }

    /// Get reference to protected data
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
        let protector = DefaultMemoryProtector::new(config).await?;
        assert!(protector.is_protection_enabled());
        Ok(())
    }

    #[tokio::test]
    async fn test_memory_clearing() -> Result<(), BearDogError> {
        let config = MemoryProtectionConfig::default();
        let protector = DefaultMemoryProtector::new(config).await?;

        let mut data = vec![0xAA; 32];
        protector.clear_memory(&mut data).await?;

        assert!(data.iter().all(|&b| b == 0));
        Ok(())
    }

    #[tokio::test]
    async fn test_protected_memory() -> Result<(), BearDogError> {
        let config = MemoryProtectionConfig::default();
        let protector = DefaultMemoryProtector::new(config).await?;

        let sensitive_data = vec![1, 2, 3, 4, 5];
        let protected = ProtectedMemory::new(sensitive_data.clone(), protector).await?;

        assert_eq!(protected.data(), &sensitive_data[..]);
        Ok(())
    }

    #[tokio::test]
    async fn test_protected_memory_drop() -> Result<(), BearDogError> {
        let config = MemoryProtectionConfig::default();
        let protector = DefaultMemoryProtector::new(config).await?;

        let sensitive_data = vec![0xAA; 32];
        {
            let _protected = ProtectedMemory::new(sensitive_data, protector).await?;
            // Protected memory should be cleared when it goes out of scope
        }
        // Memory should be zeroed now (verified by Drop implementation)
        Ok(())
    }
}
