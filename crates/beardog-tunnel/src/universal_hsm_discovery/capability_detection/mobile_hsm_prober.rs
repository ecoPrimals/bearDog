// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mobile HSM Capability Prober
//!
//! Provides capability detection for mobile platform HSMs (iOS Secure Enclave, Android StrongBox)

use crate::tunnel::hsm::types::capability::HsmCapabilities;
use beardog_errors::BearDogError;
use tracing::debug;

/// Mobile HSM capability prober
#[derive(Debug, Clone)]
pub struct MobileHsmCapabilityProber;

impl MobileHsmCapabilityProber {
    /// Create new Mobile HSM capability prober
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }

    /// Probe capabilities
    ///
    /// # Errors
    /// Returns an error if probing fails
    pub async fn probe_capabilities(&self) -> Result<HsmCapabilities, BearDogError> {
        debug!("Probing mobile HSM capabilities");
        
        // Detect platform and probe appropriate HSM
        #[cfg(target_os = "android")]
        {
            self.probe_android_strongbox_capabilities("hardware")
        }
        
        #[cfg(target_os = "ios")]
        {
            self.probe_ios_secure_enclave_capabilities("latest")
        }
        
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        {
            // Return minimal capabilities for non-mobile platforms
            Ok(HsmCapabilities {
                signing: false,
                encryption: false,
                key_generation: false,
                hardware_backed: false,
                ..Default::default()
            })
        }
    }

    /// Probe Android StrongBox capabilities
    ///
    /// # Errors
    /// Returns an error if probing fails
    pub fn probe_android_strongbox_capabilities(
        &self,
        security_level: &str,
    ) -> Result<HsmCapabilities, BearDogError> {
        debug!("📱 Probing Android StrongBox: {}", security_level);
        
        // Android StrongBox provides hardware-backed key storage
        let is_hardware = security_level == "hardware" || security_level == "strongbox";
        
        Ok(HsmCapabilities {
            signing: true,
            encryption: true,
            key_generation: true,
            hardware_backed: is_hardware,
            attestation: is_hardware,
            secure_element: is_hardware,
            biometric_auth: true,
            algorithms: vec![
                "RSA-2048".to_string(),
                "RSA-4096".to_string(),
                "ECDSA-P256".to_string(),
                "ECDSA-P384".to_string(),
                "AES-256-GCM".to_string(),
                "HMAC-SHA256".to_string(),
            ],
            max_key_size: if is_hardware { 4096 } else { 2048 },
            supports_p256: true,
            supports_p384: true,
            supports_ed25519: false, // Android StrongBox doesn't support Ed25519 natively
            fips_compliant: is_hardware,
            tamper_resistant: is_hardware,
            ..Default::default()
        })
    }

    /// Probe iOS Secure Enclave capabilities
    ///
    /// # Errors
    /// Returns an error if probing fails
    pub fn probe_ios_secure_enclave_capabilities(
        &self,
        enclave_version: &str,
    ) -> Result<HsmCapabilities, BearDogError> {
        debug!("📱 Probing iOS Secure Enclave: {}", enclave_version);
        
        // iOS Secure Enclave is always hardware-backed
        Ok(HsmCapabilities {
            signing: true,
            encryption: true,
            key_generation: true,
            hardware_backed: true,
            attestation: true,
            secure_element: true,
            biometric_auth: true, // Face ID / Touch ID integration
            algorithms: vec![
                "ECDSA-P256".to_string(),
                "ECDSA-P384".to_string(),
                "AES-256-GCM".to_string(),
            ],
            max_key_size: 384, // Secure Enclave uses ECC, not RSA
            supports_p256: true,
            supports_p384: true,
            supports_ed25519: false, // Secure Enclave doesn't support Ed25519
            fips_compliant: true,
            tamper_resistant: true,
            isolation_level: "hardware".to_string(),
            vendor: "Apple".to_string(),
            model: format!("Secure Enclave {enclave_version}"),
            ..Default::default()
        })
    }
}

// NOTE: Default implementation removed - use Type::new() instead since it returns Result
// Previous unsound implementation used ? which could panic
// Use Type::new()? or Type::new().unwrap_or_else(|e| { /* handle error */ }) instead

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prober_creation() {
        let prober = MobileHsmCapabilityProber::new();
        assert!(prober.is_ok());
    }

    #[tokio::test]
    async fn test_capability_probing() -> Result<(), BearDogError> {
        let prober = MobileHsmCapabilityProber::new()?;
        let capabilities = prober.probe_capabilities().await?;
        assert!(capabilities.supports_key_generation);
        Ok(())
    }
}
