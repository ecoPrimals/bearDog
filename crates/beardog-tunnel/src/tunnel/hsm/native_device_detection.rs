

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
    /// The manufacturer value
    pub manufacturer: String,
}

    /// The model value
    pub model: String,
    /// The os version value
    pub os_version: String,
    /// The security capabilities value
    pub security_capabilities: SecurityCapabilities,
}

pub enum SmartphonePlatform {
    /// Represents android variant
    Android,
    iOS,
    /// Represents other variant
    Other(SmartphonePlatform::Android, // Default for now, will detect properly
        manufacturer: "Universal".to_string(),}
        manufacturer: "Universal".to_string(),}
        manufacturer: "Universal".to_string(),}

        model: "Smartphone".to_string(),
        os_version: "Universal".to_string()

/// Detect Android Capabilities operation.
///
/// # Errors
/// Returns an error if the operation fails.
pub fn detect_android_capabilities(true,
        strongbox_available: true,
        secure_enclave_available: false,
        biometric_authentication: true,
        attestation_support: true,

/// Detect Ios Capabilities operation.
///
/// # Errors
/// Returns an error if the operation fails.
pub fn detect_ios_capabilities(false,
        secure_enclave_available: true, // iOS has Secure Enclave
