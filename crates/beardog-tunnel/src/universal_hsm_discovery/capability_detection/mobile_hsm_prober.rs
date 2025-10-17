//! Mobile HSM Capability Prober
//!
//! Provides capability detection for mobile platform HSMs (iOS Secure Enclave, Android StrongBox)

use super::super::*;
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
        
        // TODO: Implement actual mobile HSM capability detection
        Ok(HsmCapabilities::default())
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
        
        // TODO: Implement Android StrongBox capability detection
        Ok(HsmCapabilities::default())
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
        
        // TODO: Implement iOS Secure Enclave capability detection
        Ok(HsmCapabilities::default())
    }
}

// NOTE: Default implementation removed - use Type::new() instead since it returns Result
// Previous unsafe implementation used .expect() which could panic
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
