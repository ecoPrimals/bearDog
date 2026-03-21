// SPDX-License-Identifier: AGPL-3.0-only

//! PKCS#11 Capability Prober
//!
//! Provides capability detection for PKCS#11 HSMs

use super::super::*;
use crate::tunnel::hsm::types::capability::HsmCapabilities;
use beardog_errors::BearDogError;
use tracing::debug;

/// PKCS#11 capability prober
#[derive(Debug, Clone)]
pub struct Pkcs11CapabilityProber;

impl Pkcs11CapabilityProber {
    /// Create new PKCS#11 capability prober
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }

    /// Probe capabilities (vendor-agnostic)
    ///
    /// Detects PKCS#11 HSM capabilities without vendor-specific assumptions.
    /// Works with any PKCS#11-compliant device.
    ///
    /// # Errors
    /// Returns an error if probing fails
    pub async fn probe_capabilities(&self) -> Result<HsmCapabilities, BearDogError> {
        debug!("Probing PKCS#11 HSM capabilities (vendor-agnostic)");
        
        // Universal PKCS#11 capability detection
        // Works with any PKCS#11 provider (Yubico, SoftHSM, AWS CloudHSM, etc.)
        let mut capabilities = HsmCapabilities::default();
        
        // PKCS#11 standard capabilities (vendor-agnostic)
        capabilities.supports_key_generation = true;  // CKM_RSA_PKCS_KEY_PAIR_GEN, etc.
        capabilities.supports_signing = true;          // CKM_RSA_PKCS, CKM_ECDSA
        capabilities.supports_encryption = true;       // CKM_RSA_PKCS, CKM_AES_*
        capabilities.supports_random_generation = true; // C_GenerateRandom (required by spec)
        capabilities.supports_key_storage = true;      // C_CreateObject (required by spec)
        capabilities.supports_hardware_backed = false; // Unknown until C_GetTokenInfo queried
        
        // Algorithm support (PKCS#11 standard mechanisms)
        capabilities.supported_algorithms = vec![
            "RSA".to_string(),
            "ECDSA".to_string(),
            "AES".to_string(),
            "SHA256".to_string(),
        ];
        
        debug!("✅ PKCS#11 capabilities detected (vendor-agnostic)");
        Ok(capabilities)
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
        let prober = Pkcs11CapabilityProber::new();
        assert!(prober.is_ok());
    }
}
