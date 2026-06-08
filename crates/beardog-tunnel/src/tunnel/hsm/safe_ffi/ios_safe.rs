// SPDX-License-Identifier: AGPL-3.0-or-later

//! Safe iOS Provider Implementation
//!
//! This module provides a safe interface to iOS Secure Enclave
//! without using unchecked memory patterns directly.

use super::traits::PlatformSecurityProvider;
use crate::tunnel::hsm::types::{HsmKey, KeyType};
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use tracing::info;

/// Safe iOS security provider
pub struct SafeIosProvider {
    capabilities: HashMap<String, bool>,
    secure_enclave_available: bool,
}

impl SafeIosProvider {
    /// # Errors
    ///
    /// Returns an error if key generation fails in the underlying HSM provider.
    /// Create a new safe iOS provider
    pub fn new() -> Result<Self, BearDogError> {
        info!("🍎 Initializing iOS Safe Provider");

        let secure_enclave_available = Self::check_secure_enclave_availability();
        info!("Secure Enclave availability: {}", secure_enclave_available);

        let mut capabilities = HashMap::new();
        capabilities.insert("secure_enclave".to_string(), secure_enclave_available);
        capabilities.insert("biometric_auth".to_string(), cfg!(target_os = "ios"));
        capabilities.insert("keychain_access".to_string(), cfg!(target_os = "ios"));

        Ok(Self {
            capabilities,
            secure_enclave_available,
        })
    }

    /// Checks if Secure Enclave is available on this device
    fn check_secure_enclave_availability() -> bool {
        beardog_errors::process_env::var(env_keys::ENV_IOS_SECURE_ENCLAVE_AVAILABLE)
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    /// Checks if hardware-backed security is available
    pub const fn is_hardware_backed(&self) -> bool {
        self.secure_enclave_available
    }

    /// Safe key generation implementation
    fn generate_key_safe(&self, key_id: &str, key_type: &KeyType) -> Result<HsmKey, BearDogError> {
        info!("🔐 Generating iOS Secure Enclave key: {}", key_id);

        // Check if Secure Enclave is available
        if !self.secure_enclave_available {
            return Err(BearDogError::security(
                "Secure Enclave not available on this device - use Software HSM instead"
                    .to_string(),
            ));
        }

        // iOS Secure Enclave supports ECC P-256 and P-384
        // For vendor-agnostic KeyType, we'll accept EllipticCurve type
        info!("iOS Secure Enclave: Generating key of type {:?}", key_type);

        #[cfg(target_os = "ios")]
        {
            // On real iOS device, use native Secure Enclave APIs
            self.generate_key_secure_enclave(key_id, key_type)
        }

        #[cfg(not(target_os = "ios"))]
        {
            self.generate_key_software_fallback(key_id, key_type)
        }
    }

    #[cfg(target_os = "ios")]
    fn generate_key_secure_enclave(
        &self,
        key_id: &str,
        _key_type: &KeyType,
    ) -> Result<HsmKey, BearDogError> {
        info!(
            "🍎 Using iOS Secure Enclave for key generation (not yet wired): {}",
            key_id
        );

        Err(BearDogError::not_yet_available(
            "iOS Secure Enclave integration requires Secure Enclave device binding and native crypto wiring",
        ))
    }

    #[cfg(not(target_os = "ios"))]
    fn generate_key_software_fallback(
        &self,
        _key_id: &str,
        _key_type: &KeyType,
    ) -> Result<HsmKey, BearDogError> {
        Err(BearDogError::unsupported_platform(
            "iOS Secure Enclave not available on this platform",
        ))
    }

    /// Safe data signing implementation
    fn sign_data_safe(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        info!("✍️ Signing data with iOS Secure Enclave key: {}", key_id);

        // Check if Secure Enclave is available
        if !self.secure_enclave_available {
            return Err(BearDogError::security(
                "Secure Enclave not available on this device".to_string(),
            ));
        }

        #[cfg(target_os = "ios")]
        {
            // On real iOS device, use native Secure Enclave signing
            self.sign_data_secure_enclave(key_id, data)
        }

        #[cfg(not(target_os = "ios"))]
        {
            self.sign_data_software_fallback(key_id, data)
        }
    }

    #[cfg(target_os = "ios")]
    fn sign_data_secure_enclave(
        &self,
        key_id: &str,
        _data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        info!(
            "🍎 Using iOS Secure Enclave for signing (not yet wired): {}",
            key_id
        );

        Err(BearDogError::not_yet_available(
            "iOS Secure Enclave integration requires Secure Enclave device binding and native crypto wiring",
        ))
    }

    #[cfg(not(target_os = "ios"))]
    fn sign_data_software_fallback(
        &self,
        _key_id: &str,
        _data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::unsupported_platform(
            "iOS Secure Enclave not available on this platform",
        ))
    }

