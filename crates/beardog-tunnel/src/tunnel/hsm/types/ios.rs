// SPDX-License-Identifier: AGPL-3.0-or-later

//! iOS-specific HSM types (Secure Enclave, Keychain).

/// iOS HSM configuration
#[derive(Debug, Clone)]
pub struct IOSHsmConfig {
    /// Use Secure Enclave
    pub secure_enclave_enabled: bool,
    /// Require biometric authentication
    pub biometric_authentication: bool,
    /// Enable key attestation
    pub key_attestation: bool,
}

impl Default for IOSHsmConfig {
    fn default() -> Self {
        Self {
            secure_enclave_enabled: true,
            biometric_authentication: false,
            key_attestation: true,
        }
    }
}

/// iOS device capabilities
#[derive(Debug, Clone)]
pub struct IOSDeviceCapabilities {
    /// Secure Enclave available
    pub secure_enclave_available: bool,
    /// Biometric ID available (Face ID/Touch ID)
    pub biometric_id_available: bool,
    /// Hardware security module available
    pub hardware_security_module: bool,
}
