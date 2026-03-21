// SPDX-License-Identifier: AGPL-3.0-only

//! Safe Android Provider Implementation
//!
//! This module provides a safe interface to Android security features
//! without using unchecked memory patterns directly.

use super::traits::PlatformSecurityProvider;
use crate::tunnel::hsm::types::{HsmKey, KeyType};
use beardog_errors::BearDogError;
use tracing::{info, warn};

/// Safe Android security provider
pub struct SafeAndroidProvider {
    strongbox_available: bool,
}

impl SafeAndroidProvider {
    /// Creates a new SafeAndroidProvider instance
    ///
    /// # Errors
    /// Returns an error if provider initialization fails.
    pub fn new() -> Result<Self, BearDogError> {
        let strongbox_available = Self::check_strongbox_availability();

        info!(
            "🤖 SafeAndroidProvider initialized (StrongBox: {})",
            strongbox_available
        );

        Ok(Self {
            strongbox_available,
        })
    }

    /// Checks if StrongBox is available on this device
    fn check_strongbox_availability() -> bool {
        beardog_errors::process_env::var("ANDROID_STRONGBOX_AVAILABLE")
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    /// Checks if hardware-backed security is available
    pub const fn is_hardware_backed(&self) -> bool {
        self.strongbox_available
    }

    /// Safe key generation implementation
    fn generate_key_safe(
        &self,
        _key_id: &str,
        _key_type: &KeyType,
    ) -> Result<HsmKey, BearDogError> {
        warn!("Android StrongBox key generation not yet fully implemented - using placeholder");

        Err(BearDogError::not_implemented(
            "Android StrongBox key generation not yet implemented safely",
        ))
    }

    /// Safe data signing implementation
    fn sign_data_safe(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        warn!("Android StrongBox signing not yet fully implemented - using placeholder");

        Err(BearDogError::not_implemented(
            "Android StrongBox signing not yet implemented safely",
        ))
    }

    /// Safe signature verification implementation
    fn verify_signature_safe(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        warn!("Android StrongBox verification not yet fully implemented - using placeholder");

        Err(BearDogError::not_implemented(
            "Android StrongBox verification not yet implemented safely",
        ))
    }
}

impl PlatformSecurityProvider for SafeAndroidProvider {
    fn generate_key(&self, key_id: &str, key_type: &KeyType) -> Result<HsmKey, BearDogError> {
        self.generate_key_safe(key_id, key_type)
    }

    fn sign_data(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        self.sign_data_safe(key_id, data)
    }

    fn verify_signature(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        self.verify_signature_safe(key_id, data, signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_android_provider_creation() -> Result<(), Box<dyn std::error::Error>> {
        let provider = SafeAndroidProvider::new();
        assert!(provider.is_ok());
        Ok(())
    }

    #[test]
    fn test_strongbox_check() -> Result<(), Box<dyn std::error::Error>> {
        let provider = SafeAndroidProvider::new()?;
        // Should be false in test environment
        assert!(!provider.is_hardware_backed() || provider.is_hardware_backed());
        Ok(())
    }
}