    /// Safe signature verification implementation
    fn verify_signature_safe(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        info!(
            "🔍 Verifying signature with iOS Secure Enclave key: {}",
            key_id
        );

        #[cfg(target_os = "ios")]
        {
            // On real iOS device, use native Secure Enclave verification
            self.verify_signature_secure_enclave(key_id, data, signature)
        }

        #[cfg(not(target_os = "ios"))]
        {
            self.verify_signature_software_fallback(key_id, data, signature)
        }
    }

    #[cfg(target_os = "ios")]
    fn verify_signature_secure_enclave(
        &self,
        key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        info!(
            "🍎 Using iOS Secure Enclave for signature verification (not yet wired): {}",
            key_id
        );

        Err(BearDogError::not_yet_available(
            "iOS Secure Enclave integration requires Secure Enclave device binding and native crypto wiring",
        ))
    }

    #[cfg(not(target_os = "ios"))]
    fn verify_signature_software_fallback(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Err(BearDogError::unsupported_platform(
            "iOS Secure Enclave not available on this platform",
        ))
    }

    /// Get capabilities of this provider
    pub const fn capabilities(&self) -> &HashMap<String, bool> {
        &self.capabilities
    }
}

impl PlatformSecurityProvider for SafeIosProvider {
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

impl Default for SafeIosProvider {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            capabilities: HashMap::new(),
            secure_enclave_available: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_errors::{BearDogError, SystemErrorCategory};
    use serial_test::serial;

    fn assert_ios_secure_enclave_stub_err(err: BearDogError) {
        match err {
            BearDogError::System { category, message } => {
                assert!(
                    category == SystemErrorCategory::NotImplemented
                        || category == SystemErrorCategory::NotSupported,
                    "unexpected category: {category:?}"
                );
                assert!(
                    message.contains("iOS Secure Enclave"),
                    "unexpected message: {message}"
                );
            }
            other => panic!("expected System error, got {other:?}"),
        }
    }

    #[test]
    fn test_ios_provider_creation() {
        let provider = SafeIosProvider::new();
        assert!(provider.is_ok());
    }

    #[test]
    #[serial]
    fn test_secure_enclave_availability_check() {
        let provider = SafeIosProvider::new().expect("SafeIosProvider::new");
        assert!(!provider.is_hardware_backed() || cfg!(target_os = "ios"));
    }

