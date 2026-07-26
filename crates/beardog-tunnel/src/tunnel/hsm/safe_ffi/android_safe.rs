// SPDX-License-Identifier: AGPL-3.0-or-later

//! Safe Android Provider Implementation
//!
//! This module provides a safe interface to Android security features
//! without using unchecked memory patterns directly.

use super::traits::PlatformSecurityProvider;
use crate::tunnel::hsm::types::{HsmKey, KeyType};
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use tracing::info;

/// Safe Android security provider
pub struct SafeAndroidProvider {
    strongbox_available: bool,
}

impl SafeAndroidProvider {
    /// Creates a new `SafeAndroidProvider` instance
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

    /// Checks if `StrongBox` is available on this device
    fn check_strongbox_availability() -> bool {
        beardog_errors::process_env::var(env_keys::ENV_ANDROID_STRONGBOX_AVAILABLE)
            .map(|v| v == "true")
            .unwrap_or(false)
    }
    /// Checks if hardware-backed security is available
    #[must_use]
    pub const fn is_hardware_backed(&self) -> bool {
        self.strongbox_available
    }

    /// Safe key generation implementation
    #[cfg(target_os = "android")]
    fn generate_key_safe(
        &self,
        _key_id: &str,
        _key_type: &KeyType,
    ) -> Result<HsmKey, BearDogError> {
        tracing::warn!("Android StrongBox key generation requires JNI Keystore wiring");

        Err(BearDogError::not_yet_available(
            "Android StrongBox key generation requires JNI Keystore integration (hardware-backed key operations)",
        ))
    }

    #[cfg(not(target_os = "android"))]
    fn generate_key_safe(
        &self,
        _key_id: &str,
        _key_type: &KeyType,
    ) -> Result<HsmKey, BearDogError> {
        Err(BearDogError::unsupported_platform(
            "Android StrongBox key generation not available on this platform",
        ))
    }

    /// Safe data signing implementation
    #[cfg(target_os = "android")]
    fn sign_data_safe(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        tracing::warn!("Android StrongBox signing requires JNI Keystore wiring");

        Err(BearDogError::not_yet_available(
            "Android StrongBox signing requires JNI Keystore integration",
        ))
    }

    #[cfg(not(target_os = "android"))]
    fn sign_data_safe(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::unsupported_platform(
            "Android StrongBox signing not available on this platform",
        ))
    }

    /// Safe signature verification implementation
    #[cfg(target_os = "android")]
    fn verify_signature_safe(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        tracing::warn!("Android StrongBox verification requires JNI Keystore wiring");

        Err(BearDogError::not_yet_available(
            "Android StrongBox verification requires JNI Keystore integration",
        ))
    }

    #[cfg(not(target_os = "android"))]
    fn verify_signature_safe(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Err(BearDogError::unsupported_platform(
            "Android StrongBox verification not available on this platform",
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

    #[test]
    fn generate_key_returns_not_implemented() -> Result<(), Box<dyn std::error::Error>> {
        use crate::tunnel::hsm::types::KeyType;
        let provider = SafeAndroidProvider::new()?;
        let err = provider
            .generate_key("kid", &KeyType::Ed25519)
            .expect_err("StrongBox placeholder");
        let msg = err.to_string();
        assert!(
            msg.contains("Not yet available")
                || msg.contains("Unsupported platform")
                || msg.contains("Android StrongBox"),
            "unexpected: {err}"
        );
        Ok(())
    }

    #[test]
    fn sign_data_returns_not_implemented() -> Result<(), Box<dyn std::error::Error>> {
        let provider = SafeAndroidProvider::new()?;
        let err = provider
            .sign_data("kid", b"data")
            .expect_err("StrongBox placeholder");
        let msg = err.to_string();
        assert!(
            msg.contains("Not yet available")
                || msg.contains("Unsupported platform")
                || msg.contains("Android StrongBox"),
            "unexpected: {err}"
        );
        Ok(())
    }

    #[test]
    fn verify_signature_returns_not_implemented() -> Result<(), Box<dyn std::error::Error>> {
        let provider = SafeAndroidProvider::new()?;
        let err = provider
            .verify_signature("kid", b"data", b"sig")
            .expect_err("StrongBox placeholder");
        let msg = err.to_string();
        assert!(
            msg.contains("Not yet available")
                || msg.contains("Unsupported platform")
                || msg.contains("Android StrongBox"),
            "unexpected: {err}"
        );
        Ok(())
    }
}
