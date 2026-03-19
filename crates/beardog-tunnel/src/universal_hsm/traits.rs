// SPDX-License-Identifier: AGPL-3.0-only

//! Universal HSM Trait Definitions
//!
//! Core trait definitions for the universal HSM system.

use crate::tunnel::hsm::types::{HsmCapability, KeyType};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Universal HSM Provider trait
///
/// This trait defines the interface that all HSM providers must implement
pub trait UniversalHsmProvider: Send + Sync {
    /// Get provider information
    fn get_provider_info(&self) -> ProviderInfo;

    /// Generate a new key
    fn generate_key(&self, key_type: KeyType) -> Result<Vec<u8>, BearDogError>;

    /// Sign data with a key
    fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>;

    /// Verify a signature
    fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> Result<bool, BearDogError>;

    /// Get provider capabilities
    fn get_capabilities(&self) -> Vec<HsmCapability>;
}

/// Provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Provider name
    pub name: String,
    /// Provider type
    pub provider_type: ProviderType,
    /// Provider version
    pub version: String,
    /// Capabilities verified
    pub capabilities_verified: bool,
}

/// Provider type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderType {
    /// Mobile hardware security
    MobileHardware,
    /// Desktop hardware security
    DesktopHardware,
    /// Software HSM
    Software,
    /// PKCS#11 hardware token
    Pkcs11,
    /// TPM 2.0+
    Tpm,
    /// Cloud HSM
    Cloud,
    /// USB security token
    UsbToken,
    /// Network HSM
    NetworkHsm,
    /// Custom provider
    Custom,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderType::MobileHardware => write!(f, "Mobile Hardware Security"),
            ProviderType::DesktopHardware => write!(f, "Desktop Hardware Security"),
            ProviderType::Software => write!(f, "Software HSM"),
            ProviderType::Pkcs11 => write!(f, "PKCS#11 HSM"),
            ProviderType::Tpm => write!(f, "TPM 2.0+"),
            ProviderType::Cloud => write!(f, "Cloud HSM"),
            ProviderType::UsbToken => write!(f, "USB Security Token"),
            ProviderType::NetworkHsm => write!(f, "Network HSM"),
            ProviderType::Custom => write!(f, "Custom HSM"),
        }
    }
}

/// Platform enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    /// Mobile platform
    Mobile,
    /// Desktop platform
    Desktop,
    /// Server platform
    Server,
    /// Embedded platform
    Embedded,
    /// WebAssembly
    Wasm,
    /// Universal (all platforms)
    Universal,
}

impl Default for ProviderInfo {
    fn default() -> Self {
        Self {
            name: "Unknown Provider".to_string(),
            provider_type: ProviderType::Software,
            version: "0.0.0".to_string(),
            capabilities_verified: false,
        }
    }
}