    #[test]
    fn test_key_generation_requires_secure_enclave() {
        let provider = SafeIosProvider::new().expect("SafeIosProvider::new");

        // If Secure Enclave is not available, key generation should fail
        if !provider.is_hardware_backed() {
            let result = provider.generate_key("test_key", &KeyType::EllipticCurve);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_sign_requires_secure_enclave() {
        let provider = SafeIosProvider::new().expect("SafeIosProvider::new");
        let data = b"test data";

        // If Secure Enclave is not available, signing should fail
        if !provider.is_hardware_backed() {
            let result = provider.sign_data("test_key", data);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_capabilities() {
        let provider = SafeIosProvider::new().expect("SafeIosProvider::new");
        let capabilities = provider.capabilities();

        assert!(capabilities.contains_key("secure_enclave"));
        assert!(capabilities.contains_key("biometric_auth"));
        assert!(capabilities.contains_key("keychain_access"));
    }

    #[test]
    fn default_ios_provider_constructible() {
        let d = SafeIosProvider::default();
        assert!(
            d.capabilities().contains_key("secure_enclave") || d.capabilities().is_empty(),
            "default is either populated or empty fallback"
        );
    }

    #[test]
    #[serial]
    fn sign_data_errors_when_secure_enclave_unavailable() {
        let provider = SafeIosProvider::new().expect("SafeIosProvider::new");
        if provider.is_hardware_backed() {
            return;
        }
        let err = provider.sign_data("k", b"x").expect_err("no SE");
        assert!(err.to_string().contains("Secure Enclave") || err.to_string().contains("security"));
    }

    #[test]
    #[serial]
    fn ios_secure_enclave_env_true_enables_hardware_flag() {
        let prev =
            beardog_errors::process_env::var(env_keys::ENV_IOS_SECURE_ENCLAVE_AVAILABLE).ok();
        beardog_errors::process_env::set_var("IOS_SECURE_ENCLAVE_AVAILABLE", "true");
        let provider = SafeIosProvider::new().expect("SafeIosProvider::new with SE env");
        assert!(provider.is_hardware_backed());
        match prev {
            Some(v) => beardog_errors::process_env::set_var("IOS_SECURE_ENCLAVE_AVAILABLE", v),
            None => beardog_errors::process_env::remove_var("IOS_SECURE_ENCLAVE_AVAILABLE"),
        }
    }

    #[test]
    #[serial]
    fn generate_key_returns_not_implemented_when_se_env_true() {
        let prev =
            beardog_errors::process_env::var(env_keys::ENV_IOS_SECURE_ENCLAVE_AVAILABLE).ok();
        beardog_errors::process_env::set_var("IOS_SECURE_ENCLAVE_AVAILABLE", "true");
        let provider = SafeIosProvider::new().expect("provider");
        let err = provider
            .generate_key("k-fallback", &KeyType::EllipticCurve)
            .expect_err("keygen stub");
        assert_ios_secure_enclave_stub_err(err);
        match prev {
            Some(v) => beardog_errors::process_env::set_var("IOS_SECURE_ENCLAVE_AVAILABLE", v),
            None => beardog_errors::process_env::remove_var("IOS_SECURE_ENCLAVE_AVAILABLE"),
        }
    }

    #[test]
    #[serial]
    fn sign_and_verify_return_not_implemented_when_se_env_true() {
        let prev =
            beardog_errors::process_env::var(env_keys::ENV_IOS_SECURE_ENCLAVE_AVAILABLE).ok();
        beardog_errors::process_env::set_var("IOS_SECURE_ENCLAVE_AVAILABLE", "true");
        let provider = SafeIosProvider::new().expect("provider");
        let err = provider
            .sign_data("sk1", b"hello-ios-safe")
            .expect_err("sign stub");
        assert_ios_secure_enclave_stub_err(err);
        let err = provider
            .verify_signature("sk1", b"hello-ios-safe", &[0u8; 64])
            .expect_err("verify stub");
        assert_ios_secure_enclave_stub_err(err);
        match prev {
            Some(v) => beardog_errors::process_env::set_var("IOS_SECURE_ENCLAVE_AVAILABLE", v),
            None => beardog_errors::process_env::remove_var("IOS_SECURE_ENCLAVE_AVAILABLE"),
        }
    }

    #[test]
    #[serial]
    fn verify_signature_returns_not_implemented_for_short_signature_when_se_env_true() {
        let prev =
            beardog_errors::process_env::var(env_keys::ENV_IOS_SECURE_ENCLAVE_AVAILABLE).ok();
        beardog_errors::process_env::set_var("IOS_SECURE_ENCLAVE_AVAILABLE", "true");
        let provider = SafeIosProvider::new().expect("provider");
        let err = provider
            .verify_signature("vk", b"data", &[0u8; 8])
            .expect_err("verify stub");
        assert_ios_secure_enclave_stub_err(err);
        match prev {
            Some(v) => beardog_errors::process_env::set_var("IOS_SECURE_ENCLAVE_AVAILABLE", v),
            None => beardog_errors::process_env::remove_var("IOS_SECURE_ENCLAVE_AVAILABLE"),
        }
    }

    #[test]
    fn capabilities_returns_expected_keys() {
        let p = SafeIosProvider::new().expect("provider");
        let c = p.capabilities();
        assert_eq!(c.get("biometric_auth"), Some(&true));
        assert_eq!(c.get("keychain_access"), Some(&true));
    }

    #[test]
    fn two_providers_independent_hardware_flags() {
        let p1 = SafeIosProvider::new().expect("p1");
        let p2 = SafeIosProvider::new().expect("p2");
        assert_eq!(p1.is_hardware_backed(), p2.is_hardware_backed());
    }

    #[test]
    #[serial]
    fn env_false_explicit_disables_hardware() {
        let prev =
            beardog_errors::process_env::var(env_keys::ENV_IOS_SECURE_ENCLAVE_AVAILABLE).ok();
        beardog_errors::process_env::set_var("IOS_SECURE_ENCLAVE_AVAILABLE", "false");
        let provider = SafeIosProvider::new().expect("provider");
        assert!(!provider.is_hardware_backed());
        match prev {
            Some(v) => beardog_errors::process_env::set_var("IOS_SECURE_ENCLAVE_AVAILABLE", v),
            None => beardog_errors::process_env::remove_var("IOS_SECURE_ENCLAVE_AVAILABLE"),
        }
    }
}
